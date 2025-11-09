const VENDOR: &str = "Microsoft";
const PRODUCT: &str = "Windows 11";

#[derive(Debug)]
pub(crate) struct Windows11 {
    vendor: String,
    product: String,
    release: Release,
    editions: Editions,
    service_channel: ServiceChannel,
}

impl Windows11 {
    pub(crate) fn build(release: Release, service_channel: ServiceChannel) -> Windows11 {
        Windows11 {
            vendor: VENDOR.to_string(),
            product: PRODUCT.to_string(),
            release,
            editions: Editions(vec![]),
            service_channel,
        }
    }

    pub(crate) fn editions(mut self, editions: Editions) -> Windows11 {
        self.editions = editions;
        self
    }

    pub(super) fn to_string(&self) -> Vec<String> {
        let out = self
            .editions
            .0
            .iter()
            .map(|edition| {
                if self.service_channel.is_default() {
                    format!(
                        "{} {} {edition} {}",
                        self.vendor, self.product, self.release
                    )
                } else {
                    format!(
                        "{} {} {edition} {} {}",
                        self.vendor, self.product, self.release, self.service_channel
                    )
                }
            })
            .collect();

        out
    }
}

impl TryFrom<&str> for Windows11 {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Ok(label) = crate::parser::endoflife::EndOfLifeLabel::try_from(value) {
            crate::parser::endoflife::windows::Windows11Parser::parse(&label)
        } else {
            let label = crate::parser::generic::GenericLabel::from(value);
            crate::parser::generic::windows::Windows11Parser::parse(&label)
        }
    }
}

#[derive(Debug)]
pub(crate) struct Release(String);

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
pub(crate) struct Editions(pub(crate) Vec<Edition>);

impl Editions {
    #[allow(dead_code)]
    pub(crate) fn contains(&self, edition: Edition) -> bool {
        self.0.contains(&edition)
    }

    #[allow(dead_code)]
    fn len(&self) -> usize {
        self.0.len()
    }
}

#[derive(PartialEq, Debug)]
pub(crate) enum Edition {
    Education,
    Enterprise,
    EnterpriseMultiSession,
    Home,
    IoTEnterprise,
    Pro,
    ProEducation,
    ProForWorkstations,
}

impl std::fmt::Display for Edition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = match self {
            Edition::Education => "Education",
            Edition::Enterprise => "Enterprise",
            Edition::EnterpriseMultiSession => "Enterprise multi-session",
            Edition::Home => "Home",
            Edition::IoTEnterprise => "IoT Enterprise",
            Edition::Pro => "Pro",
            Edition::ProEducation => "Pro Education",
            Edition::ProForWorkstations => "Pro for Workstations",
        };

        write!(f, "{}", out.to_string())
    }
}

#[derive(PartialEq, Debug)]
pub(crate) enum ServiceChannel {
    GAC,
    LTSC,
}

impl ServiceChannel {
    fn is_default(&self) -> bool {
        match self {
            ServiceChannel::GAC => true,
            ServiceChannel::LTSC => false,
        }
    }
}

impl From<&str> for ServiceChannel {
    fn from(value: &str) -> Self {
        match value {
            "lts" => ServiceChannel::LTSC,
            _ => ServiceChannel::GAC,
        }
    }
}

impl Default for ServiceChannel {
    fn default() -> Self {
        ServiceChannel::GAC
    }
}

impl std::fmt::Display for ServiceChannel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = match self {
            ServiceChannel::GAC => "GAC",
            ServiceChannel::LTSC => "LTSC",
        };

        write!(f, "{}", out)
    }
}

#[cfg(test)]
mod tests {
    use crate::model;
    use super::*;

    #[test]
    fn test_from_string_iot_lts() {
        let label = Windows11::try_from("11-24h2-iot-lts").unwrap();

        assert_eq!(label.vendor, "Microsoft".to_string());
        assert_eq!(label.product, "Windows 11".to_string());
        assert_eq!(label.release.to_string(), "24H2".to_string());

        assert_eq!(label.editions.len(), Editions::all_iot().len());
        assert_eq!(label.service_channel, ServiceChannel::LTSC);
    }

    #[test]
    fn test_from_string_e_lts() {
        let label = Windows11::try_from("11-24h2-e-lts").unwrap();

        assert_eq!(label.vendor, "Microsoft".to_string());
        assert_eq!(label.product, "Windows 11".to_string());
        assert_eq!(label.release.to_string(), "24H2".to_string());

        assert_eq!(label.editions.len(), Editions::all_e().len());
        assert_eq!(label.service_channel, ServiceChannel::LTSC);
    }

    #[test]
    fn test_from_string_e() {
        let label = Windows11::try_from("11-24h2-e").unwrap();

        assert_eq!(label.vendor, "Microsoft".to_string());
        assert_eq!(label.product, "Windows 11".to_string());
        assert_eq!(label.release.to_string(), "24H2".to_string());

        assert_eq!(label.editions.len(), Editions::all_e().len());
        assert_eq!(label.service_channel, ServiceChannel::GAC);
    }

    #[test]
    fn test_from_string_w() {
        let label = Windows11::try_from("11-24h2-w").unwrap();

        assert_eq!(label.vendor, "Microsoft".to_string());
        assert_eq!(label.product, "Windows 11".to_string());
        assert_eq!(label.release.to_string(), "24H2".to_string());

        assert_eq!(label.editions.len(), Editions::all_w().len());
        assert_eq!(label.service_channel, ServiceChannel::GAC);
    }

    #[test]
    fn test_from_string_arbitrary1() {
        let label = model::Windows11::try_from("Windows 11 Professional Edition (Build 26100) (64 Bit) GA (General Availability)").unwrap();

        assert_eq!(label.vendor, "Microsoft".to_string());
        assert_eq!(label.product, "Windows 11".to_string());
        assert_eq!(label.release.to_string(), "24H2".to_string());

        assert!(label.editions.contains(model::windows_11::Edition::Pro));
        assert_eq!(label.service_channel, model::windows_11::ServiceChannel::GAC);
    }

    #[test]
    fn test_from_string_arbitrary2() {
        let label = model::Windows11::try_from("Windows 11 Enterprise Edition (Build 26100) (64 Bit) GA (General Availability)").unwrap();

        assert_eq!(label.vendor, "Microsoft".to_string());
        assert_eq!(label.product, "Windows 11".to_string());
        assert_eq!(label.release.to_string(), "24H2".to_string());

        assert!(label.editions.contains(model::windows_11::Edition::Enterprise));
        assert_eq!(label.service_channel, model::windows_11::ServiceChannel::GAC);
    }

    #[test]
    fn test_from_string_arbitrary3() {
        let label = model::Windows11::try_from("Microsoft Windows 11 Enterprise 22000.1219").unwrap();

        assert_eq!(label.vendor, "Microsoft".to_string());
        assert_eq!(label.product, "Windows 11".to_string());
        assert_eq!(label.release.to_string(), "21H2".to_string());

        assert!(label.editions.contains(Edition::Enterprise));
        assert_eq!(label.service_channel, ServiceChannel::GAC);
    }

    #[test]
    fn test_from_string_arbitrary4() {
        let label = Windows11::try_from("Microsoft Windows 11 Enterprise 21H2").unwrap();

        assert_eq!(label.vendor, "Microsoft".to_string());
        assert_eq!(label.product, "Windows 11".to_string());
        assert_eq!(label.release.to_string(), "21H2".to_string());

        assert!(label.editions.contains(Edition::Enterprise));
        assert_eq!(label.service_channel, ServiceChannel::GAC);
    }
}
