#### 配置说明
    1.每个数据源有一个连接池
    2.每个类型的数据源可能有多个
    3.可通过宏切换数据源
    4.可通过宏增加事务

#### 支持说明
    mysql
    oracle
    pg
    sqlserver
    sqlite

#### 条件控制
    通过feature控制是否引入某种数据库

#### 宏说明
###### Entity宏
    1.生成列名，表名常量
    2.生成列名方法

##### Mapper宏

##### DataSource宏

##### Transactional宏