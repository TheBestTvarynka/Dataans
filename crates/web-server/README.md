# web-server

This crate implements the sync/back up server.

The purpose of this sync server is to back up user's data and sync this data between devices. More info: [`sync_server.md`](../../doc/sync_server.md).

## Local development

1. Preparations:
  ```bash
  # Set up local Postgres database. For example, using Docker:
  docker run -d --name dataans-pg -e POSTGRES_PASSWORD=<password> -v <local dir path>:/var/lib/postgresql/data -p 5432:5432 postgres

  # Logging level
  export DATAANS_WEB_SERVER_LOG=trace

  # Set up database URL:
  export DATAANS_WEB_SERVER_DATABASE_URL=<postgres connection url>
  ```
2. Set up files storage.
  1. If you plan to use your local file system as file storage, then do the following:
     ```bash
     export files_dir=<dir for user files>
     mkdir $files_dir
     export DATAANS_WEB_SERVER_FILES_DIR=$files_dir
     ```
  2. Or if you plan to use any AWS S3 compatible object storage for storing user's files, then do the following:
     ```bash
     export DATAANS_WEB_SERVER_S3_BUCKET=<bucket name>
     export AWS_ACCESS_KEY_ID=<key id>
     export AWS_ENDPOINT_URL_S3=<endpoint url>
     export AWS_REGION=<region>
     export AWS_SECRET_ACCESS_KEY=<secret>
     export BUCKET_NAME=<bucket name>
     ```
3. Run the sync server:
  1. If you decided to use local fs as file storage:
     ```bash
     cargo run -- --features fs,dev --no-default-features
     ```
  2. If you decided to use AWS S3 compatible storage:
     ```bash
     cargo run -- --features dev
     ```

## Deployment

Currently, the app is deployed using the [fly.io](https://fly.io/) cloud provider. Thus, this guide is focused on deployment using fly.io.
If you want to deploy the sync server anywhere else, then you need to alter the guide manually.

1. Prepare the Postgres database. You can use [Fly Postgres](https://fly.io/docs/postgres/) or any other database provided. For example, [Neon](https://neon.com/). Note the Postgres connection URL.
2. Install `fly` CLI: https://fly.io/docs/flyctl/install/.
3. Launch the app:
   ```bash
   fly launch
   ```
4. Set all needed secrets:
   ```bash
   # Auth
   fly secrets set DATAANS_WEB_SERVER_CF_TEAM_NAME=<team name>
   fly secrets set DATAANS_WEB_CF_AUD=<AUD>

   # Database
   fly secrets set DATAANS_WEB_SERVER_DATABASE_URL=<postgres connection url>

   # Files (object) storage
   fly secrets set DATAANS_WEB_SERVER_S3_BUCKET=<bucket name>
   fly secrets set AWS_ACCESS_KEY_ID=<key id>
   fly secrets set AWS_ENDPOINT_URL_S3=<endpoint url>
   fly secrets set AWS_REGION=<region>
   fly secrets set AWS_SECRET_ACCESS_KEY=<secret>
   fly secrets set BUCKET_NAME=<bucket name>
   ```
5. Deploy the app:
   ```bash
   fly deploy
   ```