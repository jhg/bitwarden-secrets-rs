use std::collections::HashMap;
use std::io;
use crate::helpers::BwObject;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ConfigItem {
    pub object: BwObject,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(flatten)]
    items: HashMap<String, ConfigItem>
}

impl Config {
    /// Check current directory, and all parents, for a `.bw-secrets.json` file and load it.
    pub(crate) fn find() -> io::Result<Self> {
        let mut directory = std::env::current_dir()?;

        loop {
            let path = directory.join(".bw-secrets.json");
            if path.exists() {
                return serde_json::from_reader(std::fs::File::open(path)?)
                    .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err));
            }

            if !directory.pop() {
                break;
            }
        }

        return Err(io::Error::new(io::ErrorKind::NotFound, "No .bw-secrets.json file found"));
    }
}

impl IntoIterator for Config {
    type Item = (String, ConfigItem);
    type IntoIter = std::collections::hash_map::IntoIter<String, ConfigItem>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}
