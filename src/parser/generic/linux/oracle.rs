use crate::{model, util};
use crate::model::oracle::{Editions, Release, ServiceChannel};
use crate::parser::generic::GenericLabel;

const ERR_UNKNOWN_RELEASE: &str = "Not an Oracle Linux release.";

pub(crate) struct OracleLinuxParser();

impl OracleLinuxParser {
    pub(crate) fn parse(label: &GenericLabel) -> Result<model::OracleLinux, String> {
        if label.raw.contains("Oracle") {
            let release = Release::try_from(label)?;
            let service_channel = ServiceChannel::from(&release);

            let linux = model::OracleLinux::build(release, service_channel).editions(Editions::all());

            Ok(linux)
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
