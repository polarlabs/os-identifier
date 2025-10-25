use crate::windows::windows_10::Windows10;

//
//
#[derive(Debug)]
pub(crate) struct Windows11 {
    vendor: String,
    product: String,
    release: Release,
    editions: Editions,
    service_channel: ServiceChannel,
}

impl Windows11 {
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

impl TryFrom<&str> for Windows11 {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = value.split('-').collect();

        // Ensure at least 2 parts are present
        if parts.len() < 3 {
            Err(String::from("This is not a Windows 11."))
        } else if parts.len() == 3 {
            match parts[2] {
                "e" => {
                    Ok(
                        Windows11 {
                            vendor: "Microsoft".to_string(),
                            product: "Windows 11".to_string(),
                            release: Release::from(parts[1]),
                            editions: Editions::all_e(),
                            service_channel: ServiceChannel::GAC,
                        }
                    )
                },
                "iot" => {
                    Ok(
                        Windows11 {
                            vendor: "Microsoft".to_string(),
                            product: "Windows 11".to_string(),
                            release: Release::from(parts[1]),
                            editions: Editions::all_iot(),
                            service_channel: ServiceChannel::GAC,
                        }
                    )
                },
                "w" => {
                    Ok(
                        Windows11 {
                            vendor: "Microsoft".to_string(),
                            product: "Windows 11".to_string(),
                            release: Release::from(parts[1]),
                            editions: Editions::all_w(),
                            service_channel: ServiceChannel::GAC,
                        }
                    )
                },
                _ => Err(String::from("This is not a Windows 11."))
            }
        } else if parts.len() == 4 {
            let editions = Editions::from(parts[2]);
            let release = Release::from(parts[1]);
            let service_channel = ServiceChannel::from(parts[3]);

            Ok(
                Windows11 {
                    vendor: "Microsoft".to_string(),
                    product: "Windows 11".to_string(),
                    release,
                    editions,
                    service_channel,
                }
            )
        } else {
            Err(String::from("This is not a Windows 11."))
        }
    }
}

#[derive(Debug)]
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
        let mut editions = Editions::all_e();
        editions.0.append(&mut Editions::all_iot().0);
        editions.0.append(&mut Editions::all_w().0);

        Editions(editions.0)
    }

    fn all_e() -> Self {
        Editions(vec![
            Edition::Education,
            Edition::Enterprise,
        ])
    }

    fn all_iot() -> Self {
        Editions(vec![
            Edition::IoTEnterprise,
        ])
    }

    fn all_w() -> Self {
        Editions(vec![
            Edition::Home,
            Edition::Pro,
            Edition::ProEducation,
            Edition::ProForWorkstations,
        ])
    }

    fn none() -> Self {
        Editions(vec![])
    }

    fn contains(&self, edition: Edition) -> bool {
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
                Edition::Education,
                Edition::Enterprise,
            ],
            "iot" => vec![
                Edition::IoTEnterprise,
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
enum Edition {
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

impl From<&str> for ServiceChannel {
    fn from(value: &str) -> Self {
        match value {
            "lts" => ServiceChannel::LTSC,
            _ => ServiceChannel::GAC,
        }
    }
}

#[cfg(test)]
mod tests {
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
}
