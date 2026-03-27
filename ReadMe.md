# 持久层软件

## 支持数据库

  - [x] Mysql
  - [x] Sqlite
  - [x] PostgreSql
  - [x] Oracle(>12)
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
    select_impl! {

        #[select("select * from t_role where id = ?")]
        async fn query_role_dtos(id: String) -> Result<Vec<RoleDTO>, DatabaseError>;

        #[select("select * from t_role where id = ?")]
        async fn query_role_page(page: Page, name: String) -> Result<PageRes<RoleDTO>, DatabaseError>;
        
        #[select("select * from t_role where role_code = ? and status = ?")]
        async fn query_role_first(name: String, status: i8) -> Result<Option<RoleDTO>, DatabaseError>;
        
        #[select("select role_name from t_role where role_code = ? and status = ?")]
        #[value]   // 标记为简单值类型
        async fn query_role_name(name: String, status: i8) -> Result<Option<String>, DatabaseError>;
        
        #[select("select * from t_role where 1=1 and #{qw}")]
        async fn query_role_dtos_by_query_wrapper<'a>(name:String,query_wrapper: &OccupyQueryMapper<'a>) -> Result<Vec<RoleDTO>, DatabaseError>;
        
        #[select("select * from t_role  where 1=1 and #{qw}")]
        async fn query_role_page_query_wrapper<'a>(page: Page,name:String,name1:String,   query_wrapper: &OccupyQueryMapper<'a>) -> Result<PageRes<RoleDTO>, DatabaseError>;
        
        #[select("select * from t_role where name like concat('%',?@,'%') and #{qw}")]
        async fn query_role_first_query_wrapper<'a>(name:String,query_wrapper: &OccupyQueryMapper<'a>) -> Result<Option<RoleDTO>, DatabaseError>;
        
        #[select("select role_name from t_user u left join t_user_role ur on ur.user_id = u.id left join t_role r on r.id = ur.role_id where 1=1 and #{qw}")]
        #[value]   // 标记为简单值类型
        async fn query_role_name_query_wrapper<'a>(name:String,query_wrapper: &OccupyQueryMapper<'a>) -> Result<Option<String>, DatabaseError>;
        
        // 支持多个 OccupyQueryMapper 的示例
        #[select("select * from t_role where 1=1 and role_code =?# and #{qw} and #{qw}")]
        async fn query_role_by_multiple_wrappers<'a>(code:String,wrapper1: &OccupyQueryMapper<'a>, wrapper2: &OccupyQueryMapper<'a>) -> Result<Vec<RoleDTO>, DatabaseError>;
    }
    
    execute_impl!{
        #[sql("update t_role set role_code = ? where id = ? and #{qw} and #{qw}")]
        async fn update_role_code(role_code: String, role_code1: String,query_wrapper: &OccupyQueryMapper<'_>,query_wrapper1: &OccupyQueryMapper<'_>) -> Result<u64, DatabaseError>;
        #[sql("create table t_test(id: int)")]
        async fn create_table_test(id: i64) -> Result<u64, DatabaseError>;
        #[sql("CREATE TABLE Employees_?@ (
                EmployeeID INTEGER PRIMARY KEY,
                Name TEXT NOT NULL,
                Age INTEGER
            );
        ")]
        async fn create_table_employees(idx:i64) -> Result<u64, DatabaseError>;
    }
```
### 参数值支持：
#### 1.普通值.可以多个与？搭配，按顺序替换
#### 2.OccupyQueryMapper，可以多个，通过#{qw}按顺序替换
#### 3.普通值？搭配规则

    [select(select *from where id = ?)]
    ?:参与预编译，后以参数传入
    sql:select *from where id = ?  
    sql执行参数：abc

    [select(select *from t_user where id = ?#)]
    ?#：不参与预编译，先替换后执行,替换时候加单引号,传入参数("abc")
    sql:select *from where id = 'abc'
    sql执行参数：无

    [select(select *from ?@ where id = ?@)]
    ?@：不参与预编译，先替换后执行,替换时候不加单引号,传入参数("t_user","1")
    sql:select *from t_user where id = ?
    sql执行参数：1

    [select(select name from ?& where id = ?)]
    ?&：不参与预编译，先替换后执行,替换是增加双引号,传入参数("t_user","1")
    sql:select *from "t_user" where id = ?
    sql执行参数：1

返回值支持：
#### 1.查询所有
    返回值必须Result<PageRes<RoleDTO>, DatabaseError>, RoleDTO必须实现Mapping
#### 2.查询分页
    第一个参数必须为Page,返回值必须Result<PageRes<RoleDTO>, DatabaseError>, RoleDTO必须实现Mapping
#### 3.查询单个
    返回值必须为Result<Option<RoleDTO>, DatabaseError>, RoleDTO必须实现Mapping
#### 4.查询简单值
    返回值必须为Result<Option<String>, DatabaseError>，必须标记为value

## api

- [ ] 直接读取配置文件并注册数据源

