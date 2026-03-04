// todo:
// add more examples with arbitrary strings
// showing what we know about the OS not only the string.
// Update introducing sentence to match README.md


//! # OS Identifier
//!
//! OS Identifier resolves product / release names of operating systems used by
//! endoflife.date into canonical names.
//!
//! ```
//! use os_identifier::OS;
//!
//! fn main() {
//!     let os = OS::parse("11-24h2-w").unwrap();
//!
//!     assert_eq!(os.vendor(), String::from("Microsoft"));
//!     assert_eq!(os.product(), String::from("Windows 11"));
//!
//!     assert!(os.to_string().contains(&String::from("Microsoft Windows 11 Pro 24H2")));
//! }
//! ```
//!
//! If you already know it is a Windows, you can use this as well.
//!
//! ```
//! use os_identifier::Windows;
//!
//! fn main() {
//!     let os_strings = Windows::parse("11-24h2-w").unwrap();
//!
//!     assert!(os_strings.to_string().contains(&String::from("Microsoft Windows 11 Pro 24H2")));
//! }
//! ```
mod model;
pub use model::OS;
pub use model::Windows;

mod parser;

mod util;
