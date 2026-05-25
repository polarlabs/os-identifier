use crate::{model, util};
use crate::model::debian::{Editions, Release, ServiceChannel};
use super::super::EndOfLifeLabel;

pub(crate) struct DebianParser();

impl DebianParser {
    pub(crate) fn parse(label: &EndOfLifeLabel) -> Result<model::Debian, String> {
        if label.starts_with("debian-") {
            if let Some(last) = label.last() {
                match (util::identify_release(last, r#"[0-9]+"#), util::identify_release(last, r#"[0-9]+\.[0-9\.]+"#)) {
                    // Major release only
                    (Some(major), None) => {
                        let release = Release::from(major.as_str());
                        let service_channel = ServiceChannel::from(&release);

                        let debian = model::Debian::build(release, service_channel).editions(Editions::all());
                        Ok(debian)
                    },
                    // Minor release provided, major release does not matter
                    (_, Some(minor)) => {
                        let release = Release::from(minor.as_str());
                        let service_channel = ServiceChannel::from(&release);

                        let debian = model::Debian::build(release, service_channel).editions(Editions::all());
                        Ok(debian)
                    },
                    // No valid release provided
                    (_, _) => {
                        Err(String::from("Not Debian."))
                    }
                }
            } else {
                Err(String::from("This is not a Debian."))
            }
        } else {
            Err(String::from("This is not a Debian."))
        }
    }
}
