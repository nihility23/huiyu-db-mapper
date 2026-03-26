# 持久层软件

## 支持数据库

  - [x] Mysql
  - [x] Sqlite
  - [x] PostgreSql
  - [x] Oracle(>12)
  - [ ] SqlServer
  - [ ] Dm
  - [ ] GaussDB
  - [ ] KingbaseES

## 功能
 - [x] 多数据源切换
 - [x] 事务
 - [x] 分页
 - [x] 查询
 - [x] 更新
 - [x] 删除
 - [x] 插入

### 查询功能
#### QueryWrapper    查询条件构造器
    eq
    ne
    lt
    le
    gt
    ge
    between
    like
    likeLeft
    likeRight
    notLike
    notLikeLeft
    notLikeRight
    in
    inSql
    notIn
    notInSql
    isNotNull
    isNull
    applySql
    existsSql
    notExistsSql
    
    or_wrapper
    and_wrapper
```aiignore
    let mut query_wrapper = QueryWrapper::<RoleEntity>::new();
    query_wrapper = query_wrapper.eq("id", 1);
    query_wrapper = query_wrapper.ne(RoleEntity::ROLE_NAME, "role_001");
    query_wrapper = query_wrapper.gt(RoleEntity::STATUS, 0);
    query_wrapper = query_wrapper.lt(RoleEntity::STATUS, 1);
    query_wrapper = query_wrapper.order_by(RoleEntity::STATUS, false);
    
    query_wrapper = query_wrapper.or_wrapper(|mut query_wrapper1| {
        query_wrapper1 = query_wrapper1.eq(RoleEntity::ID, 1);
        query_wrapper1 = query_wrapper1.eq(RoleEntity::ROLE_NAME, 1);
        query_wrapper1
    });
```
生成sql:
```
select id,role_name,role_code,description,sort_order,status,is_system,create_time,update_time from t_role where id = ?  AND  role_name != ?  AND  status > ?  AND  status < ?  AND  (id = ?  OR  role_name = ?) order by status DESC
```

## 宏
    
### transactional    事务宏
```aiignore

    let res = transactional!({
        UserRoleMapper::insert(&mut user_role1).await?;
        UserRoleMapper::insert(&mut user_role2).await?;
        Ok(())
    })?;
```
### datasource       数据源宏
```
#[datasource("sqlite")]
async fn queries()->Result<(),DatabaseError>{
}
```
queries方法里面的数据库操作都会使用sqlite名称的数据源
### Mapping          查询返回映射宏
```aiignore
use huiyu_db_util::huiyu_db_macros::Mapping;

#[derive(Default,Mapping,Serialize,Deserialize)]
pub struct RoleDTO{
    pub id: Option<String>,
    pub username: Option<String>,
}
```
必须是struct，成员必须Option,搭配select_impl使用
### Entity           实体宏
```aiignore
use huiyu_db_util::huiyu_db_macros::Entity;
#[derive(Clone, Debug, Default, Serialize, Deserialize, Entity)]
#[table(name = "t_role")]
pub struct RoleEntity {
    #[id(column = "id")]
    pub id: Option<String>,  // 角色ID，varchar主键

    #[field(column = "role_name")]
    pub role_name: Option<String>,  // 角色名称

    #[field(column = "role_code")]
    pub role_code: Option<String>,  // 角色编码
}
```
指定表名，指定id和列名

### select_impl      自定义查询宏
```aiignore
impl BaseMapper<RoleEntity> for RoleMapper {}
impl RoleMapper {
    select_impl! {
        #[select("select * from t_user where id = ?")]
        async fn query_role_dtos(id: String) -> Result<Vec<RoleDTO>, DatabaseError>;

        #[select("select * from t_user where id = ?")]
        async fn query_role_page(page: Page, name: String) -> Result<PageRes<RoleDTO>, DatabaseError>;

        #[select("select * from t_user where role_code = ? and status = ?")]
        async fn query_role_first(name: String, status: i8) -> Result<Option<RoleDTO>, DatabaseError>;

        #[select("select role_name from t_user where role_code = ? and status = ?")]
        #[value]   // 标记为简单值类型
        async fn query_role_name(name: String, status: i8) -> Result<Option<String>, DatabaseError>;
        
        
        #[select("select * from t_user  #{query_wrapper}")]
        async fn query_role_dtos_by_query_wrapper(query_wrapper: OccpyQueryWrapper) -> Result<Vec<RoleDTO>, DatabaseError>;

        #[select("select * from t_user  #{query_wrapper}")]
        async fn query_role_page_query_wrapper(page: Page, query_wrapper: OccpyQueryWrapper) -> Result<PageRes<RoleDTO>, DatabaseError>;


        #[select("select * from t_user #{query_wrapper}")]
        async fn query_role_first_query_wrapper(query_wrapper: OccpyQueryWrapper) -> Result<Option<RoleDTO>, DatabaseError>;

        #[select("select role_name from t_user #{query_wrapper}")]
        #[value]   // 标记为简单值类型
        async fn query_role_name_query_wrapper(query_wrapper: OccpyQueryWrapper) -> Result<Option<String>, DatabaseError>;
    }
}
```
支持四种：
#### 1.查询所有
    返回值必须Result<PageRes<RoleDTO>, DatabaseError>, RoleDTO必须实现Mapping
#### 2.查询分页
    第一个参数必须为Page,返回值必须Result<PageRes<RoleDTO>, DatabaseError>, RoleDTO必须实现Mapping
#### 3.查询单个
    返回值必须为Result<Option<RoleDTO>, DatabaseError>, RoleDTO必须实现Mapping
#### 4.查询简单值
    返回值必须为Result<Option<String>, DatabaseError>，必须标记为value

#### api

- [ ] 类似于xml的方式，整合到注解内<if test=""></if>标签，考虑复用QueryWrapper使用占位符替代if
- [ ] 直接读取配置文件并注册数据源

