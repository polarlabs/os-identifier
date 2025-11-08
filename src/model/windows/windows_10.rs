//
// https://learn.microsoft.com/lifecycle/announcements/windows-10-1803-1809-end-of-servicing
// https://learn.microsoft.com/en-us/lifecycle/announcements/windows-10-1607-end-of-servicing
// https://learn.microsoft.com/windows/release-health/supported-versions-windows-client#enterprise-and-iot-enterprise-ltsbltsc-editions
// https://learn.microsoft.com/lifecycle/announcements/windows-10-1507-cb-cbb-end-of-servicing
// https://learn.microsoft.com/en-us/lifecycle/products/windows-10-iot-core
//

const VENDOR: &str = "Microsoft";
const PRODUCT: &str = "Windows 10";

#[derive(Debug)]
pub(crate) struct Windows10 {
    vendor: String,
    product: String,
    release: Release,
    editions: Option<Editions>,
    service_channel: ServiceChannel,
}

impl Windows10 {
    pub(crate) fn build(release: Release, service_channel: ServiceChannel) -> Windows10 {
        Windows10 {
            vendor: VENDOR.to_string(),
            product: PRODUCT.to_string(),
            release,
            editions: None,
            service_channel,
        }
    }

    pub(crate) fn product(mut self, product: &str) -> Windows10 {
        self.product = product.to_string();
        self
    }

    pub(crate) fn editions(mut self, editions: Editions) -> Windows10 {
        self.editions = Some(editions);
        self
    }

    pub(super) fn to_string(&self) -> Vec<String> {
        if let Some(editions) = &self.editions {
            let out = editions
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
        } else {
            let out = if self.service_channel.is_default() {
                format!(
                    "{} {} {}",
                    self.vendor, self.product, self.release
                )
            } else {
                format!(
                    "{} {} {} {}",
                    self.vendor, self.product, self.release, self.service_channel
                )
            };

            let out = vec![out];
            out
        }
    }
}

impl TryFrom<&str> for Windows10 {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Ok(label) = crate::parser::endoflife::EndOfLifeLabel::try_from(value) {
            crate::parser::endoflife::windows::Windows10Parser::parse(&label)
        } else {
            let label = crate::parser::generic::GenericLabel::from(value);
            crate::parser::generic::windows::Windows10Parser::parse(&label)
        }
    }
}

#[derive(Debug)]
pub(crate) struct Release(String);

impl Release {
    fn is_semi_annual(&self) -> bool {
        self.0.ends_with("H1")
    }

