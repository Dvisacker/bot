use alloy::{
    network::{Ethereum, EthereumWallet},
    providers::{
        fillers::{
            BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller,
            WalletFiller,
        },
        Identity, ProviderBuilder, RootProvider,
    },
    signers::local::PrivateKeySigner,
    transports::BoxTransport,
};
use alloy_chains::{Chain, NamedChain};
use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};
use std::{collections::HashMap, env};

pub type SignerProvider = FillProvider<
    JoinFill<
        JoinFill<
            Identity,
            JoinFill<GasFiller, JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>>,
        >,
        WalletFiller<EthereumWallet>,
    >,
    RootProvider<BoxTransport>,
    BoxTransport,
    Ethereum,
>;

static PROVIDER_MAP: Lazy<Mutex<Option<ProviderMap>>> = Lazy::new(|| Mutex::new(None));

pub fn get_default_signer() -> PrivateKeySigner {
    std::env::var("DEV_PRIVATE_KEY")
        .expect("PRIVATE_KEY must be set")
        .parse()
        .expect("should parse private key")
}

pub fn get_default_wallet() -> EthereumWallet {
    let signer: PrivateKeySigner = get_default_signer();
    let wallet = EthereumWallet::new(signer);
    wallet
}

pub fn get_default_anvil_signer() -> PrivateKeySigner {
    String::from("0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80")
        .parse()
        .unwrap()
}

pub async fn get_default_anvil_provider() -> Arc<SignerProvider> {
    let private_key: PrivateKeySigner = get_default_anvil_signer();
    let wallet = EthereumWallet::new(private_key);
    let url = "http://localhost:8545";
    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .wallet(wallet)
        .on_builtin(url)
        .await
        .unwrap();
    return Arc::new(provider);
}

pub async fn get_provider(chain: Chain, wallet: EthereumWallet) -> Arc<SignerProvider> {
    let chain = NamedChain::try_from(chain.id());

    match chain {
        Ok(NamedChain::Mainnet) => {
            let url = env::var("MAINNET_WS_URL").expect("MAINNET_WS_URL is not set");
            let provider = ProviderBuilder::new()
                .with_recommended_fillers()
                .wallet(wallet)
                .on_builtin(url.as_str())
                .await
                .unwrap();
            return Arc::new(provider);
        }
        Ok(NamedChain::Arbitrum) => {
            let url = env::var("ARBITRUM_WS_URL").expect("ARBITRUM_WS_URL is not set");
            let provider = ProviderBuilder::new()
                .with_recommended_fillers()
                .wallet(wallet)
                .on_builtin(url.as_str())
                .await
                .unwrap();
            return Arc::new(provider);
        }
        Ok(NamedChain::Optimism) => {
            let url = env::var("OPTIMISM_WS_URL").expect("OPTIMISM_WS_URL is not set");
            let provider = ProviderBuilder::new()
                .with_recommended_fillers()
                .wallet(wallet)
                .on_builtin(url.as_str())
                .await
                .unwrap();
            return Arc::new(provider);
        }
        Ok(NamedChain::Base) => {
            let url = env::var("BASE_WS_URL").expect("BASE_WS_URL is not set");
            let provider = ProviderBuilder::new()
                .with_recommended_fillers()
                .wallet(wallet)
                .on_builtin(url.as_str())
                .await
                .unwrap();
            return Arc::new(provider);
        }
        _ => panic!("Chain not supported"),
    }
}

pub type ProviderMap = HashMap<NamedChain, Arc<SignerProvider>>;

pub async fn get_provider_map() -> Arc<ProviderMap> {
    let mut provider_guard = PROVIDER_MAP.lock().unwrap();

    if provider_guard.is_none() {
        let wallet = get_default_wallet();
        let mut providers = ProviderMap::new();

        for provider in [
            NamedChain::Mainnet,
            NamedChain::Arbitrum,
            NamedChain::Optimism,
            NamedChain::Base,
        ] {
            providers.insert(
                provider,
                get_provider(Chain::from_named(provider), wallet.clone()).await,
            );
        }

        *provider_guard = Some(providers);
    }

    Arc::new(provider_guard.as_ref().unwrap().clone())
}

pub fn is_provider_map_initialized() -> bool {
    PROVIDER_MAP.lock().unwrap().is_some()
}
