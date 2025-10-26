//
//
const PRODUCT: &str = "Windows Server";
const VENDOR: &str = "Microsoft";

#[derive(Debug)]
pub(crate) struct WindowsServer2019ff {
    vendor: String,
    product: String,
    editions: Editions,
    release: Option<Release>,
    #[allow(dead_code)]
    service_channel: ServiceChannel,
}

impl WindowsServer2019ff {
    pub(super) fn to_string(&self) -> Vec<String> {
        let out = self
            .editions
            .0
            .iter()
            .map(|edition| {
                if self.release.is_none() {
                    format!(
                        "{} {} {edition}",
                        self.vendor, self.product
                    )
                } else {
                    let release = self.release.clone().unwrap();
                    format!(
                        "{} {} {edition} {release}",
                        self.vendor, self.product
                    )
                }
            })
            .collect();

        out
    }
}

impl TryFrom<&str> for WindowsServer2019ff {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = value.split('-').collect();

        if let Some(first) = parts.get(0) && let None = parts.get(1) {
            if ! (*first >= "2019" && *first <= "2999") {
                Err(String::from("Not Windows Server."))
            } else {
                let vendor = VENDOR.to_string();
                let product = PRODUCT.to_string();

                if parts.len() == 1 {
                    Ok(WindowsServer2019ff {
                        vendor,
                        product: format!("{product} {}", first),
                        editions: Editions::all(),
                        release: None,
                        service_channel: ServiceChannel::default(),
                    })
                } else if parts.len() == 2 {
                    Ok(WindowsServer2019ff {
                        vendor,
                        product: format!("{product} {}", first),
                        editions: Editions::all(),
                        release: Some(Release::from(parts[1])),
                        service_channel: ServiceChannel::default(),
                    })
                } else {
                    Err(String::from("This is not a Windows Server."))
                }
            }
        } else {
            Err(String::from("This is not a Windows Server."))
        }
    }
}

#[derive(Clone, Debug)]
struct Release(String);

impl Release {}

impl From<&str> for Release {
    fn from(value: &str) -> Self {
        Release(value.to_uppercase())
    }
}

impl std::fmt::Display for Release {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Editions(Vec<Edition>);

impl Editions {
    fn all() -> Self {
        Editions(vec![
            Edition::Datacenter,
            Edition::Standard,
        ])
    }

    #[allow(dead_code)]
    fn contains(&self, edition: Edition) -> bool {
        self.0.contains(&edition)
    }

    #[allow(dead_code)]
    fn len(&self) -> usize {
        self.0.len()
    }
}

#[derive(PartialEq, Debug)]
enum Edition {
    Datacenter,
    Standard,
}

impl std::fmt::Display for Edition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = match self {
            Edition::Datacenter => "Datacenter",
            Edition::Standard => "Standard",
        };

        write!(f, "{}", out.to_string())
    }
}

#[derive(PartialEq, Debug)]
pub(crate) enum ServiceChannel {
    LTSB,
}

impl ServiceChannel {
    #[allow(dead_code)]
    fn is_default(&self) -> bool {
        match self {
            ServiceChannel::LTSB => true,
        }
    }
}

impl Default for ServiceChannel {
    fn default() -> Self {
        ServiceChannel::LTSB
    }
}

impl std::fmt::Display for ServiceChannel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = match self {
            ServiceChannel::LTSB => "LTSB",
        };

        write!(f, "{}", out)
    }
}

impl From<&str> for ServiceChannel {
    fn from(value: &str) -> Self {
        match value {
            _ => ServiceChannel::LTSB,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_string_2019() {
        let label = WindowsServer2019ff::try_from("2019").unwrap();

        assert_eq!(label.vendor, "Microsoft".to_string());
        assert_eq!(label.product, "Windows Server 2019".to_string());
        assert_eq!(label.editions.len(), Editions::all().len());
        assert!(label.release.is_none());
    }

    #[test]
    fn test_from_string_2022() {
        let label = WindowsServer2019ff::try_from("2022").unwrap();

        assert_eq!(label.vendor, "Microsoft".to_string());
        assert_eq!(label.product, "Windows Server 2022".to_string());
        assert_eq!(label.editions.len(), Editions::all().len());
        assert!(label.release.is_none());
    }

    #[test]
    fn test_from_string_2025() {
        let label = WindowsServer2019ff::try_from("2025").unwrap();

        assert_eq!(label.vendor, "Microsoft".to_string());
        assert_eq!(label.product, "Windows Server 2025".to_string());
        assert_eq!(label.editions.len(), Editions::all().len());
        assert!(label.release.is_none());
    }
}
