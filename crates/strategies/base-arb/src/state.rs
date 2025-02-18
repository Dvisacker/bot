use alloy::primitives::Address;
use alloy::providers::Provider;
use amms::amm::{AutomatedMarketMaker, AMM};
use amms::sync;
use dashmap::DashMap;
use eyre::{eyre, Result};
use shared::cycle::Cycle;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tracing::info;

/// State represents the core state management for arbitrage opportunities
///
/// Structure visualization:
/// ```text
/// State
/// ├── Provider (handles blockchain interaction)
/// ├── Pools (active trading pools)
/// │   ├── Pool 1
/// │   ├── Pool 2
/// │   └── Pool N
/// ├── Cycles (trading cycles)
/// │   ├── Cycle 1: Token A -> Token B -> Token C -> Token A
/// │   └── Cycle 2: Token X -> Token Y -> Token X
/// └── Inventory (tokens that are allowed to be traded)
/// ```
#[derive(Debug, Clone)]
pub struct State<P: Provider> {
    provider: Arc<P>,
    pub block_number: u64,
    pub pools: DashMap<Address, AMM>,
    pub pools_cycles_map: DashMap<Address, HashSet<String>>, // Maps pool addresses to cycles they're part of
    pub cycles: HashMap<String, Cycle>,                      // All valid trading cycles identified
    pub inventory: Vec<Address>,
}

impl<P: Provider> State<P> {
    /// Creates a new State instance with the given provider and inventory of tradeable tokens
    pub fn new(provider: Arc<P>, inventory: Vec<Address>) -> Self {
        Self {
            provider,
            inventory,
            block_number: 0,
            pools: DashMap::new(),
            pools_cycles_map: DashMap::new(),
            cycles: HashMap::new(),
        }
    }

    pub fn print_pools(&self) {
        for pool in self.pools.iter() {
            info!("Pool: {}", pool.key().to_string().to_lowercase());
        }
    }

    /// Recursively finds all possible trading cycles between token_in and token_out
    /// Parameters:
    /// - pairs: Available trading pairs
    /// - token_in: Starting token
    /// - token_out: Target token
    /// - max_hops: Maximum number of trades allowed
    /// - current_pairs: Accumulated trading pairs in current path
    /// - circles: Accumulated valid cycles found
    /// - seen: Set of visited pool addresses to prevent loops
    pub fn get_cycles(
        pairs: &[AMM],
        token_in: Address,
        token_out: Address,
        max_hops: i32,
        current_pairs: &Vec<AMM>,
        circles: &mut Vec<Cycle>,
        seen: &mut HashSet<Address>,
    ) -> Result<Vec<Cycle>> {
        let mut circles_copy: Vec<Cycle> = circles.clone();

        for pair in pairs {
            let address = pair.address();
            let tokens = pair.tokens();
            let [token_a, token_b] = tokens.as_slice() else {
                todo!()
            };
            if seen.contains(&address) {
                continue;
            }

            // Determine which token would be received from this pair
            let temp_out: Address;
            if token_in == *token_a {
                temp_out = *token_b;
            } else if token_in == *token_b {
                temp_out = *token_a;
            } else {
                continue;
            }

            let mut new_seen = seen.clone();
            new_seen.insert(address);

            // If we've found a path back to our target token, we've found a cycle
            if temp_out == token_out {
                let mut pools = current_pairs.clone();
                pools.push(pair.clone());
                let cycle = Cycle::new(pools);
                circles_copy.push(cycle);
            } else if max_hops > 1 {
                // Continue searching for longer paths if we haven't hit max_hops
                let mut new_pairs: Vec<AMM> = current_pairs.clone();
                new_pairs.push(pair.clone());
                circles_copy = Self::get_cycles(
                    pairs,
                    temp_out,
                    token_out,
                    max_hops - 1,
                    &new_pairs,
                    &mut circles_copy,
                    &mut new_seen,
                )?;
            }
        }

        Ok(circles_copy)
    }

    /// Updates the active pools list
    pub fn set_pools(&self, amms: Vec<AMM>) {
        for amm in amms {
            self.pools.insert(amm.address(), amm);
        }
    }

