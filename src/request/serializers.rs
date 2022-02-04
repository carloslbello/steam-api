use steamid_ng::SteamID;
use serde::Serializer;

pub fn steamid_as_string<S>(steamid: &SteamID, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer
{
    s.serialize_str(&u64::from(steamid.clone()).to_string())
}

pub mod string {
    use std::fmt::Display;
    use std::str::FromStr;
    use serde::{de, Serializer, Deserialize, Deserializer};
    
    pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: Display,
        S: Serializer
    {
        serializer.collect_str(value)
    }
    
    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where
        T: FromStr,
        T::Err: Display,
        D: Deserializer<'de>
    {
        String::deserialize(deserializer)?.parse().map_err(de::Error::custom)
    }
}

pub mod option_string {
    use std::fmt::Display;
    use std::str::FromStr;
    use serde::{Serializer, Deserialize, Deserializer};
    
    pub fn serialize<T, S>(value: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: Display,
        S: Serializer
    {
        match value {
            Some(string) => serializer.collect_str(string),
            None => serializer.serialize_none()
        }
        
    }
    
    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
    where
        T: FromStr,
        T::Err: Display,
        D: Deserializer<'de>
    {
        let s: Option<String> = Option::<String>::deserialize(deserializer)?;
        
        if let Some(v) = s {
            return Ok(Some(v.parse::<T>().map_err(serde::de::Error::custom)?))
        }
            
        Ok(None)
    }
}