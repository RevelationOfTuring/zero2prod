use secrecy::{ExposeSecret, Secret};
use serde::Deserialize;

// 注：一个struct中所有字段都必须是可反序列化的，这样整个类型才能是可反序列化的
#[derive(Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application: ApplicationSettings,
}

#[derive(Deserialize)]
pub struct ApplicationSettings {
    pub port: u16,
    pub host: String,
}

#[derive(Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    // 密码设置为隐私数据
    pub password: Secret<String>,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> Secret<String> {
        // 将数据库url变成隐私数据
        Secret::new(format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username,
            // 使用expose_secret暴露得到隐私内部数据
            self.password.expose_secret(),
            self.host,
            self.port,
            self.database_name
        ))
    }

    pub fn conncection_string_without_db(&self) -> Secret<String> {
        // 将数据库url变成隐私数据
        Secret::new(format!(
            "postgres://{}:{}@{}:{}",
            self.username,
            // 使用expose_secret暴露得到隐私内部数据
            self.password.expose_secret(),
            self.host,
            self.port
        ))
    }
}

pub enum Environment {
    Local,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Local => "Local",
            Self::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "{} is not a supported environment. Use either `local` or `production`.",
                other
            )),
        }
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let configuration_directory = base_path.join("configuration");

    // 检查运行时环境，如果没有指定默认为local
    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT");
    let environment_filename = format!("{}.yaml", environment.as_str());

    // 初始化配置读取器
    let setting = config::Config::builder()
        // 从configuration/base.yaml文件中读取配置
        .add_source(config::File::from(
            configuration_directory.join("base.yaml"),
        ))
        .add_source(config::File::from(
            configuration_directory.join(&environment_filename),
        ))
        .build()?;
    // 尝试将读到的配置转化为Settings类型
    setting.try_deserialize::<Settings>()
}
