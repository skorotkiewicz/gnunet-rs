use crate::gnunet::{PrivateKey, PublicKey};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ego {
    pub name: String,
    pub private_key: PrivateKey,
    pub public_key: PublicKey,
}

impl Ego {
    pub fn new(name: impl Into<String>) -> Self {
        let name = name.into();
        let private_key = PrivateKey::new(format!("sk:{}", uuid::Uuid::new_v4()));
        let public_key = private_key.public_key();
        Self {
            name,
            private_key,
            public_key,
        }
    }
}

pub struct IdentityService {
    egos: Vec<Ego>,
    default_ego: Option<String>,
}

impl Default for IdentityService {
    fn default() -> Self {
        Self::new()
    }
}

impl IdentityService {
    pub fn new() -> Self {
        Self {
            egos: Vec::new(),
            default_ego: None,
        }
    }

    pub fn create_ego(&mut self, name: &str) -> Ego {
        let ego = Ego::new(name);
        self.egos.push(ego.clone());
        ego
    }

    pub fn get_ego(&self, name: &str) -> Option<&Ego> {
        self.egos.iter().find(|e| e.name == name)
    }

    pub fn set_default(&mut self, name: &str) {
        self.default_ego = Some(name.to_string());
    }

    pub fn get_default(&self) -> Option<&Ego> {
        self.default_ego
            .as_ref()
            .and_then(|name| self.get_ego(name))
    }

    pub fn list_egos(&self) -> &[Ego] {
        &self.egos
    }
}
