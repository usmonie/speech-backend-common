use rsmgclient::{Connection, ConnectParams};
use sled::{Config, Db};

pub struct DataHolder {
    pub memgragh: Connection,
    pub sled: Db
}

impl DataHolder {
    pub fn new(params: ConnectParams, sled_path: &str) -> Self {
        let config = Config::new().temporary(true);
        let sled = config.open().unwrap();

        Self {
            memgragh: Connection::connect(&params).unwrap(),
            sled,
        }
    }
}