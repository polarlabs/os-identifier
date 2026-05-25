use crate::{model, util};
use crate::model::ubuntu::{Editions, Release, ServiceChannel};
use crate::parser::generic::GenericLabel;

const ERR_UNKNOWN_RELEASE: &str = "Not an Ubuntu release.";

pub(crate) struct UbuntuParser();

impl UbuntuParser {
    pub(crate) fn parse(label: &GenericLabel) -> Result<model::Ubuntu, String> {
        if label.raw.contains("Ubuntu") {
            let release = Release::try_from(label)?;
            let service_channel = ServiceChannel::from(&release);

            let ubuntu = model::Ubuntu::build(release, service_channel).editions(Editions::all());

            Ok(ubuntu)
        } else {
            Err(String::from(ERR_UNKNOWN_RELEASE))
        }
    }
}

impl<'a> TryFrom<&GenericLabel<'a>> for Release {
    type Error = String;

    fn try_from(value: &GenericLabel<'a>) -> Result<Self, Self::Error> {
        let value = value.raw;

        match util::identify_release(value, r#"[0-9]+\.[0-9]+"#) {
            Some(release) => Ok(Release::from(release.as_str())),
            None => Err(String::from(ERR_UNKNOWN_RELEASE)),
        }
    }
}
