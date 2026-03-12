use hmac::{Hmac, Mac};
use sha2::Sha256;

use crate::error::AppError;

type HmacSha256 = Hmac<Sha256>;

pub fn verify_signature(secret: &str, signature_header: &str, payload: &[u8]) -> Result<(), AppError> {
    let Some(signature) = signature_header.strip_prefix("sha256=") else {
        return Err(AppError::InvalidSignature);
    };

    let signature_bytes = hex::decode(signature).map_err(|_| AppError::InvalidSignature)?;
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes())
        .map_err(|source| AppError::Internal(source.to_string()))?;
    mac.update(payload);
    mac.verify_slice(&signature_bytes)
        .map_err(|_| AppError::InvalidSignature)
}

#[cfg(test)]
mod tests {
    use hmac::{Hmac, Mac};
    use sha2::Sha256;

    use super::verify_signature;

    #[test]
    fn accepts_valid_signature() {
        let payload = br#"{\"hello\":\"world\"}"#;
        let mut mac = Hmac::<Sha256>::new_from_slice(b"topsecret").expect("mac");
        mac.update(payload);
        let signature = format!("sha256={}", hex::encode(mac.finalize().into_bytes()));

        verify_signature("topsecret", &signature, payload).expect("signature should verify");
    }

    #[test]
    fn rejects_invalid_signature() {
        let payload = br#"{\"hello\":\"world\"}"#;
        let signature = "sha256=aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

        let result = verify_signature("topsecret", signature, payload);
        assert!(result.is_err());
    }
}