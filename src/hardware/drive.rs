use std::fs;

use crate::utils;

pub struct Drive {
  pub path:  String,
  pub model: String,
  pub size:  utils::converter::MemorySize,
}


pub fn scan_drives() -> Vec<Drive> {
  let mut result: Vec<Drive> = Vec::new();

  for block_device in fs::read_dir( "/sys/block" ).unwrap() {
    let device = block_device.unwrap().file_name().into_string().unwrap();
    let mut model: String = String::from( "Unknown" );
    let mut size = utils::converter::memory_size_from_blocks( 0 );

    let device_data = format!( "/sys/block/{}/device", device );
    let device_type = format!( "{}/type", device_data );

    if !fs::metadata( device_type.clone() ).is_ok()
        || fs::read_to_string( device_type.clone() ).unwrap() != String::from( "0\n" ) {
      continue;
    }
    
    model = fs::read_to_string( format!( "{}/model", device_data ) ).unwrap()
      .replace( '\n', "" );
    size  = utils::converter::memory_size_from_blocks(
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
