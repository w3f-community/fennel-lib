//! Fennel RPC Connection

mod error;

use subxt::{ClientBuilder, DefaultConfig, DefaultExtra, Client, Config, PairSigner, sp_core::sr25519::Pair};
use self::error::Error;

use crate::Identity;
 use crate::database::FennelLocalDb;

type RawIdentity = [u8; 4];

/// To run this example, a local fennel node should be running.
///
/// ```bash
/// curl "https://github.com/paritytech/polkadot/releases/download/v0.9.13/polkadot" --output /usr/local/bin/polkadot --location
/// ./fennel --dev --tmp
///
/// # to fetch the metadata from a running dev node
/// curl -sX POST -H "Content-Type: application/json" --data '{"jsonrpc":"2.0","method":"state_getMetadata", "id": 1}' localhost:9933 \
///                 | jq .result \
///                 | cut -d '"' -f 2 \
///                 | xxd -r -p > ./fennel-metadata.scale
/// ```
#[subxt::subxt(runtime_metadata_path = "src/fennel/fennel-metadata.scale")]
pub mod fennel {}

type RuntimeApi = fennel::RuntimeApi<DefaultConfig, DefaultExtra<DefaultConfig>>;

pub struct TransactionHandler {
    runtime: RuntimeApi,
    // FIXME:: its not the best to mix blocking ops (rocksdb gets) with async, since 
    // long-running operations will most certainly block the async executor.
    // however with our limited data (single gets/retrieves) this should be fast 
    // enough that it is not noticeable. 
    // alternatively, the database struct could spawn all getes onto the executor as a blocking op
    db: FennelLocalDb,
}

impl TransactionHandler {
    pub async fn new() -> Result<Self, Error> {
        let runtime = ClientBuilder::new()
            .build()
            .await?
            .to_runtime_api::<RuntimeApi>();
        let db = FennelLocalDb::new()?;        

        Ok(Self { runtime, db })
    }
   
    /// submit an identity to the network
    pub async fn add_or_update(&self, id: RawIdentity, signer: Pair) -> Result<(), Error> {
        let signer = PairSigner::<DefaultConfig, DefaultExtra<DefaultConfig>, _>::new(signer);
         
        // NOTE: identity module should probably be snake case, or just named `identity`
        // api
        Ok(())
    }

    pub async fn create_identity(&self, signer: Pair) -> Result<(), Error> {
        let signer = PairSigner::<DefaultConfig, DefaultExtra<DefaultConfig>, _>::new(signer);

        let identity = self.runtime.tx()
            .identity_module()
            .create_identity()
            .sign_and_submit_then_watch(&signer)
            .await?
            .wait_for_finalized_success()
            // FIXME: Should be in error enum with GenericError
            .await
            .unwrap();
         
        let identity_event =
            identity.find_first_event::<fennel::identity_module::events::IdentityCreated>()?;

        if let Some(event) = identity_event {
            println!("Identity Create success: {event:?}");
        } else {
            println!("Failed to find identity_module::Transfer Event");
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fetch() {
        fetch_storage()
            .await
            .expect("Storage should have been fetched");
    }
}
