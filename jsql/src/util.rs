use std::collections::HashMap;
use serde_json::Value;
use serde::de::*;

pub fn deser_hashmap<'de, D: Deserializer<'de>>(deserializer: D) -> Result<HashMap<String, String>, D::Error>
{
    struct MapVisitor;

    impl<'de> Visitor<'de> for MapVisitor {
        type Value = HashMap<String, String>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            formatter.write_str("a map")
        }

        fn visit_map<A>(self, mut access: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
        {
            let mut values = HashMap::new();

            // We use RawValue here to access the JSON value exactly as it is occuring in the input.
            while let Some((key, value)) = (access.next_entry())?
                .map(|(k, v): (String, &'de serde_json::value::RawValue)| (k, v.get().to_owned()))
            {
                values.insert(key, value);
            }

            Ok(values)
        }
    }

    let visitor = MapVisitor;
    deserializer.deserialize_map(visitor)
}
///Usage:pass the param string
/// let mut deser = serde_json::Deserializer::from_str(params);
/// let map = deser_hashmap(&mut deser).unwrap();



/// The idea is to iterate over the nested dictionary and evalute for any strings having SQL injection
fn deep_keys(value: &Value, current_path: Vec<String>, output: &mut Vec<Vec<String>>) {

    if current_path.len() > 0 {
        output.push(current_path.clone());
    }

    match value {
        Value::Object(map) => {
            for (k, v) in map {
                let mut new_path = current_path.clone();
                new_path.push(k.to_owned());
                deep_keys(v,  new_path, output);

            }
        },
        Value::Array(array) => {
            for (i, v) in array.iter().enumerate() {
                let mut new_path = current_path.clone();
                new_path.push(i.to_string().to_owned());
                deep_keys(v,  new_path, output);
            }
        },
        Value::String(String) => {

        },
        _ => ()
    }
}
