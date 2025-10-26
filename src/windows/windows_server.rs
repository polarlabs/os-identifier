//
//
#[derive(Debug)]
pub(crate) struct WindowsServer {
    vendor: String,
    product: String,
    release: Release,
    editions: Editions,
    service_channel: ServiceChannel,
}

impl WindowsServer {
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

impl TryFrom<&str> for WindowsServer {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = value.split('-').collect();

        if let Some(first) = parts.get(0) {
            if *first != "11" {
                Err(String::from("Not Windows 11."))
            } else {
                // Ensure at least 2 parts are present
                if parts.len() < 3 {
                    Err(String::from("This is not a Windows 11."))
                } else if parts.len() == 3 {
                    match parts[2] {
                        "e" => Ok(WindowsServer {
                            vendor: "Microsoft".to_string(),
                            product: "Windows Server".to_string(),
                            release: Release::from(parts[1]),
                            editions: Editions::all_e(),
                            service_channel: ServiceChannel::GAC,
                        }),
                        "iot" => Ok(WindowsServer {
                            vendor: "Microsoft".to_string(),
                            product: "Windows 11".to_string(),
                            release: Release::from(parts[1]),
                            editions: Editions::all_iot(),
                            service_channel: ServiceChannel::GAC,
                        }),
                        "w" => Ok(WindowsServer {
                            vendor: "Microsoft".to_string(),
                            product: "Windows 11".to_string(),
                            release: Release::from(parts[1]),
                            editions: Editions::all_w(),
                            service_channel: ServiceChannel::GAC,
                        }),
                        _ => Err(String::from("This is not a Windows Server.")),
                    }
                } else if parts.len() == 4 {
                    let editions = Editions::from(parts[2]);
                    let release = Release::from(parts[1]);
                    let service_channel = ServiceChannel::from(parts[3]);

                    Ok(WindowsServer {
                        vendor: "Microsoft".to_string(),
                        product: "Windows Server".to_string(),
                        release,
                        editions,
                        service_channel,
                    })
                } else {
                    Err(String::from("This is not a Windows Server."))
                }
            }
        } else {
            Err(String::from("This is not Windows 11."))
        }
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
    AC,
    LTSC,
    SAC,
}

impl ServiceChannel {
    fn is_default(&self) -> bool {
        match self {
            ServiceChannel::AC => false,
            ServiceChannel::LTSC => true,
            ServiceChannel::SAC => false,
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
            ServiceChannel::AC => "AC",
            ServiceChannel::LTSC => "LTSC",
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
            _ => ServiceChannel::LTSC,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_string_none() {
        let label = WindowsServer::try_from("2025").unwrap();

        assert_eq!(label.vendor, "Microsoft".to_string());
        assert_eq!(label.product, "Windows Server".to_string());
        assert_eq!(label.release.to_string(), "2025".to_string());

        assert_eq!(label.service_channel, ServiceChannel::LTSC);
    }

    #[test]
    fn test_from_string_ac() {
        let label = WindowsServer::try_from("23h2-ac").unwrap();

        assert_eq!(label.vendor, "Microsoft".to_string());
        assert_eq!(label.product, "Windows 11".to_string());
        assert_eq!(label.release.to_string(), "24H2".to_string());

        assert_eq!(label.editions.len(), Editions::all_e().len());
        assert_eq!(label.service_channel, ServiceChannel::LTSC);
    }

    #[test]
    fn test_from_string_sac() {
        let label = WindowsServer::try_from("20h2-sac").unwrap();

        assert_eq!(label.vendor, "Microsoft".to_string());
        assert_eq!(label.product, "Windows 11".to_string());
        assert_eq!(label.release.to_string(), "24H2".to_string());

        assert_eq!(label.editions.len(), Editions::all_e().len());
        assert_eq!(label.service_channel, ServiceChannel::GAC);
    }

    #[test]
    fn test_from_string_r2() {
        let label = WindowsServer::try_from("2012-r2").unwrap();

        assert_eq!(label.vendor, "Microsoft".to_string());
        assert_eq!(label.product, "Windows 11".to_string());
        assert_eq!(label.release.to_string(), "24H2".to_string());

        assert_eq!(label.editions.len(), Editions::all_e().len());
        assert_eq!(label.service_channel, ServiceChannel::GAC);
    }

    #[test]
    fn test_from_string_r2_sp1() {
        let label = WindowsServer::try_from("2008-r2-sp1").unwrap();

        assert_eq!(label.vendor, "Microsoft".to_string());
        assert_eq!(label.product, "Windows 11".to_string());
        assert_eq!(label.release.to_string(), "24H2".to_string());

        assert_eq!(label.editions.len(), Editions::all_e().len());
        assert_eq!(label.service_channel, ServiceChannel::GAC);
    }

    #[test]
    fn test_from_string_sp2() {
        let label = WindowsServer::try_from("2008-sp2").unwrap();

        assert_eq!(label.vendor, "Microsoft".to_string());
        assert_eq!(label.product, "Windows 11".to_string());
        assert_eq!(label.release.to_string(), "24H2".to_string());

        assert_eq!(label.editions.len(), Editions::all_e().len());
        assert_eq!(label.service_channel, ServiceChannel::GAC);
    }
}
