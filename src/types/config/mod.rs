use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

pub use crate::{
    types::config::types::{
        api_configuration::ApiConfiguration, cdn_configuration::CdnConfiguration,
        defaults_configuration::DefaultsConfiguration, email_configuration::EmailConfiguration,
        endpoint_configuration::EndpointConfiguration,
        external_tokens_configuration::ExternalTokensConfiguration,
        general_configuration::GeneralConfiguration, gif_configuration::GifConfiguration,
        guild_configuration::GuildConfiguration, kafka_configuration::KafkaConfiguration,
        limit_configuration::LimitsConfiguration, login_configuration::LoginConfiguration,
        metrics_configuration::MetricsConfiguration,
        password_reset_configuration::PasswordResetConfiguration,
        rabbit_mq_configuration::RabbitMQConfiguration, region_configuration::RegionConfiguration,
        register_configuration::RegisterConfiguration,
        security_configuration::SecurityConfiguration, sentry_configuration::SentryConfiguration,
        template_configuration::TemplateConfiguration,
    },
    types::entities::ConfigEntity,
};

pub mod types;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigValue {
    pub gateway: EndpointConfiguration,
    pub cdn: CdnConfiguration,
    pub api: ApiConfiguration,
    pub general: GeneralConfiguration,
    pub limits: LimitsConfiguration,
    pub security: SecurityConfiguration,
    pub login: LoginConfiguration,
    pub register: RegisterConfiguration,
    pub regions: RegionConfiguration,
    pub guild: GuildConfiguration,
    pub gif: GifConfiguration,
    pub rabbitmq: RabbitMQConfiguration,
    pub kafka: KafkaConfiguration,
    pub templates: TemplateConfiguration,
    pub metrics: MetricsConfiguration,
    pub sentry: SentryConfiguration,
    pub defaults: DefaultsConfiguration,
    pub external: ExternalTokensConfiguration,
    pub email: EmailConfiguration,
    pub password_reset: PasswordResetConfiguration,
}

impl ConfigValue {
    pub fn to_pairs(&self) -> Vec<ConfigEntity> {
        let v = serde_json::json!(self);

        generate_pairs(&v, "")
    }

    pub fn from_pairs(pairs: Vec<ConfigEntity>) -> Self {
        pairs_to_config(pairs)
    }
}

fn generate_pairs(obj: &Value, key: &str) -> Vec<ConfigEntity> {
    let mut pairs = Vec::new();
    match obj {
        Value::Object(map) => {
            for (k, v) in map {
                let new_key = if key.is_empty() {
                    k.to_string()
                } else {
                    format!("{}_{}", key, k)
                };
                pairs.extend(generate_pairs(v, &new_key));
            }
        }
        Value::Array(arr) => {
            for (i, v) in arr.iter().enumerate() {
                let new_key = format!("{}_{}", key, i);
                pairs.extend(generate_pairs(v, &new_key));
            }
        }
        _ => pairs.push(ConfigEntity {
            key: key.to_string(),
            value: Some(obj.clone()),
        }),
    }
    pairs
}

fn pairs_to_config(pairs: Vec<ConfigEntity>) -> ConfigValue {
    let mut value = Value::Object(Map::new());

    for p in pairs {
        let keys: Vec<&str> = p.key.split('_').collect();
        let mut path = vec![];

        for (i, &key) in keys.iter().enumerate() {
            path.push(key);

            if i == keys.len() - 1 {
                insert_into(&mut value, &path, p.value.clone().unwrap_or(Value::Null));
            } else if keys[i + 1].parse::<usize>().is_ok() {
                if !path_exists(&value, &path) {
                    insert_into(&mut value, &path, Value::Array(Vec::new()));
                }
            } else if !path_exists(&value, &path) {
                insert_into(&mut value, &path, Value::Object(Map::new()));
            }
        }
    }

    serde_json::from_value(value).unwrap()
}

fn path_exists(value: &Value, path: &[&str]) -> bool {
    let mut current = value;

    for &key in path {
        match current {
            Value::Object(map) => {
                if let Some(v) = map.get(key) {
                    current = v;
                } else {
                    return false;
                }
            }
            Value::Array(arr) => {
                if let Ok(index) = key.parse::<usize>() {
                    if let Some(v) = arr.get(index) {
                        current = v;
                    } else {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            _ => return false,
        }
    }

    true
}

fn insert_into(value: &mut Value, path: &[&str], new_value: Value) {
    let last_key = path.last().unwrap();
    let parent_path = &path[0..path.len() - 1];

    let mut current = value;

    for &key in parent_path {
        current = match current {
            Value::Object(map) => map.get_mut(key).unwrap(),
            Value::Array(arr) => arr.get_mut(key.parse::<usize>().unwrap()).unwrap(),
            _ => unreachable!(),
        };
    }

    match current {
        Value::Object(map) => {
            map.insert((*last_key).to_string(), new_value);
        }
        Value::Array(arr) => {
            let index = last_key.parse::<usize>().unwrap();
            if index >= arr.len() {
                arr.resize(index + 1, Value::Null);
            }
            arr[index] = new_value;
        }
        _ => unreachable!(),
    };
}

#[cfg(test)]
mod test {
    use crate::types::config::{generate_pairs, pairs_to_config, ConfigValue};

    #[test]
    fn test_pairs() {
        let c = ConfigValue::default();
        let v = serde_json::json!(&c);

        let pairs = generate_pairs(&v, "");

        let cfg = pairs_to_config(pairs);
        assert_eq!(cfg, c)
    }
}
