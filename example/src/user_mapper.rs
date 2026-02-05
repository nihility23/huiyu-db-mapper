use db_macros::datasource;

pub struct User {
    pub id: i32,
    pub name: String,
}

pub(crate) struct UserMapper;

impl UserMapper{
    #[datasource("name")]
    pub async fn find_by_id(&self, id: i32) -> Option<User> {
        None
    }


    pub(crate) async fn find_by_id11(&self, id: i32) -> Option<User> {
        db_mapper::sql::pool::datasource::DB_NAME_REGISTRY.scope(std::cell::RefCell::new(Some("name".to_string())), async {
            {
                None
            }
        }).await
    }

}