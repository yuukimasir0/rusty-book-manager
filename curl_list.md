```
curl -v "http://localhost:8080/auth/login" -H 'Content-Type: application/json' -d '{"email":"admin@example.com", "password":"password"}' | jq .
```

```
curl -v "http://localhost:8080/auth/login" -H 'Authorization: Bearer ${ACCESS_TOKEN}' | jq .
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
5e8d0779503e428397910afca3309477
```

curl -v "http://localhost:8080/api/v1/books" -H 'Authorization: Bearer 5e8d0779503e428397910afca3309477' | jq .

curl -v "http://localhost:8080/api/v1/users" -H 'Authorization: Bearer 5e8d0779503e428397910afca3309477' | jq .

curl -v "http://localhost:8080/api/v1/books" \
-H 'Authorization: Bearer 5e8d0779503e428397910afca3309477' \
-H 'Content-Type: application/json' \   
-d '{"title":"Rust Book", "author":"me", "isbn":"123456789", "description":""}' | jq .

curl -v -X POST "http://localhost:8080/api/v1/books/d058bbf3480e4039836ad56a7d7bf0c7/checkouts" -H 'Authorization: Bearer 5e8d0779503e428397910afca3309477' | jq .

curl -v -X PUT "http://localhost:8080/api/v1/books/d058bbf3480e4039836ad56a7d7bf0c7/checkouts/ac2e166883d744f78081812199f85e68/returned" -H 'Authorization: Bearer 5e8d0779503e428397910afca3309477' | jq .

curl -v -X POST "http://localhost:8080/api/v1/books/checkouts" -H 'Authorization: Bearer 5e8d0779503e428397910afca3309477' | jq .

curl -v -X POST "http://localhost:8080/auth/logout" -H 'Authorization: Bearer 5e8d0779503e428397910afca3309477' | jq .