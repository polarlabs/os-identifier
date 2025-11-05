//! # OS Identifier
//!
//! OS Identifier resolves product / release names of operating systems used by
//! endoflife.date into canonical names.
//!
//! ```
//! use os_identifier::Windows;
//!
//! fn main() {
//!     let os_strings = Windows::try_from("11-24h2-w").unwrap();
//!
//!     assert!(os_strings.to_string().contains(&String::from("Microsoft Windows 11 Pro 24H2")));
//! }
//! ```

mod model;
pub use model::Windows;

mod parser;

mod util;
