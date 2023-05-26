use std::fs;

use crate::utils;

pub struct Drive {
  pub path:  String,
  pub model: String,
  pub size:  utils::converter::MemorySize,
}


fn get_devtype( content: String ) -> String {
  for line in content.split( '\n' ) {
    if line.contains( "DEVTYPE=" ) {
      let devtype = line.split( "DEVTYPE=" )
        .nth(1)
        .unwrap()
        .to_string();
      return devtype;
    }
  }

  String::from( "" )
}


pub fn scan_drives() -> Vec<Drive> {
  let mut result: Vec<Drive> = Vec::new();

  for block_device in fs::read_dir( "/sys/block" ).unwrap() {
    let device = block_device.unwrap()
      .file_name()
      .into_string()
      .unwrap();
    let mut model: String = String::from( "Unknown" );
    let mut size = utils::converter::memory_size_from_blocks( 0 );

    let device_data = format!( "/sys/block/{}/device", device );
    let device_uevent = format!( "/sys/block/{}/uevent", device );

    if device.starts_with( "dm" )
      || device.starts_with( "loop" )
      || get_devtype( fs::read_to_string( device_uevent ).unwrap() ) != String::from( "disk" ) {
      continue;
    }
    
    if fs::metadata( format!( "{}/model", device_data ) ).is_ok() {
      model = fs::read_to_string( format!( "{}/model", device_data ) ).unwrap()
        .replace( '\n', "" );
    } else {
      model = device.clone();
    }
    size = utils::converter::memory_size_from_blocks(
      fs::read_to_string( format!( "/sys/block/{}/size", device ) ).unwrap()
      .replace( '\n', "" )
      .parse::<i64>()
      .unwrap()
    );

    result.push( Drive{
      path: format!( "/dev/{}", device ),
      model: model,
      size: size
    } );
  }

  result
}