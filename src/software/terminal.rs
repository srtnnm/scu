use std::env;


pub fn get_name() -> String {
  env::var( "TERM" ).unwrap_or( String::from( "Unknown" ) )
}