    fn up_to_1607(&self) -> bool {
        self.0 <= String::from("1607")
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
    pub(crate) fn all() -> Self {
        let mut editions = Editions::all_e();
        editions.0.append(&mut Editions::all_w().0);

        Editions(editions.0)
    }

    pub(crate) fn all_e() -> Self {
        Editions(vec![
            Edition::Education,
            Edition::Enterprise,
            Edition::EnterpriseIoT,
        ])
    }

    pub(crate) fn all_w() -> Self {
        Editions(vec![
            Edition::Home,
            Edition::Pro,
            Edition::ProEducation,
            Edition::ProForWorkstations,
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

impl From<&str> for Editions {
    fn from(value: &str) -> Self {
        let editions = match value {
            "e" => vec![
                Edition::Education,
                Edition::Enterprise,
                Edition::EnterpriseIoT,
            ],
            "w" => vec![
                Edition::Home,
                Edition::Pro,
                Edition::ProEducation,
                Edition::ProForWorkstations,
            ],
            _ => vec![],
        };

        Editions(editions)
    }
}

#[derive(PartialEq, Debug)]
pub(crate) enum Edition {
    Education,
    Enterprise,
    EnterpriseIoT,
    Home,
    Pro,
    ProEducation,
    ProForWorkstations,
}

impl std::fmt::Display for Edition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = match self {
            Edition::Education => "Education",
            Edition::Enterprise => "Enterprise",
            Edition::EnterpriseIoT => "Enterprise IoT",
            Edition::Home => "Home",
            Edition::Pro => "Pro",
            Edition::ProEducation => "Pro Education",
            Edition::ProForWorkstations => "Pro for Workstations",
        };

        write!(f, "{}", out.to_string())
    }
}

/*
The primary difference between Current Branch (CB) and Current Branch for Business (CBB) lies in the timing of feature update deployment. CB receives feature updates immediately upon release, making it ideal for pilot testing and early adoption, while CBB receives the same updates approximately four months later, allowing for additional testing and stability validation before broad deployment.
 This staging model ensures that CBB builds have undergone a full servicing window of cumulative updates and real-world testing, enhancing their readiness for enterprise-wide use.
 CBB is available only for Windows 10 Pro and Enterprise editions, whereas CB is the default for Windows 10 Home and optional for Pro and Enterprise.
 Microsoft has since rebranded CBB as the Semi-Annual Channel (SAC) to align with its Office 365 update terminology, but the underlying deployment strategy remains focused on delayed, tested rollouts for business environments.

 */
#[derive(PartialEq, Debug)]
pub(crate) enum ServiceChannel {
    // Current Branch
    // CB: only used by Windows 10 1507, 1511
    // Current Branch for Business
    // CBB: only used by Windows 10 1507, 1511
    GAC,
    LTSB,
    LTSC,
    SAC,
}

impl ServiceChannel {
    fn is_default(&self) -> bool {
        match self {
            ServiceChannel::GAC => true,
            ServiceChannel::LTSB => false,
            ServiceChannel::LTSC => false,
            ServiceChannel::SAC => true,
        }
    }

    fn is_lts(&self) -> bool {
        match self {
            ServiceChannel::GAC => false,
            ServiceChannel::LTSB => true,
            ServiceChannel::LTSC => true,
            ServiceChannel::SAC => false,
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
            ServiceChannel::LTSB => "LTSB",
            ServiceChannel::LTSC => "LTSC",
            ServiceChannel::SAC => "SAC",
        };

        write!(f, "{}", out)
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

impl TryFrom<&Release> for ServiceChannel {
    type Error = String;

    fn try_from(value: &Release) -> Result<Self, Self::Error> {
        if value.is_semi_annual() {
            Ok(ServiceChannel::SAC)
        } else {
            // todo: improve error handling
            Err("Unknown operating system".to_string())
        }
    }
}

impl TryFrom<(&Release, &ServiceChannel)> for ServiceChannel {
    type Error = String;

    fn try_from(
        (release, service_channel): (&Release, &ServiceChannel),
    ) -> Result<Self, Self::Error> {
        if release.is_semi_annual() {
            Ok(ServiceChannel::SAC)
        } else if release.up_to_1607() && service_channel.is_lts() {
            Ok(ServiceChannel::LTSB)
        } else {
            Ok(ServiceChannel::LTSC)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_string_h2() {
        let label = Windows10::try_from("10-22h2").unwrap();

        assert_eq!(label.vendor, "Microsoft".to_string());
        assert_eq!(label.product, "Windows 10".to_string());
        assert_eq!(label.release.to_string(), "22H2".to_string());

        assert_eq!(label.editions.unwrap().len(), Editions::all().len());
        assert_eq!(label.service_channel, ServiceChannel::GAC);
    }

    #[test]
    fn test_from_string_h1() {
        let label = Windows10::try_from("10-22h1").unwrap();

        assert_eq!(label.vendor, "Microsoft".to_string());
        assert_eq!(label.product, "Windows 10".to_string());
        assert_eq!(label.release.to_string(), "22H1".to_string());

        assert_eq!(label.editions.unwrap().len(), Editions::all().len());
        assert_eq!(label.service_channel, ServiceChannel::SAC);
    }

    #[test]
    fn test_from_string_e() {
        let label = Windows10::try_from("10-1809-e").unwrap();

        assert_eq!(label.vendor, "Microsoft".to_string());
        assert_eq!(label.product, "Windows 10".to_string());
        assert_eq!(label.release.to_string(), "1809".to_string());

        assert_eq!(label.editions.unwrap().len(), Editions::all_e().len());
        assert_eq!(label.service_channel, ServiceChannel::GAC);
    }

    #[test]
    fn test_from_string_w() {
        let label = Windows10::try_from("10-1809-w").unwrap();

        assert_eq!(label.vendor, "Microsoft".to_string());
        assert_eq!(label.product, "Windows 10".to_string());
        assert_eq!(label.release.to_string(), "1809".to_string());

        assert_eq!(label.editions.unwrap().len(), Editions::all_w().len());
        assert_eq!(label.service_channel, ServiceChannel::GAC);
    }

    #[test]
    fn test_from_string_enterpise_ltsc() {
        let label = Windows10::try_from("10-1809-e-lts").unwrap();

        assert_eq!(label.vendor, "Microsoft".to_string());
        assert_eq!(label.product, "Windows 10".to_string());
        assert_eq!(label.release.to_string(), "1809".to_string());

        assert_eq!(label.editions.unwrap().len(), Editions::all_e().len());
        assert_eq!(label.service_channel, ServiceChannel::LTSC);
    }

    #[test]
    fn test_from_string_enterpise_ltsb() {
        let label = Windows10::try_from("10-1607-e-lts").unwrap();

        assert_eq!(label.vendor, "Microsoft".to_string());
        assert_eq!(label.product, "Windows 10".to_string());
        assert_eq!(label.release.to_string(), "1607".to_string());

        assert_eq!(label.editions.unwrap().len(), Editions::all_e().len());
        assert_eq!(label.service_channel, ServiceChannel::LTSB);
    }

    #[test]
    fn test_from_string_all_editions() {
        let label = Windows10::try_from("10-1507").unwrap();

        assert_eq!(label.vendor, "Microsoft".to_string());
        assert_eq!(label.product, "Windows 10".to_string());
        assert_eq!(label.release.to_string(), "1507".to_string());

        assert_eq!(label.editions.unwrap().len(), Editions::all().len());
        assert_eq!(label.service_channel, ServiceChannel::GAC);
    }

    #[test]
    fn test_from_string_iot_core() {
        let label = Windows10::try_from("10-1507-iot").unwrap();

        assert_eq!(label.vendor, "Microsoft".to_string());
        assert_eq!(label.product, "Windows 10 IoT Core".to_string());
        assert_eq!(label.release.to_string(), "1507".to_string());

        assert!(label.editions.is_none());
        assert_eq!(label.service_channel, ServiceChannel::GAC);
    }

    #[test]
    fn test_from_string_arbitrary1() {
        let label = Windows10::try_from("Windows 10 Professional Edition (Build 19045) (64 Bit) GA (General Availability)").unwrap();

        assert_eq!(label.vendor, "Microsoft".to_string());
        assert_eq!(label.product, "Windows 10".to_string());
        assert_eq!(label.release.to_string(), "22H2".to_string());

        assert!(label.editions.unwrap().contains(Edition::Pro));
        assert_eq!(label.service_channel, ServiceChannel::GAC);
    }

    #[test]
    fn test_from_string_arbitrary2() {
        let label = Windows10::try_from("Microsoft Windows 10 Pro 17763").unwrap();

        assert_eq!(label.vendor, "Microsoft".to_string());
        assert_eq!(label.product, "Windows 10".to_string());
        assert_eq!(label.release.to_string(), "1809".to_string());

        assert!(label.editions.unwrap().contains(Edition::Pro));
        assert_eq!(label.service_channel, ServiceChannel::GAC);
    }

    #[test]
    fn test_from_string_arbitrary3() {
        let label = Windows10::try_from("Microsoft Windows 10 Enterprise 21H1").unwrap();

        assert_eq!(label.vendor, "Microsoft".to_string());
        assert_eq!(label.product, "Windows 10".to_string());
        assert_eq!(label.release.to_string(), "21H1".to_string());

        assert!(label.editions.unwrap().contains(Edition::Enterprise));
        assert_eq!(label.service_channel, ServiceChannel::GAC);
    }
}
