// use std::sync::OnceLock;
// use tokio::sync::Semaphore;

// static SEMAPHORE: OnceLock<Semaphore> = OnceLock::new();

// pub fn get_semaphore() -> &'static Semaphore {
//     SEMAPHORE.get_or_init(|| Semaphore::new(3))
// }


use std::{collections::HashMap, sync::OnceLock};

use tokio::sync::Semaphore;

static PROVIDER_SEMAPHORES: OnceLock<HashMap<&'static str, Semaphore>> = OnceLock::new();

pub fn get_semaphores() -> &'static HashMap<&'static str, Semaphore> {
    PROVIDER_SEMAPHORES.get_or_init(|| {
        let mut map = HashMap::new();
        map.insert("openai", Semaphore::new(3));
        map.insert("anthropic", Semaphore::new(3));
        map.insert("free", Semaphore::new(3)); // very conservative for free models
        map
    })
}

pub fn get_provider(model: &str) -> &'static str {
    if model.ends_with(":free") {
        "free"
    } else if model.starts_with("anthropic/") {
        "anthropic"
    } else {
        "openai"
    }
}
