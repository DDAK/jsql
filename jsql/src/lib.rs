use pyo3::prelude::*;
use minijinja::value::{StructObject};
use minijinja::Environment;
use serde::de::*;
use serde_json::Value;

///  json serialization issue; used yaml instead, which is a superset of json
/// ref: https://stackoverflow.com/questions/72581178/is-it-possible-to-get-value-useing-json-serdefrom-str-of-a-json-where-string
fn query_render(template: &str, params: &str) -> String {
    let env = Environment::new();

    let ctx: serde_json::Value = serde_yaml::from_str(params).unwrap();
    env.render_str(
            template,
            ctx
            // Value::from(map)
        ).unwrap()
}

#[pyfunction]
fn query_string(template: &str, params: &str) -> PyResult<String> {
    Ok(
        query_render(template,params)
    )
}

/// A Python module implemented in Rust.
#[pymodule]
fn jsql(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(query_string, m)?)?;
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test2_render(){
        let j = r#"
    {
        "lang": "english",
        "country": "uk",
        "price_min": 6.7,
        "limit": 12,
        "id_list": [1,2,3]
    }"#;
       let result = query_render(r#"SELECT id, title_{{lang}} as title
        FROM product_{{country}}
        WHERE 1=1
        {% if price_min %}AND price > :price_min{% endif %}
        {% if id_list %}AND id IN :id_list{% endif %}
        LIMIT {{limit}}"#,j);

        assert_eq!(result, "SELECT id, title_english as title
        FROM product_uk
        WHERE 1=1
        AND price > :price_min
        AND id IN :id_list
        LIMIT 12");
    }
}

