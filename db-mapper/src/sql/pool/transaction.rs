use std::cell::RefCell;
use tokio::task_local;

task_local!{
    static TX_ID_REGISTRY: RefCell<Option<String>>;
}
#[macro_export]
macro_rules! datasource {
    // 基础版本：使用外部函数
    (#[datasource(name = $name:expr)] $vis:vis fn $method_name:ident($self:ident: &$self_type:ty $(, $arg:ident : $arg_ty:ty)*) $(-> $ret:ty)? $body:block) => {
        $vis fn $method_name($self: &$self_type $(, $arg: $arg_ty)*) $(-> $ret)? {
            // 在方法执行前调用指定的函数
            if TX_ID_REGISTRY.try_get().is_err(){
                USER_NAME.scope(RefCell::new(Some($name)), async {
                    $body.await;
                }).await;
            }
        }
    };
}

