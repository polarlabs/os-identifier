use crate::{model, util};
use crate::model::ubuntu::{Editions, Release, ServiceChannel};
use super::super::EndOfLifeLabel;

pub(crate) struct UbuntuParser();

impl UbuntuParser {
    pub(crate) fn parse(label: &EndOfLifeLabel) -> Result<model::Ubuntu, String> {
        if label.starts_with("ubuntu-linux-") || label.starts_with("ubuntu-") {
            if let Some(last) = label.last() {
                if let Some(release) = util::identify_release(last, r#"[0-9]+\.[0-9]+"#) {
                    let release = Release::from(release.as_str());
                    let service_channel = ServiceChannel::from(&release);

                    let ubuntu = model::Ubuntu::build(release, service_channel).editions(Editions::all());
                    Ok(ubuntu)
                } else {
                    Err(String::from("Not Ubuntu."))
                }
            } else {
                Err(String::from("This is not a Ubuntu Linux."))
            }
        } else {
            Err(String::from("This is not a Ubuntu Linux."))
        }
    }
}
