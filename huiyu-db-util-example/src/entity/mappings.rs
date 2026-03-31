use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use huiyu_db_util::huiyu_db_macros::Mapping;

#[derive(Default,Mapping,Serialize,Deserialize,Debug)]
pub struct RoleDTO{
    pub id: Option<String>,
    pub role_name: Option<String>,
    pub create_time: Option<DateTime<Local>>,
}