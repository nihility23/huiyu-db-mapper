# 持久层软件

## 支持数据库

  - [x] Sqlite
  - [x] Mysql
  - [x] PostgreSql
  - [x] Oracle(>12)

## 功能
 - [x] 多数据源切换
 - [x] 事务
 - [x] 分页
 - [x] 查询
 - [x] 更新
 - [x] 删除
 - [x] 插入
 - [x] 自定义查询

### 引入
    [workspace.dependencies]
    huiyu-db-mapper = {version = "0.1.0", features = ["sqlite"]}
### 注册数据源
```aiignore
fn init_postgres(){
    println!("init postgres");
    let db_config_postgres = DbConfig::new(DbType::Postgres,
                                           "postgres".to_string(),
                                           Some("10.150.2.200".to_string()),
                                           Some(5432),
                                           Some("postgres".to_string()),
                                           Some("123456".to_string()),
                                           Some("huiyu".to_string()),
                                           Some("public".to_string()),
    );
    DbTypeWrapper::register_dbs(vec![db_config_postgres]).expect("Failed to register db");
}
fn init_mysql(){
    println!("init mysql");
    let db_config_mysql = DbConfig::new(DbType::Mysql,
                                        "mysql".to_string(),
                                        Some("10.150.6.7".to_string()),
                                        Some(3306),
                                        Some("root".to_string()),
                                        Some("123456".to_string()),
                                        Some("huiyu".to_string()),
                                        Some("".to_string()),
    );
    DbTypeWrapper::register_dbs(vec![db_config_mysql]).expect("Failed to register db");
}
fn init_sqlite(){
    println!("init sqlite");
    let db_config_sqlite = DbConfig::new(
        DbType::Sqlite,
        "sqlite".to_string(),
        None, None, None, None,
        Some("E:\\test\\huiyu.db".to_string()), None
    );
    DbTypeWrapper::register_dbs(vec![db_config_sqlite]).expect("Failed to register db");
}

fn init_oracle(){
    println!("init oracle");
    let db_config_oracle = DbConfig::new(DbType::Oracle,
                                        "oracle".to_string(),
                                        Some("10.150.6.7".to_string()),
                                        Some(1521),
                                        Some("huiyu".to_string()),
                                        Some("123456".to_string()),
                                        Some("orcl".to_string()),
                                        None,
    );
    DbTypeWrapper::register_dbs(vec![db_config_oracle]).expect("Failed to register db");
}
```

### 查询功能
#### QueryWrapper    查询条件构造器

| 名称 | 用法 |
|------|------|
| eq | 等于 (=) |
| ne | 不等于 (<>) |
| lt | 小于 (<) |
| le | 小于等于 (<=) |
| gt | 大于 (>) |
| ge | 大于等于 (>=) |
| between | 在两者之间 (between ... and ...) |
| like | 模糊匹配，支持通配符 (like '%...%') |
| likeLeft | 左模糊匹配 (like '%...') |
| likeRight | 右模糊匹配 (like '...%') |
| notLike | 不匹配指定模式 (not like) |
| notLikeLeft | 不匹配左模糊模式 (not like '%...') |
| notLikeRight | 不匹配右模糊模式 (not like '...%') |
| in | 在指定集合中 (in (...)) |
| inSql | 使用子查询作为 in 的条件 (in (select ...)) |
| notIn | 不在指定集合中 (not in (...)) |
| notInSql | 使用子查询作为 not in 的条件 (not in (select ...)) |
| isNotNull | 不为空 (is not null) |
| isNull | 为空 (is null) |
| applySql | 直接拼接自定义 SQL 片段 |
| existsSql | 使用 exists 子查询 (exists (select ...)) |
| notExistsSql | 使用 not exists 子查询 (not exists (select ...)) |
| or_wrapper | 以 OR 逻辑包裹一组条件 |
| and_wrapper | 以 AND 逻辑包裹一组条件 |
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
select id,role_name,role_code,description,sort_order,status,is_system,create_time,update_time from 
t_role where id = ?  AND  role_name != ?  AND  status > ?  AND  status < ?  AND  (id = ?  OR  role_name = ?) order by status DESC
```

## 宏

### mapper宏
```aiignore
#[mapper(RoleEntity)]
pub struct RoleMapper;
```
这样该mapper就具有以下方法,其中RoleEntity是实体类，由Entity宏标注
```aiignore
// select * from $table_name where $id = ?
async fn select_by_key(key: &E::K) -> Result<Option<E>, DatabaseError>;

// select * from $table_name where $id in (?,...)
async fn select_by_keys(keys: &Vec<E::K>) -> Result<Vec<E>, DatabaseError>;

// delete from $table_name where $id = ?
async fn delete_by_key(key: &E::K) -> Result<u64, DatabaseError>;

// delete from $table_name where $id in (?,...)
async fn delete_by_keys(keys: &Vec<E::K>) -> Result<u64, DatabaseError>;

// update $table_name set $column_name = ? where id = ?
async fn update_by_key(e: &E) -> Result<u64, DatabaseError>;

