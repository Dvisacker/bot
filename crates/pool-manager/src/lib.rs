use alloy::network::Ethereum;
use alloy::primitives::Address;
use alloy::providers::Provider;
use alloy_chains::Chain;
use amms::amm::uniswap_v2::batch_request::get_v2_pool_data_batch_request;
use amms::amm::uniswap_v2::UniswapV2Pool;
use amms::amm::uniswap_v3::batch_request::get_v3_pool_data_batch_request;
use amms::amm::uniswap_v3::UniswapV3Pool;
use amms::amm::ve33::factory::Ve33Factory;
use amms::amm::AutomatedMarketMaker;
use amms::errors::AMMError;
use amms::{
    amm::{
        factory::Factory, uniswap_v2::factory::UniswapV2Factory,
        uniswap_v3::factory::UniswapV3Factory,
    },
    sync::{self},
};
use db::establish_connection;
use db::models::{NewDbPool, NewDbTag, NewDbUniV2Pool, NewDbUniV3Pool};
use db::queries::exchange::get_exchange_by_name;
use db::queries::tag::upsert_tag;
use db::queries::uni_v2_pool::batch_upsert_uni_v2_pools;
use db::queries::uni_v3_pool::batch_upsert_uni_v3_pools;
use shared::evm_helpers::get_contract_creation_block_n_ary;
use shared::pool_helpers::extract_v2_pools;
use std::sync::Arc;
use types::exchange::{ExchangeName, ExchangeType};

pub struct PoolStorageManager<P>
where
    P: Provider<Ethereum> + 'static,
{
    db_url: String,
    provider: Arc<P>,
}

