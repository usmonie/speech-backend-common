use rsmgclient::Connection;
use sled::Db;

pub struct DataHolder {
    memgragh: Connection,
    sled: Db
}

impl DataHolder {
    fn new() -> Self {
        Self { memgragh, sled }
    }
}