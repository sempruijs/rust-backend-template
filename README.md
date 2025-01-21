# Rust Backend Template

Easily get started building your backend with Rust and PostgreSQL.

## About 

Building a backend in Rust is not so hard if you know where to start.

This is a not-so-feature-rich template so you can build right away without any distractions.  
The template uses the popular [inversion of control](https://en.wikipedia.org/wiki/Inversion_of_control) scheme, which is used in many backends.

The template contains:
- Create a user endpoint.
- Get user details endpoint protected with JWT.
- Login for receiving a JWT.
- Swagger documentation for these endpoints.
- Comments so you can easily read the code, copy, modify, and extend your backend to your needs.
- An optional Nix flake for deployment.

The template uses:
- [Rocket](https://rocket.rs/), a popular and easy-to-use web framework.
- [SQLx](https://github.com/launchbadge/sqlx), for interacting with the PostgreSQL database.
- [jsonwebtoken](https://docs.rs/jsonwebtoken/latest/jsonwebtoken/), for generating JWTs.
- [utoipa](https://docs.rs/utoipa-swagger-ui/latest/utoipa_swagger_ui/), for generating [Swagger](https://swagger.io/tools/swagger-ui/) documentation.

Take a look at the [Cargo.toml](https://github.com/sempruijs/rust-backend-template/blob/main/backend/Cargo.toml).

## Usage

1. Fork or clone the repository.

```shell
git clone git@github.com:sempruijs/rust-backend-template.git
cd rust-backend-template
```

2. Remove the `flake.nix` and `flake.lock` if you don't want to use Nix.

```shell
rm flake.nix
rm flake.lock
```

3. Set up PostgreSQL on your machine. Run:

```shell
psql postgres://username:password@host:port/database_name
```

And insert the following table:

```sql
-- users table
CREATE TABLE users (
    user_id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    email TEXT UNIQUE NOT NULL,
    password TEXT NOT NULL
);
```

Note that you can easily use [migrations with SQLx](https://docs.rs/sqlx/latest/sqlx/macro.migrate.html) as your schema grows.

4. Set variables:

```shell
export DATABASE_URL="postgres://username:password@host:port/database_name"
export SECRET_KEY="Your secret key :)"
```

5. Run:

```shell
cd backend/
cargo run
```

If everything went well, you have a running backend that you can easily customize.

## Contributing

If you find a bug, spelling mistake, or have an addition that complements the minimal setup, please create an [issue](https://github.com/sempruijs/rust-backend-template/issues/new?template=Blank+issue) or [pull request](https://github.com/sempruijs/rust-backend-template/pulls).  
Thank you ❤️

## License

MIT