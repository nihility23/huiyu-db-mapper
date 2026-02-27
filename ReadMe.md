### 持久层软件

#### 支持数据库

-[x] Mysql
-[x] Oracle
-[x] Sqlite
-[x] PostgreSql
-[x] SqlServer
-[ ] Dm
-[ ] GaussDB
-[ ] KingbaseES




#### api
-[x] QueryWrapper
-[ ] 事务(transactional宏)
-[ ] Select/Update/Delete/Insert宏(类似于select注解)
-[ ] 多数据源切换(threadlocal结合datasource宏)
-[ ] 类似于xml的方式，整合到注解内<if test=""></if>标签
-[ ] features排除不需要的数据库依赖
-[ ] 引入async/await实现异步非阻塞


#### 约束
    1.数据库操作都得在spawn_block里面完成，tx和conn非线程安全cannot be shared between threads safely
    2.事务集合操作，必须作为参数传入，否则move后(scope等)不能正常提交