use std::{fmt::Write, io, mem::MaybeUninit};

/// Gets the current system uptime.
///
/// # Errors
///
/// Returns an error if the system uptime cannot be retrieved.
#[cfg_attr(feature = "hotpath", hotpath::measure)]
pub fn get_current() -> Result<String, io::Error> {
  let uptime_seconds = {
    let mut info = MaybeUninit::uninit();
    if unsafe { libc::sysinfo(info.as_mut_ptr()) } != 0 {
      return Err(io::Error::last_os_error());
    }
    #[allow(clippy::cast_sign_loss)]
    unsafe {
      info.assume_init().uptime as u64
    }
  };

  let days = uptime_seconds / 86400;
  let hours = (uptime_seconds / 3600) % 24;
  let minutes = (uptime_seconds / 60) % 60;

  let mut result = String::with_capacity(32);
  if days > 0 {
    let _ = write!(result, "{days}");
    result.push_str(if days == 1 { " day" } else { " days" });
  }
  if hours > 0 {
    if !result.is_empty() {
      result.push_str(", ");
    }
    let _ = write!(result, "{hours}");
    result.push_str(if hours == 1 { " hour" } else { " hours" });
  }
  if minutes > 0 {
    if !result.is_empty() {
      result.push_str(", ");
    }
    let _ = write!(result, "{minutes}");
    result.push_str(if minutes == 1 { " minute" } else { " minutes" });
  }
  if result.is_empty() {
    result.push_str("less than a minute");
  }

  Ok(result)
}
