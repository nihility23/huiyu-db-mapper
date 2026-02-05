use std::cell::RefCell;
use tokio::task_local;

task_local!{
    pub static TX_ID_REGISTRY: RefCell<Option<String>>;
}

pub fn get_transaction_id() -> Option<String> {
    if let Some(name) = TX_ID_REGISTRY.try_get().ok(){
        name.borrow().clone()
    } else {
        None
    }
}