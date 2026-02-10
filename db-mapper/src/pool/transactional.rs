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

pub fn set_transaction_id(tx_id: &str){
    TX_ID_REGISTRY.try_with(|name| {
        *name.borrow_mut() = Some(tx_id.to_string());
    }).ok();
}