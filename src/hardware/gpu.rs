use regex::Regex;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

fn find_pci_ids() -> Option<String> {
    let mut result = String::new();
    for entry in fs::read_dir("/usr/share").unwrap() {
        let path = entry.unwrap().path();
        let path = path.to_str().unwrap();
        if fs::metadata(path.clone()).unwrap().is_dir() {
            fs::read_dir(path).unwrap().into_iter().for_each(|e| {
                if e.as_ref().unwrap().file_name() == "pci.ids" {
                    result = e.unwrap().path().to_str().unwrap().to_string();
                }
            })
        }
        if !result.is_empty() {
            return Some(result);
        }
    }

    None
}

pub fn get_info() -> Option<BTreeMap<u8, String>> {
    let mut result: BTreeMap<u8, String> = BTreeMap::new();

    if !Path::new("/sys/class/drm").exists() {
        return None;
    }

    for entry in fs::read_dir("/sys/class/drm").unwrap() {
        let entry = entry.unwrap().path();
        let entry = entry.to_str().unwrap();
        if !Regex::new(r"card\d")
            .unwrap()
            .is_match(entry.split('/').next_back().unwrap())
        {
            continue;
        }
        let uevent_path = format!("{}/device/uevent", entry);
        if Path::new(uevent_path.as_str()).exists() {
            let mut vendor = String::new();
            let mut model = String::new();
            for line in fs::read_to_string(uevent_path)
                .unwrap()
                .split("\n")
                .into_iter()
            {
                if line.starts_with("PCI_ID") {
                    let pci_id = line.split("PCI_ID=").nth(1).unwrap().to_string();
                    vendor = String::from(match pci_id.split(':').next().unwrap() {
                        "10DE" => "NVIDIA",
                        "1002" => "AMD",
                        _ => "Unknown",
                    });
                    model = pci_id
                        .split(':')
                        .nth(1)
                        .unwrap()
                        .to_string()
                        .to_ascii_lowercase();
                }
            }
            if !model.is_empty() {
                let pci_ids = find_pci_ids();
                if pci_ids.is_some() {
                    let pci_ids = pci_ids.unwrap();

                    for line in fs::read_to_string(pci_ids).unwrap().split('\n') {
                        if line.contains(format!("{}", model).as_str())
                            && line.contains('[')
                            && line.contains(']')
                        {
                            model = line.replace(model.as_str(), "").trim().to_string();
                            model = model
                                .split('[')
                                .nth(1)
                                .unwrap()
                                .to_string()
                                .split(']')
                                .next()
                                .unwrap()
                                .to_string();
                            break;
                        }
                    }
                }

                result.insert(
                    result.len() as u8 + 1,
                    if !vendor.is_empty() {
                        format!("{} ", vendor)
                    } else {
                        "".to_string()
                    } + model.as_str(),
                );
            }
        }
    }

    if !result.is_empty() {
        return Some(result);
    }

    None
}
