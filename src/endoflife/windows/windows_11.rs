use crate::endoflife::EndOfLifeLabel;
use crate::windows::Windows11;

pub(crate) struct Windows11Parser();

impl Windows11Parser {
    pub(crate) fn parse(label: &EndOfLifeLabel) -> Result<Windows11, String> {
        if let Some(first) = label.get(0) {
            if first != "11" {
                Err(String::from("Not Windows 11."))
            } else {
                // Ensure at least 2 parts are present
                if label.len() < 3 {
                    Err(String::from("This is not a Windows 11."))
                } else if label.len() == 3 {
                    let vendor = "Microsoft".to_string();
                    let product = "Windows 11".to_string();
                    let release = crate::windows::windows_11::Release::from(label.get(1).unwrap());
                    let service_channel = crate::windows::windows_11::ServiceChannel::GAC;

                    let windows = Windows11::build(vendor, product, release, service_channel);

                    match label.get(2) {
                        Some("e") => Ok(windows.editions(crate::windows::windows_11::Editions::all_e())),
                        Some("iot") => Ok(windows.editions(crate::windows::windows_11::Editions::all_iot())),
                        Some("w") => Ok(windows.editions(crate::windows::windows_11::Editions::all_w())),
                        _ => Err(String::from("This is not a Windows 11.")),
                    }
                } else if label.len() == 4 {
                    let vendor = "Microsoft".to_string();
                    let product = "Windows 11".to_string();
                    let release = crate::windows::windows_11::Release::from(label.get(1).unwrap());
                    let service_channel = crate::windows::windows_11::ServiceChannel::from(label.get(3).unwrap());

                    let windows = Windows11::build(vendor, product, release, service_channel);

                    Ok(windows.editions(crate::windows::windows_11::Editions::from(label.get(2).unwrap())))
                } else {
                    Err(String::from("This is not a Windows 11."))
                }
            }
        } else {
            Err(String::from("This is not Windows 11."))
        }
    }
}

impl crate::windows::windows_11::Editions {
    #[allow(dead_code)]
    fn all() -> Self {
        let mut editions = crate::windows::windows_11::Editions::all_e();
        editions.0.append(&mut crate::windows::windows_11::Editions::all_iot().0);
        editions.0.append(&mut crate::windows::windows_11::Editions::all_w().0);

        crate::windows::windows_11::Editions(editions.0)
    }

    pub(crate) fn all_e() -> Self {
        crate::windows::windows_11::Editions(vec![
            crate::windows::windows_11::Edition::Education,
            crate::windows::windows_11::Edition::Enterprise,
            crate::windows::windows_11::Edition::EnterpriseMultiSession,
        ])
    }

    pub(crate) fn all_iot() -> Self {
        crate::windows::windows_11::Editions(vec![crate::windows::windows_11::Edition::IoTEnterprise])
    }

    pub(crate) fn all_w() -> Self {
        crate::windows::windows_11::Editions(vec![
            crate::windows::windows_11::Edition::Home,
            crate::windows::windows_11::Edition::Pro,
            crate::windows::windows_11::Edition::ProEducation,
            crate::windows::windows_11::Edition::ProForWorkstations,
        ])
    }
}

impl From<&str> for crate::windows::windows_11::Editions {
    fn from(value: &str) -> Self {
        let editions = match value {
            "e" => vec![
                crate::windows::windows_11::Edition::Education,
                crate::windows::windows_11::Edition::Enterprise,
                crate::windows::windows_11::Edition::EnterpriseMultiSession,
            ],
            "iot" => vec![crate::windows::windows_11::Edition::IoTEnterprise],
            "w" => vec![
                crate::windows::windows_11::Edition::Home,
                crate::windows::windows_11::Edition::Pro,
                crate::windows::windows_11::Edition::ProEducation,
                crate::windows::windows_11::Edition::ProForWorkstations,
            ],
            _ => vec![],
        };

        crate::windows::windows_11::Editions(editions)
    }
}
