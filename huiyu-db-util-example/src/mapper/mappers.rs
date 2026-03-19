use huiyu_db_util::huiyu_db_mapper::query::base_mapper::BaseMapper;
use crate::entity::entities::{UserEntity, UserRoleEntity, RoleEntity, RolePermissionEntity, PermissionEntity};

pub struct UserMapper;

impl BaseMapper<UserEntity> for UserMapper {}

pub struct UserRoleMapper;

impl BaseMapper<UserRoleEntity> for UserRoleMapper {}

pub struct RoleMapper;

impl BaseMapper<RoleEntity> for RoleMapper {}

pub struct RolePermissionMapper;

impl BaseMapper<RolePermissionEntity> for RolePermissionMapper {}

pub struct PermissionMapper;

impl BaseMapper<PermissionEntity> for PermissionMapper {}