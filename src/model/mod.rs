mod windows;
pub(crate) use windows::*;

const ERR_UNKNOWN_OS: &str = "Unknown operating system:";

// Public interface
pub struct OS(OperatingSystem);

enum OperatingSystem {
    Windows(Windows),
}

// Public interface
pub struct Windows(windows::Windows);

impl OS {
    pub fn parse(label: &str) -> Result<OS, String> {
        let os = OperatingSystem::try_from(label)?;

        Ok(OS(os))
    }

    pub fn vendor(&self) -> String {
        match &self.0 {
            OperatingSystem::Windows(w) => w.vendor(),
        }
    }

    pub fn product(&self) -> String {
        match &self.0 {
            OperatingSystem::Windows(w) => w.product(),
        }
    }
}

impl Windows {
    pub fn parse(label: &str) -> Result<Windows, String> {
        let windows = windows::Windows::try_from(label)?;

        Ok(Windows(windows))
    }

    pub fn vendor(&self) -> String {
        self.0.vendor()
    }

    pub fn product(&self) -> String {
        self.0.product()
    }
}

impl TryFrom<&str> for OperatingSystem {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Ok(windows) = windows::Windows::try_from(value) {
            Ok(OperatingSystem::Windows(Windows(windows)))
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
                write!(f, "{}", os.to_string())
            }
        }
    }
}

impl std::fmt::Display for Windows {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // todo: remove debug :?
        write!(f, "{:?}", self.0.to_string())
    }
}
