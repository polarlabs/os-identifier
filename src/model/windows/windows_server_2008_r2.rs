//
//
const PRODUCT: &str = "Windows Server 2008 R2";
const VENDOR: &str = "Microsoft";

#[derive(Debug)]
pub(crate) struct WindowsServer2008R2 {
    vendor: String,
    product: String,
    editions: Editions,
    release: Option<Release>,
    #[allow(dead_code)]
    service_channel: ServiceChannel,
}

impl WindowsServer2008R2 {
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

impl TryFrom<&str> for WindowsServer2008R2 {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = value.split('-').collect();

        if let Some(first) = parts.get(0) && let Some(second) = parts.get(1) {
            if ! (*first == "2008" && *second == "r2") {
                Err(String::from("Not Windows Server 2008 R2."))
            } else {
                let vendor = VENDOR.to_string();
                let product = PRODUCT.to_string();

                if parts.len() == 2 {
                    Ok(WindowsServer2008R2 {
                        vendor,
                        product,
                        editions: Editions::all(),
                        release: None,
                        service_channel: ServiceChannel::default(),
                    })
                } else if parts.len() == 3 {
                    Ok(WindowsServer2008R2 {
                        vendor,
                        product,
                        editions: Editions::all(),
                        release: Some(Release::from(parts[2])),
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
            Edition::Enterprise,
            Edition::Foundation,
            Edition::HPC,
            Edition::Standard,
            Edition::Web,
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
    Enterprise,
    Foundation,
    HPC,
    Standard,
    Web,
}

impl std::fmt::Display for Edition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = match self {
            Edition::Datacenter => "Datacenter",
            Edition::Enterprise => "Enterprise",
            Edition::Foundation => "Foundation",
            Edition::HPC => "HPC",
            Edition::Standard => "Standard",
            Edition::Web => "Web",
        };

        write!(f, "{}", out.to_string())
    }
}

#[derive(PartialEq, Debug)]
pub(crate) enum ServiceChannel {
    LTSC,
}

impl ServiceChannel {
    #[allow(dead_code)]
    fn is_default(&self) -> bool {
        match self {
            ServiceChannel::LTSC => true,
        }
    }
}

impl Default for ServiceChannel {
    fn default() -> Self {
        ServiceChannel::LTSC
    }
}

impl std::fmt::Display for ServiceChannel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = match self {
            ServiceChannel::LTSC => "LTSC",
        };

        write!(f, "{}", out)
    }
}

impl From<&str> for ServiceChannel {
    fn from(value: &str) -> Self {
        match value {
            _ => ServiceChannel::LTSC,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_string_wo_release() {
        let label = WindowsServer2008R2::try_from("2008-r2").unwrap();

        assert_eq!(label.vendor, "Microsoft".to_string());
        assert_eq!(label.product, "Windows Server 2008 R2".to_string());
        assert_eq!(label.editions.len(), Editions::all().len());
        assert!(label.release.is_none());
    }

    #[test]
    fn test_from_string_w_release() {
        let label = WindowsServer2008R2::try_from("2008-r2-sp1").unwrap();

        assert_eq!(label.vendor, "Microsoft".to_string());
        assert_eq!(label.product, "Windows Server 2008 R2".to_string());
        assert_eq!(label.editions.len(), Editions::all().len());
        assert!(label.release.is_some());
        assert_eq!(label.release.unwrap().to_string(), "SP1".to_string());
    }
}
