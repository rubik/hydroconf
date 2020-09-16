//! # Quickstart
//!
//! Suppose you have the following file structure:
//!
//! ```text
//! ├── config
//! │   ├── .secrets.toml
//! │   └── settings.toml
//! └── your-executable
//! ```
//!
//! `settings.toml`:
//!
//! ```toml
//! [default]
//! pg.port = 5432
//! pg.host = 'localhost'
//!
//! [production]
//! pg.host = 'db-0'
//! ```
//!
//! `.secrets.toml`:
//!
//! ```toml
//! [default]
//! pg.password = 'a password'
//!
//! [production]
//! pg.password = 'a strong password'
//! ```
//!
//! Then, in your executable source (make sure to add `serde = { version = "1.0",
//! features = ["derive"] }` to your dependencies):
//!
//! ```rust
//! use serde::Deserialize;
//! use hydroconf::Hydroconf;
//!
//! #[derive(Debug, Deserialize)]
//! struct Config {
//!     pg: PostgresConfig,
//! }
//!
//! #[derive(Debug, Deserialize)]
//! struct PostgresConfig {
//!     host: String,
//!     port: u16,
//!     password: String,
//! }
//!
//! fn main() {
//!     let conf: Config = match Hydroconf::default().hydrate() {
//!         Ok(c) => c,
//!         Err(e) => {
//!             println!("could not read configuration: {:#?}", e);
//!             std::process::exit(1);
//!         }
//!     };
//!
//!     println!("{:#?}", conf);
//! }
//! ```
//!
//! If you compile and execute the program (making sure the executable is in the
//! same directory where the `config` directory is), you will see the following:
//!
//! ```sh
//! $ ./your-executable
//! Config {
//!     pg: PostgresConfig {
//!         host: "localhost",
//!         port: 5432,
//!         password: "a password"
//!     }
//! }
//! ```
//!
//! Hydroconf will select the settings in the `[default]` table by default. If you
//! set `ENV_FOR_HYDRO` to `production`, Hydroconf will overwrite them with the
//! production ones:
//!
//! ```sh
//! $ ENV_FOR_HYDRO=production ./your-executable
//! Config {
//!     pg: PostgresConfig {
//!         host: "db-0",
//!         port: 5432,
//!         password: "a strong password"
//!     }
//! }
//! ```
//!
//! Settings can always be overridden with environment variables:
//!
//! ```bash
//! $ HYDRO_PG__PASSWORD="an even stronger password" ./your-executable
//! Config {
//!     pg: PostgresConfig {
//!         host: "localhost",
//!         port: 5432,
//!         password: "an even stronger password"
//!     }
//! }
//! ```
//! # Environment variables
//! There are two formats for the environment variables:
//!
//! 1. those that control how Hydroconf works have the form `*_FOR_HYDRO`;
//! 2. those that override values in your configuration have the form `HYDRO_*`.
//!
//! For example, to specify where Hydroconf should look for the configuration
//! files, you can set the variable `ROOT_PATH_FOR_HYDRO`. In that case, it's no
//! longer necessary to place the binary in the same directory as the
//! configuration. Hydroconf will search directly from the root path you specify.
//!
//! Here is a list of all the currently supported environment variables to
//! configure how Hydroconf works:
//!
//! * `ROOT_PATH_FOR_HYDRO`: specifies the location from which Hydroconf should
//!   start searching configuration files. By default, Hydroconf will start from
//!   the directory that contains your executable;
//! * `SETTINGS_FILE_FOR_HYDRO`: exact location of the main settings file;
//! * `SECRETS_FILE_FOR_HYDRO`: exact location of the file containing secrets;
//! * `ENV_FOR_HYDRO`: the environment to load after loading the `default` one
//!   (e.g. `development`, `testing`, `staging`, `production`, etc.). By default,
//!   Hydroconf will load the `development` environment, unless otherwise
//!   specified.
//! * `ENVVAR_PREFIX_FOR_HYDRO`: the prefix of the environement variables holding
//!   your configuration -- see group number 2. above. By default it's `HYDRO`
//!   (note that you don't have to include the `_` separator, as that is added
//!   automatically);
//! * `ENVVAR_NESTED_SEP_FOR_HYDRO`: the separator in the environment variables
//!   holding your configuration that signals a nesting point. By default it's `__`
//!   (double underscore), so if you set `HYDRO_REDIS__HOST=localhost`, Hydroconf
//!   will match it with the nested field `redis.host` in your configuration.
//!
//! # Hydroconf initialization
//! You can create a new Hydroconf struct in two ways.
//!
//! The first one is to use the `Hydroconf::default()` method, which will use the
//! default settings. The default constructor will attempt to load the settings
//! from the environment variables (those in the form `*_FOR_HYDRO`), and if it
//! doesn't find them it will use the default values. The alternative is to create
//! a `HydroSettings` struct manually and pass it to `Hydroconf`:
//!
//! ```rust
//! let hydro_settings = HydroSettings::default()
//!     .set_envvar_prefix("MYAPP".into())
//!     .set_env("staging".into());
//! let hydro = Hydroconf::new(hydro_settings);
//! ```
//!
//! Note that `HydroSettings::default()` will still try to load the settings from
//! the environment before you overwrite them.
//!
//! # The hydration process
//! ## 1. Configuration loading
//! When you call `Hydroconf::hydrate()`, Hydroconf starts looking for your
//! configuration files and if it finds them, it loads them. The search starts from
//! `HydroSettings.root_path`; if the root path is not defined, Hydroconf will use
//! `std::env::current_exe()`. From this path, Hydroconf generates all the possible
//! candidates by walking up the directory tree, also searching in the `config`
//! subfolder at each level. For example, if the root path is
//! `/home/user/www/api-server/dist`, Hydroconf will try the following paths, in
//! this order:
//!
//! 1. `/home/user/www/api-server/dist/config`
//! 2. `/home/user/www/api-server/dist`
//! 3. `/home/user/www/api-server/config`
//! 4. `/home/user/www/api-server`
//! 5. `/home/user/www/config`
//! 6. `/home/user/www`
//! 7. `/home/user/config`
//! 8. `/home/user`
//! 9. `/home/config`
//! 10. `/home`
//! 11. `/config`
//! 12. `/`
//!
//! In each directory, Hydroconf will search for the files
//! `settings.{toml,json,yaml,ini,hjson}` and
//! `.secrets.{toml,json,yaml,ini,hjson}`. As soon as one of those (or both) are
//! found, the search stops and Hydroconf won't search the remaining upper levels.
//!
//! ## 2. Merging
//! In this step, Hydroconf merges the values from the different environments
//! from the configuration files discovered in the previous step. Hydroconf
//! first checks if an environment called `default` exists: in that case those
//! values are selected first. Then, it checks if the environment specified for
//! Hydroconf (`ENV_FOR_HYDRO`, or "development" if not specified) exists and in
//! that case it selects those values and merges them with the existing ones.
//!
//! ## 3. `.env` file overrides
//! In this step Hydroconf starts from the root path (the same one from step 1),
//! and walks the filesystem upward in search of an `.env` file. If it finds
//! one, it parses it and merges those values with the existing ones.
//!
//! ## 4. Environment variables overrides
//! In this step Hydroconf merges the values from all environment variables that
//! you defined with the Hydro prefix (`HYDRO_` by default, as explained in the
//! [previous section](#environment-variables)).
//!
//! ## 5. Deserialization
//! Finally, Hydroconf tries to deserialize the configuration into the return
//! type you specify, which should be your configuration struct.
//!
//! # Best practices
//! In order to keep your configuration simple, secure and effective, Hydroconf
//! makes it easy for you to follow these best practices:
//! 1. keep the non-secret values inside `config/settings.{toml,yaml,json,...}`
//!    separated by environment; in particular, define a "default" environment
//!    which contains the base settings, and specialize it in all the additional
//!    environments that you need (e.g. "development", "testing", "staging",
//!    "production", etc.);
//! 2. keep the secret values inside `config/.secrets.{toml,yaml,json,...}`
//!    separated by environment and exclude this file from version control;
//! 3. define the environment variable `ENV_FOR_DYNACONF` to specify which
//!    environment should be loaded (besides the "default" one, which is always
//!    loaded first);
//! 4. if you want to override some values, or specify some secret values which
//!    are not in the secret file, define the environment variables `HYDRO_*`
//!    (or use a custom prefix and define `ENVVAR_PREFIX_FOR_HYDRO`).

mod env;
mod hydro;
mod settings;
mod utils;

pub use hydro::{Config, ConfigError, Environment, File, Hydroconf};
pub use settings::HydroSettings;
