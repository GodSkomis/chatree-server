use std::{collections::HashMap, sync::Arc};

use tokio::sync::{broadcast, RwLock};
use uuid::Uuid;

use crate::cache::cache::TimedCache;



pub type Clients = Arc<RwLock<HashMap<Uuid, broadcast::Sender<String>>>>;
pub type Tickets = Arc<RwLock<TimedCache<Uuid, String>>>;
pub type Users = Arc<RwLock<HashMap<Uuid, String>>>;


#[derive(Debug, Clone)]
pub struct AppState {
    pub clients: Clients,
    pub tickets: Tickets,
    pub users: Users,
    pub tx: broadcast::Sender<String>,
}