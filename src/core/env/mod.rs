pub mod macros;

use figment::Figment;
use figment::providers::{Env, Format, Toml};
use rust_embed::Embed;
use serde::de::DeserializeOwned;
use std::fmt::Debug;
use std::str::from_utf8;

#[derive(Embed)]
#[folder = "config/"]
struct ConfigAsset;

pub fn current_env() -> String {
    std::env::var("RHYON_ENV").unwrap_or_else(|_| "local".to_string())
}

pub fn load_config<T: DeserializeOwned + Debug>() -> T {
    let env = current_env();

    let config = Figment::new()
        .merge_optional_config("default")
        .merge_optional_config(&env)
        .merge(Env::prefixed("RHYON_").split("_")) // 支持环境变量覆盖
        .extract()
        .unwrap_or_else(|e| {
            panic!("❌  无法加载配置 (环境: {}): {}", env, e);
        });

    tracing::debug!("加载配置： {:?}", config);

    config
}

trait FigmentTrait {
    fn merge_optional_config(self, name: &str) -> Self;
}

impl FigmentTrait for Figment {
    fn merge_optional_config(self, name: &str) -> Self {
        if let Some(config) = load_config_file(name) {
            self.merge(Toml::string(&config))
        } else {
            self
        }
    }
}

fn load_config_file(name: &str) -> Option<String> {
    let filename = format!("{}.toml", name);

    ConfigAsset::get(&filename).map(|file| {
        from_utf8(file.data.as_ref())
            .expect("配置文件内容不是 UTF-8")
            .to_string()
    })
}
