use std::collections::HashMap;

pub type Options = HashMap<String, OptionValue>;

#[derive(PartialEq)]
pub enum OptionValue {
    String(String),
    U32(u32),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_values() {
        let mut opts = Options::new();
        opts.insert("key".to_string(), OptionValue::String("value".to_string()));
        assert!(opts[&"key".to_string()] == OptionValue::String("value".to_string()));
    }

    #[test]
    fn u32_values() {
        let mut opts = Options::new();
        opts.insert("num".to_string(), OptionValue::U32(5));
        assert!(opts[&"num".to_string()] == OptionValue::U32(5));
    }
}
