//! Cheesy way to easily wrap text in console colors.
//! Example:
//! ```
//! use phd::color;
//! println!("{}Error: {}{}", color::Red, "Something broke.", color::Reset);
//! ```

use std::{
    fmt,
    sync::atomic::{AtomicBool, Ordering as AtomicOrdering},
};

/// Whether to show colors or not.
/// Defaults to true.
static SHOW_COLORS: AtomicBool = AtomicBool::new(true);

/// Hide colors.
pub fn hide_colors() {
    SHOW_COLORS.swap(false, AtomicOrdering::Relaxed);
}

/// Are we showing colors are not?
pub fn showing_colors() -> bool {
    SHOW_COLORS.load(AtomicOrdering::Relaxed)
}

macro_rules! color {
    ($t:ident, $code:expr) => {
        #[allow(missing_docs)]
        pub struct $t;
        impl fmt::Display for $t {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                if showing_colors() {
                    write!(f, "\x1b[{}m", $code)
                } else {
                    write!(f, "")
                }
            }
        }
    };
}

color!(Black, 90);
color!(Red, 91);
color!(Green, 92);
color!(Yellow, 93);
color!(Blue, 94);
color!(Magenta, 95);
color!(Cyan, 96);
color!(White, 97);

color!(DarkBlack, 30);
color!(DarkRed, 31);
color!(DarkGreen, 32);
color!(DarkYellow, 33);
color!(DarkBlue, 34);
color!(DarkMagenta, 35);
color!(DarkCyan, 36);
color!(DarkWhite, 37);

color!(Reset, 0);
color!(Bold, 1);
color!(Underline, 4);
