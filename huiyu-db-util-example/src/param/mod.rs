use serde::{Deserialize, Serialize};

#[derive(Serialize,Debug, Deserialize)]
pub(crate) struct UserQueryParam{
    pub(crate) current_page : Option<u32>,
    pub(crate) page_size : Option<u32>,
    pub(crate) id : Option<i64>,
    pub(crate) user_name : Option<String>,
    pub(crate) login_name : Option<String>,
}