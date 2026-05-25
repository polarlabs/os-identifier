use crate::{model, util};
use crate::model::oracle::{Editions, Release, ServiceChannel};
use super::super::EndOfLifeLabel;

pub(crate) struct OracleLinuxParser();

impl OracleLinuxParser {
    pub(crate) fn parse(label: &EndOfLifeLabel) -> Result<model::OracleLinux, String> {
        if label.starts_with("oracle-linux-") {
            if let Some(last) = label.last() {
                match (util::identify_release(last, r#"[0-9]+"#), util::identify_release(last, r#"[0-9]+\.[0-9]+"#)) {
                    // Major release only
                    (Some(major), None) => {
                        let release = Release::from(major.as_str());
                        let service_channel = ServiceChannel::from(&release);

                        let rhel = model::OracleLinux::build(release, service_channel).editions(Editions::all());
                        Ok(rhel)
                    },
                    // Minor release provided, major release does not matter
                    (_, Some(minor)) => {
                        let release = Release::from(minor.as_str());
                        let service_channel = ServiceChannel::from(&release);

                        let rhel = model::OracleLinux::build(release, service_channel).editions(Editions::all());
                        Ok(rhel)
                    },
                    // No valid release provided
                    (_, _) => {
                        Err(String::from("Not Oracle Linux."))
                    }
                }
            } else {
                Err(String::from("This is not an Oracle Linux."))
            }
        } else {
            Err(String::from("This is not an Oracle Linux."))
        }
    }
}
