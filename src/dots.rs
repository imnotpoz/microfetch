use crate::{colors::IS_NO_COLOR, BLUE, CYAN, GREEN, MAGENTA, RED, RESET, YELLOW};

macro_rules! GLYPH  {() => {""}}

macro_rules! GAP    {() => {"  "}}

const NO_COLORS_STR: &str = concat!(
  GLYPH!(), GAP!(),
  GLYPH!(), GAP!(),
  GLYPH!(), GAP!(),
  GLYPH!(), GAP!(),
  GLYPH!(), GAP!(),
  GLYPH!(),
);

const COLORS_STR: &str = concat!(
  BLUE!(),    GLYPH!(), GAP!(),
  CYAN!(),    GLYPH!(), GAP!(),
  GREEN!(),   GLYPH!(), GAP!(),
  YELLOW!(),  GLYPH!(), GAP!(),
  RED!(),     GLYPH!(), GAP!(),
  MAGENTA!(), GLYPH!(), RESET!(),
);

#[must_use]
#[cfg_attr(feature = "hotpath", hotpath::measure)]
pub fn print_dots() -> &'static str {
  if *IS_NO_COLOR {
    NO_COLORS_STR
  } else {
    COLORS_STR
  }
}
