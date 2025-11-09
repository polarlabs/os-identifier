This library seamlessly transforms diverse operating system product and release names from various tools 
into consistent, standardized canonical names, empowering reliable identification and integration.

It supports unstructured, inconsistent, and human-entered input as well as the identifiers used by the 
endoflife.date API.

# Unstructured Input

Inventory tools or software provisioning tools often use names like the following to describe an operating system 
instance. This library converts strings like this into a standardized form.

```rust
pub fn main() {
    let label = model::Windows11::try_from("Windows 11 Enterprise Edition (Build 26100) (64 Bit) GA (General Availability)").unwrap();

    assert_eq!(label.vendor, "Microsoft".to_string());
    assert_eq!(label.product, "Windows 11".to_string());
    assert_eq!(label.release.to_string(), "24H2".to_string());

    assert!(label.editions.contains(model::windows_11::Edition::Enterprise));
    assert_eq!(label.service_channel, model::windows_11::ServiceChannel::GAC);
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
    let label = Windows11::try_from("11-24h2-e-lts").unwrap();

    assert_eq!(label.vendor, "Microsoft".to_string());
    assert_eq!(label.product, "Windows 11".to_string());
    assert_eq!(label.release.to_string(), "24H2".to_string());

    assert_eq!(label.editions.len(), Editions::all_e().len());
    assert_eq!(label.service_channel, ServiceChannel::LTSC);
}
```

# Supported products

As of 08.11.25 arbitrary, unstructured names are supported for Windows 10 and 11.

Regarding releases available at the endoflife.date API, the library supports any release 
related to these products:

* [Windows](https://endoflife.date/api/v1/products/windows) up to Windows 11 25H2
* [Windows Server](https://endoflife.date/api/v1/products/windows-server) up to Windows Server 2025.

# Roadmap

* Support for Linux operating systems
* Support other Windows operating systems:
  * [Windows Embedded](https://endoflife.date/api/v1/products/windows-embedded)
  * [Windows Nano Server](https://endoflife.date/api/v1/products/windows-nano-server)
  * [Windows Server Core](https://endoflife.date/api/v1/products/windows-server-core)

# License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or https://opensource.org/license/apache-2-0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
