use crate::pci_ids::PciIdentifiers;
use regex::Regex;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

fn lower(_str: &str) -> String {
    let mut result = String::from(_str);
    result.make_ascii_lowercase();
    result
}

pub fn get_info() -> Option<BTreeMap<u8, String>> {
    let mut result: BTreeMap<u8, String> = BTreeMap::new();

    if !Path::new("/sys/class/drm").exists() {
        return None;
    }

    let drm_content = fs::read_dir("/sys/class/drm");
    if !drm_content.is_ok() {
        return None;
    }

    for entry in drm_content.unwrap() {
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
                        "8086" => "Intel",
                        _ => "Unknown",
                    });
                    model = pci_id.to_string().to_ascii_lowercase();
                } else if line.starts_with("PCI_SUBSYS_ID") {
                    if !line.is_empty() {
                        model.push_str(" ");
                    }
                    model.push_str(
                        line.split("PCI_SUBSYS_ID=")
                            .nth(1)
                            .unwrap()
                            .to_string()
                            .as_str(),
                    );
                }
            }
            if !model.is_empty() {
                if PciIdentifiers::contains_key(lower(model.as_str()).as_str()) {
                    let id = lower(model.as_str());
                    let name = PciIdentifiers::get(id.as_str());
                    if name.is_some() {
                        model = name.unwrap().to_string();
                    }
                } else if model.contains(' ')
                    && PciIdentifiers::contains_key(
                        lower(model.split(' ').next().unwrap()).as_str(),
                    )
                {
                    let id = lower(model.split(' ').next().unwrap());
                    let name = PciIdentifiers::get(id.as_str());
                    if name.is_some() {
                        model = name.unwrap().to_string();
                    }
                }
                if model.contains('[') && model.contains(']') {
                    model = model
                        .split('[')
                        .nth(1)
                        .unwrap()
                        .split(']')
                        .next()
                        .unwrap()
                        .to_string();
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
