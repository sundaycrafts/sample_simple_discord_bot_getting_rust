use crate::hex::Hex;
use anyhow::{anyhow, Result};
use ed25519_compact::{PublicKey, Signature};
use http::HeaderMap;

pub struct DiscordSignature {
    public_key: PublicKey,
}

impl TryFrom<&Hex> for DiscordSignature {
    type Error = anyhow::Error;

    fn try_from(hex: &Hex) -> std::result::Result<Self, Self::Error> {
        Ok(DiscordSignature {
            public_key: PublicKey::from_slice(hex.as_ref().as_slice())?,
        })
    }
}

impl DiscordSignature {
    pub fn verify(&self, headers: &HeaderMap, body: &str) -> Result<()> {
        let sig_hex = Hex::try_from(
            headers
                .get("X-Signature-Ed25519")
                .ok_or(anyhow::anyhow!("Missing signature"))?
                .to_str()?,
        )?;

        let header_timestamp = headers
            .get("X-Signature-Timestamp")
            .ok_or(anyhow::anyhow!("Missing timestamp"))?;

        let message = format!("{}{}", header_timestamp.to_str()?, body);

        let signature = Signature::from_slice(sig_hex.as_ref().as_slice())?;

        self.public_key
            .verify(message.as_bytes(), &signature)
            .map_err(|e| anyhow!(e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_compact::*;
    use http::HeaderValue;

    fn setup_signed_headers(body: &str) -> Result<(HeaderMap, Hex)> {
        // Generate a keypair
        let keypair = KeyPair::from_seed(Seed::default()); // Seed::new(seed_arr("test"))

        // Encode public key
        let pk = keypair.pk;
        let pk_hex = Hex::try_from(hex::encode(pk.as_slice()).as_str()).unwrap();

        // Create a timestamp
        let timestamp = "1627383847";

        // Sign the message with the timestamp
        let message = format!("{}{}", timestamp, body);
        let signature = keypair.sk.sign(message, Some(Noise::default()));

        // setup headers for the request
        let mut headers = HeaderMap::new();
        headers.insert(
            "X-Signature-Ed25519",
            HeaderValue::from_str(&hex::encode(signature.as_slice())).unwrap(),
        );
        headers.insert(
            "X-Signature-Timestamp",
            HeaderValue::from_str(timestamp).unwrap(),
        );

        Ok((headers, pk_hex))
    }

    #[test]
    fn test_verify_signature_valid() {
        let body = "This is a test message.";
        let (headers, pk_hex) = setup_signed_headers(body).unwrap();
        let sig = DiscordSignature::try_from(&pk_hex).unwrap();
        sig.verify(&headers, body).unwrap();
    }

    #[test]
    fn test_verify_signature_invalid() {
        let body = "This is a test message.";
        let (_, pk_hex1) = setup_signed_headers(body).unwrap();
        let (headers2, _) = setup_signed_headers(body).unwrap();

        let sig1 = DiscordSignature::try_from(&pk_hex1).unwrap();
        assert!(sig1.verify(&headers2, body).is_err());
    }
}