impl<P> PoolStorageManager<P>
where
    P: Provider<Ethereum> + 'static,
{
    pub fn new(db_url: &str, provider: Arc<P>) -> Self {
        Self {
            db_url: db_url.to_string(),
            provider,
        }
    }

    pub async fn store_pools_from_factory(
        &self,
        chain: Chain,
        exchange_name: ExchangeName,
        factory_address: Address,
        from_block: Option<u64>,
        to_block: Option<u64>,
        step: u64,
        tag: Option<String>,
    ) -> Result<(), AMMError> {
        let mut conn = establish_connection(&self.db_url);
        let exchange = get_exchange_by_name(
            &mut conn,
            &chain.named().unwrap().to_string(),
            &exchange_name.to_string(),
        )
        .unwrap();

        let exchange_type =
            ExchangeType::from_str(&exchange.exchange_type).expect("Invalid exchange type");

        match exchange_type {
            ExchangeType::UniV2 => {
                self.store_univ2_pools_from_factory(chain, exchange_name, factory_address, tag)
                    .await
            }
            ExchangeType::UniV3 => {
                self.store_univ3_pools_from_factory(
                    chain,
                    exchange_name,
                    factory_address,
                    from_block,
                    to_block,
                    step,
                    tag,
                )
                .await
            }
            ExchangeType::Ve33 => {
                self.store_ve33_pools_from_factory(chain, exchange_name, factory_address, tag)
                    .await
            }
            _ => Err(AMMError::UnknownPoolType),
        }
    }

    pub async fn store_pools(
        &self,
        chain: Chain,
        exchange_name: ExchangeName,
        pool_addresses: Vec<Address>,
        tag: Option<String>,
    ) -> Result<(), AMMError> {
        let mut conn = establish_connection(&self.db_url);
        let exchange = get_exchange_by_name(
            &mut conn,
            &chain.named().unwrap().to_string(),
            &exchange_name.to_string(),
        )
        .unwrap();
        let exchange_type =
            ExchangeType::from_str(&exchange.exchange_type).expect("Invalid exchange type");

        match exchange_type {
            ExchangeType::UniV2 => {
                self.store_univ2_pools(chain, exchange_name, pool_addresses, tag)
                    .await
            }
            ExchangeType::UniV3 => {
                self.store_univ3_pools(chain, exchange_name, pool_addresses, tag)
                    .await
            }
            _ => Err(AMMError::UnknownPoolType),
        }
    }

    pub async fn store_univ3_pools(
        &self,
        chain: Chain,
        exchange_name: ExchangeName,
        pool_addresses: Vec<Address>,
        tag: Option<String>,
    ) -> Result<(), AMMError> {
        let mut conn = establish_connection(&self.db_url);
        let mut pools = pool_addresses
            .iter()
            .map(|pool_address| {
                let pool = UniswapV3Pool::new_empty(*pool_address, chain.named().unwrap()).unwrap();
                pool
            })
            .collect::<Vec<UniswapV3Pool>>();
        if let Some(ref tag) = tag {
            upsert_tag(&mut conn, &NewDbTag { name: tag.clone() }).unwrap();
        }

        for chunk in pools.chunks_mut(50) {
            get_v3_pool_data_batch_request(chunk, None, self.provider.clone()).await?;

            let new_pools = chunk
                .iter_mut()
                .map(|pool| {
                    pool.exchange_type = ExchangeType::UniV3;
                    pool.exchange_name = exchange_name;
                    pool.chain = chain.named().unwrap();
                    pool.to_new_db_pool(tag.clone())
                })
                .filter_map(|db_pool| {
                    if let NewDbPool::UniV3(v3_pool) = db_pool {
                        Some(v3_pool)
                    } else {
                        None
                    }
                })
                .collect::<Vec<NewDbUniV3Pool>>();

            batch_upsert_uni_v3_pools(&mut conn, &new_pools).unwrap();
            tracing::info!("Inserted {:?} pools", new_pools.len());
        }

        Ok(())
    }

    pub async fn store_univ2_pools(
        &self,
        chain: Chain,
        exchange_name: ExchangeName,
        pool_addresses: Vec<Address>,
        tag: Option<String>,
    ) -> Result<(), AMMError> {
        let mut conn = establish_connection(&self.db_url);
        let mut pools = pool_addresses
            .iter()
            .map(|pool_address| {
                let pool = UniswapV2Pool::new_empty(*pool_address, chain.named().unwrap()).unwrap();
                pool
            })
            .collect::<Vec<UniswapV2Pool>>();
        if let Some(ref tag) = tag {
            upsert_tag(&mut conn, &NewDbTag { name: tag.clone() }).unwrap();
        }

        for chunk in pools.chunks_mut(50) {
            get_v2_pool_data_batch_request(chunk, self.provider.clone()).await?;

            let new_pools = chunk
                .iter_mut()
                .map(|pool| {
                    pool.exchange_type = ExchangeType::UniV2;
                    pool.exchange_name = exchange_name;
                    pool.chain = chain.named().unwrap();
                    pool.to_new_db_pool(tag.clone())
                })
                .filter_map(|db_pool| {
                    if let NewDbPool::UniV2(v2_pool) = db_pool {
                        Some(v2_pool)
                    } else {
                        None
                    }
                })
                .collect::<Vec<NewDbUniV2Pool>>();

            batch_upsert_uni_v2_pools(&mut conn, &new_pools).unwrap();
            tracing::info!("Inserted {:?} pools", new_pools.len());
        }

        Ok(())
    }

    /// Stores Uniswap V3 pools in the database.
    ///
    /// This function fetches Uniswap V3 pools from logs within a specified block range,
    /// populates their data, and stores them in the database.
    pub async fn store_univ3_pools_from_factory(
        &self,
        chain: Chain,
        exchange_name: ExchangeName,
        factory_address: Address,
        from_block: Option<u64>,
        to_block: Option<u64>,
        step: u64,
        tag: Option<String>,
    ) -> Result<(), AMMError> {
        let mut conn = establish_connection(&self.db_url);

        let start_block = from_block.unwrap_or(0);
        let end_block = to_block.unwrap_or(self.provider.get_block_number().await.unwrap());

        if let Some(ref tag) = tag {
            upsert_tag(&mut conn, &NewDbTag { name: tag.clone() }).unwrap();
        }

        let contract_creation_block = get_contract_creation_block_n_ary(
            self.provider.clone(),
            factory_address,
            start_block,
            end_block,
            4,
        )
        .await
        .unwrap();

        let contract_creation_block = if contract_creation_block > start_block {
            contract_creation_block
        } else {
            start_block
        };

        let factory = UniswapV3Factory::new(factory_address, contract_creation_block);

        for block in (contract_creation_block..=end_block).step_by(step as usize) {
            let addresses = factory
                .get_pools_from_logs(block, block + step - 1, step, self.provider.clone())
                .await?
                .iter()
                .map(|pool| pool.address())
                .collect::<Vec<Address>>();

            self.store_univ3_pools(chain, exchange_name, addresses, tag.clone())
                .await?;
        }

        Ok(())
    }

    /// Stores Uniswap V2 pools in the database.
    pub async fn store_univ2_pools_from_factory(
        &self,
        chain: Chain,
        exchange_name: ExchangeName,
        factory_address: Address,
        tag: Option<String>,
    ) -> Result<(), AMMError> {
        let factory = Factory::UniswapV2Factory(UniswapV2Factory::new(factory_address, 0, 3000));

        // NOTE: The sync step seems redundant and can probably be removed
        let (amms, _) = sync::sync_amms(vec![factory], self.provider.clone(), None, 100000, false)
            .await
            .unwrap();
        let pools = extract_v2_pools(&amms);
        let addresses = pools
            .iter()
            .map(|pool| pool.address())
            .collect::<Vec<Address>>();

        self.store_univ2_pools(chain, exchange_name, addresses, tag.clone())
            .await?;

        Ok(())
    }

    pub async fn store_ve33_pools_from_factory(
        &self,
        chain: Chain,
        exchange_name: ExchangeName,
        factory_address: Address,
        tag: Option<String>,
    ) -> Result<(), AMMError> {
        let mut conn = establish_connection(&self.db_url);
        let factory = Factory::Ve33Factory(Ve33Factory::new(factory_address, 0, 3000));

        let (amms, _) = sync::sync_amms(vec![factory], self.provider.clone(), None, 100000, true)
            .await
            .unwrap();

        let mut pools = extract_v2_pools(&amms);

        for mut chunk in pools.chunks_mut(50) {
            // add additional data such as the exchange name
            get_v2_pool_data_batch_request(&mut chunk, self.provider.clone()).await?;
            let new_pools = chunk
                .iter_mut()
                .map(|pool| {
                    pool.exchange_type = ExchangeType::Ve33;
                    pool.exchange_name = exchange_name;
                    pool.chain = chain.named().unwrap();
                    pool.to_new_db_pool(tag.clone())
                })
                .filter_map(|db_pool| {
                    if let NewDbPool::UniV2(v2_pool) = db_pool {
                        Some(v2_pool)
                    } else {
                        None
                    }
                })
                .collect::<Vec<NewDbUniV2Pool>>();

            // upsert pools in the database
            batch_upsert_uni_v2_pools(&mut conn, &new_pools).unwrap();
            tracing::info!("Inserted {:?} pools", new_pools.len());
        }

        Ok(())
    }
}
