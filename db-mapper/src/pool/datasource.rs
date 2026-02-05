use std::cell::RefCell;
use tokio::task_local;

task_local!{
    pub static DB_NAME_REGISTRY: RefCell<Option<String>>;
}

pub fn get_datasource() -> Option<String> {
    if let Some(name) = DB_NAME_REGISTRY.try_get().ok(){
        name.borrow().clone()
    } else {
        None
    }
}