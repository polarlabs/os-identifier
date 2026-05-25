This library seamlessly transforms diverse operating system product and release names from various tools 
into consistent, standardized canonical names, empowering reliable identification and integration.

It supports unstructured, inconsistent, and human-entered input as well as the identifiers used by the 
endoflife.date API.

# Unstructured Input

Inventory tools or software provisioning tools often use names like the following to describe an operating system 
instance. This library converts strings like this into a standardized form.

```rust
pub fn main() { 
  let os = OS::parse("Windows 11 Enterprise Edition (Build 26100) (64 Bit) GA (General Availability)").unwrap();

  assert_eq!(os.vendor(), "Microsoft".to_string());
  assert_eq!(os.product(), "Windows 11".to_string());
  assert_eq!(os.release(), "24H2".to_string());

  assert!(os.is_enterprise());
  assert!(!os.is_lts());
}
```

```rust
use os_identifier::Linux;

pub fn main() {
  let os = OS::parse("Ubuntu 24.04 LTS (Noble Numbat)").unwrap();

  assert_eq!(os.vendor(), "Canonical".to_string());
  assert_eq!(os.product(), "Ubuntu Linux".to_string());
  assert_eq!(os.release(), "24H2".to_string());

  assert!(os.is_enterprise());
  assert!(os.is_lts());
}
```

# Structured input from endoflife.date API

The endoflife.date API provides well-structured data and standardized names for products and releases. However, the 
names used are not familiar, e.g. the product windows has a release called 11-24h2-w. This refers to quite some 
Windows 11 operating systems, such as

* Microsoft Windows 11 Home 24H2
* Microsoft Windows 11 Pro 24H2
* [...]

This library resolves Windows releases referred to as "11-24h2-w" into a list of standardized operating system names 
covered by this name.

```rust
fn main() {
    let os = OS::parse("windows-11-24h2-e-lts").unwrap();

    assert_eq!(os.vendor(), "Microsoft".to_string());
    assert_eq!(os.product(), "Windows 11".to_string());
    assert_eq!(os.release(), "24H2".to_string());

    assert_eq!(os.is_enterprise());
    assert_eq!(os.is_lts());
}
```

# Supported products

As of 25.05.2026, arbitrary, unstructured names are supported for Windows 10 and 11, Windows Server 2019ff, 
Debian, Oracle Linux, RHEL and Ubuntu.

Regarding releases available at the endoflife.date API, the library supports any release 
related to these products:

* [Debian](https://endoflife.date/api/v1/products/debian)
* [Oracle Linux](https://endoflife.date/api/v1/products/oracle-linux)
* [RHEL](https://endoflife.date/api/v1/products/rhel)
* [Ubuntu](https://endoflife.date/api/v1/products/ubuntu)
* [Windows](https://endoflife.date/api/v1/products/windows) up to Windows 11 26H1
* [Windows Server](https://endoflife.date/api/v1/products/windows-server) up to Windows Server 2025.

# Roadmap

* Support other Linux operating systems
* Support other Windows operating systems:
  * [Windows Embedded](https://endoflife.date/api/v1/products/windows-embedded)
  * [Windows Nano Server](https://endoflife.date/api/v1/products/windows-nano-server)
  * [Windows Server Core](https://endoflife.date/api/v1/products/windows-server-core)

# License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or https://opensource.org/license/apache-2-0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
