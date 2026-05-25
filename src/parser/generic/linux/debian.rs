use crate::{model, util};
use crate::model::debian::{Editions, Release, ServiceChannel};
use crate::parser::generic::GenericLabel;

const ERR_UNKNOWN_RELEASE: &str = "Not a Debian Linux release.";

pub(crate) struct DebianParser();

impl DebianParser {
    pub(crate) fn parse(label: &GenericLabel) -> Result<model::Debian, String> {
        if label.raw.contains("Debian") {
            let release = Release::try_from(label)?;
            let service_channel = ServiceChannel::from(&release);

            let debian = model::Debian::build(release, service_channel).editions(Editions::all());

            Ok(debian)
        } else {
            Err(String::from(ERR_UNKNOWN_RELEASE))
        }
    }
}

impl<'a> TryFrom<&GenericLabel<'a>> for Release {
    type Error = String;

    fn try_from(value: &GenericLabel<'a>) -> Result<Self, Self::Error> {
        let value = value.raw;

        match (util::identify_release(value, r#"[0-9]+"#), util::identify_release(value, r#"[0-9]+\.[0-9\.]+"#)) {
            (Some(major), None) => Ok(Release::from(major.as_str())),
            (_, Some(minor)) => Ok(Release::from(minor.as_str())),
            (_, _) => Err(format!("{} ({})", String::from(ERR_UNKNOWN_RELEASE), value)),
        }
    }
}
