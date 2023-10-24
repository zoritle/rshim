use fs_err as fs;
use std::{
    collections::HashMap,
    env,
    io::{Error, ErrorKind},
    path::{Path, PathBuf},
};
pub struct Shim {
    pub target_path: PathBuf,
    pub args: Option<Vec<String>>,
}

impl Shim {
    pub fn init() -> Result<Self, Error> {
        let shim_path = get_shim_file_path()?;
        let kvs = parse_shim_file(&shim_path)?;
        let target_path = match kvs.get("path") {
            Some(p) => PathBuf::from(p),
            None => {
                return Err(Error::new(
                    ErrorKind::NotFound,
                    format!("no path key in {}", shim_path.to_string_lossy()),
                ))
            }
        };
        let args = kvs.get("args").map(|a| {
            a.split_whitespace()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
        });
        Ok(Self { target_path, args })
    }
}

fn get_shim_file_path() -> Result<PathBuf, Error> {
    let mut current_exe = env::current_exe().map_err(|e| {
        Error::new(
            ErrorKind::Other,
            format!("acquiring shim executable path: {}", e),
        )
    })?;
    if !current_exe.set_extension("shim") {
        return Err(Error::new(
            ErrorKind::Other,
            format!("{} is not a file", current_exe.to_string_lossy()),
        ));
    }
    Ok(current_exe)
}
use unicode_bom::Bom;
fn parse_shim_file(shim_path: &Path) -> Result<HashMap<String, String>, Error> {
    let mut kvs = HashMap::new();

    let raw_content = fs::read_to_string(shim_path).map_err(|e| {
        Error::new(
            ErrorKind::Other,
            format!("reading {}: {}", shim_path.to_string_lossy(), e),
        )
    })?;
    //NOTE: expedient trick for utf-8 with bom
    let bom = Bom::from(raw_content.as_bytes());
    for line in raw_content[bom.len()..]
        .lines()
        .filter(|l| !l.trim().is_empty())
    {
        if let Some((key, value)) = line.split_once("=") {
            let key = key.trim();
            let value = value.trim();
            kvs.insert(key.to_string(), value.to_string());
        } else {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!("invalid line in shim file: {}", line),
            ));
        }
    }

    Ok(kvs)
}
