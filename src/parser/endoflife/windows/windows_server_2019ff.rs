use crate::{model, util};
use crate::model::windows_server_2019ff::{Editions, ServiceChannel};
use super::super::EndOfLifeLabel;

const ERR_UNKNOWN_RELEASE: &str = "Not a Windows Server release.";

include!(concat!(env!("OUT_DIR"), "/windows_server_2019ff_build_to_release_map.rs"));

pub(crate) struct WindowsServer2019ffParser();

impl WindowsServer2019ffParser {
    pub(crate) fn parse(label: &EndOfLifeLabel) -> Result<model::WindowsServer2019ff, String> {
        if label.len() < 3 {
            Err(String::from("This is not a Windows Server."))
        } else {
            if let Some(third) = label.get(2) {
                match util::identify_release(third, RELEASE_PATTERN) {
                    Some(version) => {
                        let windows = model::WindowsServer2019ff::build(&version, None, ServiceChannel::LTSC)
                            .editions(Editions::all());
                        Ok(windows)
                    },
                    None => Err(String::from(ERR_UNKNOWN_RELEASE)),
                }
            } else {
                Err(String::from(ERR_UNKNOWN_RELEASE))
            }
        }
    }
}
