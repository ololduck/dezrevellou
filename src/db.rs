use std::sync::Arc;

use tokio::sync::Mutex;

use crate::models::Comment;
use serde_json::{from_reader, to_string_pretty};
use std::fs::File;
use std::io;
use std::io::Write;
use std::ops::Deref;

use log::{debug, info};
use std::path::Path;

pub type Db = Arc<Mutex<Vec<Comment>>>;

const DB_FNAME: &str = "comments.json";

#[cfg(not(test))]
pub fn init_db() -> Db {
    match File::open(DB_FNAME) {
        Ok(json) => {
            if json.metadata().expect("could not read metadata").is_file() {
                let comments = from_reader(json).expect("Could not read db file");
                info!("read file {} for db initialization", DB_FNAME);
                return Arc::new(Mutex::new(comments));
            }
            Arc::new(Mutex::new(Vec::new()))
        }
        Err(_) => {
            info!("Init'd new db");
            Arc::new(Mutex::new(Vec::new()))
        }
    }
}

#[cfg(test)]
pub fn init_db() -> Db {
    Arc::new(Mutex::new(Vec::new()))
}

pub async fn save_db(db: Db) -> Result<(), io::Error> {
    let r = if !Path::new(DB_FNAME).exists() {
        info!("DB save file does not exist, creating…");
        File::create(DB_FNAME)
    } else {
        File::open(DB_FNAME)
    };
    match r {
        Ok(mut f) => {
            debug!("getting lock...");
            let c = db.lock().await;
            debug!("writing db file…");
            f.write_all(
                to_string_pretty(&c.deref())
                    .expect("Could not serialize Db")
                    .as_bytes(),
            )?;
            debug!("finished writing db file");
            Ok(())
        }
        Err(e) => Err(e),
    }
}
