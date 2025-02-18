use alloy::providers::Provider;
use alloy::{hex, sol};
use alloy_chains::NamedChain;
use alloy_primitives::aliases::U24;
use alloy_primitives::{Address, Bytes, FixedBytes, U160, U256};
use alloy_rpc_types::TransactionReceipt;
use alloy_sol_types::{SolCall, SolValue};
use eyre::Result;
use std::str::FromStr;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use types::exchange::ExchangeName;

use addressbook::Addressbook;
use odos_client::client::{assemble_odos_swap, get_odos_quote};

use crate::bindings::batchexecutor::BatchExecutor::{
    requireProfitableCall, singlecallCall, BatchExecutorInstance, DynamicCall,
};
use crate::bindings::erc20::ERC20;
use crate::bindings::iaerodromerouter::{IAerodromeRouter, IRouter};
use crate::bindings::imorpho::IMorpho;
use crate::bindings::ipool::IPool::{
    self, borrowCall, flashLoanCall, liquidationCallCall, repayCall,
};
use crate::bindings::iuniswapv2router::IUniswapV2Router;
use crate::bindings::iuniswapv3pool::IUniswapV3Pool;
use crate::bindings::iuniswapv3router::IUniswapV3Router::{
    self, exactInputCall, ExactInputParams, ExactInputSingleParams,
};
use crate::bindings::weth::WETH;

sol! {
    #[allow(missing_docs)]
    #[derive(Debug, PartialEq, Eq)]
    struct FallbackData {
        bytes[] callback_params;
        bytes return_data;
    }
}

sol! {
    #[allow(missing_docs)]
    #[derive(Debug, PartialEq, Eq)]
    struct MarketParams {
        address loanToken;
        address collateralToken;
        address oracle;
        address irm;
        uint256 lltv;
    }
}

// a calldata enum that can be either a swap or a multicall
#[derive(Debug, Clone, Default)]
pub struct CallbackContext {
    data_index: u64,
    sender: Address,
}

pub struct ContractAddresses {
    pub aave_v3_pool_address: Address,
    pub uniswap_v3_router_address: Address,
    pub uniswap_v2_router_address: Address,
    pub morpho_pool_address: Address,
    pub aerodrome_router_address: Address,
}

pub struct BatchExecutorClient<P>
where
    P: Provider,
{
    pub chain: NamedChain,
    pub executor: BatchExecutorInstance<(), Arc<P>>,
    pub owner: Address,
    pub total_value: U256,
    pub calldata: Vec<Bytes>,
    pub addressbook: Addressbook,
    addresses: ContractAddresses,
}

