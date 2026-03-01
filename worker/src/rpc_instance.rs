use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;

use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RpcInstanceHandle {
    pub id: String,
}

impl RpcInstanceHandle {
    pub fn new(id: impl Into<String>) -> Self {
        Self { id: id.into() }
    }
}

#[derive(Debug, Default)]
pub struct RpcInstanceRegistry {
    next_id: AtomicU64,
    instances: Mutex<HashMap<String, JsValue>>,
}

impl RpcInstanceRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&self, instance: JsValue) -> RpcInstanceHandle {
        let id = format!("rpc-instance-{}", self.next_id.fetch_add(1, Ordering::Relaxed));
        self.instances
            .lock()
            .expect("lock instance registry")
            .insert(id.clone(), instance);
        RpcInstanceHandle { id }
    }

    pub fn get(&self, handle: &RpcInstanceHandle) -> Option<JsValue> {
        self.instances
            .lock()
            .expect("lock instance registry")
            .get(&handle.id)
            .cloned()
    }

    pub fn release(&self, handle: &RpcInstanceHandle) -> bool {
        self.instances
            .lock()
            .expect("lock instance registry")
            .remove(&handle.id)
            .is_some()
    }
}

#[cfg(all(test, target_arch = "wasm32"))]
mod tests {
    use super::*;

    #[test]
    fn register_get_release_cycle() {
        let registry = RpcInstanceRegistry::new();
        let instance = JsValue::UNDEFINED;

        let handle = registry.register(instance.clone());
        let fetched = registry.get(&handle).expect("instance exists");
        assert!(fetched.is_undefined());

        assert!(registry.release(&handle));
        assert!(registry.get(&handle).is_none());
    }

    #[test]
    fn double_release_returns_false_second_time() {
        let registry = RpcInstanceRegistry::new();
        let handle = registry.register(JsValue::UNDEFINED);

        assert!(registry.release(&handle));
        assert!(!registry.release(&handle));
    }
}
