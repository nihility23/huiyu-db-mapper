use chrono::{DateTime, Local};
use huiyu_db_util::huiyu_db_macros::Entity;
use serde::{Deserialize, Serialize};

// 用户实体
#[derive(Clone, Debug, Default, Serialize, Deserialize, Entity)]
#[table(name = "T_USER", case_sensitive=true)]
pub struct UserEntity {
    #[id(column = "ID", auto_increment = true, key_generate_type = "auto_increment")]
    pub id: Option<i64>,  // 自增主键，使用i64类型,数据库返回i64，确保兼容性，不能丢失精度

    #[field(column = "USERNAME")]
    pub username: Option<String>,  // 用户名

    #[field(column = "PASSWORD")]
    pub password: Option<String>,  // 密码

    #[field(column = "REAL_NAME")]
    pub real_name: Option<String>,  // 真实姓名

    #[field(column = "EMAIL")]
    pub email: Option<String>,  // 邮箱

    #[field(column = "PHONE")]
    pub phone: Option<String>,  // 手机号

    #[field(column = "GENDER")]
    pub gender: Option<i8>,  // 性别：0-未知 1-男 2-女

    #[field(column = "BIRTHDAY")]
    pub birthday: Option<DateTime<Local>>,  // 生日，日期类型

    #[field(column = "HEIGHT")]
    pub height: Option<f32>,  // 身高，小数类型

    #[field(column = "IS_ACTIVE")]
    pub is_active: Option<bool>,  // 是否激活，布尔类型

    #[field(column = "LOGIN_COUNT")]
    pub login_count: Option<i32>,  // 登录次数

    #[field(column = "LAST_LOGIN_TIME")]
    pub last_login_time: Option<DateTime<Local>>,  // 最后登录时间

    #[field(column = "PROFILE")]
    pub profile: Option<String>,  // 个人简介，文本类型

    #[field(column = "CREATE_TIME")]
    pub create_time: Option<DateTime<Local>>,  // 创建时间

    #[field(column = "UPDATE_TIME")]
    pub update_time: Option<DateTime<Local>>,  // 更新时间
}

// 角色实体
#[derive(Clone, Debug, Default, Serialize, Deserialize, Entity)]
#[table(name = "t_role")]
pub struct RoleEntity {
    #[id(column = "id")]
    pub id: Option<String>,  // 角色ID，varchar主键

    #[field(column = "role_name")]
    pub role_name: Option<String>,  // 角色名称

    #[field(column = "role_code")]
    pub role_code: Option<String>,  // 角色编码

    #[field(column = "description")]
    pub description: Option<String>,  // 角色描述

    #[field(column = "sort_order")]
    pub sort_order: Option<i32>,  // 排序序号

    #[field(column = "status")]
    pub status: Option<i8>,  // 状态：0-禁用 1-启用

    #[field(column = "is_system")]
    pub is_system: Option<bool>,  // 是否系统内置

    #[field(column = "create_time")]
    pub create_time: Option<DateTime<Local>>,  // 创建时间

    #[field(column = "update_time")]
    pub update_time: Option<DateTime<Local>>,  // 更新时间
}

// 权限实体
#[derive(Clone, Debug, Default, Serialize, Deserialize, Entity)]
#[table(name = "t_permission")]
pub struct PermissionEntity {
    #[id(column = "id")]
    pub id: Option<String>,  // 权限ID，varchar主键

    #[field(column = "perm_name")]
    pub perm_name: Option<String>,  // 权限名称

    #[field(column = "perm_code")]
    pub perm_code: Option<String>,  // 权限编码

    #[field(column = "perm_type")]
    pub perm_type: Option<i8>,  // 权限类型：1-菜单 2-按钮 3-接口

    #[field(column = "parent_id")]
    pub parent_id: Option<String>,  // 父权限ID

    #[field(column = "path")]
    pub path: Option<String>,  // 路由路径

    #[field(column = "component")]
    pub component: Option<String>,  // 组件路径

    #[field(column = "icon")]
    pub icon: Option<String>,  // 图标

    #[field(column = "sort_order")]
    pub sort_order: Option<i32>,  // 排序序号

    #[field(column = "visible")]
    pub visible: Option<bool>,  // 是否可见

    #[field(column = "create_time")]
    pub create_time: Option<DateTime<Local>>,  // 创建时间

    #[field(column = "update_time")]
    pub update_time: Option<DateTime<Local>>,  // 更新时间
}

// 用户-角色关联实体
#[derive(Clone, Debug, Default, Serialize, Deserialize, Entity)]
#[table(name = "t_user_role",case_sensitive=true)]
pub struct UserRoleEntity {
    #[id(column = "id")]
    pub id: Option<String>,  // 关联ID，varchar主键

    #[field(column = "user_id")]
    pub user_id: Option<i64>,  // 用户ID

    #[field(column = "role_id")]
    pub role_id: Option<String>,  // 角色ID

    #[field(column = "create_time")]
    pub create_time: Option<DateTime<Local>>,  // 创建时间
}

// 角色-权限关联实体
#[derive(Clone, Debug, Default, Serialize, Deserialize, Entity)]
#[table(name = "t_role_permission")]
pub struct RolePermissionEntity {
    #[id(column = "id")]
    pub id: Option<String>,  // 关联ID，varchar主键

    #[field(column = "role_id")]
    pub role_id: Option<String>,  // 角色ID

    #[field(column = "permission_id")]
    pub permission_id: Option<String>,  // 权限ID

    #[field(column = "create_time")]
    pub create_time: Option<DateTime<Local>>,  // 创建时间
}

// 为了方便使用，可以定义一些枚举类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gender {
    Unknown = 0,
    Male = 1,
    Female = 2,
}

impl Gender {
    pub fn from_i8(value: i8) -> Option<Self> {
        match value {
            0 => Some(Gender::Unknown),
            1 => Some(Gender::Male),
            2 => Some(Gender::Female),
            _ => None,
        }
    }

    pub fn to_i8(&self) -> i8 {
        *self as i8
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PermissionType {
    Menu = 1,
    Button = 2,
    Api = 3,
}

impl PermissionType {
    pub fn from_i8(value: i8) -> Option<Self> {
        match value {
            1 => Some(PermissionType::Menu),
            2 => Some(PermissionType::Button),
            3 => Some(PermissionType::Api),
            _ => None,
        }
    }

    pub fn to_i8(&self) -> i8 {
        *self as i8
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Disabled = 0,
    Enabled = 1,
}


// 可以添加一些实用的方法
impl UserEntity {
    // 检查用户是否激活
    pub fn is_active(&self) -> bool {
        self.is_active.unwrap_or(false)
    }

    // 获取完整姓名
    pub fn get_display_name(&self) -> String {
        self.real_name.clone().unwrap_or_else(|| self.username.clone().unwrap_or_default())
    }
}

// 如果需要使用Cargo.toml依赖，可以添加以下内容：
/*
[dependencies]
chrono = { version = "0.4", features = ["serde"] }
serde = { version = "1.0", features = ["derive"] }
*/