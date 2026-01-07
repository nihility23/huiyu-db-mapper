
#[derive(Debug)]
pub struct DbConfig{
    host:Option<String>,
    port:Option<u32>,
    database:Option<String>,
    username:Option<String>,
    password:Option<String>,
    schema:Option<String>,
    name:Option<String>,
}

impl DbConfig{
    pub fn new()->DbConfig{
        Self{
            host:None,
            port:None,
            database:None,
            username:None,
            password:None,
            schema:None,
            name:None,
        }
    }
}