impl<P> BatchExecutorClient<P>
where
    P: Provider,
{
    pub async fn new(address: Address, chain: NamedChain, provider: Arc<P>) -> Self {
        let executor = BatchExecutorInstance::new(address, provider.clone());
        let addressbook = Addressbook::load().unwrap();
        let total_value = U256::ZERO;
        let owner = executor.OWNER().call().await.unwrap()._0;

        let aave_v3_pool_address = addressbook
            .get_lending_pool(&chain, "aave_v3")
            .expect("Aave v3 pool not found");

        let uniswap_v3_router_address = addressbook
            .get_uni_v3_swap_router(&chain, ExchangeName::UniswapV3)
            .expect("Uniswap v3 router not found");

        let uniswap_v2_router_address = addressbook
            .get_uni_v2_swap_router(&chain, ExchangeName::UniswapV2)
            .expect("Uniswap v2 router not found");

        let morpho_pool_address = addressbook.get_lending_pool(&chain, "morpho").unwrap();
        let aerodrome_router_address = addressbook
            .get_ve33_router(&chain, ExchangeName::Aerodrome)
            .expect("Aerodrome router not found");

        Self {
            chain,
            executor,
            owner,
            calldata: Vec::new(),
            total_value,
            addressbook,
            addresses: ContractAddresses {
                aave_v3_pool_address,
                uniswap_v3_router_address,
                uniswap_v2_router_address,
                morpho_pool_address,
                aerodrome_router_address,
            },
        }
    }

    // BASIC CALLS

    pub fn add_approve_erc20(
        &mut self,
        token: Address,
        spender: Address,
        value: U256,
    ) -> &mut Self {
        let call = ERC20::approveCall { spender, value };
        let encoded = call.abi_encode();

        self.add_call(token, U256::ZERO, Bytes::from(encoded), None, None);

        self
    }

    pub fn add_approve_all_erc20(&mut self, token: Address, spender: Address) -> &mut Self {
        let call = ERC20::approveCall {
            spender,
            value: U256::ZERO,
        };
        let encoded = call.abi_encode();
        let dynamic_call = self.erc20_balance_of(token, self.owner, 4 + 32);

        self.add_call(
            token,
            U256::ZERO,
            Bytes::from(encoded),
            None,
            Some(vec![dynamic_call]),
        );

        self
    }

    pub fn add_wrap_eth(&mut self, weth: Address, amount: U256) -> &mut Self {
        let call = WETH::depositCall {};
        let encoded = call.abi_encode();

        self.add_call(weth, amount, Bytes::from(encoded), None, None)
    }

    pub fn add_unwrap_eth(&mut self, weth: Address, amount: U256) -> &mut Self {
        let call = WETH::withdrawCall { amount };
        let encoded = call.abi_encode();

        self.add_call(weth, U256::ZERO, Bytes::from(encoded), None, None)
    }

    pub fn add_transfer_erc20(
        &mut self,
        token: Address,
        recipient: Address,
        value: U256,
    ) -> &mut Self {
        let call = ERC20::transferCall {
            to: recipient,
            value,
        };
        let encoded = call.abi_encode();

        self.add_call(token, U256::ZERO, Bytes::from(encoded), None, None)
    }

    pub fn add_transfer_from_erc20(
        &mut self,
        token: Address,
        owner: Address,
        recipient: Address,
        value: U256,
    ) -> &mut Self {
        let call = ERC20::transferFromCall {
            from: owner,
            to: recipient,
            value,
        };
        let encoded = call.abi_encode();

        self.add_call(token, U256::ZERO, Bytes::from(encoded), None, None)
    }

    pub fn withdraw_funds(&mut self, token: Address, recipient: Address) -> &mut Self {
        let call = ERC20::transferCall {
            to: recipient,
            value: U256::ZERO,
        };
        let dynamic_call = self.erc20_balance_of(token, self.owner, 4 + 32);
        let encoded = call.abi_encode();
        self.add_call(
            token,
            U256::ZERO,
            Bytes::from(encoded),
            None,
            Some(vec![dynamic_call]),
        )
    }

    // DYNAMIC CALLS

    pub fn erc20_balance_of(&mut self, asset: Address, owner: Address, offset: u64) -> DynamicCall {
        let call = ERC20::balanceOfCall { account: owner };
        let calldata = call.abi_encode();

        let dynamic_call = DynamicCall {
            to: asset,
            data: Bytes::from(calldata),
            offset,
            length: 32,
            resOffset: 0,
        };

        return dynamic_call;
    }

    // AAVE V3

    pub fn add_aave_v3_supply(&mut self, asset: Address, amount: U256) -> &mut Self {
        let call = IPool::supplyCall {
            asset,
            amount,
            onBehalfOf: *self.executor.address(),
            referralCode: 0,
        };
        let encoded = call.abi_encode();

        self.add_call(
            self.addresses.aave_v3_pool_address,
            U256::ZERO,
            Bytes::from(encoded),
            None,
            None,
        )
    }

    pub fn add_aave_v3_borrow(
        &mut self,
        asset: Address,
        amount: U256,
        interest_rate_mode: U256,
    ) -> &mut Self {
        let call = borrowCall {
            asset,
            amount,
            onBehalfOf: *self.executor.address(),
            interestRateMode: interest_rate_mode,
            referralCode: 0,
        };
        let encoded = call.abi_encode();

        self.add_call(
            self.addresses.aave_v3_pool_address,
            U256::ZERO,
            Bytes::from(encoded),
            None,
            None,
        )
    }

    pub fn add_aave_v3_repay(
        &mut self,
        asset: Address,
        amount: U256,
        interest_rate_mode: U256,
    ) -> &mut Self {
        let call = repayCall {
            asset,
            amount,
            interestRateMode: interest_rate_mode,
            onBehalfOf: *self.executor.address(),
        };
        let encoded = call.abi_encode();

        self.add_call(
            self.addresses.aave_v3_pool_address,
            U256::ZERO,
            Bytes::from(encoded),
            None,
            None,
        )
    }

    pub fn add_aave_v3_withdraw(&mut self, asset: Address, amount: U256) -> &mut Self {
        let call = IPool::withdrawCall {
            asset,
            amount,
            to: *self.executor.address(),
        };
        let encoded = call.abi_encode();

        self.add_call(
            self.addresses.aave_v3_pool_address,
            U256::ZERO,
            Bytes::from(encoded),
            None,
            None,
        )
    }

    pub fn add_aave_v3_liquidate(
        &mut self,
        collateral: Address,
        debt: Address,
        user: Address,
        amount: U256,
    ) -> &mut Self {
        let call = liquidationCallCall {
            collateralAsset: collateral,
            debtAsset: debt,
            user,
            debtToCover: amount,
            receiveAToken: false,
        };
        let encoded = call.abi_encode();

        self.add_call(
            self.addresses.aave_v3_pool_address,
            U256::ZERO,
            Bytes::from(encoded),
            None,
            None,
        )
    }

    // MORPHO

    pub fn add_morpho_supply(
        &mut self,
        market: IMorpho::MarketParams,
        assets: U256,
        shares: U256,
        callbacks: Option<Bytes>,
    ) -> &mut Self {
        let call = IMorpho::supplyCall {
            marketParams: market,
            assets,
            shares,
            onBehalf: *self.executor.address(),
            data: callbacks.unwrap_or(Bytes::default()),
        };
        let encoded = call.abi_encode();

        self.add_call(
            self.addresses.morpho_pool_address,
            U256::ZERO,
            Bytes::from(encoded),
            None,
            None,
        )
    }

    // UNISWAP V3

    // this is for a multi-hop swap
    pub fn add_uniswap_v3_multihop_swap(
        &mut self,
        path: Vec<Address>,
        amount_out_minimum: U256,
        recipient: Address,
    ) -> &mut Self {
        let params = ExactInputParams {
            path: Bytes::from(path.concat()),
            recipient,
            amountIn: U256::ZERO,
            amountOutMinimum: amount_out_minimum,
        };
        let dynamic_call = self.erc20_balance_of(path[0], self.owner, 4 + 32 * 3);
        let call = exactInputCall { params };
        let encoded = call.abi_encode();

        self.add_call(
            self.addresses.uniswap_v3_router_address,
            U256::ZERO,
            Bytes::from(encoded),
            None,
            Some(vec![dynamic_call]),
        )
    }

    pub fn add_uniswap_v3_single_swap(
        &mut self,
        token_in: Address,
        token_out: Address,
        amount_in: U256,
        fee: U24,
    ) -> &mut Self {
        let swap = ExactInputSingleParams {
            tokenIn: token_in,
            tokenOut: token_out,
            fee: fee,
            recipient: *self.executor.address(),
            amountIn: amount_in,
            amountOutMinimum: U256::ZERO,
            sqrtPriceLimitX96: U160::ZERO,
        };

        let call = IUniswapV3Router::exactInputSingleCall { params: swap };

        self.add_call(
            self.addresses.uniswap_v3_router_address,
            U256::ZERO,
            Bytes::from(call.abi_encode()),
            None,
            None,
        )
    }

    pub fn add_uniswap_v3_single_swap_all(
        &mut self,
        token_in: Address,
        token_out: Address,
        fee: U24,
    ) -> &mut Self {
        let swap = ExactInputSingleParams {
            tokenIn: token_in,
            tokenOut: token_out,
            fee: fee,
            recipient: *self.executor.address(),
            amountIn: U256::ZERO, // replaced by dynamic balance call.
            amountOutMinimum: U256::ZERO,
            sqrtPriceLimitX96: U160::ZERO,
        };

        let dynamic_call = DynamicCall {
            to: token_in,
            data: Bytes::from(
                ERC20::balanceOfCall {
                    account: *self.executor.address(),
                }
                .abi_encode(),
            ),
            offset: 4 + 32 * 4, // amount is the 5th parameter
            length: 32,
            resOffset: 0,
        };

        let call = IUniswapV3Router::exactInputSingleCall { params: swap };

        self.add_call(
            self.addresses.uniswap_v3_router_address,
            U256::ZERO,
            Bytes::from(call.abi_encode()),
            None,
            Some(vec![dynamic_call]),
        )
    }

    // UNISWAP V2
    pub fn add_uniswap_v2_single_swap(
        &mut self,
        amount_in: U256,
        token_in: Address,
        token_out: Address,
        deadline: Option<U256>,
    ) -> &mut Self {
        let default_deadline = U256::from(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
                + 1000,
        );

        let call = IUniswapV2Router::swapExactTokensForTokensCall {
            amountIn: amount_in,
            amountOutMin: U256::ZERO,
            path: vec![token_in, token_out],
            to: *self.executor.address(),
            deadline: deadline.unwrap_or(default_deadline),
        };
        let encoded = call.abi_encode();

        self.add_call(
            self.addresses.uniswap_v2_router_address,
            U256::ZERO,
            Bytes::from(encoded),
            None,
            None,
        )
    }

    pub fn add_uniswap_v2_single_swap_all(
        &mut self,
        token_in: Address,
        token_out: Address,
        deadline: Option<U256>,
    ) -> &mut Self {
        let default_deadline = U256::from(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
                + 1000,
        );

        let deadline = deadline.unwrap_or(default_deadline);

        let dynamic_call = DynamicCall {
            to: token_in,
            data: Bytes::from(
                ERC20::balanceOfCall {
                    account: *self.executor.address(),
                }
                .abi_encode(),
            ),
            offset: 4 + 0, // amount is the first parameter
            length: 32,
            resOffset: 0,
        };

        let call = IUniswapV2Router::swapExactTokensForTokensCall {
            amountIn: U256::ZERO,
            amountOutMin: U256::ZERO,
            path: vec![token_in, token_out],
            to: *self.executor.address(),
            deadline,
        };

        let encoded = call.abi_encode();

        self.add_call(
            self.addresses.uniswap_v2_router_address,
            U256::ZERO,
            Bytes::from(encoded),
            None,
            Some(vec![dynamic_call]),
        )
    }

    pub fn add_uniswap_v3_flash_loan(
        &mut self,
        pool: Address,
        assets: [Address; 2],
        amounts: [U256; 2],
        fee: U256,
        callbacks: Vec<Bytes>,
    ) -> &mut Self {
        // Unpack assets and amounts
        let [asset0, asset1] = assets;
        let [amount0, amount1] = amounts;

        // (fee in basis points, e.g. 3000 = 0.3%)
        let fee0 = amount0
            .checked_mul(fee)
            .and_then(|x| x.checked_div(U256::from(1_000_000)))
            .unwrap();
        let fee1 = amount1
            .checked_mul(fee)
            .and_then(|x| x.checked_div(U256::from(1_000_000)))
            .unwrap();
        let amount0 = amount0 + fee0;
        let amount1 = amount1 + fee1;

        // Transfer calls for repayment
        let transfer0 = self.build_transfer_erc20(asset0, pool, amount0 + fee0);
        let transfer1 = self.build_transfer_erc20(asset1, pool, amount1 + fee1);

        let callback_params = [&callbacks[..], &[transfer0, transfer1]].concat();
        let return_data = Bytes::from(FixedBytes::<32>::from([
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 1,
        ]));

        let params = FallbackData {
            callback_params,
            return_data,
        };

        let flash_call = IUniswapV3Pool::flashCall {
            recipient: *self.executor.address(),
            amount0: amount0,
            amount1: amount1,
            data: Bytes::from(params.abi_encode()),
        };
        let encoded = flash_call.abi_encode();

        self.add_call(
            pool,
            U256::ZERO,
            Bytes::from(encoded),
            Some(CallbackContext {
                sender: pool,
                data_index: 2, // uniswapV3FlashCallback(uint256,uint256,bytes)
            }),
            None,
        )
    }

    // AERODROME

    pub fn add_aerodrome_single_swap(
        &mut self,
        amount_in: U256,
        token_in: Address,
        token_out: Address,
        deadline: Option<U256>,
        stable: Option<bool>,
    ) -> &mut Self {
        let default_deadline = U256::from(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
                + 1000,
        );

        let deadline = deadline.unwrap_or(default_deadline);
        let stable = stable.unwrap_or(false);

        let aerodrome_router_address = self.addresses.aerodrome_router_address;
        let aerodrome_factory_address = self
            .addressbook
            .get_ve33_factory(&self.chain, ExchangeName::Aerodrome)
            .expect("Aerodrome factory not found");

        let route = IRouter::Route {
            from: token_in,
            to: token_out,
            stable,
            factory: aerodrome_factory_address,
        };
        let call = IAerodromeRouter::swapExactTokensForTokensCall {
            amountIn: amount_in,
            amountOutMin: U256::ZERO,
            routes: vec![route],
            to: *self.executor.address(),
            deadline,
        };

        self.add_approve_erc20(token_in, aerodrome_router_address, amount_in)
            .add_call(
                aerodrome_router_address,
                U256::ZERO,
                Bytes::from(call.abi_encode()),
                None,
                None,
            )
    }

    pub fn add_aerodrome_single_swap_all(
        &mut self,
        token_in: Address,
        token_out: Address,
        deadline: Option<U256>,
        stable: Option<bool>,
    ) -> &mut Self {
        let default_deadline = U256::from(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
                + 1000,
        );

        let deadline = deadline.unwrap_or(default_deadline);
        let stable = stable.unwrap_or(false);

        let aerodrome_factory_address = self
            .addressbook
            .get_ve33_factory(&self.chain, ExchangeName::Aerodrome)
            .expect("Aerodrome factory not found");

        let aerodrome_router_address = self
            .addressbook
            .get_ve33_router(&self.chain, ExchangeName::Aerodrome)
            .expect("Aerodrome router not found");

        // Create the dynamic call to get balance
        let dynamic_call = DynamicCall {
            to: token_in,
            data: Bytes::from(
                ERC20::balanceOfCall {
                    account: *self.executor.address(),
                }
                .abi_encode(),
            ),
            offset: 4 + 0, // amount is the first parameter
            length: 32,
            resOffset: 0,
        };

        // Create the route
        let route = IRouter::Route {
            from: token_in,
            to: token_out,
            stable,
            factory: aerodrome_factory_address,
        };

        // Create the swap call
        let swap_call = IAerodromeRouter::swapExactTokensForTokensCall {
            amountIn: U256::ZERO, // replaced by dynamic call
            amountOutMin: U256::ZERO,
            routes: vec![route],
            to: *self.executor.address(),
            deadline,
        };

        // Add approve for token_in
        self.add_approve_erc20(token_in, aerodrome_router_address, U256::MAX)
            .add_call(
                aerodrome_router_address,
                U256::ZERO,
                Bytes::from(swap_call.abi_encode()),
                None,
                Some(vec![dynamic_call]),
            )
    }

    pub fn add_swap(
        &mut self,
        exchange: ExchangeName,
        token_in: Address,
        token_out: Address, // not sure if this is needed
        amount_in: U256,
        deadline: Option<U256>,
        stable: Option<bool>,
        fee: Option<U24>,
    ) -> &mut Self {
        match exchange {
            ExchangeName::UniswapV3 => {
                let swap_fee = fee.expect("Fee is required for UniswapV3");
                self.add_uniswap_v3_single_swap(token_in, token_out, amount_in, swap_fee)
            }
            ExchangeName::UniswapV2 => {
                self.add_uniswap_v2_single_swap(amount_in, token_in, token_out, deadline)
            }
            ExchangeName::Aerodrome => {
                self.add_aerodrome_single_swap_all(token_in, token_out, deadline, stable)
            }
            _ => {
                panic!("Unsupported exchange: {}", exchange);
            }
        }
    }

    pub fn add_swap_all(
        &mut self,
        exchange: ExchangeName,
        token_in: Address,
        token_out: Address,
        deadline: Option<U256>,
        stable: Option<bool>,
        fee: Option<U24>,
    ) -> &mut Self {
        match exchange {
            ExchangeName::UniswapV3 => {
                let swap_fee = fee.expect("Fee is required for UniswapV3");
                self.add_uniswap_v3_single_swap_all(token_in, token_out, swap_fee)
            }
            ExchangeName::UniswapV2 => {
                self.add_uniswap_v2_single_swap_all(token_in, token_out, deadline)
            }
            ExchangeName::Aerodrome => {
                self.add_aerodrome_single_swap_all(token_in, token_out, deadline, stable)
            }
            _ => {
                panic!("Unsupported exchange: {}", exchange);
            }
        }
    }

    // MORPHO

    pub fn add_morpho_flash_loan(
        &mut self,
        morpho_pool_address: Address,
        asset: Address,
        amount: U256,
        callbacks: Vec<Bytes>,
    ) -> &mut Self {
        // (fee in basis points, e.g. 3000 = 0.3%)
        let approve = self.build_approve_erc20(asset, morpho_pool_address, amount);
        let callback_params = [&callbacks[..], &[approve]].concat();
        let return_data = Bytes::from(FixedBytes::<32>::from([
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 1,
        ]));

        let params = FallbackData {
            callback_params,
            return_data,
        };

        let flash_call = IMorpho::flashLoanCall {
            token: asset,
            assets: amount, // morpho chose a weird variable name for the amount
            data: Bytes::from(params.abi_encode()),
        };
        let encoded = flash_call.abi_encode();

        // Add the call with the appropriate context
        self.add_call(
            morpho_pool_address,
            U256::ZERO,
            Bytes::from(encoded),
            Some(CallbackContext {
                sender: morpho_pool_address,
                data_index: 1, // onMorphoFlashLoan(uint256 assets, bytes calldata data)
            }),
            None,
        )
    }

    // FLASH LOANS
    pub fn add_aave_v3_flash_loan(
        &mut self,
        asset: Address,
        amount: U256,
        premium: U256,
        callbacks: Vec<Bytes>,
    ) -> &mut Self {
        // This consists of 1) The array of callback calls calldata and 2) The callback return value (specific to each flashloan)
        let fee = amount
            .checked_mul(premium)
            .and_then(|x| x.checked_div(U256::from(10_000)))
            .unwrap();
        let amount_with_premium = amount + fee;
        let approve_premium_call_data = self.build_approve_erc20(
            asset,
            self.addresses.aave_v3_pool_address,
            amount_with_premium,
        );

        let callback_params = [&callbacks[..], &[approve_premium_call_data]].concat();
        let return_data = Bytes::from(FixedBytes::<32>::from([
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 1,
        ]));

        let params = FallbackData {
            callback_params,
            return_data,
        };

        let flash_loan_call = flashLoanCall {
            receiverAddress: *self.executor.address(),
            assets: vec![asset],
            amounts: vec![amount],
            interestRateModes: vec![U256::from(0)],
            onBehalfOf: *self.executor.address(),
            params: Bytes::from(params.abi_encode()),
            referralCode: 0,
        };
        let encoded = flash_loan_call.abi_encode();

        let context = Some(CallbackContext {
            sender: self.addresses.aave_v3_pool_address,
            data_index: 4, // executeOperation(address[],uint256[],uint256[],address,bytes)
        });

        self.add_call(
            self.addresses.aave_v3_pool_address,
            U256::ZERO,
            Bytes::from(encoded),
            context,
            None,
        )
    }

    pub fn add_uniswap_v2_flash_swap(
        &mut self,
        pool: Address,
        assets: [Address; 2],
        amounts: [U256; 2],
        callbacks: Vec<Bytes>,
    ) -> &mut Self {
        // Unpack assets and amounts
        let [asset0, asset1] = assets;
        let [amount0, amount1] = amounts;

        // This consists of callback calls and the callback return value
        let approve0 = self.build_approve_erc20(asset0, pool, amount0);
        let approve1 = self.build_approve_erc20(asset1, pool, amount1);

        let callback_params = [&callbacks[..], &[approve0, approve1]].concat();
        let return_data = Bytes::default(); // Empty bytes - "0x"

        let params = FallbackData {
            callback_params,
            return_data,
        };

        // Create the flash call
        let flash_call = IUniswapV3Pool::flashCall {
            recipient: *self.executor.address(),
            amount0: amount0,
            amount1: amount1,
            data: Bytes::from(params.abi_encode()),
        };
        let encoded = flash_call.abi_encode();

        // Add the call with the appropriate context
        self.add_call(
            pool,
            U256::ZERO,
            Bytes::from(encoded),
            Some(CallbackContext {
                sender: pool,
                data_index: 3, // uniswapV2Call(address,uint256,uint256,bytes)
            }),
            None,
        )
    }

    pub async fn add_odos_swap(
        &mut self,
        input_amount: U256,
        input_token: Address,
        output_token: Address,
        slippage: f64,
    ) -> &mut Self {
        let quote = get_odos_quote(
            self.chain,
            input_token,
            input_amount,
            output_token,
            *self.executor.address(),
            slippage,
        )
        .await
        .unwrap();
        let response = assemble_odos_swap(&quote, *self.executor.address(), false)
            .await
            .unwrap();

        let tx_data = hex::decode(response.transaction.data.strip_prefix("0x").unwrap()).unwrap();

        self.add_approve_erc20(
            input_token,
            Address::from_str(&response.transaction.to).unwrap(),
            input_amount,
        )
        .add_call(
            Address::from_str(&response.transaction.to).unwrap(),
            U256::from_str(&response.transaction.value).unwrap(),
            Bytes::from(tx_data),
            None,
            None,
        )
    }

    // INTERNAL

    pub fn encoded_context(&self, context: CallbackContext) -> Result<FixedBytes<32>> {
        let data_index = context.data_index.to_be_bytes();
        let padded_data_index = FixedBytes::<12>::left_padding_from(&data_index);
        let sender = FixedBytes::<20>::from(context.sender);
        let context = padded_data_index.concat_const(sender);

        Ok(context)
    }

    pub fn build_call(
        &mut self,
        target: Address,
        value: U256,
        calldata: Bytes,
        context: Option<CallbackContext>,
        dynamic_calls: Option<Vec<DynamicCall>>,
    ) -> Bytes {
        let context = self.encoded_context(context.unwrap_or_default()).unwrap();
        let single_call = singlecallCall {
            target,
            value,
            context,
            callData: calldata,
            dynamicCalls: dynamic_calls.unwrap_or_default(),
        };

        return Bytes::from(single_call.abi_encode());
    }

    pub fn build_call_with_dynamic_call(
        &mut self,
        target: Address,
        value: U256,
        calldata: Bytes,
        context: Option<CallbackContext>,
        dynamic_call: DynamicCall,
    ) -> Bytes {
        let context = self.encoded_context(context.unwrap()).unwrap();
        let single_call = singlecallCall {
            target,
            value,
            context,
            callData: calldata,
            dynamicCalls: vec![dynamic_call],
        };

        return Bytes::from(single_call.abi_encode());
    }

    pub fn build_approve_erc20(
        &mut self,
        asset: Address,
        recipient: Address,
        amount: U256,
    ) -> Bytes {
        let call = ERC20::approveCall {
            spender: recipient,
            value: amount,
        };

        return self.build_call(
            asset,
            U256::ZERO,
            Bytes::from(call.abi_encode()),
            None,
            None,
        );
    }

    pub fn build_transfer_erc20(
        &mut self,
        asset: Address,
        recipient: Address,
        amount: U256,
    ) -> Bytes {
        let call = ERC20::transferCall {
            to: recipient,
            value: amount,
        };
        return self.build_call(
            asset,
            U256::ZERO,
            Bytes::from(call.abi_encode()),
            None,
            None,
        );
    }

    pub fn add_call(
        &mut self,
        target: Address,
        value: U256,
        calldata: Bytes,
        context: Option<CallbackContext>,
        dynamic_calls: Option<Vec<DynamicCall>>,
    ) -> &mut Self {
        let context = self.encoded_context(context.unwrap_or_default()).unwrap();

        let single_call = singlecallCall {
            target,
            value,
            context,
            callData: calldata,
            dynamicCalls: dynamic_calls.unwrap_or_default(),
        };

        let encoded = Bytes::from(single_call.abi_encode());

        self.calldata.push(encoded);
        self.total_value += value;

        self
    }

    pub fn require_profitable(&mut self, token: Address, amount_in: U256) -> &mut Self {
        let call = requireProfitableCall {
            token,
            amount: amount_in,
        };

        let encoded = Bytes::from(call.abi_encode());

        self.calldata.push(encoded);
        self.total_value += U256::ZERO;

        self
    }

    pub fn set_calldata(&mut self, calldata: Vec<Bytes>) {
        self.calldata = calldata;
    }

    pub fn get(&mut self) -> (Vec<Bytes>, U256) {
        let calldata = self.calldata.clone();
        let total_value = self.total_value;
        (calldata, total_value)
    }

    pub fn flush(&mut self) -> (Vec<Bytes>, U256) {
        let calldata = self.calldata.clone();
        let total_value = self.total_value;
        self.calldata.clear();
        self.total_value = U256::ZERO;
        (calldata, total_value)
    }

    pub async fn estimate_gas(&mut self) -> Result<u64> {
        let (calldata, total_value) = self.get();
        let call = self
            .executor
            .batchCall(calldata)
            .from(self.owner)
            .value(total_value);
        let gas_estimate = call.estimate_gas().await?;
        Ok(gas_estimate)
    }

    pub async fn exec(&mut self) -> Result<(bool, TransactionReceipt)> {
        let (calldata, total_value) = self.flush();
        let call = self
            .executor
            .batchCall(calldata)
            .from(self.owner)
            .value(total_value);
        let pending_tx = call.send().await?;
        let receipt = pending_tx.get_receipt().await?;
        Ok((true, receipt))
    }
}

