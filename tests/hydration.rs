use std::env;
use std::path::PathBuf;
use serde::Deserialize;
use hydroconf::{ConfigError, Hydroconf};

#[derive(Debug, PartialEq, Deserialize)]
struct Config {
    pg: PostgresConfig,
}

#[derive(Debug, PartialEq, Deserialize)]
struct PostgresConfig {
    host: String,
    port: u16,
    password: String,
}

fn get_data_path() -> PathBuf {
    let mut target_dir = PathBuf::from(
        env::current_exe()
            .expect("exe path")
            .parent()
            .expect("exe parent"),
    );
    while target_dir.file_name() != Some(std::ffi::OsStr::new("target")) {
        if !target_dir.pop() {
            panic!("Cannot find target directory");
        }
    }
    target_dir.pop();
    target_dir.join("tests/data")
}

#[test]
fn test_default_hydration() {
    env::set_var("ROOT_PATH_FOR_HYDRO", get_data_path().into_os_string().into_string().unwrap());
    let conf: Result<Config, ConfigError> = Hydroconf::default().hydrate();
    assert!(conf.is_ok());
    assert_eq!(conf.unwrap(), Config {
            pg: PostgresConfig {
                host: "localhost".into(),
                port: 5432,
                password: "a password".into(),
            },
        }
    );
    env::remove_var("ROOT_PATH_FOR_HYDRO");
}

#[test]
fn test_default_hydration_with_env() {
    env::set_var("ROOT_PATH_FOR_HYDRO", get_data_path().into_os_string().into_string().unwrap());
    env::set_var("ENV_FOR_HYDRO", "production");
    let conf: Result<Config, ConfigError> = Hydroconf::default().hydrate();
    assert!(conf.is_ok());
    assert_eq!(conf.unwrap(), Config {
            pg: PostgresConfig {
                host: "db-0".into(),
                port: 5432,
                password: "a strong password".into(),
            },
        }
    );
    env::remove_var("ROOT_PATH_FOR_HYDRO");
    env::remove_var("ENV_FOR_HYDRO");
}

#[test]
fn test_default_hydration_with_override() {
    env::set_var("ROOT_PATH_FOR_HYDRO", get_data_path().into_os_string().into_string().unwrap());
    env::set_var("HYDRO_PG__PORT", "1234");
    let conf: Result<Config, ConfigError> = Hydroconf::default().hydrate();
    assert!(conf.is_ok());
    assert_eq!(conf.unwrap(), Config {
            pg: PostgresConfig {
                host: "localhost".into(),
                port: 1234,
                password: "a password".into(),
            },
        }
    );
    env::remove_var("ROOT_PATH_FOR_HYDRO");
    env::remove_var("HYDRO_PG__PORT");
}

#[test]
fn test_default_hydration_with_env_and_override() {
    env::set_var("ROOT_PATH_FOR_HYDRO", get_data_path().into_os_string().into_string().unwrap());
    env::set_var("ENV_FOR_HYDRO", "production");
    env::set_var("HYDRO_PG__PORT", "1234");
    let conf: Result<Config, ConfigError> = Hydroconf::default().hydrate();
    assert!(conf.is_ok());
    assert_eq!(conf.unwrap(), Config {
            pg: PostgresConfig {
                host: "db-0".into(),
                port: 1234,
                password: "a strong password".into(),
            },
        }
    );
    env::remove_var("ROOT_PATH_FOR_HYDRO");
    env::remove_var("ENV_FOR_HYDRO");
    env::remove_var("HYDRO_PG__PORT");
}

#[test]
fn test_default_hydration_with_env_vars_only() {
    env::set_var("ENV_FOR_HYDRO", "production");
    env::set_var("HYDRO_PG__HOST", "staging-db-23");
    env::set_var("HYDRO_PG__PORT", "29378");
    env::set_var("HYDRO_PG__PASSWORD", "a super strong password");
    let conf: Result<Config, ConfigError> = Hydroconf::default().hydrate();
    assert!(conf.is_ok());
    assert_eq!(conf.unwrap(), Config {
            pg: PostgresConfig {
                host: "staging-db-23".into(),
                port: 29378,
                password: "a super strong password".into(),
            },
        }
    );
    env::remove_var("ENV_FOR_HYDRO");
    env::remove_var("HYDRO_PG__PORT");
    env::remove_var("HYDRO_PG__HOST");
    env::remove_var("HYDRO_PG__PASSWORD");
}
