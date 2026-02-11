#[derive(Debug)]
pub struct Page {
    pub current_page: u64,
    pub page_size: u64,
}

#[derive(Debug)]
pub struct PageRes<T> {
    pub total_size: u64,
    pub page_size: u64,
    pub total_page: u64,
    pub records: Option<Vec<T>>,
}

impl<T> PageRes<T> {
    pub fn new() -> PageRes<T> {
        PageRes {
            total_size: 0,
            page_size: 0,
            total_page: 0,
            records: None,
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
            records: Some(records),
        }
    }
}