#[cfg(test)]
mod tests {
    use provider::get_anvil_provider_arc;
    use shared::{
        evm_helpers::compute_v3_pool_address,
        token_helpers::{get_token_balance, parse_token_units},
    };

    use super::*;
    use alloy_primitives::{aliases::U24, U256};
    use std::{env, str::FromStr};
    use types::token::TokenIsh;

    const CHAIN: NamedChain = NamedChain::Base;

    #[tokio::test]
    async fn test_wrap_eth() -> Result<()> {
        dotenv::dotenv().ok();
        let addressbook = Addressbook::load().unwrap();
        let weth = addressbook.get_weth(&CHAIN).unwrap();
        let provider = get_anvil_provider_arc().await;

        let executor_address = Address::from_str(&env::var("EXECUTOR_ADDRESS").unwrap()).unwrap();
        let mut encoder = BatchExecutorClient::new(executor_address, CHAIN, provider.clone()).await;

        let amount = parse_token_units(&CHAIN, &TokenIsh::Address(weth), "1")
            .await
            .unwrap();

        let balance_before = get_token_balance(provider.clone(), weth, executor_address)
            .await
            .unwrap();

        let (_success, _tx_hash) = encoder.add_wrap_eth(weth, amount).exec().await?;

        let balance_after = get_token_balance(provider.clone(), weth, executor_address)
            .await
            .unwrap();

        assert_eq!(balance_after, balance_before + amount, "Incorrect balance");

        Ok(())
    }

