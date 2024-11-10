use dotenv::dotenv;
use std::env;

pub const WVM_DATA_SETTLER: &str = "0x55dA54ee977FBe734d5250F0558bc4B2FBe36b2a";

pub fn get_env_var(key: &str) -> Result<String, env::VarError> {
    dotenv().ok();
    match env::var(key) {
        Ok(val) => Ok(val),
        Err(e) => Err(e),
    }
}