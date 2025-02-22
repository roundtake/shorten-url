# shorten-url

Roundtake shorten url service.

This project is written in Rust, using Actix Web for web API service and Sea ORM for PostgreSQL database manipulation. 

## Get started

1. Install Rust
2. Clone the repo
3. Run `cargo run`

### Database migration

1. Start PostgreSQL. You can either run it on your local machine or use docker.
2. Install sea-orm-cli

    Before execute the following command, you might need to create the `shorten_url` database manually.

    ```sh
    cargo install sea-orm-cli
    ```

3. Run migration command

    ```sh
    sea-orm-cli migration up
    ```

### Generate database entity code

```sh
sea-orm-cli generate entity \
  -u postgres://postgres:postgres@localhost:5432/shorten_url \
  -o src/entities
```

Replace the database URL with yours.
