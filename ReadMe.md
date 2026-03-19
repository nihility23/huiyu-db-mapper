### 持久层软件

#### 支持数据库

  - [x] Mysql
  - [x] Sqlite
  - [x] PostgreSql
  - [ ] Oracle
  - [ ] SqlServer
  - [ ] Dm
  - [ ] GaussDB
  - [ ] KingbaseES

#### 宏
    
 - [x] transactional
 - [x] datasource
 - [x] Entity

#### api
- [x] QueryWrapper
- [x] 事务(transactional宏)
- [x] 多数据源切换(threadlocal结合datasource宏)
- [x] features排除不需要的数据库依赖
- [x] 引入async/await实现异步非阻塞
- [ ] 类似于xml的方式，整合到注解内<if test=""></if>标签
- [ ] Select/Update/Delete/Insert宏(类似于select注解)
- [ ] Select注解时候支持分页参数