// insert $table_name into ($id,$column,...) values (?,?,...)
async fn insert(e: &mut E) -> Result<Option<E::K>, DatabaseError>;

// insert $table_name into ($id,$column,...) values (?,?,...),(?,?,...)
async fn insert_batch(entities: Vec<E>) -> Result<u64, DatabaseError>;

// select count(*) from (select * from $table_name where $column = ? ...)
// select * from $table_name where $column = ? ... limit ?,?
async fn select_page<'a>(page: Page,query_wrapper: &QueryWrapper<'a, E>,) -> Result<PageRes<E>, DatabaseError>;

// select * from $table_name where $column = ? ...
async fn select<'a>(query_wrapper: &QueryWrapper<'a, E>,) -> Result<Vec<E>, DatabaseError>;

// select * from $table_name where $column = ? ... limit 1
async fn select_one<'a>(query_wrapper: &QueryWrapper<'a, E>,) -> Result<Option<E>, DatabaseError>;

// update $table_name set $column_name = ? where $column = ? ...
async fn update<'a>(e: &E, query_wrapper: &QueryWrapper<'a, E>,) -> Result<u64, DatabaseError>;

// update $table_name set $column_name = ? where $column = ? ...
async fn update_with_null<'a>(&self, e: &E, query_wrapper: &QueryWrapper<'a, E>,) -> Result<u64, DatabaseError>;

// delete from $table_name where $column = ? ...
async fn delete<'a>(query_wrapper: &QueryWrapper<'a, E>) -> Result<u64, DatabaseError>;
```


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
use huiyu_db_mapper::huiyu_db_mapper_macros::Mapping;

#[derive(Default,Mapping,Serialize,Deserialize)]
pub struct RoleDTO{
    pub id: Option<String>,
    pub username: Option<String>,
}
```
必须是struct，成员必须Option,搭配select_impl使用
### Entity           实体宏
```aiignore
use huiyu_db_mapper::huiyu_db_mapper_macros::Entity;
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
impl RoleMapper {
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
        
        #[select("select * from t_role where 1=1  #{qw}")]
        async fn query_role_dtos_by_query_wrapper<'a>(name:String,query_wrapper: &OccupyQueryMapper<'a>) -> Result<Vec<RoleDTO>, DatabaseError>;
        
        #[select("select * from t_role  where 1=1  #{qw}")]
        async fn query_role_page_query_wrapper<'a>(page: Page,name:String,name1:String,   query_wrapper: &OccupyQueryMapper<'a>) -> Result<PageRes<RoleDTO>, DatabaseError>;
        
        #[select("select * from t_role where name like concat('%',?@,'%')  #{qw}")]
        async fn query_role_first_query_wrapper<'a>(name:String,query_wrapper: &OccupyQueryMapper<'a>) -> Result<Option<RoleDTO>, DatabaseError>;
        
        #[select("select role_name from t_user u left join t_user_role ur on ur.user_id = u.id left join t_role r on r.id = ur.role_id where 1=1 and #{qw}")]
        #[value]   // 标记为简单值类型
        async fn query_role_name_query_wrapper<'a>(name:String,query_wrapper: &OccupyQueryMapper<'a>) -> Result<Option<String>, DatabaseError>;
        
        // 支持多个 OccupyQueryMapper 的示例
        #[select("select * from t_role where 1=1 and role_code =?#  #{qw}  #{qw}")]
        async fn query_role_by_multiple_wrappers<'a>(code:String,wrapper1: &OccupyQueryMapper<'a>, wrapper2: &OccupyQueryMapper<'a>) -> Result<Vec<RoleDTO>, DatabaseError>;
    }
    
    execute_impl!{
        #[sql("update t_role set role_code = ? where id = ?  #{qw}  #{qw}")]
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
}
```
### 例子地址

https://github.com/nihility23/huiyu-db-mapper/tree/master/huiyu-db-mapper-example
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

    [select(select *from ?@ where id = ?)]
    ?@：不参与预编译，先替换后执行,替换时候不加单引号,传入参数("t_user","1")
    sql:select *from t_user where id = ?
    sql执行参数：1

    [select(select name from ?& where id = ?)]
    ?&：不参与预编译，先替换后执行,替换是增加双引号,传入参数("t_user","1")
    sql:select *from "t_user" where id = ?
    sql执行参数：1

### 返回值支持：
#### 1.查询所有
    返回值必须Result<PageRes<T>, DatabaseError>, T必须实现Mapping
#### 2.查询分页
    第一个参数必须为Page,返回值必须Result<PageRes<T>, DatabaseError>, T必须实现Mapping
#### 3.查询单个
    返回值必须为Result<Option<T>, DatabaseError>, T必须实现Mapping
#### 4.查询简单值
    返回值必须为Result<Option<String>, DatabaseError>，必须标记为value

## 未来支持

- [ ] 直接读取配置文件并注册数据源
- [ ] 优化默认数据源

## 注意事项
#### 1.实体及映射成员必须Option包裹
#### 2.事务宏必须返回Result<T,DatasourceError>，以确定是否回滚
#### 3.必须有个名称为default的数据源，后期考虑优化，找不到对应名称使用第一个注册的数据源
