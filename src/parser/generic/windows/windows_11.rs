use crate::{model, util};
use crate::model::windows_11::{Edition, Editions, Release, ServiceChannel};
use crate::parser::generic::GenericLabel;

const ERR_UNKNOWN_RELEASE: &str = "Not a Windows 11 release.";
const ERR_UNKNOWN_EDITION: &str = "Not a Windows 11 edition.";
const ERR_UNKNOWN_SERVICE_CHANNEL: &str = "Not a Windows 11 service channel.";

include!(concat!(env!("OUT_DIR"), "/windows_11_build_to_release_map.rs"));

fn resolve_build_to_release(build: &str) -> Result<String, String> {
    if let Some(release) = BUILD_TO_RELEASE_MAP.get(build) {
        Ok(release.get(0).unwrap().to_string())
    } else {
        Err(format!("Build '{}' does not exist.", build))
    }
}

pub(crate) struct Windows11Parser();

impl Windows11Parser {
    pub(crate) fn parse(label: &GenericLabel) -> Result<model::Windows11, String> {
        let edition = Edition::try_from(label)?;
        let release = Release::try_from(label)?;
        let service_channel = ServiceChannel::try_from(label).unwrap_or_default();

        let windows11 = model::Windows11::build(release, service_channel).editions(Editions(vec![edition]));

        Ok(windows11)
    }
}

impl<'a> TryFrom<&GenericLabel<'a>> for Release {
    type Error = String;

    fn try_from(value: &GenericLabel<'a>) -> Result<Self, Self::Error> {
        let value = value.raw;

        // Look for a build number or identify the release
        if let Some(build) = crate::util::find_number_with_digits(value, 5) {
            resolve_build_to_release(build.as_str())
                .map_or_else(
                    |_e| Err(String::from(ERR_UNKNOWN_RELEASE)),
                    |release| Ok(Release::from(release.as_str()))
                )
        } else {
            match util::identify_release(value, RELEASE_PATTERN) {
                None => Err(String::from(ERR_UNKNOWN_RELEASE)),
                Some(release) => Ok(model::windows_11::Release::from(release.as_str()))
            }
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
    use super::*;

    #[test]
    fn test_build_to_release_1() {
        let release = resolve_build_to_release("26100");

        assert_eq!(release, Ok(String::from("24H2")));
    }
}
