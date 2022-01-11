use rpc::atlas::atlas_graph_client::AtlasGraphClient;
use tonic::transport::Channel;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AtlasConfig {
    // server address.
    pub address: std::net::SocketAddrV4,
    // user name
    pub username: String,
    // password
    pub password: String,
}

/// Connect to AtlasGraph
pub struct AtlasConnection {
    #[doc(alias = "rpc_client")]
    // pub client: RefCell<AtlasGraphClient<Channel>>,
    pub client: AtlasGraphClient<Channel>,
    pub session_id: Vec<u8>,
    pub config: AtlasConfig,
}

impl AtlasConnection {
    pub async fn new(config: AtlasConfig) -> Self {
        let mut client = AtlasGraphClient::connect(format!("http://{}", config.address))
            .await
            .unwrap();
        let session_id = client
            .authenticate(rpc::atlas::AuthenticateRequest {
                username: config.username.clone(),
                password: config.password.clone(),
            })
            .await
            .unwrap()
            .into_inner()
            .session_id;

        Self {
            client,
            session_id,
            config,
        }
    }

    
}
