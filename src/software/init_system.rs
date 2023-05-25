use std::fs;

pub fn detect() -> String {
  let mut link: String = String::new();

  let buff = fs::read_to_string("/proc/1/cmdline").unwrap()
    .as_str()
    .replace('\0', "");
  let mut buf = buff.as_str();

  if buf.contains(' ') {
    buf = buf.split(' ')
      .next()
      .unwrap();
  }

  let metadata = fs::symlink_metadata(buf);
  if metadata.is_ok() &&
    metadata.unwrap()
      .file_type()
      .is_symlink() {
        link = fs::read_link(buf).unwrap()
          .into_os_string()
          .into_string()
          .unwrap();
  }

  String::from( 
    match link.as_str() {
      "openrc-init"            => "OpenRC",
      "/lib/systemd/systemd"   => "SystemD",
      "../lib/systemd/systemd" => "SystemD",
      "/lib/sysvinit/init"     => "SysVinit",
      "../lib/sysvinit/init"   => "SysVinit",
      _                        => "Unknown"
    }
  )
}
