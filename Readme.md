# Example Project

Rust server using Tonic for a gRPC server and Diesel for a postgreSQL ORM.

Tonic: https://docs.rs/tonic/latest/tonic/
Diesel: https://docs.rs/diesel/latest/diesel/

## Structure

[protos/orders/v1/orders.proto](protos/orders/v1/orders.proto)

Contains the proto definitions, including service, rpcs, and messages.

[build.rs](build.rs)

Instructs cargo check/build/run to build the protos into rust code.

[src/lib.rs](src/lib.rs)

Includes the compiled protos in the project.

[src/bin/main.rs](src/bin/main.rs)

Starts the server.

[src/services/orders/service.rs](src/services/orders/service.rs)

Implementations for the service and rpcs defined in the proto file. The service must have a method for each rpc with the defined request and response messages.

[src/models/orders.rs](src/models/orders.rs) and [src/models/payments.rs](src/models/payments.rs)

DB access methods.

## Diesel setup

https://diesel.rs/guides/getting-started.html

Make sure you have postgresql, rust, and cargo installed.

Install diesel_cli with only postgres: `cargo install diesel_cli --no-default-features --features postgres`

Note: This project already has a /.env file, so no need to run `echo DATABASE_URL=postgres://username:password@localhost/diesel_demo > .env`. The username is set to "postgres", which is the default on some systems, but you may have to change it (likely to your computer's username). The password should not matter unless you set a postgresql password in the past.

Migrations have already been written, so run `diesel migration run` to create postgres tables and prepopulate them with example data.

## Tonic setup

https://github.com/hyperium/tonic
