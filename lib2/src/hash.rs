use sha2::{Digest, Sha256};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hash([u8; 32]);

impl Hash {
    pub fn new(data: &[u8]) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(data);
        Hash(hasher.finalize().into())
    }

    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    pub fn as_hex(&self) -> String {
        self.0.iter().map(|b| format!("{:02x}", b)).collect()
    }

    pub fn combine(left: &Hash, right: &Hash) -> Self {
        let mut combined_data = Vec::with_capacity(64);
        combined_data.extend_from_slice(left.as_bytes());
        combined_data.extend_from_slice(right.as_bytes());
        Self::new(&combined_data)
    }

    pub fn empty() -> Self {
        Hash([0u8; 32])
    }

    pub fn from_hex(hex: &str) -> Result<Self, Box<dyn std::error::Error>> {
        if hex.len() != 64 {
            // Each byte is represented by 2 hex characters, so 32 bytes = 64 hex chars
            return Err("invalid hex string length: expected 64 characters".into());
        }

        let mut hash = [0u8; 32];
        for i in 0..32 {
            let byte = u8::from_str_radix(&hex[i * 2..i * 2 + 2], 16)
                .map_err(|e| format!("failed to parse hex byte at position {}: {}", i, e))?;
            hash[i] = byte;
        }

        Ok(Hash::new(&hash))
    }
}

mod test {
    #[test]
    fn test_hash() {
        let data = b"hello world";
        let hash = crate::hash::Hash::new(data);
        assert_eq!(
            hash.as_hex(),
            "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"
        );
    }
}
