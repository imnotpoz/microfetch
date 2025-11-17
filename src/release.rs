use std::{
  fmt::Write as _,
  fs::File,
  io::{self, Read},
};

use nix::sys::utsname::UtsName;

#[must_use]
#[cfg_attr(feature = "hotpath", hotpath::measure)]
pub fn get_system_info(utsname: &UtsName) -> String {
  let sysname = utsname.sysname().to_str().unwrap_or("Unknown");
  let release = utsname.release().to_str().unwrap_or("Unknown");
  let machine = utsname.machine().to_str().unwrap_or("Unknown");

  // Pre-allocate capacity: sysname + " " + release + " (" + machine + ")"
  let capacity = sysname.len() + 1 + release.len() + 2 + machine.len() + 1;
  let mut result = String::with_capacity(capacity);

  write!(result, "{sysname} {release} ({machine})").unwrap();
  result
}

/// Gets the pretty name of the OS from `/etc/os-release`.
///
/// # Errors
///
/// Returns an error if `/etc/os-release` cannot be read.
#[cfg_attr(feature = "hotpath", hotpath::measure)]
pub fn get_os_pretty_name() -> Result<String, io::Error> {
  // We use a stack-allocated buffer here, which seems to perform MUCH better
  // than `BufReader`. In hindsight, I should've seen this coming.
  let mut buffer = String::with_capacity(1024);
  File::open("/etc/os-release")?.read_to_string(&mut buffer)?;

  for line in buffer.lines() {
    if let Some(pretty_name) = line.strip_prefix("PRETTY_NAME=") {
      if let Some(trimmed) = pretty_name
        .strip_prefix('"')
        .and_then(|s| s.strip_suffix('"'))
      {
        return Ok(trimmed.to_owned());
      }
      return Ok(pretty_name.to_owned());
    }
  }
  Ok("Unknown".to_owned())
}
