use std::{env, time::Duration};



// Cache
pub const DEFAULT_RECORD_LIFETIME: Duration = Duration::from_secs(600);


// Ticket
pub const TICKET_LENGTH: usize = 32;
pub const TICKET_LIFETIME: usize = 5 * 5 * 60;// seconds


// Authorization
pub const AUTHORIZATION_HEADER: &str = "Authorization";


// Postgres
pub const MAX_CONNECTIONS: u32 = 5;
