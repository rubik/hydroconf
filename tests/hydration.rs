use hydroconf::{ConfigError, HydroSettings, Hydroconf};
use serde::Deserialize;
use std::env;
use std::path::{Path, PathBuf};

#[derive(Debug, PartialEq, Deserialize)]
struct Config {
    pg: PostgresConfig,
    redis_url: String,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
struct DBConfig {
    redis_url: String,
}

#[derive(Debug, PartialEq, Deserialize)]
struct PostgresConfig {
    host: String,
    port: u16,
    password: String,
}

fn get_data_path(suffix: &str) -> PathBuf {
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
    target_dir.join(format!("tests/data{}", suffix))
}

#[test]
fn test_default_hydration() {
    env::set_var(
        "ROOT_PATH_FOR_HYDRO",
        get_data_path("").into_os_string().into_string().unwrap(),
    );
    let conf: Result<Config, ConfigError> = Hydroconf::default().hydrate();
    assert_eq!(
        conf.unwrap(),
        Config {
            pg: PostgresConfig {
                host: "localhost".into(),
                port: 5432,
                password: "a password".into(),
            },
            redis_url: "redis://".to_string(),
        }
    );
    env::remove_var("ROOT_PATH_FOR_HYDRO");
}

#[test]
fn test_default_hydration_with_env() {
    env::set_var(
        "ROOT_PATH_FOR_HYDRO",
        get_data_path("").into_os_string().into_string().unwrap(),
    );
    env::set_var("ENV_FOR_HYDRO", "production");
    let conf: Result<Config, ConfigError> = Hydroconf::default().hydrate();
    assert_eq!(
        conf.unwrap(),
        Config {
            pg: PostgresConfig {
                host: "db-0".into(),
                port: 5432,
                password: "a strong password".into(),
            },
            redis_url: "redis://".to_string(),
        }
    );
    env::remove_var("ROOT_PATH_FOR_HYDRO");
    env::remove_var("ENV_FOR_HYDRO");
}

#[test]
fn test_default_hydration_with_override() {
    env::set_var(
        "ROOT_PATH_FOR_HYDRO",
        get_data_path("").into_os_string().into_string().unwrap(),
    );
    env::set_var("HYDRO_PG__PORT", "1234");
    let conf: Result<Config, ConfigError> = Hydroconf::default().hydrate();
    assert_eq!(
        conf.unwrap(),
        Config {
            pg: PostgresConfig {
                host: "localhost".into(),
                port: 1234,
                password: "a password".into(),
            },
            redis_url: "redis://".to_string(),
        }
    );
    env::remove_var("ROOT_PATH_FOR_HYDRO");
    env::remove_var("HYDRO_PG__PORT");
}

#[test]
fn test_default_hydration_with_env_and_override() {
    env::set_var(
        "ROOT_PATH_FOR_HYDRO",
        get_data_path("").into_os_string().into_string().unwrap(),
    );
    env::set_var("ENV_FOR_HYDRO", "production");
    env::set_var("HYDRO_PG__PORT", "1234");
    env::set_var("HYDRO_REDIS_URL", "redis://?db=1");
    let conf: Result<Config, ConfigError> = Hydroconf::default().hydrate();
    assert_eq!(
        conf.unwrap(),
        Config {
            pg: PostgresConfig {
                host: "db-0".into(),
                port: 1234,
                password: "a strong password".into(),
            },
            redis_url: "redis://?db=1".to_string()
        }
    );
    env::remove_var("ROOT_PATH_FOR_HYDRO");
    env::remove_var("ENV_FOR_HYDRO");
    env::remove_var("HYDRO_PG__PORT");
    env::remove_var("HYDRO_REDIS_URL");
}

#[test]
fn test_default_hydration_with_env_vars_only() {
    env::set_var("ENV_FOR_HYDRO", "production");
    env::set_var("HYDRO_PG__HOST", "staging-db-23");
    env::set_var("HYDRO_PG__PORT", "29378");
    env::set_var("HYDRO_PG__PASSWORD", "a super strong password");
    env::set_var("HYDRO_REDIS_URL", "redis://");
    let conf: Result<Config, ConfigError> = Hydroconf::default().hydrate();
    assert_eq!(
        conf.unwrap(),
        Config {
            pg: PostgresConfig {
                host: "staging-db-23".into(),
                port: 29378,
                password: "a super strong password".into(),
            },
            redis_url: "redis://".to_string(),
        }
    );
    env::remove_var("ENV_FOR_HYDRO");
    env::remove_var("HYDRO_PG__PORT");
    env::remove_var("HYDRO_PG__HOST");
    env::remove_var("HYDRO_PG__PASSWORD");
    env::remove_var("HYDRO_REDIS_URL");
}

#[test]
fn test_custom_hydration() {
    env::set_var("HYDRO_PG__PORT", "2378");
    env::set_var("MYAPP_PG___PORT", "29378");
    let settings = HydroSettings::default()
        .set_root_path(get_data_path(""))
        .set_env("production".into())
        .set_envvar_prefix("MYAPP".into())
        .set_envvar_nested_sep("___".into());
    let conf: Result<Config, ConfigError> = Hydroconf::new(settings).hydrate();
    assert_eq!(
        conf.unwrap(),
        Config {
            pg: PostgresConfig {
                host: "db-0".into(),
                port: 29378,
                password: "a strong password".into(),
            },
            redis_url: "redis://".to_string(),
        }
    );
    env::remove_var("HYDRO_PG__PORT");
    env::remove_var("MYAPP_PG___PORT");
}

#[test]
fn test_multiple_dotenvs() {
    env::set_var(
        "ROOT_PATH_FOR_HYDRO",
        get_data_path("2").into_os_string().into_string().unwrap(),
    );
    env::set_var("ENV_FOR_HYDRO", "development");

    let conf: Result<Config, ConfigError> = Hydroconf::default().hydrate();
    assert_eq!(
        conf.unwrap(),
        Config {
            pg: PostgresConfig {
                host: "localhost".into(),
                port: 15330,
                password: "a password".into(),
            },
            redis_url: "redis://".to_string(),
        }
    );

    env::set_var("ENV_FOR_HYDRO", "production");
    let conf: Result<Config, ConfigError> = Hydroconf::default().hydrate();
    assert_eq!(
        conf.unwrap(),
        Config {
            pg: PostgresConfig {
                host: "db-0".into(),
                port: 12329,
                password: "a strong password".into(),
            },
            redis_url: "redis://".to_string(),
        }
    );

    env::set_var(
        "ROOT_PATH_FOR_HYDRO",
        get_data_path("3").into_os_string().into_string().unwrap(),
    );
    env::set_var("ENV_FOR_HYDRO", "development");

    let conf: Result<Config, ConfigError> = Hydroconf::default().hydrate();
    assert_eq!(
        conf.unwrap(),
        Config {
            pg: PostgresConfig {
                host: "localhost".into(),
                port: 12329,
                password: "a password".into(),
            },
            redis_url: "redis://".to_string(),
        }
    );

    env::set_var("ENV_FOR_HYDRO", "production");
    let conf: Result<Config, ConfigError> = Hydroconf::default().hydrate();
    assert_eq!(
        conf.unwrap(),
        Config {
            pg: PostgresConfig {
                host: "db-0".into(),
                port: 9999,
                password: "a strong password".into(),
            },
            redis_url: "redis://".to_string(),
        }
    );

    env::set_var(
        "ROOT_PATH_FOR_HYDRO",
        get_data_path("3").into_os_string().into_string().unwrap(),
    );
    env::set_var("ENV_FOR_HYDRO", "development");
    env::set_var("ENVVAR_PREFIX_FOR_HYDRO", "APP_");

    let conf: Result<Config, ConfigError> = Hydroconf::default().hydrate();
    assert_eq!(
        conf.unwrap(),
        Config {
            pg: PostgresConfig {
                host: "localhost".into(),
                port: 5432,
                password: "a password".into(),
            },
            redis_url: "redis://".to_string(),
        }
    );

    env::set_var("ENV_FOR_HYDRO", "production");
    let conf: Result<Config, ConfigError> = Hydroconf::default().hydrate();
    assert_eq!(
        conf.unwrap(),
        Config {
            pg: PostgresConfig {
                host: "db-0".into(),
                port: 5432,
                password: "a strong password".into(),
            },
            redis_url: "redis://".to_string(),
        }
    );
}

#[test]
/// Test that local settings override settings
fn test_local_settings() {
    env::set_var(
        "ROOT_PATH_FOR_HYDRO",
        get_data_path("4").into_os_string().into_string().unwrap(),
    );
    env::set_var("ENV_FOR_HYDRO", "development");

    let conf: Result<Config, ConfigError> = Hydroconf::default().hydrate();
    assert_eq!(
        conf.unwrap(),
        Config {
            pg: PostgresConfig {
                host: "localhost".into(),
                port: 5432,
                password: "a password".into(),
            },
            redis_url: "redis://".to_string(),
        }
    );

    env::set_var("ENV_FOR_HYDRO", "production");
    let conf: Result<Config, ConfigError> = Hydroconf::default().hydrate();
    assert_eq!(
        conf.unwrap(),
        Config {
            pg: PostgresConfig {
                host: "db-0".into(),
                port: 5555,
                password: "a strong password".into(),
            },
            redis_url: "redis://".to_string(),
        }
    );
}

#[test]
fn test_key_case_convertible() {
    let config_dir = get_data_path("_custom_filename");
    env::set_var("HATTHOC_REDIS_URL", "redis://?db=1");
    let s = HydroSettings {
        root_path: Some(config_dir),
        settings_file: Some(Path::new("base_settings.toml").into()),
        secrets_file: Some(Path::new(".secrets.toml").into()),
        env: "development".into(),
        envvar_prefix: "HATTHOC".into(),
        encoding: "utf-8".into(),
        envvar_nested_sep: "__".into(),
    };
    let conf: Result<DBConfig, ConfigError> = Hydroconf::new(s).hydrate();
    assert_eq!(
        conf.unwrap(),
        DBConfig {
            redis_url: "redis://?db=1".into(),
        }
    );
}
