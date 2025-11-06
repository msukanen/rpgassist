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

pub mod serial_strings {
    use serde::{Deserialize, Deserializer};

    /// Deserializes any field that can be either
    /// **a)** single String or
    /// **b)** Vec<String>.
    /// 
    /// ```jsonc
    /// { "field": "Some string" },
    /// { "field": ["String #1", "String #2"] },
    /// /*
    /// struct Field {
    ///     ...
    ///     #[serde(deserialize_with = "deserialize_strings_to_vec")]
    ///     field: Vec<String>,
    ///     ...
    /// }
    /// */
    /// ```
    pub fn deserialize_strings_to_vec<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
    where D: Deserializer<'de> {
        // helper for the two shapes
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum StringHalp {
            S(String),
            M(Vec<String>),
        }

        match StringHalp::deserialize(deserializer)? {
            StringHalp::S(s) => Ok(vec![s]),
            StringHalp::M(v) => Ok(v),
        }
    }
}