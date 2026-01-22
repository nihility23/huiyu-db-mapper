use db_mapper::query::base_mapper::BaseMapper;
use db_mapper::base::entity::{ColumnInfo, Entity};
use db_mapper::base::page::{Page, PageRes};
use db_mapper::base::param::{get_param_value, ParamValue};
use db_mapper::query::query_wrapper::QueryWrapper;
pub struct UserMapper;

pub struct UserEntity{
    id: Option<i64>,
    name: Option<String>,
    age: Option<i32>,
    sex: Option<u8>,
}
/***
    宏生成以下代码
 */
impl Entity for UserEntity {
    type K = i64;

    fn key(&self) -> Self::K{
        self.id.unwrap().into()
    }

    fn key_name() -> &'static str{"id"}

    fn column_names() -> Vec<&'static str> {
        vec!["ID","NAME","AGE","SEX"]
    }
    fn field_names() -> Vec<&'static str> {
        vec!["id", "name", "age" , "sex"]
    }

    fn table_name() -> &'static str {
        "t_user"
    }

    fn new() -> Self {
        Self{
            id:None,
            name:None,
            age:None,
            sex:None
        }
    }

    fn get_value_by_field_name(&self, field_name: &str) -> ParamValue {
        match field_name {
            "ID"=> {
                get_param_value(self.id.clone())
            }
            "NAME"=>{
                get_param_value(self.name.clone())
            }
            "AGE"=>{
                get_param_value(self.age.clone())
            }
            "SEX"=>{
                get_param_value(self.sex.clone())
            }
            _ => ParamValue::Null,
        }
    }

    fn get_value_by_column_name(&self, column_name: &str) -> ParamValue {
        match column_name {
            "id"=> {
                get_param_value(self.id.clone())
            }
            "name"=>{
                get_param_value(self.name.clone())
            }
            "age"=>{
                get_param_value(self.age.clone())
            }
            "sex"=>{
                get_param_value(self.sex.clone())
            }
            _ => ParamValue::Null,
        }
    }


    fn set_value_by_field_name(&mut self,field_name: &str, value : ParamValue){
        match field_name {
            "ID"=> {
                self.id = value.into();
            }
            "NAME"=>{
                self.name = value.into();
            }
            "AGE"=>{
                self.age = value.into();
            }
            "SEX"=>{
                self.sex = value.into();
            }
            _ => {},
        }
    }

    fn set_value_by_column_name(&mut self, column_name: &str, value: ParamValue) {
        match column_name {
            "id"=> {
                self.id = value.into();
            }
            "name"=>{
                self.name = value.into();
            }
            "age"=>{
                self.age = value.into();
            }
            "sex"=>{
                self.sex = value.into();
            }
            _ => {},
        }
    }

    fn get_column_infos() -> Vec<ColumnInfo> {
        let column_infos = Vec::new();
        
        
        column_infos
    }
}



impl UserEntity{

    // 宏生成
    fn new() -> Self {
        Self{
            id: None,
            name: None,
            age: None,
            sex: None,
        }
    }
}

/***
    通过宏生成一下代码
***/
impl BaseMapper<UserEntity> for UserMapper {
    fn select_by_key(&self, key: &i64) -> Result<Option<UserEntity>, db_mapper::base::error::DatabaseError> {
        // select *from $table_name where $key = $key
        return Ok(None)
    }

    fn select_by_keys(&self, key: &Vec<<UserEntity as Entity>::K>) -> Result<Vec<UserEntity>, db_mapper::base::error::DatabaseError> {
        todo!()
    }

    fn delete_by_key(&self, key: &<UserEntity as Entity>::K) -> Result<u32, db_mapper::base::error::DatabaseError> {
        todo!()
    }

    fn delete_by_keys(&self, key: &Vec<<UserEntity as Entity>::K>) -> Result<u32, db_mapper::base::error::DatabaseError> {
        todo!()
    }

    fn update_by_key(&self, user_entity: &UserEntity) -> Result<u32, db_mapper::base::error::DatabaseError> {
        todo!()
    }

    fn insert(&self, entity: &UserEntity) -> Result<<UserEntity as Entity>::K, db_mapper::base::error::DatabaseError> {
        todo!()
    }

    fn insert_batch(&self, entities: &Vec<UserEntity>) -> Result<u32, db_mapper::base::error::DatabaseError> {
        todo!()
    }

    fn select_page(&self, page: Page, query_wrapper: &QueryWrapper<UserEntity>) -> Result<PageRes<UserEntity>, db_mapper::base::error::DatabaseError> {
        todo!()
    }

    fn select(&self, query_wrapper: &QueryWrapper<UserEntity>) -> Result<Option<Vec<UserEntity>>, db_mapper::base::error::DatabaseError> {
        todo!()
    }

    fn select_one(&self, query_wrapper: &QueryWrapper<UserEntity>) -> Result<Option<UserEntity>, db_mapper::base::error::DatabaseError> {
        todo!()
    }

    fn update(&self, entity: &UserEntity, query_wrapper: &QueryWrapper<UserEntity>) -> Result<u32, db_mapper::base::error::DatabaseError> {
        todo!()
    }

    fn delete(&self, query_wrapper: &QueryWrapper<UserEntity>) -> Result<u32, db_mapper::base::error::DatabaseError> {
        todo!()
    }
}

