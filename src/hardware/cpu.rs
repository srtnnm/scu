use std::fs;
use regex::Regex;
use crate::utils;


pub struct CPUInfo {
  pub model:    String,
  pub vendor:   String,
  pub max_freq: utils::converter::Frequency,
  pub cores:    i8,
}

// additional info are in /sys/devices/system/cpu
// max frequency -> /sys/devices/system/cpu/cpu0/cpufreq/cpuinfo_max_freq

fn extract_i64( _str: &str ) -> i64 {
  let re = Regex::new( r"\d+" ).unwrap();
  re.find( _str )
    .unwrap()
    .as_str()
    .parse::<i64>()
    .unwrap()
}

pub fn get_info() -> CPUInfo {
  let mut model: String = String::from( "" );
  let mut vendor: String = String::from( "" );
  let mut max_freq = utils::converter::frequency_from_hz( 0 );
  let mut cores: i8 = 0;

  // parse /proc/cpuinfo
  for line in fs::read_to_string( "/proc/cpuinfo" ).expect( "NO /proc/cpuinfo FILE" )
    .split( '\n' ) {
      if line.is_empty() { break; }

      if line.contains( "model name\t:" )
      {
        model = String::from(
          line.split( ": " )
          .nth(1)
          .unwrap()
          );
      } else if line.contains( "vendor_id\t:" ) {
        vendor = String::from(
          match line.split( ": " )
          .nth(1)
          .unwrap() {
            "GenuineIntel" => "Intel",
            "AuthenticAMD" => "AMD",
            _              => "Unknown"
          }

          );
      } else if line.contains( "cpu cores\t:" ) {
        cores = line.split( ": " )
          .nth(1)
          .unwrap()
          .parse::<i8>()
          .unwrap();
      }
    }

  // get max_freq
  let max_freq_file_path = "/sys/devices/system/cpu/cpu0/cpufreq/cpuinfo_max_freq";
  if fs::metadata( max_freq_file_path ).is_ok() {
    max_freq = utils::converter::frequency_from_hz(
      extract_i64( fs::read_to_string( max_freq_file_path ).unwrap()
                   .as_str() )
      );
  }

  CPUInfo {
    model:    model,
    vendor:   vendor,
    max_freq: max_freq,
    cores:    cores,
  }
}
