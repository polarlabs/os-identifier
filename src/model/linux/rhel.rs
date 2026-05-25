const VENDOR: &str = "Red Hat";
const PRODUCT: &str = "Red Hat Enterprise Linux";

#[derive(Debug)]
pub(crate) struct RedHatEnterpriseLinux {
    vendor: String,
    product: String,
    release: Release,
    editions: Editions,
    service_channel: ServiceChannel,
}

impl RedHatEnterpriseLinux {
    pub(crate) fn build(release: Release, service_channel: ServiceChannel) -> RedHatEnterpriseLinux {
        RedHatEnterpriseLinux {
            vendor: VENDOR.to_string(),
            product: PRODUCT.to_string(),
            release,
            editions: Editions(vec![]),
            service_channel,
        }
    }

    pub(super) fn vendor(&self) -> &str {
        self.vendor.as_str()
    }

    pub(super) fn product(&self) -> &str {
        self.product.as_str()
    }

    pub(super) fn release(&self) -> String {
        self.release.to_string()
    }

    pub(crate) fn editions(mut self, editions: Editions) -> RedHatEnterpriseLinux {
        self.editions = editions;
        self
    }

    pub(crate) fn is_enterprise(&self) -> bool {
        true
    }

    pub(crate) fn is_lts(&self) -> bool {
        true
    }

    pub(super) fn to_string(&self) -> Vec<String> {
        vec![format!(
            "{} {}",
            self.product, self.release
        )]
    }
}

impl TryFrom<&str> for RedHatEnterpriseLinux {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Ok(label) = crate::parser::endoflife::EndOfLifeLabel::try_from(value) {
            crate::parser::endoflife::linux::RedHatEnterpriseLinuxParser::parse(&label)
        } else {
            let label = crate::parser::generic::GenericLabel::from(value);
            crate::parser::generic::linux::RedHatEnterpriseLinuxParser::parse(&label)
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
    pub(crate) fn all() -> Self {
        Editions(vec![])
    }

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
    Core,
    Desktop,
    Server,
}

impl std::fmt::Display for Edition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = match self {
            Edition::Core => "Core",
            Edition::Desktop => "Desktop",
            Edition::Server => "Server",
        };

        write!(f, "{}", out.to_string())
    }
}

#[derive(PartialEq, Debug)]
pub(crate) enum ServiceChannel {
    LTS,
}

impl From<&Release> for ServiceChannel {
    fn from(value: &Release) -> ServiceChannel {
        ServiceChannel::LTS
    }
}

impl Default for ServiceChannel {
    fn default() -> Self {
        ServiceChannel::LTS
    }
}

impl std::fmt::Display for ServiceChannel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = match self {
            ServiceChannel::LTS => "LTS",
        };

        write!(f, "{}", out)
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_string_1() {
        let label = RedHatEnterpriseLinux::try_from("rhel-10").unwrap();

        assert_eq!(label.vendor, "Red Hat".to_string());
        assert_eq!(label.product, "Red Hat Enterprise Linux".to_string());
        assert_eq!(label.release.to_string(), "10".to_string());

        assert_eq!(label.editions.len(), Editions::all().len());
        assert_eq!(label.service_channel, ServiceChannel::LTS);
    }

    #[test]
    fn test_from_string_2() {
        let label = RedHatEnterpriseLinux::try_from("rhel-9").unwrap();

        assert_eq!(label.vendor, "Red Hat".to_string());
        assert_eq!(label.product, "Red Hat Enterprise Linux".to_string());
        assert_eq!(label.release.to_string(), "9".to_string());

        assert_eq!(label.editions.len(), Editions::all().len());
        assert_eq!(label.service_channel, ServiceChannel::LTS);
    }

    #[test]
    fn test_from_string_3() {
        let label = RedHatEnterpriseLinux::try_from("rhel-10.1").unwrap();

        assert_eq!(label.vendor, "Red Hat".to_string());
        assert_eq!(label.product, "Red Hat Enterprise Linux".to_string());
        assert_eq!(label.release.to_string(), "10.1".to_string());

        assert_eq!(label.editions.len(), Editions::all().len());
        assert_eq!(label.service_channel, ServiceChannel::LTS);
    }

    #[test]
    fn test_from_string_4() {
        let label = RedHatEnterpriseLinux::try_from("rhel-5.11").unwrap();

        assert_eq!(label.vendor, "Red Hat".to_string());
        assert_eq!(label.product, "Red Hat Enterprise Linux".to_string());
        assert_eq!(label.release.to_string(), "5.11".to_string());

        assert_eq!(label.editions.len(), Editions::all().len());
        assert_eq!(label.service_channel, ServiceChannel::LTS);
    }

    #[test]
    fn test_from_string_arbitrary1() {
        let label = RedHatEnterpriseLinux::try_from("Red Hat Enterprise Linux 9.4 (Plow)").unwrap();

        assert_eq!(label.vendor, "Red Hat".to_string());
        assert_eq!(label.product, "Red Hat Enterprise Linux".to_string());
        assert_eq!(label.release.to_string(), "9.4".to_string());

        assert_eq!(label.editions.len(), 0);
        assert_eq!(label.service_channel, ServiceChannel::LTS);
    }

    #[test]
    fn test_from_string_arbitrary2() {
        let label = RedHatEnterpriseLinux::try_from("Red Hat Enterprise Linux 6.10").unwrap();

        assert_eq!(label.vendor, "Red Hat".to_string());
        assert_eq!(label.product, "Red Hat Enterprise Linux".to_string());
        assert_eq!(label.release.to_string(), "6.10".to_string());

        assert_eq!(label.editions.len(), 0);
        assert_eq!(label.service_channel, ServiceChannel::LTS);
    }

    #[test]
    fn test_from_string_arbitrary3() {
        let label = RedHatEnterpriseLinux::try_from("Red Hat Enterprise Linux Server release 5.11 (Tikanga)").unwrap();

        assert_eq!(label.vendor, "Red Hat".to_string());
        assert_eq!(label.product, "Red Hat Enterprise Linux".to_string());
        assert_eq!(label.release.to_string(), "5.11".to_string());

        assert_eq!(label.editions.len(), 0);
        assert_eq!(label.service_channel, ServiceChannel::LTS);
    }
}
