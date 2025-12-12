use std::sync::LazyLock;

// All this because concat!() doesn't accept const parameters
// See https://github.com/rust-lang/rust/issues/31383
#[macro_export]
macro_rules! RESET    {() => {"\x1b[0m"}}
#[macro_export]
macro_rules! BLUE     {() => {"\x1b[34m"}}
#[macro_export]
macro_rules! CYAN     {() => {"\x1b[34m"}}
#[macro_export]
macro_rules! GREEN    {() => {"\x1b[32m"}}
#[macro_export]
macro_rules! YELLOW   {() => {"\x1b[33m"}}
#[macro_export]
macro_rules! RED      {() => {"\x1b[31m"}}
#[macro_export]
macro_rules! MAGENTA  {() => {"\x1b[35m"}}

pub struct Colors {
  pub reset:   &'static str,
  pub blue:    &'static str,
  pub cyan:    &'static str,
  pub green:   &'static str,
  pub yellow:  &'static str,
  pub red:     &'static str,
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
      }
    } else {
      Self {
        reset:   RESET!(),
        blue:    BLUE!(),
        cyan:    CYAN!(),
        green:   GREEN!(),
        yellow:  YELLOW!(),
        red:     RED!(),
      }
    }
  }
}

pub static IS_NO_COLOR: LazyLock<bool> = LazyLock::new(|| {
  // Check for NO_COLOR once at startup
  const NO_COLOR: *const libc::c_char = c"NO_COLOR".as_ptr();
  unsafe { !libc::getenv(NO_COLOR).is_null() }
});

pub static COLORS: LazyLock<Colors> = LazyLock::new(|| {
  Colors::new(*IS_NO_COLOR)
});
