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
#
# if __name__ == __main__:
#     from sqlalchemy import create_engine
#     engine = create_engine("sqlite:///:memory:", echo=True)
#     metadata_obj = MetaData()
#     users = Table(
#         "users",
#         metadata_obj,
#         Column("id", Integer, primary_key=True),
#         Column("name", String),
#         Column("fullname", String),
#     )
#     metadata_obj.create_all(engine)
#     query = "INSERT INTO users (id, name, fullname) VALUES ({% if id %}:id{% endif %},{% if name %}:name{% endif %}, {% if fullname %}:fullname{% endif %})"
#     params = {"id":2, "name":'wendy', "fullname":'Wendy Williams'}
#     _text = text(query_string(query, json.dumps(params)))