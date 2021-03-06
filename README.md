<div align="center">
  <img alt="Hydroconf logo" src="https://github.com/rubik/hydroconf/raw/master/images/logo.png" height="130" />
</div>

<div align="center">
  <h1>Hydroconf</h1>
  <p>Effortless configuration management for Rust. Keep your apps hydrated!</p>
  <a target="_blank" href="https://travis-ci.org/rubik/hydroconf">
    <img src="https://img.shields.io/travis/rubik/hydroconf?style=for-the-badge" alt="Build">
  </a>
  <a target="_blank" href="https://coveralls.io/github/rubik/hydroconf">
    <img src="https://img.shields.io/coveralls/github/rubik/hydroconf?style=for-the-badge" alt="Code Coverage">
  </a>
  <a target="_blank" href="https://crates.io/crates/hydroconf">
   <img src="https://img.shields.io/crates/d/hydroconf?style=for-the-badge" alt="Downloads (all time)">
  <a>
  <a href="https://github.com/rubik/hydroconf/blob/master/LICENSE">
    <img src="https://img.shields.io/crates/l/hydroconf?style=for-the-badge" alt="ISC License">
  </a>
  <br>
  <br>
</div>

Hydroconf is a configuration management library for Rust, based on [config-rs]
and heavily inspired by Python's [dynaconf].

# Features
* Inspired by the [12-factor] configuration principles
* Effective separation of sensitive information (secrets)
* Layered system for multi environments (e.g. development, staging, production,
  etc.)
* Sane defaults, with a 1-line configuration loading
* Read from [JSON], [TOML], [YAML], [HJSON], [INI] files

The [config-rs] library is a great building block, but it does not provide a
default mechanism to load configuration and merge secrets, while keeping the
different environments separated. Hydroconf fills this gap.

[config-rs]: https://github.com/mehcode/config-rs
[dynaconf]: https://github.com/rochacbruno/dynaconf
[12-factor]: https://12factor.net/config
[JSON]: https://github.com/serde-rs/json
[TOML]: https://github.com/toml-lang/toml
[YAML]: https://github.com/chyh1990/yaml-rust
[HJSON]: https://github.com/hjson/hjson-rust
[INI]: https://github.com/zonyitoo/rust-ini

# Quickstart

Suppose you have the following file structure:

```
├── config
│   ├── .secrets.toml
│   └── settings.toml
└── your-executable
```

`settings.toml`:

```toml
[default]
pg.port = 5432
pg.host = 'localhost'

[production]
pg.host = 'db-0'
```

`.secrets.toml`:

```toml
[default]
pg.password = 'a password'

[production]
pg.password = 'a strong password'
```

Then, in your executable source (make sure to add `serde = { version = "1.0",
features = ["derive"] }` to your dependencies):

```rust
use serde::Deserialize;
use hydroconf::Hydroconf;

#[derive(Debug, Deserialize)]
struct Config {
    pg: PostgresConfig,
}

#[derive(Debug, Deserialize)]
struct PostgresConfig {
    host: String,
    port: u16,
    password: String,
}

fn main() {
    let conf: Config = match Hydroconf::default().hydrate() {
        Ok(c) => c,
        Err(e) => {
            println!("could not read configuration: {:#?}", e);
            std::process::exit(1);
        }
    };

    println!("{:#?}", conf);
}
```

If you compile and execute the program (making sure the executable is in the
same directory where the `config` directory is), you will see the following:

```sh
$ ./your-executable
Config {
    pg: PostgresConfig {
        host: "localhost",
        port: 5432,
        password: "a password"
    }
}
```

Hydroconf will select the settings in the `[default]` table by default. If you
set `ENV_FOR_HYDRO` to `production`, Hydroconf will overwrite them with the
production ones:

```sh
$ ENV_FOR_HYDRO=production ./your-executable
Config {
    pg: PostgresConfig {
        host: "db-0",
        port: 5432,
        password: "a strong password"
    }
}
```

Settings can always be overridden with environment variables:

```bash
$ HYDRO_PG__PASSWORD="an even stronger password" ./your-executable
Config {
    pg: PostgresConfig {
        host: "localhost",
        port: 5432,
        password: "an even stronger password"
    }
}
```

The description of all Hydroconf configuration options and how the program
configuration is loaded can be found in the
[documentation](https://docs.rs/hydroconf).

<div>
  <small>
    Logo made by <a href="https://www.flaticon.com/authors/freepik" title="Freepik">Freepik</a> from <a href="https://www.flaticon.com" title="Flaticon">www.flaticon.com</a>
  </small>
</div>
