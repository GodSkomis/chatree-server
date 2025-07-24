use timedmap::TimedMap;
use std::time::Duration;
use std::hash::Hash;

use crate::settings::DEFAULT_RECORD_LIFETIME;

pub trait Cache<T, U> {
    fn get(&self, key: &T) -> Option<U>;
    fn set(&self, key: T, value: U, lifetime: Option<Duration>);
    fn remove(&self, key: &T) -> Option<U>;
}

#[derive(Debug)]
pub struct TimedCache<T, U> {
    storage: TimedMap<T, U>
}

impl<T, U> TimedCache<T, U> {
    pub fn new() -> Self {
        TimedCache { 
            storage: TimedMap::new()
        }
    }

    fn get_default_lifetime() -> Duration {
        DEFAULT_RECORD_LIFETIME.clone()
    }
}


impl<T, U> Cache<T, U> for TimedCache<T, U>
where
    T: Eq + PartialEq + Hash + Clone,
    U: Clone, 
{
    fn get(&self, key: &T) -> Option<U> {
        self.storage.get(&key)
    }

    fn set(&self, key: T, value: U, lifetime: Option<Duration>) {
        self.storage.insert(
            key,
            value,
            match lifetime {
                Some(_lifetime) => _lifetime,
                None => Self::get_default_lifetime()
            });
    }
    
    fn remove(&self, key: &T) -> Option<U> {
        self.storage.remove(key)
    }
}