    /// Returns cycles that contain any of the provided AMMs
    pub fn get_updated_cycles(&self, amms: Vec<AMM>) -> Result<Vec<Cycle>> {
        let mut cycles = vec![];
        for amm in amms {
            let pool_address = amm.address();
            let pool_cycles = self.pools_cycles_map.get(&pool_address);
            if let Some(pool_cycles) = pool_cycles {
                for cycle_id in pool_cycles.iter() {
                    let cycle = self
                        .cycles
                        .get(cycle_id)
                        .ok_or_else(|| eyre!("Cycle not found with id: {}", cycle_id))?
                        .clone();
                    cycles.push(cycle);
                }
            }
        }

        Ok(cycles)
    }

    /// Updates all possible trading cycles in the system
    /// Returns cycles that meet the profit threshold (-0.50%)
    pub fn update_cycles(&mut self) -> Result<Vec<Cycle>> {
        let mut all_cycles = vec![];

        // Find cycles starting from each token in inventory
        for token in self.inventory.iter() {
            let pools = self
                .pools
                .iter()
                .map(|entry| entry.value().clone())
                .collect::<Vec<_>>();

            let cycles = Self::get_cycles(
                &pools,
                token.clone(),
                token.clone(),
                3, // Maximum of 3 hops in a cycle
                &vec![],
                &mut vec![],
                &mut HashSet::new(),
            )?;

            all_cycles.extend(cycles);
        }

        tracing::info!("Found {} potential cycles", all_cycles.len());

        // Filter cycles by profit threshold
        let profit_threshold = -0.50;

        // Update the pools_cycles_map with new cycles
        for cycle in all_cycles.clone() {
            let profit_perc = cycle.get_profit_perc();
            if profit_perc > profit_threshold {
                let id = cycle.id.clone();
                self.cycles.insert(id.clone(), cycle.clone());

                for pool in &cycle.amms {
                    let pool_address = pool.address();
                    let pool_cycles = self.pools_cycles_map.get_mut(&pool_address);
                    if let Some(mut c) = pool_cycles {
                        c.insert(id.clone());
                    } else {
                        self.pools_cycles_map
                            .insert(pool_address, HashSet::from([id.clone()]));
                    }
                }
            } else {
                tracing::info!("Cycle {} has no profit", cycle);
                let amms = cycle.amms.clone();
                for amm in amms {
                    let pool_address = amm.address();
                    let pool_cycles = self.pools_cycles_map.get_mut(&pool_address);
                    if let Some(mut c) = pool_cycles {
                        c.remove(&cycle.id);
                    }
                }
                self.cycles.remove(&cycle.id);
            }
        }

        let profitable_cycles: Vec<Cycle> =
            self.cycles.values().map(|cycle| cycle.clone()).collect();

        // info!("Found {} cycles", self.cycles.len());
        // info!("Nb cycles: {}", nb_cycles);
        Ok(profitable_cycles)
    }

    /// Synchronizes pool states with current blockchain state
    /// TODO: Very inefficient/inelegant, should be optimized
    pub async fn update_pools(&self) -> Result<()> {
        let mut univ3_pools = self
            .pools
            .iter()
            .map(|entry| entry.value().clone())
            .filter(|amm| matches!(amm, AMM::UniswapV3Pool(_)))
            .collect::<Vec<_>>();

        sync::populate_amms(
            &mut univ3_pools,
            self.block_number,
            self.provider.clone(),
            true,
        )
        .await?;

        for amm in univ3_pools.iter() {
            self.pools.insert(amm.address(), amm.clone());
        }

        // univ2 pools
        let mut univ2_pools = self
            .pools
            .iter()
            .map(|entry| entry.value().clone())
            .filter(|amm| matches!(amm, AMM::UniswapV2Pool(_)))
            .collect::<Vec<_>>();

        sync::populate_amms(
            &mut univ2_pools,
            self.block_number,
            self.provider.clone(),
            true,
        )
        .await?;

        for amm in univ2_pools.iter() {
            self.pools.insert(amm.address(), amm.clone());
        }

        Ok(())
    }

    /// Updates the current block number
    pub async fn update_block_number(&mut self, block_number: u64) -> Result<()> {
        self.block_number = block_number;
        Ok(())
    }

    /// Returns all active pools
    pub async fn get_all_pools(&self) -> Vec<AMM> {
        self.pools
            .iter()
            .map(|entry| entry.value().clone())
            .collect()
    }
}
