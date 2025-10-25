use crate::windows::windows_7::Windows7;

//
// https://learn.microsoft.com/lifecycle/announcements/windows-10-1803-1809-end-of-servicing
// https://learn.microsoft.com/en-us/lifecycle/announcements/windows-10-1607-end-of-servicing
// https://learn.microsoft.com/windows/release-health/supported-versions-windows-client#enterprise-and-iot-enterprise-ltsbltsc-editions
// https://learn.microsoft.com/lifecycle/announcements/windows-10-1507-cb-cbb-end-of-servicing
// https://learn.microsoft.com/en-us/lifecycle/products/windows-10-iot-core
//
#[derive(Debug)]
pub(crate) struct Windows10 {
    vendor: String,
    product: String,
    release: Release,
    editions: Editions,
    service_channel: ServiceChannel,
}

impl Windows10 {
    pub(super) fn to_string(&self) -> Vec<String> {
        let out = self.editions.0
            .iter()
            .map(|edition| {
                if self.service_channel.is_default() {
                    format!("{} {} {edition} {}", self.vendor, self.product, self.release)
                } else {
                    format!("{} {} {edition} {} {}", self.vendor, self.product, self.release, self.service_channel)
                }

            })
            .collect();

        out
    }
}

impl TryFrom<&str> for Windows10 {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = value.split('-').collect();

        if let Some(first) = parts.get(0) {
            if *first != "10" {
                Err(String::from("Not Windows 10."))
            } else {

                // Ensure at least 2 parts are present
                if parts.len() < 2 {
                    Err(String::from("This is not a Windows 10."))
                } else if parts.len() == 2 {
                    let release = Release::from(parts[1]);
                    let service_channel = ServiceChannel::try_from(&release).unwrap_or_else(
                        |_| parts
                            .get(3)
                            .map(|&s| ServiceChannel::from(s))
                            .unwrap_or_default()
                    );

                    Ok(
                        Windows10 {
                            vendor: "Microsoft".to_string(),
                            product: "Windows 10".to_string(),
                            release,
                            editions: Editions::all(),
                            service_channel,
                        }
                    )
                } else if parts.len() == 3 {
                    match parts[2] {
                        "e" => {
                            Ok(
                                Windows10 {
                                    vendor: "Microsoft".to_string(),
                                    product: "Windows 10".to_string(),
                                    release: Release::from(parts[1]),
                                    editions: Editions::all_e(),
                                    service_channel: ServiceChannel::GAC,
                                }
                            )
                        },
                        "iot" => {
                            Ok(
                                Windows10 {
                                    vendor: "Microsoft".to_string(),
                                    product: "Windows 10 IoT Core".to_string(),
                                    release: Release::from(parts[1]),
                                    editions: Editions::none(),
                                    service_channel: ServiceChannel::GAC,
                                }
                            )
                        },
                        "w" => {
                            Ok(
                                Windows10 {
                                    vendor: "Microsoft".to_string(),
                                    product: "Windows 10".to_string(),
                                    release: Release::from(parts[1]),
                                    editions: Editions::all_w(),
                                    service_channel: ServiceChannel::GAC,
                                }
                            )
                        },
                        _ => Err(String::from("This is not a Windows 10."))
                    }
                } else if parts.len() == 4 {
                    let editions = Editions::from(parts[2]);
                    let release = Release::from(parts[1]);
                    let service_channel = ServiceChannel::from(parts[3]);
                    let service_channel = ServiceChannel::try_from((&release, &service_channel)).unwrap_or_default();

                    Ok(
                        Windows10 {
                            vendor: "Microsoft".to_string(),
                            product: "Windows 10".to_string(),
                            release,
                            editions,
                            service_channel,
                        }
                    )
                } else {
                    Err(String::from("This is not a Windows 10."))
                }
            }
        } else {
            Err(String::from("This is not Windows 10."))
        }
    }
}

#[derive(Debug)]
struct Release(String);

impl Release {
    fn is_semi_annual(&self) -> bool{
        self.0.ends_with("H1")
    }

