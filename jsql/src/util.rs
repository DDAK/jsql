use std::collections::HashMap;

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