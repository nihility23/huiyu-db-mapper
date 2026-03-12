use chrono::{DateTime, Local};
use huiyu_db_util::huiyu_db_macros::Entity;
use serde::{Deserialize, Serialize};
use huiyu_db_util::huiyu_db_mapper::query::base_mapper::BaseMapper;
use huiyu_db_util::huiyu_db_mapper_core::base::entity::*;
use huiyu_db_util::huiyu_db_mapper_core::base::param::ParamValue;

#[derive(Clone, Debug,Entity,Serialize,Deserialize)]
#[table(name = "t_app")]
#[derive(Default)]
pub struct AppEntity{
    #[id(column = "id", auto_increment = true)]
    pub id: Option<String>,
    #[field(column = "app_name")]
    pub app_name: Option<String>,
    #[field(column = "app_key")]
    pub app_key: Option<String>,
    #[field(column = "app_secret")]
    pub app_secret: Option<String>,
    #[field(column = "create_time")]
    pub create_time: Option<i64>,
}

#[derive(Clone, Debug,Entity,Serialize,Deserialize)]
#[table(name = "bed")]
#[derive(Default)]
pub struct BedEntity{
    #[id(column = "id", auto_increment = true)]
    pub id: Option<String>,
    #[field(column = "room_id")]
    pub room_id: Option<String>,
    #[field(column = "bed_name")]
    pub bed_name: Option<String>,
    #[field(column = "bed_type")]
    pub bed_type: Option<String>,
}

#[derive(Clone, Debug,Entity)]
#[table(name = "t_user")]
#[derive(Default)]
pub struct UserEntity{
    #[id(column = "id", auto_increment = true)]
    pub id: Option<String>,
    #[field(column = "user_name")]
    pub user_name: Option<String>,
    #[field(column = "password")]
    pub password: Option<String>,
    #[field(column = "login_name")]
    pub login_name: Option<String>,
    #[field(column = "login_name")]
    pub create_time: Option<DateTime<Local>>,
}

pub struct BedMapper;
impl BaseMapper<BedEntity> for BedMapper {}

pub struct AppMapper;

impl BaseMapper<AppEntity> for AppMapper {}

pub struct UserMapper;

impl BaseMapper<UserEntity> for UserMapper {}