use std::path::PathBuf;

pub fn get_var<'a, T>(key: &'a str, suffix: &'a str) -> Option<T>
where
    T: FromVar,
{
    let full_key = format!("{}{}", key, suffix);
    match std::env::var(full_key) {
        Err(_) => None,
        Ok(v) => FromVar::parse(v),
    }
}

pub fn get_var_default<'a, T>(key: &'a str, suffix: &'a str, default: T) -> T
where
    T: FromVar,
{
    get_var(key, suffix).unwrap_or(default)
}

pub trait FromVar {
    fn parse(var: String) -> Option<Self>
    where
        Self: Sized;
}

impl FromVar for PathBuf {
    fn parse(var: String) -> Option<Self> {
        Some(PathBuf::from(var))
    }
}

impl FromVar for String {
    fn parse(var: String) -> Option<Self> {
        Some(var)
    }
}
