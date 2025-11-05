use crate::model;

use crate::parser::endoflife::EndOfLifeLabel;


pub(crate) struct Windows11Parser();

impl Windows11Parser {
    pub(crate) fn parse(label: &EndOfLifeLabel) -> Result<model::Windows11, String> {
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
                    let release = model::windows_11::Release::from(label.get(1).unwrap());
                    let service_channel = model::windows_11::ServiceChannel::GAC;

                    let windows = model::Windows11::build(vendor, product, release, service_channel);

                    match label.get(2) {
                        Some("e") => Ok(windows.editions(crate::model::windows_11::Editions::all_e())),
                        Some("iot") => Ok(windows.editions(crate::model::windows_11::Editions::all_iot())),
                        Some("w") => Ok(windows.editions(crate::model::windows_11::Editions::all_w())),
                        _ => Err(String::from("This is not a Windows 11.")),
                    }
                } else if label.len() == 4 {
                    let vendor = "Microsoft".to_string();
                    let product = "Windows 11".to_string();
                    let release = crate::model::windows_11::Release::from(label.get(1).unwrap());
                    let service_channel = crate::model::windows_11::ServiceChannel::from(label.get(3).unwrap());

                    let windows = model::Windows11::build(vendor, product, release, service_channel);

                    Ok(windows.editions(model::windows_11::Editions::from(label.get(2).unwrap())))
                } else {
                    Err(String::from("This is not a Windows 11."))
                }
            }
        } else {
            Err(String::from("This is not Windows 11."))
        }
    }
}

impl model::windows_11::Editions {
    #[allow(dead_code)]
    fn all() -> Self {
        let mut editions = model::windows_11::Editions::all_e();
        editions.0.append(&mut model::windows_11::Editions::all_iot().0);
        editions.0.append(&mut model::windows_11::Editions::all_w().0);

        model::windows_11::Editions(editions.0)
    }

    pub(crate) fn all_e() -> Self {
        model::windows_11::Editions(vec![
            model::windows_11::Edition::Education,
            model::windows_11::Edition::Enterprise,
            model::windows_11::Edition::EnterpriseMultiSession,
        ])
    }

    pub(crate) fn all_iot() -> Self {
        model::windows_11::Editions(vec![model::windows_11::Edition::IoTEnterprise])
    }

    pub(crate) fn all_w() -> Self {
        model::windows_11::Editions(vec![
            model::windows_11::Edition::Home,
            model::windows_11::Edition::Pro,
            model::windows_11::Edition::ProEducation,
            model::windows_11::Edition::ProForWorkstations,
        ])
    }
}

impl From<&str> for model::windows_11::Editions {
    fn from(value: &str) -> Self {
        let editions = match value {
            "e" => vec![
                model::windows_11::Edition::Education,
                model::windows_11::Edition::Enterprise,
                model::windows_11::Edition::EnterpriseMultiSession,
            ],
            "iot" => vec![model::windows_11::Edition::IoTEnterprise],
            "w" => vec![
                model::windows_11::Edition::Home,
                model::windows_11::Edition::Pro,
                model::windows_11::Edition::ProEducation,
                model::windows_11::Edition::ProForWorkstations,
            ],
            _ => vec![],
        };

        model::windows_11::Editions(editions)
    }
}
