#### 数据类型映射
| Rust类型(ParamValue枚举) | Postgres类型(Type) |
|----------------------|------------------|
| Decimal              | NUMERIC          | 
| I8                   | CHAR             |
| I16                  | INT2             |
| I32                  | INT4             |
| I64                  | INT8             |
| U8                   | CHAR             |
| U16                  | INT2             |
| U32                  | INT4             |
| U64                  | INT8             |
| F32                  | FLOAT4           |
| F64                  | FLOAT8           |
| Bool                 | BOOL             |
| String               | VARCHAR          |
| DateTime             | TIMESTAMP,DATE   |
| Blob                 | BYTEA            |
| Clob                 | TEXT             |
| Null                 | VOID             |    

