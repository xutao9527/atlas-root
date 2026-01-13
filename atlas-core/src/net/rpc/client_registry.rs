use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::net::rpc::client::client::AtlasRpcClient;
use crate::net::rpc::router::AtlasModuleId;

pub type RpcClientRef = Arc<AtlasRpcClient>;


#[derive(Clone, Default)]
pub struct RpcClientRegistry {
    clients: Arc<RwLock<HashMap<AtlasModuleId, RpcClientRef>>>, // module_id -> client
}

impl RpcClientRegistry {
    pub fn new() -> Self {
        Self {
            clients: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn register(&self, module_id: AtlasModuleId, client: AtlasRpcClient) {
        let mut map = self.clients.write().await;
        map.insert(module_id, Arc::new(client));
    }

    pub async fn get(&self, module_id: AtlasModuleId) -> Option<RpcClientRef> {
        let map = self.clients.read().await;
        map.get(&module_id).cloned()
    }

    pub async fn contains(&self, module_id: AtlasModuleId) -> bool {
        let map = self.clients.read().await;
        map.contains_key(&module_id)
    }
}