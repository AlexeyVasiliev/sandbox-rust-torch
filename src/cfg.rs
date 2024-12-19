extern crate dotenv;
use std::env::{self, VarError};
pub struct Config {
    pub nats_conn_string: String,
    pub nats_subject_in:String,
    pub nats_subject_out:String,
}

impl Config {
    pub fn build() -> Result<Config, VarError> {
        let nats_conn_string = match env::var("NATS_CONN") {
            Ok(nats_conn_string) => nats_conn_string,
            Err(error) =>  return Err(error),
        };
        let nats_subject_in = match env::var("NATS_SUBJECT_IN") {
            Ok(nats_subject_in) => nats_subject_in,
            Err(error) =>  return Err(error),
        };
        let nats_subject_out = match env::var("NATS_SUBJECT_OUT") {
            Ok(nats_subject_out) => nats_subject_out,
            Err(error) =>  return Err(error),
        };
      
        Ok(Config {
            nats_conn_string,
            nats_subject_in,
            nats_subject_out
        })
    }
    pub fn init_env() {
        dotenv::dotenv().ok(); // Reads the .env file
    }
}