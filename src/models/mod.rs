pub mod user;
pub mod chat;
pub mod message;


pub mod prelude {
    use ferroid::{
        MonotonicClock,
        UNIX_EPOCH,
        SnowflakeDiscordId,
        AtomicSnowflakeGenerator,
        SnowflakeGeneratorAsyncTokioExt
    };
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    type SnowflakeGeneratorType = AtomicSnowflakeGenerator<SnowflakeDiscordId, MonotonicClock>;

    pub struct SnowflakeGenerator {
        generator: SnowflakeGeneratorType
    }

    impl  SnowflakeGenerator {
        pub fn new() ->  Self {
            let clock = MonotonicClock::with_epoch(UNIX_EPOCH);
            let machine_id = Self::get_machine_id();
            Self { generator: AtomicSnowflakeGenerator::new( machine_id, clock )}
        }

        fn get_machine_id() -> u64 {
            let machine_id: String = machine_uid::get().unwrap();
            let mut hasher = DefaultHasher::new();
            machine_id.hash(&mut hasher);
            hasher.finish()
        }

        async fn generate_id(&self) -> SnowflakeDiscordId {
            self.generator.try_next_id_async().await.unwrap()
        }
    }

}