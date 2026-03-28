use crate::model;
use crate::model::windows_11::{Edition, Editions, Release, ServiceChannel};
use super::super::EndOfLifeLabel;

pub(crate) struct Windows11Parser();

impl Windows11Parser {
    pub(crate) fn parse(label: &EndOfLifeLabel) -> Result<model::Windows11, String> {
        if label.len() < 4 {
            Err(String::from("This is not a Windows 11."))
        } else {
            if let Some(second) = label.get(1) {
                if second != "11" {
                    Err(String::from("Not Windows 11."))
                } else {
                    if label.len() == 4 {
                        let release = Release::from(label.get(2).unwrap());
                        let service_channel = ServiceChannel::GAC;

                        let windows = model::Windows11::build(release, service_channel);

                        match label.get(3) {
                            Some("e") => Ok(windows.editions(Editions::all_e())),
                            Some("iot") => Ok(windows.editions(Editions::all_iot())),
                            Some("w") => Ok(windows.editions(Editions::all_w())),
                            _ => Err(String::from("This is not a Windows 11.")),
                        }
                    } else if label.len() == 5 {
                        let release = Release::from(label.get(2).unwrap());
                        let service_channel = ServiceChannel::from(label.get(4).unwrap());

                        let windows = model::Windows11::build(release, service_channel);

                        Ok(windows.editions(Editions::from(label.get(3).unwrap())))
                    } else {
                        Err(String::from("This is not a Windows 11."))
                    }
                }
            } else {
                Err(String::from("This is not Windows 11."))
            }
        }
    }
}

impl From<&str> for Editions {
    fn from(value: &str) -> Self {
        let editions = match value {
            "e" => vec![
                Edition::Education,
                Edition::Enterprise,
                Edition::EnterpriseMultiSession,
            ],
            "iot" => vec![Edition::IoTEnterprise],
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
