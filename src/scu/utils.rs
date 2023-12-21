pub mod converter;
pub mod libc;

pub mod string {
    pub fn remove_multiple_spaces(_str: String) -> String {
        _str.split(' ')
            .filter(|s| !s.is_empty())
            .fold(String::new(), |a, b| a + b + " ")
            .trim()
            .to_string()
    }
}

pub fn which(name: &str) -> Option<String> {
    let mut result: String = String::new();

    let env_path = std::env::var("PATH");
    if env_path.is_err() {
        return None;
    }
    let env_path = env_path.unwrap();

    for path in env_path.split(':') {
        if !path.is_empty() {
            match std::fs::read_dir(path) {
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

pub fn scan_dir(path: std::path::PathBuf) -> Vec<std::path::PathBuf> {
    let mut result: Vec<std::path::PathBuf> = Vec::new();
    if let Ok(readdir) = std::fs::read_dir(path) {
        for entry in readdir {
            if let Ok(entry) = entry {
                if entry.metadata().unwrap().is_dir() {
                    scan_dir(entry.path())
                        .iter()
                        .for_each(|e| result.push(e.clone()));
                } else {
                    result.push(entry.path());
                }
            }
        }
    }
    result
}
