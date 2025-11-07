use crate::model;
use crate::model::windows_11::{Edition, Editions, Release, ServiceChannel};
use crate::parser::generic::GenericLabel;

const ERR_UNKNOWN_RELEASE: &str = "Not a Windows 11 release.";
const ERR_UNKNOWN_EDITION: &str = "Not a Windows 11 edition.";
const ERR_UNKNOWN_SERVICE_CHANNEL: &str = "Not a Windows 11 service channel.";

pub(crate) struct Windows11Parser();

impl Windows11Parser {
    pub(crate) fn parse(label: &GenericLabel) -> Result<model::Windows11, String> {
        let edition = Edition::try_from(label)?;
        let release = Release::try_from(label)?;
        let service_channel = ServiceChannel::try_from(label)?;

        Ok(model::Windows11 {
            vendor: "Microsoft".to_string(),
            product: "Windows 11".to_string(),
            release,
            editions: Editions(vec![edition]),
            service_channel,
        })


    }
}

impl<'a> TryFrom<&GenericLabel<'a>> for Release {
    type Error = String;

    fn try_from(value: &GenericLabel<'a>) -> Result<Self, Self::Error> {
        let value = value.raw;

        // Look for a build number
        if let Some(build) = crate::util::find_number_with_digits(value, 5) {
            super::resolve_build_to_release(build.as_str())
                .map_or_else(
                    |_e| Err(String::from(ERR_UNKNOWN_RELEASE)),
                    |release| Ok(Release::from(release.as_str()))
                )
        } else {
            // todo: look for a release name
            Err(String::from(ERR_UNKNOWN_RELEASE))
        }
    }
}

impl<'a> TryFrom<&GenericLabel<'a>> for Edition {
    type Error = String;

    fn try_from(value: &GenericLabel<'a>) -> Result<Self, Self::Error> {
        let value = value.raw;

        if crate::util::contains_any_word(value, &["Education Edition", "Education"]) {
            Ok(Edition::Education)
        } else if crate::util::contains_any_word(value, &["Enterprise Edition", "Enterprise"]) {
            Ok(Edition::Enterprise)
        } else if crate::util::contains_any_word(value, &["Home Edition", "Home"]) {
            Ok(Edition::Home)
        } else if crate::util::contains_any_word(value, &["Professional Edition", "Professional", "Pro"]) {
            Ok(Edition::Pro)
        } else {
            Err(String::from(ERR_UNKNOWN_EDITION))
        }
    }
}

impl<'a> TryFrom<&GenericLabel<'a>> for ServiceChannel {
    type Error = String;

    fn try_from(value: &GenericLabel<'a>) -> Result<Self, Self::Error> {
        let value = value.raw;

        if crate::util::contains_any_word(value, &["General Availability", "GA"]) {
            Ok(ServiceChannel::GAC)
        } else if crate::util::contains_any_word(value, &["LTSC"]) {
            Ok(ServiceChannel::LTSC)
        } else {
            Err(String::from(ERR_UNKNOWN_SERVICE_CHANNEL))
        }
    }
}

#[cfg(test)]
mod tests {

}
