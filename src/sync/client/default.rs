use crate::client::lazy::LazyClient;
#[cfg(feature = "acl")]
use crate::client::AclClient;
use crate::client::{Client as AsyncClient, IClient as IAsyncClient, LazyDefaultChannel};
use crate::sync::client::{ClientState, ClientVariant, IClient};
use crate::txn::Txn;
use crate::{Endpoints, Result};
use async_trait::async_trait;
use failure::Error;
use http::Uri;
use std::convert::TryInto;

///
/// Inner state for default Client
///
#[derive(Debug)]
#[doc(hidden)]
pub struct Default {
    async_client: AsyncClient,
}

#[async_trait]
impl IClient for Default {
    type AsyncClient = AsyncClient;

    type Client = LazyClient<Self::Channel>;
    type Channel = LazyDefaultChannel;

    fn client(&self) -> Self::Client {
        self.async_client.extra.client()
    }

    fn clients(self) -> Vec<Self::Client> {
        self.async_client.extra.clients()
    }

    fn async_client_ref(&self) -> &Self::AsyncClient {
        &self.async_client
    }

    fn async_client(self) -> Self::AsyncClient {
        self.async_client
    }

    fn new_txn(&self) -> Txn<Self::Client> {
        self.async_client_ref().new_txn()
    }

    #[cfg(feature = "acl")]
    async fn login<T: Into<String> + Send + Sync>(
        self,
        user_id: T,
        password: T,
    ) -> Result<AclClient<Self::Channel>, Error> {
        self.async_client.login(user_id, password).await
    }
}

///
/// Default client.
///
pub type Client = ClientVariant<Default>;

impl Client {
    ///
    /// Create new Sync Dgraph client for interacting v DB.
    ///
    /// The client can be backed by multiple endpoints (to the same server, or multiple servers in a cluster).
    ///
    /// # Arguments
    ///
    /// * `endpoints` - one endpoint or vector of endpoints
    ///
    /// # Errors
    ///
    /// * endpoints vector is empty
    /// * item in vector cannot by converted into Uri
    ///
    /// # Example
    ///
    /// ```
    /// use dgraph_tonic::sync::Client;
    ///
    /// // vector of endpoints
    /// let client = Client::new(vec!["http://127.0.0.1:19080", "http://127.0.0.1:19080"]).expect("Dgraph client");
    /// // one endpoint
    /// let client = Client::new("http://127.0.0.1:19080").expect("Dgraph client");
    /// ```
    ///
    pub fn new<S: TryInto<Uri>, E: Into<Endpoints<S>>>(endpoints: E) -> Result<Self, Error> {
        let extra = Default {
            async_client: AsyncClient::new(endpoints)?,
        };
        let state = Box::new(ClientState::new());
        Ok(Self { state, extra })
    }
}
