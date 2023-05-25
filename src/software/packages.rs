use crate::utils;
use std::process::Command;
use std::collections::BTreeMap;

pub struct PackageManager {
  pub manager: &'static str,
  pub count_of_packages: i32
}

pub fn detect_manager( managers: Vec<&'static str> ) -> &'static str {
  for mgr in managers {
    if !utils::fs::_which( mgr ).is_empty() {
      return mgr;
    }
  }

  "Unknown"
}

pub fn get_info() -> PackageManager {
  let list_packages_command = BTreeMap::from([
    ( "pacman", Vec::from( [ "-Qq" ] ) ), 
    ( "dpkg",   Vec::from( [ "-f", "'.\n'", "-W" ] ) ),
    ( "rpm",    Vec::from( [ "-qa" ] ) )
  ]);

  let manager = detect_manager( list_packages_command.clone().into_keys().collect() );
  let mut result = PackageManager{ manager: manager.clone(), count_of_packages: 0 };

  if manager == "Unknown" {
    return result;
  } else {
    if list_packages_command.contains_key( manager ) {
      let args: Vec<String> = list_packages_command.get( manager )
        .unwrap()
        .iter()
        .map(|s| s.to_string())
        .collect();

      result.count_of_packages = Command::new( manager ).args( args )
        .output()
        .unwrap()
        .stdout
        .iter()
        .filter( |b| **b == b'\n' )
        .count() as i32;
    }
  }

  result
}
