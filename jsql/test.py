from jsql import query_string
import json
d = { "lang": "value1",
    "country": "uk",
    "price_min": 6.7,
    "limit": 12,
    "id_list": [1,2,3]
    }
print(query_string(
                         "SELECT id, title_{{lang}} as title\
                          FROM product_{{country}}\
                          WHERE 1=1\
                          {% if price_min %}AND price > :price_min{% endif %}\
                          {% if id_list %}AND id IN :id_list{% endif %}\
                          LIMIT {{limit}}", json.dumps(d)
                          ))