use crate::domains::authorizer::Authorizer;
use discord_authorize::discord_signature::DiscordSignature;
use discord_authorize::hex::Hex;
use http::HeaderMap;
use std::error::Error;

pub struct ProductionAuthorizer {
    signature: DiscordSignature,
}

impl ProductionAuthorizer {
    pub fn new(public_key: &str) -> Self {
        let hex = Hex::try_from(public_key).expect("Invalid public key");
        ProductionAuthorizer {
            signature: DiscordSignature::try_from(&hex).unwrap(),
        }
    }
}

impl Authorizer for ProductionAuthorizer {
    fn authorize(&self, header: &HeaderMap, raw_bod: &str) -> Result<(), Box<dyn Error>> {
        self.signature.verify(&header, raw_bod)?;
        Ok(())
    }
}
