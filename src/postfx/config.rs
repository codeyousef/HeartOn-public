use bevy_asset::Asset;
use bevy_reflect::TypePath;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, Asset, TypePath)]
pub struct PostFxConfig {
    pub chain: Vec<EffectConfig>,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffectConfig {
    pub effect_type: String,
    pub enabled: bool,
    pub params: HashMap<String, EffectParam>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EffectParam {
    Float(f32),
    Int(i32),
    Bool(bool),
    String(String),
    Vec2([f32; 2]),
    Vec3([f32; 3]),
    Vec4([f32; 4]),
}

impl EffectParam {
    pub fn as_float(&self) -> Option<f32> {
        match self {
            EffectParam::Float(v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_int(&self) -> Option<i32> {
        match self {
            EffectParam::Int(v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            EffectParam::Bool(v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_vec3(&self) -> Option<[f32; 3]> {
        match self {
            EffectParam::Vec3(v) => Some(*v),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct EffectChain {
    pub effects: Vec<EffectConfig>,
}

impl Default for PostFxConfig {
    fn default() -> Self {
        Self {
            chain: vec![
                EffectConfig {
                    effect_type: "simple_bloom".to_string(),
                    enabled: true,
                    params: {
                        let mut params = HashMap::new();
                        params.insert("threshold".to_string(), EffectParam::Float(0.8));
                        params.insert("intensity".to_string(), EffectParam::Float(0.3));
                        params
                    },
                },
                EffectConfig {
                    effect_type: "tone_mapping".to_string(),
                    enabled: true,
                    params: {
                        let mut params = HashMap::new();
                        params.insert("exposure".to_string(), EffectParam::Float(1.0));
                        params
                    },
                },
                EffectConfig {
                    effect_type: "vignette".to_string(),
                    enabled: true,
                    params: {
                        let mut params = HashMap::new();
                        params.insert("intensity".to_string(), EffectParam::Float(0.3));
                        params.insert("smoothness".to_string(), EffectParam::Float(0.5));
                        params
                    },
                },
            ],
            enabled: true,
        }
    }
}

impl PostFxConfig {
    pub fn performance() -> Self {
        Self {
            chain: vec![EffectConfig {
                effect_type: "tone_mapping".to_string(),
                enabled: true,
                params: {
                    let mut params = HashMap::new();
                    params.insert("exposure".to_string(), EffectParam::Float(1.0));
                    params
                },
            }],
            enabled: true,
        }
    }

    pub fn cinematic() -> Self {
        Self {
            chain: vec![
                EffectConfig {
                    effect_type: "simple_bloom".to_string(),
                    enabled: true,
                    params: {
                        let mut params = HashMap::new();
                        params.insert("threshold".to_string(), EffectParam::Float(0.6));
                        params.insert("intensity".to_string(), EffectParam::Float(0.5));
                        params
                    },
                },
                EffectConfig {
                    effect_type: "tone_mapping".to_string(),
                    enabled: true,
                    params: {
                        let mut params = HashMap::new();
                        params.insert("exposure".to_string(), EffectParam::Float(1.2));
                        params
                    },
                },
                EffectConfig {
                    effect_type: "vignette".to_string(),
                    enabled: true,
                    params: {
                        let mut params = HashMap::new();
                        params.insert("intensity".to_string(), EffectParam::Float(0.5));
                        params.insert("smoothness".to_string(), EffectParam::Float(0.4));
                        params
                    },
                },
            ],
            enabled: true,
        }
    }

    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = PostFxConfig::default();
        assert_eq!(config.chain.len(), 3);
        assert!(config.enabled);
    }

    #[test]
    fn test_performance_preset() {
        let config = PostFxConfig::performance();
        assert_eq!(config.chain.len(), 1);
        assert_eq!(config.chain[0].effect_type, "tone_mapping");
    }

    #[test]
    fn test_cinematic_preset() {
        let config = PostFxConfig::cinematic();
        assert_eq!(config.chain.len(), 3);
        assert!(config.enabled);
    }

    #[test]
    fn test_json_serialization() {
        let config = PostFxConfig::default();
        let json = config.to_json().unwrap();
        let deserialized = PostFxConfig::from_json(&json).unwrap();
        assert_eq!(deserialized.chain.len(), config.chain.len());
    }

    #[test]
    fn test_effect_param_types() {
        let float_param = EffectParam::Float(1.5);
        assert_eq!(float_param.as_float(), Some(1.5));
        assert_eq!(float_param.as_int(), None);

        let int_param = EffectParam::Int(42);
        assert_eq!(int_param.as_int(), Some(42));
        assert_eq!(int_param.as_float(), None);

        let bool_param = EffectParam::Bool(true);
        assert_eq!(bool_param.as_bool(), Some(true));

        let vec3_param = EffectParam::Vec3([1.0, 2.0, 3.0]);
        assert_eq!(vec3_param.as_vec3(), Some([1.0, 2.0, 3.0]));
    }
}
