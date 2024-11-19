use bitcoin::Network as BdkNetwork;
use wasm_bindgen::prelude::*;

/// The cryptocurrency network to act on.
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Network {
    /// Mainnet Bitcoin.
    Bitcoin,
    /// Bitcoin's testnet network. (In future versions this will be combined
    /// into a single variant containing the version)
    Testnet,
    /// Bitcoin's testnet4 network. (In future versions this will be combined
    /// into a single variant containing the version)
    Testnet4,
    /// Bitcoin's signet network.
    Signet,
    /// Bitcoin's regtest network.
    Regtest,
}

impl From<BdkNetwork> for Network {
    fn from(network: BdkNetwork) -> Self {
        match network {
            BdkNetwork::Testnet => Network::Testnet,
            BdkNetwork::Testnet4 => Network::Testnet4,
            BdkNetwork::Signet => Network::Signet,
            BdkNetwork::Regtest => Network::Regtest,
            _ => Network::Bitcoin,
        }
    }
}

impl From<Network> for BdkNetwork {
    fn from(network: Network) -> Self {
        match network {
            Network::Bitcoin => BdkNetwork::Bitcoin,
            Network::Testnet => BdkNetwork::Testnet,
            Network::Testnet4 => BdkNetwork::Testnet4,
            Network::Signet => BdkNetwork::Signet,
            Network::Regtest => BdkNetwork::Regtest,
        }
    }
}