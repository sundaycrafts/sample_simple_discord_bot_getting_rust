pub struct Hex(Vec<u8>);

impl AsRef<Vec<u8>> for Hex {
    fn as_ref(&self) -> &Vec<u8> {
        &self.0
    }
}

impl TryFrom<&str> for Hex {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Hex, Self::Error> {
        Ok(Hex(hex::decode(value)?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_from_str_success_on_valid_hex() {
        assert!(Hex::try_from(hex::encode("valid").as_str()).is_ok());
    }

    #[test]
    fn test_try_from_str_fail_on_invalid_hex() {
        assert!(Hex::try_from("invalid").is_err());
    }
}
