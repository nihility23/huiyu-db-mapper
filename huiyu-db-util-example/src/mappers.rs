use huiyu_db_util::huiyu_db_mapper::query::base_mapper::BaseMapper;
use crate::entities::{UserEntity, UserRoleEntity, RoleEntity, RolePermissionEntity, PermissionEntity};

pub struct UserMapper;

impl BaseMapper<UserEntity> for UserMapper {}

pub struct UserRoleMapper;

impl BaseMapper<UserRoleEntity> for UserMapper {}

pub struct RoleMapper;

impl BaseMapper<RoleEntity> for UserMapper {}

pub struct RolePermissionMapper;

impl BaseMapper<RolePermissionEntity> for UserMapper {}

pub struct PermissionMapper;

impl BaseMapper<PermissionEntity> for UserMapper {}