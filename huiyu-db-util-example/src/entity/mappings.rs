use huiyu_db_util::huiyu_db_macros::Mapping;

#[derive(Default,Mapping)]
pub struct RoleDTO{
    pub user_name: Option<String>,
}