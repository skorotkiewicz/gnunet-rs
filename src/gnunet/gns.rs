use crate::gnunet::{PeerIdentity, PublicKey};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GnsRecord {
    pub record_type: String,
    pub data: String,
    pub expiration: u64,
    pub flags: u32,
}

pub struct GnsService {
    zone_cache: std::collections::HashMap<String, Vec<GnsRecord>>,
    local_zone: Option<PublicKey>,
}

impl Default for GnsService {
    fn default() -> Self {
        Self::new()
    }
}

impl GnsService {
    pub fn new() -> Self {
        Self {
            zone_cache: std::collections::HashMap::new(),
            local_zone: None,
        }
    }

    pub fn set_local_zone(&mut self, zone: PublicKey) {
        self.local_zone = Some(zone);
    }

    pub fn lookup(
        &self,
        name: &str,
        zone: &PublicKey,
        record_type: &str,
    ) -> Option<Vec<GnsRecord>> {
        let key = format!("{}:{}:{}", zone.as_str(), name, record_type);
        self.zone_cache.get(&key).cloned()
    }

    pub fn lookup_in_local_zone(&self, name: &str, record_type: &str) -> Option<Vec<GnsRecord>> {
        let zone = self.local_zone.as_ref()?;
        self.lookup(name, zone, record_type)
    }

    pub fn store_record(&mut self, name: &str, zone: &PublicKey, record: GnsRecord) {
        let key = format!("{}:{}:{}", zone.as_str(), name, record.record_type);
        self.zone_cache.entry(key).or_default().push(record);
    }

    pub fn create_identity_record(&self, peer: &PeerIdentity, username: &str) -> GnsRecord {
        GnsRecord {
            record_type: "IDENTITY".to_string(),
            data: format!("{}:{}", peer.as_str(), username),
            expiration: u64::MAX,
            flags: 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordType {
    A,
    AAAA,
    CNAME,
    NS,
    PKEY,
    EDKEY,
    GNS2DNS,
    IDENTITY,
    SOCIAL,
    TEXT,
    BOX,
}

impl RecordType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::A => "A",
            Self::AAAA => "AAAA",
            Self::CNAME => "CNAME",
            Self::NS => "NS",
            Self::PKEY => "PKEY",
            Self::EDKEY => "EDKEY",
            Self::GNS2DNS => "GNS2DNS",
            Self::IDENTITY => "IDENTITY",
            Self::SOCIAL => "SOCIAL",
            Self::TEXT => "TEXT",
            Self::BOX => "BOX",
        }
    }
}
