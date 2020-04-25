#![recursion_limit = "256"]
mod protos;
mod server;
#[cfg(test)]
mod tests;

use server::reader_server::create_server;
use std::env;

const READER_SERVICE_PORT_ENV_VAR_NAME: &str = "READER_SERVICE_PORT";
const DEFAULT_READER_SERVICE_PORT: usize = 55055;

const READER_SERVICE_HOSTNAME_ENV_VAR_NAME: &str = "READER_SERVICE_HOST";
const DEFAULT_READER_SERVICE_HOSTNAME: &str = "127.0.0.1";

fn get_env_var_or_default<T: std::str::FromStr + Clone>(
    env_var_name: &str,
    env_var_default_value: T,
) -> T {
    env::var(env_var_name)
        .and_then(|env_var_str_value| {
            str::parse(&env_var_str_value).map_err(|_| std::env::VarError::NotPresent)
        })
        .unwrap_or(env_var_default_value)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port = get_env_var_or_default(
        READER_SERVICE_PORT_ENV_VAR_NAME,
        DEFAULT_READER_SERVICE_PORT,
    );
    let hostname = get_env_var_or_default(
        READER_SERVICE_HOSTNAME_ENV_VAR_NAME,
        DEFAULT_READER_SERVICE_HOSTNAME.to_string(),
    );
    create_server(&hostname, port)
}
