# PKL Moment Backend

## Running locally

1. Install [Rust](https://www.rust-lang.org/) via [rustup](https://www.rust-lang.org/learn/get-started) or your package
   manager
2. Install [cargo-shuttle](https://docs.shuttle.rs/getting-started/installation) **NOTE:** You do **NOT** need to login
   to
   Shuttle
3. Run [PostgresQL](https://www.postgresql.org/) with its port assigned to `localhost:5432` or `127.0.0.1:5432`
4. Change your directory to the backend directory:
    ```bash
    cd pkl-moment/backend
    ```
5. Run the project with:
    ```bash
    cargo shuttle run
    ```