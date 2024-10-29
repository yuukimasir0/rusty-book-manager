```
curl -v "http://localhost:8080/auth/login" -H 'Content-Type: application/json' -d '{"email":"eleazar.fig@example.com", "password":"password"}' | jq .
```

```
curl -v "http://localhost:8080/api/v1/users" -H 'Authorization: Bearer ${ACCESS_TOKEN}' | jq .
```

```
curl -v "http://localhost:8080/api/v1/books" -H 'Authorization: Bearer ${ACCESS_TOKEN}' | jq .
```

```
curl -v "http://localhost:8080/api/v1/users" \
-H 'Authorization: Bearer ${ACCESS_TOKEN}' \
-H 'Content-Type: application/json' \
-d '{"name":"yamada", "email":"yamada@example.com", "password":"password", "role":"User"}' | jq .
```

```
curl -v "http://localhost:8080/api/v1/books" \
-H 'Authorization: Bearer ${ACCESS_TOKEN}' \
-H 'Content-Type: application/json' \
-d '{"title":"Rust Book", "author":"me", "isbn":"123456789", "description":""}' | jq .
```

```
08ab29c964594f93ac1b9f980e970641
```

curl -v "http://localhost:8080/api/v1/books" -H 'Authorization: Bearer 08ab29c964594f93ac1b9f980e970641' | jq .

curl -v "http://localhost:8080/api/v1/users" -H 'Authorization: Bearer 08ab29c964594f93ac1b9f980e970641' | jq .

curl -v "http://localhost:8080/api/v1/books" \
-H 'Authorization: Bearer 08ab29c964594f93ac1b9f980e970641' \
-H 'Content-Type: application/json' \
-d '{"title":"Rust Book", "author":"me", "isbn":"123456789", "description":""}' | jq .