    fn up_to_1607(&self) -> bool{
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
struct Editions(Vec<EditionE>);

impl Editions {
    fn all() -> Self {
        let mut editions = Editions::all_e();
        editions.0.append(&mut Editions::all_w().0);

        Editions(editions.0)
    }

    fn all_e() -> Self {
        Editions(vec![
            EditionE::Education,
            EditionE::Enterprise,
            EditionE::EnterpriseIoT,
        ])
    }

    fn all_w() -> Self {
        Editions(vec![
            EditionE::Home,
            EditionE::Pro,
            EditionE::ProEducation,
            EditionE::ProForWorkstations,
        ])
    }

    fn none() -> Self {
        Editions(vec![])
    }

    fn contains(&self, edition: EditionE) -> bool {
        self.0.contains(&edition)
    }

    fn len(&self) -> usize {
        self.0.len()
    }
}

impl From<&str> for Editions {
    fn from(value: &str) -> Self {
        let editions = match value {
            "e" => vec![
                EditionE::Education,
                EditionE::Enterprise,
                EditionE::EnterpriseIoT,
            ],
            "w" => vec![
                EditionE::Home,
                EditionE::Pro,
                EditionE::ProEducation,
                EditionE::ProForWorkstations,
            ],
            _ => vec![],
        };

        Editions(editions)
    }
}

#[derive(PartialEq, Debug)]
enum EditionE {
    Education,
    Enterprise,
    EnterpriseIoT,
    Home,
    Pro,
    ProEducation,
    ProForWorkstations,
}

impl std::fmt::Display for EditionE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = match self {
            EditionE::Education => "Education",
            EditionE::Enterprise => "Enterprise",
            EditionE::EnterpriseIoT => "Enterprise IoT",
            EditionE::Home => "Home",
            EditionE::Pro => "Pro",
            EditionE::ProEducation => "Pro Education",
            EditionE::ProForWorkstations => "Pro for Workstations",
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

    fn try_from((release, service_channel): (&Release, &ServiceChannel)) -> Result<Self, Self::Error> {
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

        assert_eq!(label.editions.len(), Editions::all().len());
        assert_eq!(label.service_channel, ServiceChannel::GAC);
    }

    #[test]
    fn test_from_string_h1() {
        let label = Windows10::try_from("10-22h1").unwrap();

        assert_eq!(label.vendor, "Microsoft".to_string());
        assert_eq!(label.product, "Windows 10".to_string());
        assert_eq!(label.release.to_string(), "22H1".to_string());

        assert_eq!(label.editions.len(), Editions::all().len());
        assert_eq!(label.service_channel, ServiceChannel::SAC);
    }

    #[test]
    fn test_from_string_e() {
        let label = Windows10::try_from("10-1809-e").unwrap();

        assert_eq!(label.vendor, "Microsoft".to_string());
        assert_eq!(label.product, "Windows 10".to_string());
        assert_eq!(label.release.to_string(), "1809".to_string());

        assert_eq!(label.editions.len(), Editions::all_e().len());
        assert_eq!(label.service_channel, ServiceChannel::GAC);
    }

    #[test]
    fn test_from_string_w() {
        let label = Windows10::try_from("10-1809-w").unwrap();

        assert_eq!(label.vendor, "Microsoft".to_string());
        assert_eq!(label.product, "Windows 10".to_string());
        assert_eq!(label.release.to_string(), "1809".to_string());

        assert_eq!(label.editions.len(), Editions::all_w().len());
        assert_eq!(label.service_channel, ServiceChannel::GAC);
    }

    #[test]
    fn test_from_string_enterpise_ltsc() {
        let label = Windows10::try_from("10-1809-e-lts").unwrap();

        assert_eq!(label.vendor, "Microsoft".to_string());
        assert_eq!(label.product, "Windows 10".to_string());
        assert_eq!(label.release.to_string(), "1809".to_string());

        assert_eq!(label.editions.len(), Editions::all_e().len());
        assert_eq!(label.service_channel, ServiceChannel::LTSC);
    }

    #[test]
    fn test_from_string_enterpise_ltsb() {
        let label = Windows10::try_from("10-1607-e-lts").unwrap();

        assert_eq!(label.vendor, "Microsoft".to_string());
        assert_eq!(label.product, "Windows 10".to_string());
        assert_eq!(label.release.to_string(), "1607".to_string());

        assert_eq!(label.editions.len(), Editions::all_e().len());
        assert_eq!(label.service_channel, ServiceChannel::LTSB);
    }

    #[test]
    fn test_from_string_all_editions() {
        let label = Windows10::try_from("10-1507").unwrap();

        assert_eq!(label.vendor, "Microsoft".to_string());
        assert_eq!(label.product, "Windows 10".to_string());
        assert_eq!(label.release.to_string(), "1507".to_string());

        assert_eq!(label.editions.len(), Editions::all().len());
        assert_eq!(label.service_channel, ServiceChannel::GAC);
    }

    #[test]
    fn test_from_string_iot_core() {
        let label = Windows10::try_from("10-1507-iot").unwrap();

        assert_eq!(label.vendor, "Microsoft".to_string());
        assert_eq!(label.product, "Windows 10 IoT Core".to_string());
        assert_eq!(label.release.to_string(), "1507".to_string());

        assert_eq!(label.editions.len(), 0);
        assert_eq!(label.service_channel, ServiceChannel::GAC);
    }
}
