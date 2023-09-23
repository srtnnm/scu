use std::env;
use std::fs;

pub fn which(name: &str) -> Option<String> {
    let mut result: String = String::new();

    let env_path = env::var("PATH");
    if env_path.is_err() {
        return None;
    }
    let env_path = env_path.unwrap();

    for path in String::from(env_path).split(':') {
        if !path.is_empty() {
            match fs::read_dir(&path) {
                Ok(readdir) => {
                    readdir.for_each(|file| {
                        let file = file.unwrap();
                        if !file.metadata().unwrap().is_dir() && file.file_name() == name {
                            result = String::from(file.path().as_path().to_str().unwrap());
                        }
                    });
                }
                Err(_) => {
                    continue;
                }
            }
            if !result.is_empty() {
                break;
            }
        }
    }

    if !result.is_empty() {
        return Some(result);
    }
    None
}
