# Check docker db image
```
docker exec -it db psql -U postgres -d postgres -c "SELECT * FROM table_name;"
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