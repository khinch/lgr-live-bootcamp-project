use dotenvy::dotenv;
use lazy_static::lazy_static;
use std::env as std_env;

pub const JWT_COOKIE_NAME: &str = "jwt";

lazy_static! {
    pub static ref JWT_SECRET: String = set_token();
    pub static ref APP_SERVICE_EXTERNAL_ADDRESS: String = load_or_default(
        "APP_SERVICE_EXTERNAL_ADDRESS",
        "http://localhost:8000"
    );
    pub static ref APP_SERVICE_CONTAINER_ADDRESS: String = load_or_default(
        "APP_SERVICE_CONTAINER_ADDRESS",
        "http://localhost:8000"
    );
}

fn load_env() {
    dotenv().ok();
}

fn set_token() -> String {
    load_env();
    let secret =
        std_env::var(env::JWT_SECRET_ENV_VAR).expect("JWT_SECRET must be set.");
    if secret.is_empty() {
        panic!("JWT_SECRET must not be empty.");
    }
    secret
}

fn load_or_default(variable_name: &str, default_value: &str) -> String {
    load_env();

    match std_env::var(variable_name) {
        Ok(value) => {
            if value.is_empty() {
                String::from(default_value)
            } else {
                value
            }
        }
        Err(_) => String::from(default_value),
    }
}

pub mod env {
    pub const JWT_SECRET_ENV_VAR: &str = "JWT_SECRET";
}

pub mod prod {
    pub const APP_ADDRESS: &str = "0.0.0.0:3000";
}

pub mod test {
    pub const APP_ADDRESS: &str = "127.0.0.1:0";
}
