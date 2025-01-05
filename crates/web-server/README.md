# we-server


```bash
psql "postgres://postgres:quest1\!@127.0.0.1:5432/"
~/.cargo/bin/sqlx migrate run --database-url "postgres://postgres:quest1\!@127.0.0.1:5432/"

docker container start dataans-pg
docker run --name dataans-pg -e POSTGRES_PASSWORD=quest1! -v ./pg-volume:/var/lib/postgresql/data -p 5432:5432 postgres

export DATABASE_URL=postgres://postgres:quest1\!@127.0.0.1:5432/
```
