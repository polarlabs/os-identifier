use crate::model;
use super::super::EndOfLifeLabel;

pub(crate) struct Windows10Parser();

impl Windows10Parser {
    pub(crate) fn parse(label: &EndOfLifeLabel) -> Result<model::Windows10, String> {
        if label.len() < 3 {
            Err(String::from("This is not a Windows 10."))
        } else {
            if let Some(second) = label.get(1) {
                if second != "10" {
                    Err(String::from("Not Windows 10."))
                } else {
                    if label.len() == 3 {
                        let release = model::windows_10::Release::from(label.get(2).unwrap());
                        let service_channel = model::windows_10::ServiceChannel::try_from(&release).unwrap_or_default();

                        let windows10 = model::Windows10::build(release, service_channel).editions(model::windows_10::Editions::all());
                        Ok(windows10)
                    } else if label.len() == 4 {
                        match label.get(3) {
                            Some("e") => {
                                let release = model::windows_10::Release::from(label.get(2).unwrap());
                                let service_channel = model::windows_10::ServiceChannel::GAC;

                                let windows10 = model::Windows10::build(release, service_channel).editions(model::windows_10::Editions::all_e());
                                Ok(windows10)
                            },
                            Some("iot") => {
                                let release = model::windows_10::Release::from(label.get(2).unwrap());
                                let service_channel = model::windows_10::ServiceChannel::GAC;

                                let windows10 = model::Windows10::build(release, service_channel)
                                    .iot_core("Windows 10 IoT Core");

                                Ok(windows10)
                            },
                            Some("w") => {
                                let release = model::windows_10::Release::from(label.get(2).unwrap());
                                let service_channel = model::windows_10::ServiceChannel::GAC;

                                let windows10 = model::Windows10::build(release, service_channel)
                                    .editions(model::windows_10::Editions::all_w());

                                Ok(windows10)
                            },
                            _ => Err(String::from("This is not a Windows 10.")),
                        }
                    } else if label.len() == 5 {
                        let editions = model::windows_10::Editions::from(label.get(3).unwrap());
                        let release = model::windows_10::Release::from(label.get(2).unwrap());
                        let service_channel = model::windows_10::ServiceChannel::from(label.get(4).unwrap());
                        let service_channel =
                            model::windows_10::ServiceChannel::try_from((&release, &service_channel)).unwrap_or_default();

                        let windows10 = model::Windows10::build(release, service_channel)
                            .editions(editions);

                        Ok(windows10)
                    } else {
                        Err(String::from("This is not a Windows 10."))
                    }
                }
            } else {
                Err(String::from("This is not Windows 10."))
            }
        }
    }
}
