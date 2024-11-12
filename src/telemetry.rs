use tracing::{
    subscriber::set_global_default,
    Subscriber,
};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

// 用约束作为返回值类型，避免写出繁琐的真实类型
// 注：需要显式地将返回类型标记为Send+Sync，以便后面可以将其传递给ini_subscriber()
pub fn get_subscriber(name: String, env_filter: String) -> impl Subscriber + Send + Sync {
    // 如果没有设置'RUST_LOG'环境变量，则输出所有'info'及以上级别的跨度
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));
    let formatting_layer = BunyanFormattingLayer::new(
        name,
        // 将格式化的跨度输出到stdout
        std::io::stdout,
    );
    // with由SubscriberExt trait提供，可以拓展tracing_subscriber的Subscriber
    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
}

// 将一个订阅器设置为全局默认值，用于处理所有跨度。
// 这个函数只可被调用一次
pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    // 将log中的记录导入trace中。用于将程序中非我们手动指定tracing发出消息（如actix-web架构中通过log输出的日志）导入到tracing的订阅器中
    LogTracer::init().expect("Failed to set logger");
    // set_global_default用于指定处理跨度的订阅器
    set_global_default(subscriber).expect("Failed to set subscriber");
}
