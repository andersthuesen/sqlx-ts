use crate::cli::{Cli, DatabaseType};
use std::env::var;

/// Config is used to determine connection configurations for primary target Database
/// It uses 2 sources of config and they are used in following priorities
/// 1. any configuration via CLI options
/// 2. any dotenv configured options
#[derive(Clone)]
pub struct Config {
    pub db_host: String,
    pub db_port: i32,
    pub db_user: String,
    pub db_pass: Option<String>,
    pub db_name: Option<String>,
}

fn required_var_msg(key: &str) -> String {
    format!(
        "{} is not provided neither by an environment variable or CLI argument",
        key
    )
}

impl Config {
    pub fn new(cli_args: Cli) -> Config {
        return Config {
            db_host: match cli_args.db_host {
                Some(db_host) => db_host,
                None => var("DB_HOST").expect(required_var_msg("DB_HOST").as_str()),
            },
            db_port: match cli_args.db_port {
                Some(db_port) => db_port,
                None => var("DB_PORT")
                    .map(|x| x.trim().to_owned())
                    .map(|x| {
                        x.to_string()
                            .parse::<i32>()
                            .expect("DB_PORT is not a valid integer")
                    })
                    .expect(required_var_msg("DB_PORT").as_str()),
            },
            db_user: match cli_args.db_user {
                Some(db_user) => db_user,
                None => var("DB_USER").expect(required_var_msg("DB_USER").as_str()),
            },
            db_pass: match cli_args.db_pass {
                Some(db_pass) => Some(db_pass),
                None => var("DB_PASS").ok(),
            },
            db_name: match cli_args.db_name {
                Some(db_name) => Some(db_name),
                None => var("DB_NAME").ok(),
            },
        };
    }

    pub fn get_postgres_cred(&self) -> String {
        format!(
            "host={} user={} password={} port={:?}",
            self.db_host,
            self.db_user,
            self.db_pass.as_ref().unwrap_or(&"".to_string()),
            self.db_port,
        )
    }
}
