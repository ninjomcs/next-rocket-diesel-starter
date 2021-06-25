# Next.js + Rocket + Diesel starter
A simple and minimal template for development of web applications using Next.js as a frontend and Rocket as a backend with Diesel as an ORM. 

## Setup
1. Ensure you have the latest stable versions of [rustup](https://rustup.rs/) and [Node.js](https://nodejs.org/) installed
2. Clone the repository, run `npm i`
3. Follow the [Diesel getting started guide](https://diesel.rs/guides/getting-started) for configuring and setting up Diesel

## Usage
### Notes
For development, use `npm run dev`. This starts both the Next.js development server as well as the Rocket server. There is a proxy in place for all requests to `localhost:3000/api`, which goes to `localhost:8000/api`. Therefore, you can simply fetch data using `fetch("/api")`. 

### Rust database example
To use the database connection, simply pass the `DbConn` type as a request guard to a handler. The following example uses the fictional `Logs` type and is adapted from the [Rocket documentation](https://rocket.rs/v0.4/guide/state/#databases).
```rust
#[get("/logs/<id>")]
fn get_logs(conn: DbConn, id: usize) -> Logs {
    logs::filter(id.eq(log_id)).load(&*conn)
}
```