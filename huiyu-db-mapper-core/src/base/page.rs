use serde::{Deserialize, Serialize};

#[derive(Debug,Copy, Clone, Serialize, Deserialize)]
pub struct Page {
    pub current_page: u64,
    pub page_size: u64,
}

impl Page {
    pub fn new(current_page: u64, page_size: u64) -> Page {
        Page{current_page,page_size,}
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageRes<T> {
    pub total_size: u64,
    pub page_size: u64,
    pub total_page: u64,
    pub records: Vec<T>,
}

impl<T> PageRes<T> {
    pub fn new() -> PageRes<T> {
        PageRes {
            total_size: 0,
            page_size: 0,
            total_page: 0,
            records: Vec::new(),
        }
    }

    pub fn new_from_records(total_size: u64, page_size: u64, records: Vec<T>) -> PageRes<T> {
        let mut total_page = 0;
        if total_size > 0 && page_size > 0 {
            total_page = (total_size + page_size - 1) / page_size;
        }
        PageRes {
            total_size,
            page_size,
            total_page,
            records,
        }
    }
}
