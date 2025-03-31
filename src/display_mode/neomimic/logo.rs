use regex_lite::Regex;

const TUX: &str = "        $8#####
       $8#######
       $8##$7^$8#$7^$8##
       $8#$3#####$8#
     $8##$7##$3###$7##$8##
    $8#$7##########$8##
   $8#$7############$8##
   $8#$7############$8###
  $3##$8#$7###########$8##$3#
$3######$8#$7#######$8#$3######
$3#######$8#$7#####$8#$3#######
  $3#####$8#######$3#####";
pub const TUX_WIDTH: usize = 21;
pub const TUX_HEIGHT: usize = 12;

pub fn print_logo() {
    let mut linux_logo = TUX.to_string();

    let color_re = Regex::new(r"\$\d").unwrap();
    for color in color_re.find_iter(&linux_logo.clone()) {
        if let Some(color_int) = color
            .as_str()
            .strip_prefix("$")
            .and_then(|color| color.parse::<u64>().ok())
        {
            linux_logo = linux_logo.replace(color.as_str(), &format!("\x1b[38;5;{color_int}m"));
        }
    }

    print!("{linux_logo}");
    println!("\x1b[0m");
}
