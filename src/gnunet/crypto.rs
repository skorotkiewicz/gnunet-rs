use serde::{Deserialize, Serialize};
use std::ffi::{CStr, CString};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PeerIdentity(String);

impl PeerIdentity {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn generate() -> Self {
        Self(uuid::Uuid::new_v4().to_string())
    }

    pub fn from_gnunet(peer: &gnunet_sys::GNUNET_PeerIdentity) -> Self {
        unsafe {
            // GNUNET_i2s returns a pointer to a static buffer, no free needed
            let cstr = gnunet_sys::GNUNET_i2s(peer);
            Self(CStr::from_ptr(cstr).to_string_lossy().into_owned())
        }
    }

    pub fn to_gnunet(&self) -> gnunet_sys::GNUNET_PeerIdentity {
        let mut peer: gnunet_sys::GNUNET_PeerIdentity = unsafe { std::mem::zeroed() };
        let cstr = CString::new(self.0.as_str()).unwrap();
        unsafe {
            gnunet_sys::GNUNET_CRYPTO_eddsa_public_key_from_string(
                cstr.as_ptr(),
                self.0.len(),
                &mut peer.public_key,
            );
        }
        peer
    }
}

impl Default for PeerIdentity {
    fn default() -> Self {
        Self::generate()
    }
}

impl std::fmt::Display for PeerIdentity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HashCode(String);

impl HashCode {
    pub fn new(hash: impl Into<String>) -> Self {
        Self(hash.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn generate(data: &[u8]) -> Self {
        let mut hash: gnunet_sys::GNUNET_HashCode = unsafe { std::mem::zeroed() };
        unsafe {
            gnunet_sys::GNUNET_CRYPTO_hash(
                data.as_ptr() as *const libc::c_void,
                data.len(),
                &mut hash,
            );
        }

        let mut encoded: gnunet_sys::GNUNET_CRYPTO_HashAsciiEncoded = unsafe { std::mem::zeroed() };
        unsafe {
            gnunet_sys::GNUNET_CRYPTO_hash_to_enc(&hash, &mut encoded);
        }

        let s = unsafe {
            CStr::from_ptr(&encoded.encoding as *const _ as *const libc::c_char)
                .to_string_lossy()
                .into_owned()
        };
        Self(s)
    }

    pub fn from_gnunet(hash: &gnunet_sys::GNUNET_HashCode) -> Self {
        let mut encoded: gnunet_sys::GNUNET_CRYPTO_HashAsciiEncoded = unsafe { std::mem::zeroed() };
        unsafe {
            gnunet_sys::GNUNET_CRYPTO_hash_to_enc(hash, &mut encoded);
        }

        let s = unsafe {
            CStr::from_ptr(&encoded.encoding as *const _ as *const libc::c_char)
                .to_string_lossy()
                .into_owned()
        };
        Self(s)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicKey(String);

impl PublicKey {
    pub fn new(key: impl Into<String>) -> Self {
        Self(key.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn from_gnunet_eddsa(key: &gnunet_sys::GNUNET_CRYPTO_EddsaPublicKey) -> Self {
        unsafe {
            let cstr = gnunet_sys::GNUNET_CRYPTO_eddsa_public_key_to_string(key);
            let s = CStr::from_ptr(cstr).to_string_lossy().into_owned();
            gnunet_sys::GNUNET_xfree_(cstr as *mut libc::c_void, c"crypto.rs".as_ptr(), line!() as libc::c_int);
            Self(s)
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivateKey(String);

impl PrivateKey {
    pub fn new(key: impl Into<String>) -> Self {
        Self(key.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn public_key(&self) -> PublicKey {
        PublicKey::new(format!("pub:{}", self.0))
    }

    pub fn generate_eddsa() -> Self {
        let mut key: gnunet_sys::GNUNET_CRYPTO_EddsaPrivateKey = unsafe { std::mem::zeroed() };
        unsafe {
            gnunet_sys::GNUNET_CRYPTO_eddsa_key_create(&mut key);
        }

        unsafe {
            let cstr = gnunet_sys::GNUNET_CRYPTO_eddsa_private_key_to_string(&key);
            let s = CStr::from_ptr(cstr).to_string_lossy().into_owned();
            gnunet_sys::GNUNET_xfree_(cstr as *mut libc::c_void, c"crypto.rs".as_ptr(), line!() as libc::c_int);
            gnunet_sys::GNUNET_CRYPTO_eddsa_key_clear(&mut key);
            Self(s)
        }
    }
}
