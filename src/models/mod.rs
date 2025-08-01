use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::AsyncPgConnection;

pub mod user;
pub mod chat;
pub mod message;


pub type AppPool = Pool<AsyncPgConnection>;


pub mod prelude {
    use ferroid::{
        MonotonicClock,
        UNIX_EPOCH,
        SnowflakeDiscordId,
        AtomicSnowflakeGenerator,
        SnowflakeGeneratorAsyncTokioExt
    };
    use std::collections::hash_map::DefaultHasher;
    use std::fmt::Debug;
    use std::hash::{Hash, Hasher};

    type SnowflakeGeneratorType = AtomicSnowflakeGenerator<SnowflakeDiscordId, MonotonicClock>;

    pub struct SnowflakeGenerator {
        generator: SnowflakeGeneratorType,
        machine_id: u64
    }

    impl SnowflakeGenerator {
        pub fn new() ->  Self {
            let clock = MonotonicClock::with_epoch(UNIX_EPOCH);
            let machine_id = Self::get_machine_id();
            Self {
                generator: AtomicSnowflakeGenerator::new( machine_id.clone(), clock ),
                machine_id: machine_id
            }
        }

        fn get_machine_id() -> u64 {
            let machine_id: String = machine_uid::get().unwrap();
            let mut hasher = DefaultHasher::new();
            machine_id.hash(&mut hasher);
            hasher.finish() & 0b1111111111
        }

        pub async fn generate_id(&self) -> i64 {
            self.generator.try_next_id_async().await.unwrap().to_raw() as i64
        }
    }

    impl Debug for SnowflakeGenerator {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "SnowflakeGenerator MachineID: {}, ", self.machine_id)
        }
    }

}