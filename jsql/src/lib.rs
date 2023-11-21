mod util;
#[macro_use]
extern crate ref_thread_local;
use ref_thread_local::RefThreadLocal;

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

use pyo3::prelude::*;
use minijinja::value::{StructObject};
use minijinja::Environment;
use serde::de::*;
use serde_json::Value;
use std::io::Read;
use pyo3::prelude::*;

ref_thread_local! {
    static managed CONTEXT: Mutex<Vec<String>> = Mutex::new(Vec::new());
    static managed PARAM_COUNT: AtomicUsize = AtomicUsize::new(0);
}

/*
Good reference : https://github.com/wseaton/jinjasql-rs/blob/main/src/lib.rs
for async pg for rust based..
 */
/// filter used for binding a single variable, outside of an in-clause or identity expression
/// eg. WHERE date = {{ date }} => WHERE date = "2020-10-01"
pub fn bind(value: String) -> String {
    let current_count = PARAM_COUNT.borrow().fetch_add(1, Ordering::SeqCst) ;
    CONTEXT.borrow().lock().unwrap().push(value.clone());
    format!("${}", current_count)
}

/// filter used for generating in-clauses
/// eg. WHERE thing IN ('cat', 'hat', 'bat')
pub fn bind_in_clause(value: Vec<String>) -> String {
    let mut outputs: Vec<String> = Vec::new();

    for val in value {
        let current_count = PARAM_COUNT.borrow().fetch_add(1, Ordering::SeqCst) ;
        CONTEXT.borrow().lock().unwrap().push(val);

        let pushed =  format!("${}", current_count);
        outputs.push(pushed)
    }

    let res = format!("({})", &outputs.join(", "));
    res
}

///  json serialization issue; used yaml instead, which is a superset of json
/// ref: https://stackoverflow.com/questions/72581178/is-it-possible-to-get-value-useing-json-serdefrom-str-of-a-json-where-string
fn query_render(template: &str, params: &str) -> (String, Vec<String>) {
    let mut env = Environment::new();
    env.add_filter("bind", bind);
    env.add_filter("InClause", bind_in_clause);
    let ctx:Value = serde_yaml::from_str(params).unwrap();
    let query= env.render_str(
        template,
        ctx
    ).unwrap();
    let ctx = CONTEXT.borrow().lock().unwrap().to_vec();
    (query,ctx)
}
#[pyfunction]
fn sql(
        template: &str,
        params: &str,
    ) -> (String,Vec<String>) {
       let (qry, prm) =  query_render(template,params);
    (qry,prm)
 }

/// A Python module implemented in Rust.
#[pymodule]
fn jsql(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sql, m)?)?;
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
       let (result, param) = sql(r#"SELECT id, title_{{lang}} as title
        FROM product_{{country}}
        WHERE 1=1
        {% if price_min %}AND price > {{price_min|bind}}{% endif %}
        {% if id_list %}AND id IN {{id_list | InClause}} {% endif %}
        LIMIT {{limit|bind}}"#,j);

        assert_eq!(result, "SELECT id, title_english as title
        FROM product_uk
        WHERE 1=1
        AND price > $6.7
        AND id IN :id_list
        LIMIT 12");
    }
}

