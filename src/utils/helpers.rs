use chrono::{DateTime, Utc};
use percent_encoding::{percent_encode, NON_ALPHANUMERIC};
use serde::{Deserialize, Deserializer};

pub fn deserialize_datetime<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(DateTime::parse_from_rfc3339(&s)
        .map_err(serde::de::Error::custom)?
        .with_timezone(&Utc))
}

pub fn build_query_string(params: &[(&str, &str)]) -> String {
    params
        .iter()
        .map(|(k, v)| {
            format!(
                "{}={}",
                percent_encode(k.as_bytes(), NON_ALPHANUMERIC),
                percent_encode(v.as_bytes(), NON_ALPHANUMERIC)
            )
        })
        .collect::<Vec<String>>()
        .join("&")
}

pub fn truncate_string(s: &str, max_length: usize) -> String {
    if s.len() <= max_length {
        s.to_string()
    } else {
        format!("{}...", &s[..max_length - 3])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_query_string() {
        let params = vec![("key1", "value1"), ("key2", "value 2")];
        assert_eq!(build_query_string(&params), "key1=value1&key2=value%202");
    }

    #[test]
    fn test_truncate_string() {
        assert_eq!(truncate_string("Hello, world!", 10), "Hello, ...");
        assert_eq!(truncate_string("Short", 10), "Short");
    }
}
