use crate::{model, util};
use crate::model::windows_10::{Edition, Editions, Release, ServiceChannel};
use crate::parser::generic::GenericLabel;

const ERR_UNKNOWN_RELEASE: &str = "Not a Windows 10 release.";
const ERR_UNKNOWN_EDITION: &str = "Not a Windows 10 edition.";
const ERR_UNKNOWN_SERVICE_CHANNEL: &str = "Not a Windows 10 service channel.";

include!(concat!(env!("OUT_DIR"), "/windows_10_build_to_release_map.rs"));

pub(crate) struct Windows10Parser();

impl Windows10Parser {
    pub(crate) fn parse(label: &GenericLabel) -> Result<model::Windows10, String> {
        let edition = Edition::try_from(label)?;
        let release = Release::try_from(label)?;
        let service_channel = ServiceChannel::try_from(label).unwrap_or_default();

        let windows10 = model::Windows10::build(release, service_channel).editions(Editions(vec![edition]));

        Ok(windows10)
    }
}

impl<'a> TryFrom<&GenericLabel<'a>> for Release {
    type Error = String;

    fn try_from(value: &GenericLabel<'a>) -> Result<Self, Self::Error> {
        let value = value.raw;

        // Look for a build number or identify the release
        if let Some(build) = util::find_number_with_digits(value, 5) {
            util::resolve_build_to_release(build.as_str(), BUILD_TO_RELEASE_MAP)
                .map_or_else(
                    |_e| Err(String::from(ERR_UNKNOWN_RELEASE)),
                    |release| Ok(model::windows_10::Release::from(release.as_str()))
                )
        } else {
            match util::identify_release(value, RELEASE_PATTERN) {
                None => Err(String::from(ERR_UNKNOWN_RELEASE)),
                Some(release) => Ok(model::windows_10::Release::from(release.as_str()))
            }
        }
    }
}

impl<'a> TryFrom<&GenericLabel<'a>> for Edition {
    type Error = String;

    fn try_from(value: &GenericLabel<'a>) -> Result<Self, Self::Error> {
        let value = value.raw;

        if util::contains_any_word(value, &["Education Edition", "Education"]) {
            Ok(Edition::Education)
        } else if util::contains_any_word(value, &["Enterprise Edition", "Enterprise"]) {
            Ok(Edition::Enterprise)
        } else if util::contains_any_word(value, &["Home Edition", "Home"]) {
            Ok(Edition::Home)
        } else if util::contains_any_word(value, &["Professional Edition", "Professional", "Pro"]) {
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

        if util::contains_any_word(value, &["General Availability", "GA"]) {
            Ok(ServiceChannel::GAC)
        } else if util::contains_any_word(value, &["LTSC"]) {
            Ok(ServiceChannel::LTSC)
        } else {
            Err(String::from(ERR_UNKNOWN_SERVICE_CHANNEL))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    #[test]
    fn test_build_to_release_1() {
        let release = util::resolve_build_to_release("19045", BUILD_TO_RELEASE_MAP);

        assert_eq!(release, Ok(String::from("22H2")));
    }
}
