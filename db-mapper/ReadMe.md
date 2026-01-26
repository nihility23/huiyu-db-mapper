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
生成Mapper接口

##### DataSource宏
宏具有name属性，可根据name切换数据源

##### Transactional宏
可以对代码片段和方法进行事务修饰
当代码片段或者方法上没有Result的时候，捕获panic回滚
当方法有result返回的时候看其是否返回错误，有错误回滚否则提交

1.判断是否task_local中存在事务
2.如果不存在获取连接，开始事务
3.执行器执行时获取task_local中的事务
4.事务执行方法
5.提交或者回滚事务
6.清除task_local中的事务


##### ResultMap宏
对于select宏用于DTO的注释

##### Select/Update/Delete
用于自定义SQL
