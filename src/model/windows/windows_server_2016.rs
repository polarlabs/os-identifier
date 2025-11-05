//
//
const PRODUCT: &str = "Windows Server 2016";
const VENDOR: &str = "Microsoft";

#[derive(Debug)]
pub(crate) struct WindowsServer2016 {
    vendor: String,
    product: String,
    editions: Editions,
    release: Option<Release>,
    #[allow(dead_code)]
    service_channel: ServiceChannel,
}

impl WindowsServer2016 {
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

impl TryFrom<&str> for WindowsServer2016 {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = value.split('-').collect();

        if let Some(first) = parts.get(0) {
            if *first != "2016" {
                Err(String::from("Not Windows Server 2016."))
            } else {
                let vendor = VENDOR.to_string();
                let product = PRODUCT.to_string();

                if parts.len() == 1 {
                    Ok(WindowsServer2016 {
                        vendor,
                        product,
                        editions: Editions::all(),
                        release: None,
                        service_channel: ServiceChannel::default(),
                    })
                } else if parts.len() == 2 {
                    Ok(WindowsServer2016 {
                        vendor,
                        product,
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
            Edition::Essentials,
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
    Essentials,
    Standard,
}

impl std::fmt::Display for Edition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = match self {
            Edition::Datacenter => "Datacenter",
            Edition::Essentials => "Essentials",
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
    fn test_from_string_wo_release() {
        let label = WindowsServer2016::try_from("2016").unwrap();

        assert_eq!(label.vendor, "Microsoft".to_string());
        assert_eq!(label.product, "Windows Server 2016".to_string());
        assert_eq!(label.editions.len(), Editions::all().len());
        assert!(label.release.is_none());
    }
}
