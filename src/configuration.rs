use secrecy::{ExposeSecret, SecretString};
#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub host: String,
    pub username: String,
    // 数据库密码是敏感信息,需要进行加密
    pub password: SecretString,
    pub port: u16,
    pub database_name: String,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    // merge方法已经过时了，从官方文档可知从0.12版本之后建议使用ConfigBuilder
    // let mut settings = config::Config::default();
    // settings.merge(config::File::with_name("configuration"))?;
    // settings.try_into()
    let settings = config::Config::builder()
        .add_source(config::File::new(
            "configuration.yaml",
            config::FileFormat::Yaml,
        ))
        .build()?;
    settings.try_deserialize::<Settings>()
}

impl DatabaseSettings {
    // 获取pg数据库连接url
    // 将整个数据库连接url进行加密
    pub fn get_connection_string(&self) -> SecretString {
        SecretString::from(format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port,
            self.database_name
        ))
    }

    pub fn get_connection_string_without_db(&self) -> SecretString {
        SecretString::from(format!(
            "postgres://{}:{}@{}:{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port
        ))
    }
}
