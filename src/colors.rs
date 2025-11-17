use std::{env, sync::LazyLock};

pub struct Colors {
  pub reset:   &'static str,
  pub blue:    &'static str,
  pub cyan:    &'static str,
  pub green:   &'static str,
  pub yellow:  &'static str,
  pub red:     &'static str,
  pub magenta: &'static str,
}

impl Colors {
  const fn new(is_no_color: bool) -> Self {
    if is_no_color {
      Self {
        reset:   "",
        blue:    "",
        cyan:    "",
        green:   "",
        yellow:  "",
        red:     "",
        magenta: "",
      }
    } else {
      Self {
        reset:   "\x1b[0m",
        blue:    "\x1b[34m",
        cyan:    "\x1b[36m",
        green:   "\x1b[32m",
        yellow:  "\x1b[33m",
        red:     "\x1b[31m",
        magenta: "\x1b[35m",
      }
    }
  }
}

pub static COLORS: LazyLock<Colors> = LazyLock::new(|| {
  // check for NO_COLOR once at startup
  let is_no_color = env::var("NO_COLOR").is_ok();
  Colors::new(is_no_color)
});

#[must_use]
pub fn print_dots() -> String {
  format!(
    "{}  {}  {}  {}  {}  {}  {}",
    COLORS.blue,
    COLORS.cyan,
    COLORS.green,
    COLORS.yellow,
    COLORS.red,
    COLORS.magenta,
    COLORS.reset,
  )
}
