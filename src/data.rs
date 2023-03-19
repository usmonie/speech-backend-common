use rsmgclient::Connection;
use sled::Db;

pub struct DataHolder {
    pub memgragh: Connection,
    pub sled: Db
}

impl DataHolder {
    fn new() -> Self {
        Self { memgragh, sled }
    }
}