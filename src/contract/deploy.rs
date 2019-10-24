//! Implementation for creating instances for deployed contracts and deploying
//! new contracts.

use crate::errors::DeployError;
use crate::transaction::TransactionBuilder;
use crate::future::{CompatCallFuture, Web3Unpin};
use crate::contract::Instance;
use crate::truffle::{Abi, Artifact};
use ethabi::{ErrorKind as AbiErrorKind, Result as AbiResult};
use futures::compat::Future01CompatExt;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use web3::api::Web3;
use web3::contract::tokens::{Tokenize};
use web3::types::Address;
use web3::Transport;

/// Future for creating a deployed contract instance.
pub struct DeployedFuture<T: Transport> {
    /// Deployed arguments: `web3` provider and artifact.
    args: Option<(Web3Unpin<T>, Artifact)>,
    /// Underlying future for retrieving the network ID.
    network_id: CompatCallFuture<T, String>,
}

impl<T: Transport> DeployedFuture<T> {
    pub(crate) fn from_args(web3: Web3<T>, artifact: Artifact) -> DeployedFuture<T> {
        let net = web3.net();
        DeployedFuture {
            args: Some((web3.into(), artifact)),
            network_id: net.version().compat(),
        }
    }

    /// Take value of our passed in `web3` provider.
    fn args(self: Pin<&mut Self>) -> (Web3<T>, Artifact) {
        let (web3, artifact) = self
            .get_mut()
            .args
            .take()
            .expect("should be called only once");
        (web3.into(), artifact)
    }

    /// Get a pinned reference to the inner `CallFuture` for retrieving the
    /// current network ID.
    fn network_id(self: Pin<&mut Self>) -> Pin<&mut CompatCallFuture<T, String>> {
        Pin::new(&mut self.get_mut().network_id)
    }
}

impl<T: Transport> Future for DeployedFuture<T> {
    type Output = Result<Instance<T>, DeployError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        self.as_mut().network_id().poll(cx).map(|network_id| {
            let network_id = network_id?;
            let (web3, artifact) = self.args();

            let address = match artifact.networks.get(&network_id) {
                Some(network) => network.address,
                None => return Err(DeployError::NotFound(network_id)),
            };

            Ok(Instance {
                web3,
                abi: artifact.abi,
                address,
            })
        })
    }
}

/// Builder for specifying options for deploying a contract.
pub struct DeployBuilder<T: Transport> {
    /// The ABI for the contract that is to be deployed.
    abi: Abi,
    /// The deployment code for the contract.
    code: String,
    /// The linked libraries.
    libs: HashMap<String, Address>,
    /// The underlying transaction used t
    tx: TransactionBuilder<T>,
}

impl<T: Transport> DeployBuilder<T> {
    pub(crate) fn new<P>(web3: Web3<T>, artifact: Artifact, params: P) -> AbiResult<DeployBuilder<T>>
    where
        P: Tokenize,
    {
        unimplemented!()
    }
}

/// Future for deploying a contract instance.
pub struct DeployFuture<T: Transport> {
    /// Deployed arguments: `web3` provider and artifact.
    args: Option<(Web3Unpin<T>, Artifact)>,

    /// Underlying future for retrieving the network ID.
    network_id: CompatCallFuture<T, String>,
}