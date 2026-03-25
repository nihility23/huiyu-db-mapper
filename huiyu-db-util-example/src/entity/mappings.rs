use serde::{Deserialize, Serialize};
use huiyu_db_util::huiyu_db_macros::Mapping;

#[derive(Default,Mapping,Serialize,Deserialize)]
pub struct RoleDTO{
    pub id: Option<String>,
    pub username: Option<String>,
}