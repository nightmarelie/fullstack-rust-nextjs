# App init
```
docker compose build
docker compose up
```

# Check docker db image

```
docker exec -it db psql -U postgres -d postgres -c "SELECT * FROM user;"
```

```
docker exec -it db psql -U postgres -d postgres

\l # list databases
\dt # list tables
\c database_name # connect to database
\q # quit
```

# Rust init

```
cargo new backend
```

# curl api

```
curl -X GET http://localhost:8080/api/rust/users
curl -X GET http://localhost:8080/api/rust/users/1
curl -X POST http://localhost:8080/api/rust/users -H "Content-Type: application/json" -d '{"name": "test", "email":"test@test"}'
```