    #[tokio::test]
    async fn test_swap_usdc_for_dai_via_uniswap_v3() -> Result<()> {
        dotenv::dotenv().ok();
        let addressbook = Addressbook::load().unwrap();
        let weth = addressbook.get_weth(&CHAIN).unwrap();
        let usdc = addressbook.get_usdc(&CHAIN).unwrap();
        let uni_v3_router = addressbook
            .get_uni_v3_swap_router(&CHAIN, ExchangeName::UniswapV3)
            .unwrap();
        let provider = get_anvil_provider_arc().await;

        let executor_address = Address::from_str(&env::var("EXECUTOR_ADDRESS").unwrap()).unwrap();
        let mut encoder = BatchExecutorClient::new(executor_address, CHAIN, provider.clone()).await;

        let amount = parse_token_units(&CHAIN, &TokenIsh::Address(weth), "1")
            .await
            .unwrap();

        let (success, _tx_hash) = encoder
            .add_wrap_eth(weth, amount)
            .add_approve_erc20(weth, uni_v3_router, U256::MAX)
            .add_uniswap_v3_single_swap(weth, usdc, amount, U24::from(500u32))
            .exec()
            .await?;

        assert!(success, "Transaction failed");

        let balance_usdc = get_token_balance(provider.clone(), usdc, executor_address)
            .await
            .unwrap();

        assert!(balance_usdc > U256::ZERO, "USDC balance is zero");

        Ok(())
    }

