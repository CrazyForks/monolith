use std::collections::HashMap;
use std::path::Path;

use redb::{Database, Error, TableDefinition};

pub struct CacheMetadataItem {
    data: Option<Vec<u8>>, // Asset's blob; used for caching small files or if on-disk database isn't utilized
    media_type: Option<String>, // MIME-type, things like "text/plain", "image/png"...
    charset: Option<String>, // "UTF-8", "UTF-16"...
}

// #[derive(Debug)]
pub struct Cache {
    min_file_size: usize, // Only use database for assets larger than this size (in bytes), otherwise keep them in RAM
    metadata: HashMap<String, CacheMetadataItem>, // Dictionary of metadata (and occasionally data [mostly for very small files])
    db: Option<Database>, // Pointer to database instance; None if not yet initialized or if failed to initialize
    db_ok: Option<bool>, // None by default, Some(true) if was able to initialize database, Some (false) if an error occured
}

const TABLE: TableDefinition<&str, &[u8]> = TableDefinition::new("_"); // The one and only table used for caching assets

impl Cache {
    pub fn new(min_file_size: usize, db_file_path: Option<String>) -> Cache {
        let mut cache = Cache {
            min_file_size: min_file_size,
            metadata: HashMap::new(),
            db: None,
            db_ok: None,
        };

        // Initialize database
        if db_file_path.is_some() {
            match Database::create(Path::new(&db_file_path.unwrap())) {
                Ok(db) => {
                    cache.db = Some(db);
                    cache.db_ok = Some(true);
                    cache
                }
                Err(..) => {
                    // eprintln!("Error: unable to initialize cache database in {}", &temp_file.path().display());
                    cache.db_ok = Some(false);
                    cache
                }
            }
        } else {
            cache.db_ok = Some(false);
            cache
        }
    }

    pub fn set(&mut self, key: &str, data: &Vec<u8>, media_type: String, charset: String) {
        // Store metadata information outside of database (data, media type, charset)
        let mut cache_metadata_item: CacheMetadataItem = CacheMetadataItem {
            data: if self.db_ok.is_some() && self.db_ok.unwrap() {
                None
            } else {
                Some((&*data.to_owned()).to_vec())
            },
            media_type: Some(media_type.to_owned()),
            charset: Some(charset),
        };

        // TODO: If it's already there, but the updated value exceeds min_file_size, delete metadata[].data and store it in database

        // TODO: If database is not there, attempt to initialize it

        if (self.db_ok.is_none() || !self.db_ok.unwrap()) || data.len() <= self.min_file_size {
            cache_metadata_item.data = Some((&*data.to_owned()).to_vec());
        } else {
            match self.db.as_ref().unwrap().begin_write() {
                Ok(write_txn) => {
                    {
                        let mut table = write_txn.open_table(TABLE).unwrap();
                        table.insert(key, &*data.to_owned()).unwrap();
                    }
                    write_txn.commit().unwrap();
                }
                Err(..) => {
                    // Fallback to caching in RAM
                    cache_metadata_item.data = Some((&*data.to_owned()).to_vec());
                }
            }
        }

        self.metadata
            .insert((*key).to_string(), cache_metadata_item);
    }

    pub fn get(&self, key: &str) -> Result<(Vec<u8>, String, String), Error> {
        if self.metadata.contains_key(key) {
            let metadata_item = self.metadata.get(key).unwrap();

            if metadata_item.data.is_some() {
                return Ok((
                    metadata_item.data.as_ref().unwrap().to_vec(),
                    metadata_item.media_type.as_ref().expect("").to_string(),
                    metadata_item.charset.as_ref().expect("").to_string(),
                ));
            } else {
                if self.db_ok.is_some() && self.db_ok.unwrap() {
                    let read_txn = self.db.as_ref().unwrap().begin_read()?;
                    let table = read_txn.open_table(TABLE)?;
                    let key = table.get(key)?; //.ok_or(Self::GetError::NotFound)?;
                    let bytes = key.unwrap();
                    return Ok((
                        bytes.value().to_vec(),
                        metadata_item.media_type.as_ref().expect("").to_string(),
                        metadata_item.charset.as_ref().expect("").to_string(),
                    ));
                }
            }
        }

        Err(Error::TransactionInProgress) // XXX
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.metadata.contains_key(key)
    }
}
