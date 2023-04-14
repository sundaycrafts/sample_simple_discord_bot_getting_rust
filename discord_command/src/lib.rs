use serde::{Deserialize, Deserializer, Serialize, Serializer};

const ASK_EXP: &str = "ask";

#[derive(Debug)]
pub enum Command {
    Ask,
}

impl Serialize for Command {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(match *self {
            Command::Ask => ASK_EXP,
        })
    }
}

impl<'de> Deserialize<'de> for Command {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            ASK_EXP => Ok(Command::Ask),
            _ => Err(serde::de::Error::custom("invalid command")),
        }
    }
}
