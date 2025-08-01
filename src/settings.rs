use std::time::Duration;



// Cache
pub const DEFAULT_RECORD_LIFETIME: Duration = Duration::from_secs(600);


// Ticket
pub const TICKET_LENGTH: usize = 32;
pub const TICKET_LIFETIME: usize = 5 * 60;// seconds


// JWT
pub const JWT_SECRET: &[u8] = b"supersecret";