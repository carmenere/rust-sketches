# Postgresql
## Init db
```bash
make -f psql.mk init && make -f sqlx.mk run
```

<br>

## Reinit db
```bash
make -f psql.mk clean init && make -f sqlx.mk run
```

# App
## Build and run app
```bash
make -f cargo.mk build && make -f app.mk run
```

<br>

# curl
## POST person
```bash
curl -vvv -X POST http://localhost:8888/persons -H "Content-Type: application/json" -d \
'
{"name": "Anton", "surname": "Romanov", "age": 37}
'
```

<br>

## GET person
```bash
curl -v -X GET http://localhost:8888/persons/1
```

<br>

## GET build_version
```bash
curl -v -X GET http://localhost:8888/build_version
```

<br>

# Tests
## First run
```bash
make -f psql.mk init && make -f sqlx.mk run && make -f cargo.mk test
```

<br>

## Subsequent runs
```bash
make -f psql.mk clean init && make -f sqlx.mk run && make -f cargo.mk test
```