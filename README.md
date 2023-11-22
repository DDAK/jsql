# jsql
A python library in Rust using Pyo3 for templatized sql queries using Rust's minijinja.

# Installation
    
- You need to install `maturin` using pip to be able to build & use this repo locally.
    ``` bash
    pip install maturin
    ``` 
  
- Clone & build this repo using 
  ``` bash
  git clone https://github.com/DDAK/jsql.git && cd jsql && maturin develop
  ``` 
# Usage
You can import jsql and use sql function from it. Query should be written in a jinja template format [see](https://docs.rs/minijinja/latest/minijinja/syntax/index.html) this for details.
Here we provide addition filter which do special action (`bind` and `InClause`). This causes rendering engine to keep track of any attributes tagged in a template as the parameters.  
``` python
from jsql import sql

import json
d = { "lang": "value1",
    "country": "uk; SELECT * from password;",
    "price_min": 6.7,
    "limit": 12,
    "id_list": [1,2,3]
    }
query = "SELECT id, title_{{lang}} as title\
                 FROM product_{{country}}\
                 WHERE 1=1\
                 {% if price_min %}AND price > {{price_min|bind}}{% endif %}\
                 {% if id_list %}AND id IN {{id_list | InClause}} {% endif %}\
                 LIMIT {{limit|bind}}"

def _sql(query,params):
    return sql(query, json.dumps(d))


print(_sql(query,d))
```
# Next
-[ ] return params as a dictionary/ HashMap, instead of a vec<String>. 
 
-[ ] Wrap the `asynpg` or `sqlalchemy` as a decorator or within the library.

- [ ] Added further test cases

# References
This project is heavily inspired, & borrows from:
- [python-jsql](https://github.com/hzarka/python-jsql.git)
- [jinasql-rs](https://github.com/wseaton/jinjasql-rs.git)