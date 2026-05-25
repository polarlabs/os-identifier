const VENDOR: &str = "Debian";
const PRODUCT: &str = "Debian Linux";

#[derive(Debug)]
pub(crate) struct Debian {
    vendor: String,
    product: String,
    release: Release,
    editions: Editions,
    service_channel: ServiceChannel,
}

impl Debian {
    pub(crate) fn build(release: Release, service_channel: ServiceChannel) -> Debian {
        Debian {
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
    
    pub(crate) fn editions(mut self, editions: Editions) -> Debian {
        self.editions = editions;
        self
    }
    
    pub(crate) fn is_enterprise(&self) -> bool {
        false
    }

    pub(crate) fn is_lts(&self) -> bool {
        false
    }
    
    pub(super) fn to_string(&self) -> Vec<String> {
        vec![format!(
            "{} {}",
            self.product, self.release
        )]
    }
}

impl TryFrom<&str> for Debian {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Ok(label) = crate::parser::endoflife::EndOfLifeLabel::try_from(value) {
            crate::parser::endoflife::linux::DebianParser::parse(&label)
        } else {
            let label = crate::parser::generic::GenericLabel::from(value);
            crate::parser::generic::linux::DebianParser::parse(&label)
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

impl ServiceChannel {
    fn is_default(&self) -> bool {
        match self {
            ServiceChannel::LTS => true,
        }
    }
}

impl From<&Release> for ServiceChannel {
    fn from(_: &Release) -> ServiceChannel {
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
        let label = Debian::try_from("debian-13").unwrap();

        assert_eq!(label.vendor, "Debian".to_string());
        assert_eq!(label.product, "Debian Linux".to_string());
        assert_eq!(label.release.to_string(), "13".to_string());

        assert_eq!(label.editions.len(), Editions::all().len());
        assert_eq!(label.service_channel, ServiceChannel::LTS);
    }

    fn test_from_string_2() {
        let label = Debian::try_from("debian-6").unwrap();

        assert_eq!(label.vendor, "Debian".to_string());
        assert_eq!(label.product, "Debian Linux".to_string());
        assert_eq!(label.release.to_string(), "6".to_string());

        assert_eq!(label.editions.len(), Editions::all().len());
        assert_eq!(label.service_channel, ServiceChannel::LTS);
    }

    fn test_from_string_3() {
        let label = Debian::try_from("debian-13.5").unwrap();

        assert_eq!(label.vendor, "Debian".to_string());
        assert_eq!(label.product, "Debian Linux".to_string());
        assert_eq!(label.release.to_string(), "13.5".to_string());

        assert_eq!(label.editions.len(), Editions::all().len());
        assert_eq!(label.service_channel, ServiceChannel::LTS);
    }

    fn test_from_string_4() {
        let label = Debian::try_from("debian-6.0.10").unwrap();

        assert_eq!(label.vendor, "Debian".to_string());
        assert_eq!(label.product, "Debian Linux".to_string());
        assert_eq!(label.release.to_string(), "6.0.10".to_string());

        assert_eq!(label.editions.len(), Editions::all().len());
        assert_eq!(label.service_channel, ServiceChannel::LTS);
    }

    #[test]
    fn test_from_string_arbitrary1() {
        let label = Debian::try_from("Debian GNU/Linux 13 (trixie)").unwrap();

        assert_eq!(label.vendor, "Debian".to_string());
        assert_eq!(label.product, "Debian Linux".to_string());
        assert_eq!(label.release.to_string(), "13".to_string());

        assert_eq!(label.editions.len(), 0);
        assert_eq!(label.service_channel, ServiceChannel::LTS);
    }

    #[test]
    fn test_from_string_arbitrary2() {
        let label = Debian::try_from("Debian GNU/Linux 6.0").unwrap();

        assert_eq!(label.vendor, "Debian".to_string());
        assert_eq!(label.product, "Debian Linux".to_string());
        assert_eq!(label.release.to_string(), "6.0".to_string());

        assert_eq!(label.editions.len(), 0);
        assert_eq!(label.service_channel, ServiceChannel::LTS);
    }
}
