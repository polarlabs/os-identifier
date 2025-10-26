//
//
const PRODUCT: &str = "Windows Server";
const VENDOR: &str = "Microsoft";

#[derive(Debug)]
pub(crate) struct WindowsServer1709ff {
    vendor: String,
    product: String,
    editions: Editions,
    release: Option<Release>,
    #[allow(dead_code)]
    service_channel: ServiceChannel,
}

impl WindowsServer1709ff {
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

impl TryFrom<&str> for WindowsServer1709ff {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = value.split('-').collect();

        if let Some(first) = parts.get(0) && let Some(second) = parts.get(1) {
            if ! (*first >= "1709" && ( *second == "ac" || *second == "sac" )) {
                Err(String::from("Not Windows Server."))
            } else {
                let vendor = VENDOR.to_string();
                let product = PRODUCT.to_string();

                if parts.len() == 2 {
                    let service_channel = ServiceChannel::from(parts[1]);
                    let editions = Editions::from(&service_channel);

                    Ok(WindowsServer1709ff {
                        vendor,
                        product,
                        editions,
                        release: Some(Release::from(*first)),
                        service_channel,
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

impl From<&ServiceChannel> for Editions {
    fn from(value: &ServiceChannel) -> Self {
        match value {
            ServiceChannel::AC => {
                let edition = Edition::Datacenter;
                Editions(vec![edition])
            },
            ServiceChannel::SAC => Editions::all(),
        }
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
    AC,
    SAC,
}

impl ServiceChannel {
    #[allow(dead_code)]
    fn is_default(&self) -> bool {
        match self {
            ServiceChannel::SAC => true,
            ServiceChannel::AC => true,
        }
    }
}

impl Default for ServiceChannel {
    fn default() -> Self {
        ServiceChannel::SAC
    }
}

impl std::fmt::Display for ServiceChannel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = match self {
            ServiceChannel::AC => "AC",
            ServiceChannel::SAC => "SAC",
        };

        write!(f, "{}", out)
    }
}

impl From<&str> for ServiceChannel {
    fn from(value: &str) -> Self {
        match value {
            "ac" => ServiceChannel::AC,
            "sac" => ServiceChannel::SAC,
            _ => ServiceChannel::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_string_1709_sac() {
        let label = WindowsServer1709ff::try_from("1709-sac").unwrap();

        assert_eq!(label.vendor, "Microsoft".to_string());
        assert_eq!(label.product, "Windows Server".to_string());
        assert_eq!(label.editions.len(), Editions::all().len());
        assert_eq!(label.release.unwrap().0, "1709");
        assert_eq!(label.service_channel, ServiceChannel::SAC);
    }

    #[test]
    fn test_from_string_20h2_sac() {
        let label = WindowsServer1709ff::try_from("20h2-sac").unwrap();

        assert_eq!(label.vendor, "Microsoft".to_string());
        assert_eq!(label.product, "Windows Server".to_string());
        assert_eq!(label.editions.len(), Editions::all().len());
        assert_eq!(label.release.unwrap().0, "20H2");
        assert_eq!(label.service_channel, ServiceChannel::SAC);
    }

    #[test]
    fn test_from_string_23h2_ac() {
        let label = WindowsServer1709ff::try_from("23h2-ac").unwrap();

        assert_eq!(label.vendor, "Microsoft".to_string());
        assert_eq!(label.product, "Windows Server".to_string());

        assert_eq!(label.editions.len(), 1);
        assert!(label.editions.0.contains(&Edition::Datacenter));

        assert_eq!(label.release.unwrap().0, "23H2");
        assert_eq!(label.service_channel, ServiceChannel::AC);
    }
}
