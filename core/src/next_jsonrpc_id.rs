use crate::json_rpc::JsonRpcId;
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_ID: AtomicU64 = AtomicU64::new(1);

pub fn next_jsonrpc_id() -> JsonRpcId {
    JsonRpcId::Number(NEXT_ID.fetch_add(1, Ordering::Relaxed).into())
}

// Optional: if you ever want to reset in tests
#[cfg(test)]
pub fn reset_next_id_for_tests() {
    NEXT_ID.store(1, Ordering::SeqCst);
}
