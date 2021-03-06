use std::fs;
use std::path::{Path, PathBuf};

use crate::helpers::WriteOrDeleteFile;
use async_std::fs::File;
use async_std::io::Result;
use walkdir::WalkDir;

#[derive(Clone)]
pub struct Storage {
    path: PathBuf,
}

impl Storage {
    pub fn try_new(path: &PathBuf) -> Result<Self> {
        Ok(Self {
            path: path.to_path_buf(),
        })
    }

    pub fn create_dir(&self, path: &Path, tpe: &str) -> std::io::Result<()> {
        match tpe {
            "data" => {
                for i in 0..256 {
                    fs::create_dir_all(self.path.join(path).join(tpe).join(format!("{:02x}", i)))?
                }
                Ok(())
            }
            _ => fs::create_dir_all(self.path.join(path).join(tpe)),
        }
    }

    pub fn read_dir(&self, path: &Path, tpe: &str) -> impl Iterator<Item = walkdir::DirEntry> {
        WalkDir::new(self.path.join(path).join(tpe))
            .into_iter()
            .filter_map(walkdir::Result::ok)
            .filter(|e| e.file_type().is_file())
    }

    pub fn filename(&self, path: &Path, tpe: &str, name: &str) -> PathBuf {
        match tpe {
            "config" => self.path.join(path).join("config"),
            "data" => self.path.join(path).join(tpe).join(&name[0..2]).join(name),
            _ => self.path.join(path).join(tpe).join(name),
        }
    }

    pub async fn open_file(&self, path: &Path, tpe: &str, name: &str) -> Result<File> {
        let file_path = self.filename(path, tpe, name);
        Ok(File::open(file_path).await?)
    }

    pub async fn create_file(
        &self,
        path: &Path,
        tpe: &str,
        name: &str,
    ) -> Result<WriteOrDeleteFile> {
        let file_path = self.filename(path, tpe, name);
        WriteOrDeleteFile::new(file_path).await
    }

    pub fn remove_file(&self, path: &Path, tpe: &str, name: &str) -> Result<()> {
        let file_path = self.filename(path, tpe, name);
        Ok(fs::remove_file(&file_path)?)
    }
}
