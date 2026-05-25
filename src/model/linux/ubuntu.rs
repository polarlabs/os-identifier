const VENDOR: &str = "Canonical";
const PRODUCT: &str = "Ubuntu Linux";

#[derive(Debug)]
pub(crate) struct Ubuntu {
    vendor: String,
    product: String,
    release: Release,
    editions: Editions,
    service_channel: ServiceChannel,
}

impl Ubuntu {
    pub(crate) fn build(release: Release, service_channel: ServiceChannel) -> Ubuntu {
        Ubuntu {
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

    pub(crate) fn editions(mut self, editions: Editions) -> Ubuntu {
        self.editions = editions;
        self
    }

    pub(crate) fn is_enterprise(&self) -> bool {
        self.is_lts()
    }

    pub(crate) fn is_lts(&self) -> bool {
        match self.service_channel {
            ServiceChannel::Interim => false,
            ServiceChannel::LTS => true,
        }
    }

    pub(super) fn to_string(&self) -> Vec<String> {
        let out = if self.service_channel.is_default() {
            format!(
                "{} {}",
                self.product, self.release
            )
        } else {
            format!(
                "{} {} {}",
                self.product, self.release, self.service_channel
            )
        };

        vec![out]
    }
}

impl TryFrom<&str> for Ubuntu {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Ok(label) = crate::parser::endoflife::EndOfLifeLabel::try_from(value) {
            crate::parser::endoflife::linux::UbuntuParser::parse(&label)
        } else {
            let label = crate::parser::generic::GenericLabel::from(value);
            crate::parser::generic::linux::UbuntuParser::parse(&label)
        }
    }
}

#[derive(Debug)]
pub(crate) struct Release(String);

impl Release {
    fn major_is_even(&self) -> bool {
        let release: Vec<&str> = self.0.split('.').collect();
        let major = release[0];
        if major.parse::<i32>().unwrap() % 2 == 0 {
            true
        } else {
            false
        }
    }

    fn ends_with_04(&self) -> bool {
        self.0.ends_with(".04")
    }
}

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
        Editions(vec![
            Edition::Core,
            Edition::Desktop,
            Edition::Server,
        ])
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
    Interim,
    LTS,
}

impl ServiceChannel {
    fn is_default(&self) -> bool {
        match self {
            ServiceChannel::Interim => true,
            ServiceChannel::LTS => false,
        }
    }
}

impl From<&str> for ServiceChannel {
    fn from(value: &str) -> Self {
        match value {
            "lts" => ServiceChannel::LTS,
            _ => ServiceChannel::Interim,
        }
    }
}

impl From<&Release> for ServiceChannel {
    fn from(value: &Release) -> ServiceChannel {
        if value.major_is_even() && value.ends_with_04() {
            ServiceChannel::LTS
        } else {
            ServiceChannel::Interim
        }
    }
}

impl Default for ServiceChannel {
    fn default() -> Self {
        ServiceChannel::Interim
    }
}

impl std::fmt::Display for ServiceChannel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = match self {
            ServiceChannel::Interim => "",
            ServiceChannel::LTS => "LTS",
        };

        write!(f, "{}", out)
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_release_is_lts_true() {
        let release = Release("24.04".to_string());

        assert!(release.major_is_even());
        assert!(release.ends_with_04());

        let service_channel = ServiceChannel::from(&release);
        assert_eq!(service_channel, ServiceChannel::LTS);
    }

    #[test]
    fn test_release_is_lts_false_1() {
        let release = Release("22.10".to_string());

        assert!(release.major_is_even());
        assert!(!release.ends_with_04());

        let service_channel = ServiceChannel::from(&release);
        assert_eq!(service_channel, ServiceChannel::Interim);
    }

    #[test]
    fn test_release_is_lts_false_2() {
        let release = Release("25.04".to_string());

        assert!(!release.major_is_even());
        assert!(release.ends_with_04());

        let service_channel = ServiceChannel::from(&release);
        assert_eq!(service_channel, ServiceChannel::Interim);
    }

    #[test]
    fn test_from_string_1() {
        let label = Ubuntu::try_from("ubuntu-24.04").unwrap();

        assert_eq!(label.vendor, "Canonical".to_string());
        assert_eq!(label.product, "Ubuntu Linux".to_string());
        assert_eq!(label.release.to_string(), "24.04".to_string());

        assert_eq!(label.editions.len(), Editions::all().len());
        assert_eq!(label.service_channel, ServiceChannel::LTS);
    }

    fn test_from_string_2() {
        let label = Ubuntu::try_from("ubuntu-linux-24.04").unwrap();

        assert_eq!(label.vendor, "Canonical".to_string());
        assert_eq!(label.product, "Ubuntu Linux".to_string());
        assert_eq!(label.release.to_string(), "24.04".to_string());

        assert_eq!(label.editions.len(), Editions::all().len());
        assert_eq!(label.service_channel, ServiceChannel::LTS);
    }

    #[test]
    fn test_from_string_arbitrary1() {
        let label = Ubuntu::try_from("Ubuntu 24.04 LTS").unwrap();

        assert_eq!(label.vendor, "Canonical".to_string());
        assert_eq!(label.product, "Ubuntu Linux".to_string());
        assert_eq!(label.release.to_string(), "24.04".to_string());

        assert_eq!(label.editions.len(), Editions::all().len());
        assert_eq!(label.service_channel, ServiceChannel::LTS);
    }

    #[test]
    fn test_from_string_arbitrary2() {
        let label = Ubuntu::try_from("Ubuntu 24.04 LTS (Noble Numbat)").unwrap();

        assert_eq!(label.vendor, "Canonical".to_string());
        assert_eq!(label.product, "Ubuntu Linux".to_string());
        assert_eq!(label.release.to_string(), "24.04".to_string());

        assert_eq!(label.editions.len(), Editions::all().len());
        assert_eq!(label.service_channel, ServiceChannel::LTS);
    }

    #[test]
    fn test_from_string_arbitrary3() {
        let label = Ubuntu::try_from("Ubuntu 16.04.7 LTS").unwrap();

        assert_eq!(label.vendor, "Canonical".to_string());
        assert_eq!(label.product, "Ubuntu Linux".to_string());
        assert_eq!(label.release.to_string(), "16.04".to_string());

        assert_eq!(label.editions.len(), Editions::all().len());
        assert_eq!(label.service_channel, ServiceChannel::LTS);
    }

    #[test]
    fn test_from_string_arbitrary4() {
        let label = Ubuntu::try_from("Ubuntu precise (12.04 LTS)").unwrap();

        assert_eq!(label.vendor, "Canonical".to_string());
        assert_eq!(label.product, "Ubuntu Linux".to_string());
        assert_eq!(label.release.to_string(), "12.04".to_string());

        assert_eq!(label.editions.len(), Editions::all().len());
        assert_eq!(label.service_channel, ServiceChannel::LTS);
    }
}
