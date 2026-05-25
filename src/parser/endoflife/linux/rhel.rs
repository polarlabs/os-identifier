use crate::{model, util};
use crate::model::rhel::{Editions, Release, ServiceChannel};
use super::super::EndOfLifeLabel;

pub(crate) struct RedHatEnterpriseLinuxParser();

impl RedHatEnterpriseLinuxParser {
    pub(crate) fn parse(label: &EndOfLifeLabel) -> Result<model::RedHatEnterpriseLinux, String> {
        if label.starts_with("rhel-") {
            if let Some(last) = label.last() {
                match (util::identify_release(last, r#"[0-9]+"#), util::identify_release(last, r#"[0-9]+\.[0-9]+"#)) {
                    // Major release only
                    (Some(major), None) => {
                        let release = Release::from(major.as_str());
                        let service_channel = ServiceChannel::from(&release);

                        let rhel = model::RedHatEnterpriseLinux::build(release, service_channel).editions(Editions::all());
                        Ok(rhel)
                    },
                    // Minor release provided, major release does not matter
                    (_, Some(minor)) => {
                        let release = Release::from(minor.as_str());
                        let service_channel = ServiceChannel::from(&release);

                        let rhel = model::RedHatEnterpriseLinux::build(release, service_channel).editions(Editions::all());
                        Ok(rhel)
                    },
                    // No valid release provided
                    (_, _) => {
                        Err(String::from("Not RHEL."))
                    }
                }
            } else {
                Err(String::from("This is not a RHEL."))
            }
        } else {
            Err(String::from("This is not a RHEL."))
        }
    }
}