    #[tokio::test]
    async fn test_uniswap_v3_flash_loan() -> Result<()> {
        dotenv::dotenv().ok();
        let addressbook = Addressbook::load().unwrap();
        let provider = get_anvil_provider_arc().await;

        let executor_address = Address::from_str(&env::var("EXECUTOR_ADDRESS").unwrap()).unwrap();
        let mut encoder = BatchExecutorClient::new(executor_address, CHAIN, provider.clone()).await;

        let usdc = addressbook.get_usdc(&CHAIN).unwrap();
        let weth = addressbook.get_weth(&CHAIN).unwrap();
        let amount = parse_token_units(&CHAIN, &TokenIsh::Address(usdc), "1")
            .await
            .unwrap();

        let pool_address =
            compute_v3_pool_address(&CHAIN, ExchangeName::UniswapV3, usdc, weth, 500)?;

        let assets = [usdc, weth];
        let amounts = [amount, U256::ZERO];
        let fee = U256::from(500u32);
        let callbacks = vec![];

        encoder
            .add_uniswap_v3_flash_loan(pool_address, assets, amounts, fee, callbacks)
            .exec()
            .await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_morpho_flash_loan() -> Result<()> {
        dotenv::dotenv().ok();
        let addressbook = Addressbook::load().unwrap();
        let provider = get_anvil_provider_arc().await;

        let executor_address = Address::from_str(&env::var("EXECUTOR_ADDRESS").unwrap()).unwrap();
        let pool_address = addressbook.get_lending_pool(&CHAIN, "morpho").unwrap();
        let mut encoder = BatchExecutorClient::new(executor_address, CHAIN, provider.clone()).await;

        let weth = addressbook.get_weth(&CHAIN).unwrap();
        let amount = parse_token_units(&CHAIN, &TokenIsh::Address(weth), "1")
            .await
            .unwrap();

        let callbacks = vec![];

        encoder
            .add_approve_erc20(weth, pool_address, amount)
            .add_morpho_flash_loan(pool_address, weth, amount, callbacks)
            .exec()
            .await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_aave_v3_supply() -> Result<()> {
        dotenv::dotenv().ok();
        let addressbook = Addressbook::load().unwrap();
        let provider = get_anvil_provider_arc().await;

        let executor_address = Address::from_str(&env::var("EXECUTOR_ADDRESS").unwrap()).unwrap();
        let mut encoder = BatchExecutorClient::new(executor_address, CHAIN, provider.clone()).await;

        let weth = addressbook.get_weth(&CHAIN).unwrap();
        let amount = parse_token_units(&CHAIN, &TokenIsh::Address(weth), "1")
            .await
            .unwrap();

        let pool_address = addressbook.get_lending_pool(&CHAIN, "aave_v3").unwrap();

        encoder
            .add_wrap_eth(weth, amount)
            .add_transfer_erc20(weth, executor_address, amount)
            .add_approve_erc20(weth, pool_address, amount)
            .add_aave_v3_supply(weth, amount)
            .exec()
            .await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_aave_v3_borrow() -> Result<()> {
        dotenv::dotenv().ok();
        let addressbook = Addressbook::load().unwrap();
        let provider = get_anvil_provider_arc().await;

        let executor_address = Address::from_str(&env::var("EXECUTOR_ADDRESS").unwrap()).unwrap();
        let mut encoder = BatchExecutorClient::new(executor_address, CHAIN, provider.clone()).await;

        let weth = addressbook.get_weth(&CHAIN).unwrap();
        let usdc = addressbook.get_usdc(&CHAIN).unwrap();
        let amount_weth = parse_token_units(&CHAIN, &TokenIsh::Address(weth), "1")
            .await
            .unwrap();
        let amount_usdc = parse_token_units(&CHAIN, &TokenIsh::Address(usdc), "10")
            .await
            .unwrap();

        let pool_address = addressbook.get_lending_pool(&CHAIN, "aave_v3").unwrap();

        encoder
            .add_wrap_eth(weth, amount_weth)
            .add_transfer_erc20(weth, executor_address, amount_weth)
            .add_approve_erc20(weth, pool_address, amount_weth)
            .add_aave_v3_supply(weth, amount_weth)
            .add_aave_v3_borrow(usdc, amount_usdc, U256::from(2))
            .exec()
            .await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_aave_v3_repay() -> Result<()> {
        dotenv::dotenv().ok();
        let addressbook = Addressbook::load().unwrap();
        let provider = get_anvil_provider_arc().await;

        let executor_address = Address::from_str(&env::var("EXECUTOR_ADDRESS").unwrap()).unwrap();
        let mut encoder = BatchExecutorClient::new(executor_address, CHAIN, provider.clone()).await;

        let weth = addressbook.get_weth(&CHAIN).unwrap();
        let usdc = addressbook.get_usdc(&CHAIN).unwrap();
        let amount_weth = parse_token_units(&CHAIN, &TokenIsh::Address(weth), "1")
            .await
            .unwrap();
        let amount_usdc = parse_token_units(&CHAIN, &TokenIsh::Address(usdc), "1000")
            .await
            .unwrap();

        let pool_address = addressbook.get_lending_pool(&CHAIN, "aave_v3").unwrap();

        encoder
            .add_wrap_eth(weth, amount_weth)
            .add_transfer_erc20(weth, executor_address, amount_weth)
            .add_approve_erc20(weth, pool_address, amount_weth)
            .add_aave_v3_supply(weth, amount_weth)
            .add_aave_v3_borrow(usdc, amount_usdc, U256::from(2))
            .exec()
            .await?;

        encoder
            .add_approve_erc20(usdc, pool_address, amount_usdc)
            .add_aave_v3_repay(usdc, amount_usdc, U256::from(2))
            .add_aave_v3_withdraw(weth, amount_weth)
            .exec()
            .await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_aave_v3_flashloan() -> Result<()> {
        dotenv::dotenv().ok();
        let addressbook = Addressbook::load().unwrap();
        let provider = get_anvil_provider_arc().await;

        let executor_address = Address::from_str(&env::var("EXECUTOR_ADDRESS").unwrap()).unwrap();
        let mut encoder = BatchExecutorClient::new(executor_address, CHAIN, provider.clone()).await;

        let weth = addressbook.get_weth(&CHAIN).unwrap();
        let aave_pool_address = addressbook.get_lending_pool(&CHAIN, "aave_v3").unwrap();
        let amount_weth = parse_token_units(&CHAIN, &TokenIsh::Address(weth), "1")
            .await
            .unwrap();

        let aave_pool = IPool::new(aave_pool_address, provider.clone());

        let premium = aave_pool.FLASHLOAN_PREMIUM_TOTAL().call().await.unwrap()._0;
        let callbacks = vec![];

        encoder
            .add_wrap_eth(weth, amount_weth)
            .add_transfer_erc20(weth, executor_address, amount_weth)
            .add_aave_v3_flash_loan(weth, amount_weth, U256::from(premium), callbacks)
            .exec()
            .await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_odos_swap() -> Result<()> {
        dotenv::dotenv().ok();
        let provider = get_anvil_provider_arc().await;
        let executor_address = Address::from_str(&env::var("EXECUTOR_ADDRESS").unwrap()).unwrap();
        let chain = NamedChain::Base;
        let addressbook = Addressbook::load().unwrap();
        let weth = addressbook.get_weth(&chain).unwrap();
        let usdc = addressbook.get_usdc(&chain).unwrap();
        let input_amount = U256::from(1000000000000000000u128); // 1 WETH
        let slippage = 0.5;

        let mut encoder = BatchExecutorClient::new(executor_address, CHAIN, provider.clone()).await;

        let (_success, receipt) = encoder
            .add_wrap_eth(weth, input_amount)
            .add_transfer_erc20(weth, executor_address, input_amount)
            .add_odos_swap(input_amount, weth, usdc, slippage)
            .await
            .exec()
            .await?;

        assert!(receipt.status(), "Transaction failed");

        Ok(())
    }

    #[tokio::test]
    async fn test_aerodrome_swap_all() -> Result<()> {
        dotenv::dotenv().ok();
        let addressbook = Addressbook::load().unwrap();
        let provider = get_anvil_provider_arc().await;
        let executor_address = Address::from_str(&env::var("EXECUTOR_ADDRESS").unwrap()).unwrap();
        let mut encoder = BatchExecutorClient::new(executor_address, CHAIN, provider.clone()).await;

        let weth = addressbook.get_weth(&CHAIN).unwrap();
        let usdc = addressbook.get_usdc(&CHAIN).unwrap();
        let amount = parse_token_units(&CHAIN, &TokenIsh::Address(weth), "0.0001")
            .await
            .unwrap();

        let initial_usdc_balance = get_token_balance(provider.clone(), usdc, executor_address)
            .await
            .unwrap();

        let (_success, receipt) = encoder
            .add_wrap_eth(weth, amount)
            .add_transfer_erc20(weth, executor_address, amount)
            .add_aerodrome_single_swap_all(weth, usdc, None, None)
            .exec()
            .await?;

        assert!(receipt.status(), "Transaction failed");

        // Get final USDC balance
        let final_usdc_balance = get_token_balance(provider.clone(), usdc, executor_address)
            .await
            .unwrap();

        // Verify USDC balance increased
        assert!(
            final_usdc_balance > initial_usdc_balance,
            "USDC balance did not increase"
        );

        // Verify WETH balance is 0
        let final_weth_balance = get_token_balance(provider.clone(), weth, executor_address)
            .await
            .unwrap();
        assert_eq!(
            final_weth_balance,
            U256::ZERO,
            "WETH balance should be zero after swapping all"
        );

        Ok(())
    }
}
