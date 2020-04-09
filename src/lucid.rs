use std::sync::{Arc, RwLock};

use crate::{
    configuration::Configuration,
    kvstore::{Encryption, MemoryStore},
    server::Server,
};

pub struct Lucid {
    configuration: Arc<RwLock<Configuration>>,
}

impl Lucid {
    pub fn new(configuration: Configuration) -> Self {
        Lucid {
            configuration: Arc::new(RwLock::new(configuration)),
        }
    }

    pub async fn run(&self) -> Result<(), std::io::Error> {
        let configuration = self.configuration.read().unwrap();

        let encryption = if configuration.encryption.enabled {
            if configuration.encryption.private_key.is_empty() {
                panic!("The private key must be filled.");
            } else {
                Some(Encryption::serpent(
                    hex::decode(configuration.encryption.private_key.as_str()).unwrap(),
                ))
            }
        } else {
            None
        };

        let server = Server::new(
            self.configuration.clone(),
            Arc::new(MemoryStore::new(encryption)),
        );
        server.run().await;
        Ok(())
    }
}
