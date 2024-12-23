use alloy::rpc::types::Log;
use bindings::{iuniswapv2pair::IUniswapV2Pair, iuniswapv3pool::IUniswapV3Pool};
use engine::{
    collectors::block_collector::NewBlock, executors::mempool_executor::SubmitTxToMempool,
};

/// Core Event enum for the current strategy.
#[derive(Clone)]
pub enum Event {
    NewBlock(NewBlock),
    UniswapV2Swap(IUniswapV2Pair::Swap),
    UniswapV2Sync(IUniswapV2Pair::Sync),
    UniswapV3Swap(IUniswapV3Pool::Swap),
    Log(Log),
}

/// Core Action enum for the current strategy.
#[derive(Debug, Clone)]
pub enum Action {
    SubmitTx(SubmitTxToMempool),
}
