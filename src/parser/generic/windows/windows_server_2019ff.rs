use crate::{model, util};
use crate::model::windows_server_2019ff::{Edition, Editions, Release, ServiceChannel};
use crate::parser::generic::GenericLabel;

const ERR_UNKNOWN_RELEASE: &str = "Not a Windows Server release.";
const ERR_UNKNOWN_EDITION: &str = "Not a Windows Server edition.";
const ERR_UNKNOWN_SERVICE_CHANNEL: &str = "Not a Windows Server service channel.";

include!(concat!(env!("OUT_DIR"), "/windows_server_2019ff_build_to_release_map.rs"));

pub(crate) struct WindowsServer2019ffParser();

impl WindowsServer2019ffParser {
    pub(crate) fn parse(label: &GenericLabel) -> Result<model::WindowsServer2019ff, String> {
        let version = Release::try_from(label)?;
        let edition = Edition::try_from(label)?;
        let service_channel = ServiceChannel::try_from(label).unwrap_or_default();

        let windows = model::WindowsServer2019ff::build(&version.to_string(), None, service_channel).editions(Editions(vec![edition]));

        Ok(windows)
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
                    |release| Ok(Release::from(release.as_str()))
                )
        } else {
            match util::identify_release(value, RELEASE_PATTERN) {
                Some(release) => Ok(Release::from(release.as_str())),
                None => Err(String::from(ERR_UNKNOWN_RELEASE)),
            }
        }
    }
}

impl<'a> TryFrom<&GenericLabel<'a>> for Edition {
    type Error = String;

    fn try_from(value: &GenericLabel<'a>) -> Result<Self, Self::Error> {
        let value = value.raw;

        if util::contains_any_word(value, &["Standard"]) {
            Ok(Edition::Standard)
        } else if util::contains_any_word(value, &["Datacenter"]) {
            Ok(Edition::Datacenter)
        } else {
            Err(String::from(ERR_UNKNOWN_EDITION))
        }
    }
}

impl<'a> TryFrom<&GenericLabel<'a>> for ServiceChannel {
    type Error = String;

    fn try_from(value: &GenericLabel<'a>) -> Result<Self, Self::Error> {
        let value = value.raw;

        if util::contains_any_word(value, &["LTSC"]) {
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
        let release = util::resolve_build_to_release("26100", BUILD_TO_RELEASE_MAP);

        assert_eq!(release, Ok(String::from("2025")));
    }
}
