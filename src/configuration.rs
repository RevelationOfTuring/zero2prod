use serde::Deserialize;

// 注：一个struct中所有字段都必须是可反序列化的，这样整个类型才能是可反序列化的
#[derive(Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
}

#[derive(Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }

    pub fn conncection_string_without_db(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}",
            self.username, self.password, self.host, self.port
        )
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    // 初始化配置读取器
    let setting = config::Config::builder()
        // 从configuration.yaml文件中读取配置
        .add_source(config::File::new(
            "configuration.yaml",
            config::FileFormat::Yaml,
        ))
        .build()?;
    // 尝试将读到的配置转化为Settings类型
    setting.try_deserialize::<Settings>()
}
