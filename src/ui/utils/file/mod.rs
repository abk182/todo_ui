use std::{fs, io};

use std::fs::create_dir;

fn use_dir(dir: Option<&str>) -> Result<&str, io::Error> {
    let path = dir.unwrap_or("todos");
    match create_dir(path) {
        Ok(_) => Ok(path),
        Err(e) => {
            if e.kind() == io::ErrorKind::AlreadyExists {
                Ok(path)
            } else {
                Err(e)
            }
        }
    }
}

pub fn list_files(dir: Option<&str>) -> Result<Vec<String>, io::Error> {
    let mut vector: Vec<String> = vec![];
    let entities = fs::read_dir(use_dir(dir)?)?;

    for entity in entities {
        let file_name = entity?
            .file_name()
            .into_string()
            .unwrap_or_else(|err| panic!("can't convert file name into string. {:?}", err));

        vector.push(file_name);
    }

    Ok(vector)
}
