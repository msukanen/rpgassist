/// Ordering (de)serializer functions live here.
pub mod serial_ordering {
    use std::cmp::Ordering;

    use serde::{Deserialize, Deserializer, Serializer, de::Error};

    /// 'Ordering' deserializer.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Ordering, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.to_lowercase().as_str().chars().nth(0) {
            Some('l') => Ok(Ordering::Less),
            Some('e') => Ok(Ordering::Equal),
            Some('g') => Ok(Ordering::Greater),
            other => Err(Error::custom(format!(
                "Unknown Ordering variant: {other:?}"
            )))
        }
    }

    /// 'Ordering' serializer.
    pub fn serialize<S>(ordering: &Ordering, serializer: S) -> Result<S::Ok, S::Error>
    where 
        S: Serializer,
    {
        serializer.serialize_str(match ordering {
            Ordering::Less => "Less",
            Ordering::Equal => "Equal",
            Ordering::Greater => "Greater",
        })
    }
}