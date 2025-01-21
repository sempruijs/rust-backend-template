# Rust backend template

Easily get started building your backend with Rust and postgresql.

## About 

Building a backend in Rust is not so hard if you know where to start.

This is a not so feature-rich template so you can build right away without any distraction.
The template uses the populair [inversion of control](https://en.wikipedia.org/wiki/Inversion_of_control) scheme which is used in many backends.

The template contains:
- create a user endpoint
- get user details endpoint protected with JWT.
- login for recieving a jwt.
- Swagger documentation for these endpoints
- Comments so you can easily read the code, copy, modify and extend your backend to your needs.
- An optional nix flake for deployment.

The template uses:
- [rocket](https://rocket.rs/), a populair easy to use web framwork.
- [sqlx](https://github.com/launchbadge/sqlx), for interacting with the postgres database.
- [jsonwebtoken](https://docs.rs/jsonwebtoken/latest/jsonwebtoken/), for generating jwt's.
- [utoipa](https://docs.rs/utoipa-swagger-ui/latest/utoipa_swagger_ui/), for generating [swager](https://swagger.io/tools/swagger-ui/) documentation.

Take a look at the [Cargo.toml](https://github.com/sempruijs/rust-backend-template/blob/main/backend/Cargo.toml)

## Usage

1. Fork or clone the repository.

```shell
git clone git@github.com:sempruijs/rust-backend-template.git
cd rust-backend-template
```

2. Remove the flake.nix and flake.lock if you don't want to use Nix.

```shell
rm flake.nix
rm flake.lock
```

3. Setup postgres on your machine. Run

```shell
psql postgres://username:password@host:port/database_name
```

And insert the following table.

```sql
-- users table
CREATE TABLE users (
    user_id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    email TEXT UNIQUE NOT NULL,
    password TEXT NOT NULL
);
```

Note that you can easily use [migrations with sqlx](https://docs.rs/sqlx/latest/sqlx/macro.migrate.html) when your schema grows.

4. Set variables:

```shell
export DATABASE_URL="postgres://username:password@host:port/database_name"
export SECRET_KEY="Your secret key :)"
```

5. And run:
```
cd backend/
cargo run
```

If everything went well you have a running backend that you can easily customize.

## Contributing

If you find a bug, spell mistake or have an adition that complements the minimal setup, please create an [issue](https://github.com/sempruijs/rust-backend-template/issues/new?template=Blank+issue) or [pull request](https://github.com/sempruijs/rust-backend-template/pulls).
Thank you <3

## License

MIT
