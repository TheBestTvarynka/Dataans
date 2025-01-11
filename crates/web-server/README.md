# we-server


```bash
psql "postgres://postgres:quest1\!@127.0.0.1:5432/"
~/.cargo/bin/sqlx migrate run --database-url "postgres://postgres:quest1\!@127.0.0.1:5432/"

docker container start dataans-pg
docker run --name dataans-pg -e POSTGRES_PASSWORD=quest1! -v ./pg-volume:/var/lib/postgresql/data -p 5432:5432 postgres

export DATAANS_WEB_SERVER_DATABASE_URL=postgres://postgres:quest1\!@127.0.0.1:5432/
export DATABASE_URL=postgres://postgres:quest1\!@127.0.0.1:5432/
export DATAANS_SERVER_ENCRYPTION_KEY=4586b451ed3d3557c856b210908124f0f46f2539ab6e62ce4180c147f935178b
export DATAANS_SERVER_DOMAIN=dataans.com
```
