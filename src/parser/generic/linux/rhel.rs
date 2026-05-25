use crate::{model, util};
use crate::model::rhel::{Editions, Release, ServiceChannel};
use crate::parser::generic::GenericLabel;

const ERR_UNKNOWN_RELEASE: &str = "Not an RHEL release.";

pub(crate) struct RedHatEnterpriseLinuxParser();

impl RedHatEnterpriseLinuxParser {
    pub(crate) fn parse(label: &GenericLabel) -> Result<model::RedHatEnterpriseLinux, String> {
        if label.raw.contains("Red Hat") || label.raw.contains("RHEL") {
            let release = Release::try_from(label)?;
            let service_channel = ServiceChannel::from(&release);

            let rhel = model::RedHatEnterpriseLinux::build(release, service_channel).editions(Editions::all());

            Ok(rhel)
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
