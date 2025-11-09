pub(crate) mod endoflife;

pub(crate) mod generic;

mod windows;
pub use windows::Windows;

use crate::model;

const ERR_UNKNOWN_OS: &str = "Unknown operating system:";

pub struct OS(OperatingSystem);

enum OperatingSystem {
    Windows(model::Windows)
}

impl OS {
    pub fn parse(label: &str) -> Result<OS, String> {
        let os = OperatingSystem::try_from(label)?;

        Ok(OS(os))
    }
}

impl TryFrom<&str> for OperatingSystem {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Ok(windows) = model::Windows::try_from(value) {
            Ok(OperatingSystem::Windows(windows))
        } else {
            Err(format!("{} \"{}\"", ERR_UNKNOWN_OS, value))
        }
    }
}

impl std::fmt::Display for OS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.to_string())
    }
}

impl std::fmt::Display for OperatingSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OperatingSystem::Windows(os) => {
                // todo: remove debug :?
                write!(f, "{:?}", os.to_string())
            }
        }
    }
}
