<div align="center">
  <img alt="Hydroconf logo" src="https://github.com/rubik/hydroconf/raw/master/images/logo.png" height="130" />
</div>

<div align="center">
  <h1>Hydroconf</h1>
  <p>Configuration management for Rust. Keep your apps hydrated!</p>
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
different environment separated. Hydroconf fills this gap.

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

Then, in your executable source:

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

# Environment variables
There are two formats for the environment variables:

1. those that control how Hydroconf works have the form `*_FOR_HYDRO`;
2. those that override values in your configuration have the form `HYDRO_*`.

For example, to specify where Hydroconf should look for the configuration
files, you can set the variable `ROOT_PATH_FOR_HYDRO`. In that case, it's no
longer necessary to place the binary in the same directory as the
configuration. Hydroconf will search directly from the root path you specify.

Here is a list of all the currently supported environment variables to
configure how Hydroconf works:

* `ROOT_PATH_FOR_HYDRO`: specifies the location from which Hydroconf should
  start searching configuration files. By default, Hydroconf will start from
  the directory that contains your executable;
* `SETTINGS_FILE_FOR_HYDRO`: exact location of the main settings file;
* `SECRETS_FILE_FOR_HYDRO`: exact location of the file containing secrets;
* `ENV_FOR_HYDRO`: the environment to load after loading the `default` one
  (e.g. `development`, `testing`, `staging`, `production`, etc.). By default,
  Hydroconf will load the `development` environment, unless otherwise
  specified.
* `ENVVAR_PREFIX_FOR_HYDRO`: the prefix of the environement variables holding
  your configuration -- see group number 2. above. By default it's `HYDRO`
  (note that you don't have to include the `_` separator, as that is added
  automatically);
* `ENVVAR_NESTED_SEP_FOR_HYDRO`: the separator in the environment variables
  holding your configuration that signals a nesting point. By default it's `__`
  (double underscore), so if you set `HYDRO_REDIS__HOST=localhost`, Hydroconf
  will match it to the nested field `redis.host` in your configuration.

TODO

<div>
  <small>
    Logo made by <a href="https://www.flaticon.com/authors/freepik" title="Freepik">Freepik</a> from <a href="https://www.flaticon.com" title="Flaticon">www.flaticon.com</a>
  </small>
</div>
