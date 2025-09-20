use std::{collections::HashMap, sync::Arc};
use tokio::sync::{broadcast, RwLock};

use crate::{auth::ticket::TicketService, cache::cache::TimedCache, models::{prelude::SnowflakeGenerator, AppPool}};


pub type Clients = Arc<RwLock<HashMap<i64, broadcast::Sender<String>>>>;
pub type Users = Arc<RwLock<HashMap<i64, String>>>;


#[derive(Clone)]
pub struct AppState {
    pub clients: Clients,
    pub tickets: Arc<TicketService>,
    pub users: Users,
    pub tx: broadcast::Sender<String>,
    pub snowflake_generator: Arc<SnowflakeGenerator>,
    pub pool: AppPool,
}