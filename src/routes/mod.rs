mod health_check;
mod subscriptions;

// 注：
// - mod health_check + pub use health_check::函数名A：别的模块可以直接以crate::routes::函数名A的方式调用该函数
// - 如果pub mod health_check：别的模块需要以crate::routes::health_check::函数名A的方式调用该函数
pub use health_check::*;
pub use subscriptions::*;
