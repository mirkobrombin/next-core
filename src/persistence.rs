use crate::bottle::Bottle;
use crate::Error;
use std::fs;
use std::path::PathBuf;

pub struct Persistence {
    base_path: PathBuf,
}

impl Persistence {
    pub fn new(base_path: impl Into<PathBuf>) -> Self {
        Self {
            base_path: base_path.into(),
        }
    }

    fn index_file(&self) -> PathBuf {
        self.base_path.join("bottles.json")
    }

    pub fn load_bottles(&self) -> Result<Vec<Bottle>, Error> {
        let path = self.index_file();
        if !path.exists() {
            return Ok(Vec::new());
        }

        let content = fs::read_to_string(&path).map_err(Error::Io)?;
        let bottles: Vec<Bottle> = serde_json::from_str(&content).map_err(|e| Error::Io(e.into()))?;
        Ok(bottles)
    }

    pub fn save_bottles(&self, bottles: &[Bottle]) -> Result<(), Error> {
        if !self.base_path.exists() {
            fs::create_dir_all(&self.base_path).map_err(Error::Io)?;
        }

        let content = serde_json::to_string_pretty(bottles).map_err(|e| Error::Io(e.into()))?;
        fs::write(self.index_file(), content).map_err(Error::Io)?;
        Ok(())
    }
}
