mod linux;
pub(crate) use linux::*;

mod windows;
pub(crate) use windows::*;

const ERR_UNKNOWN_OS: &str = "Unknown operating system:";

// Public interface
#[derive(Debug)]
pub struct OS(OperatingSystem);

#[derive(Debug)]
enum OperatingSystem {
    Linux(Linux),
    Windows(Windows),
}

// Public interface
#[derive(Debug)]
pub struct Linux(linux::Linux);

// Public interface
#[derive(Debug)]
pub struct Windows(windows::Windows);

impl OS {
    pub fn parse(label: &str) -> Result<OS, String> {
        let os = OperatingSystem::try_from(label)?;

        Ok(OS(os))
    }

    pub fn vendor(&self) -> String {
        match &self.0 {
            OperatingSystem::Linux(l) => l.vendor(),
            OperatingSystem::Windows(w) => w.vendor(),
        }
    }

    pub fn product(&self) -> String {
        match &self.0 {
            OperatingSystem::Linux(l) => l.product(),
            OperatingSystem::Windows(w) => w.product(),
        }
    }

    pub fn release(&self) -> String {
        match &self.0 {
            OperatingSystem::Linux(l) => l.release(),
            OperatingSystem::Windows(w) => w.release(),
        }
    }
    
    pub fn is_enterprise(&self) -> bool {
        match &self.0 {
            OperatingSystem::Linux(l) => l.is_enterprise(),
            OperatingSystem::Windows(w) => w.is_enterprise(),
        }
    }

    pub fn is_lts(&self) -> bool {
        match &self.0 {
            OperatingSystem::Linux(l) => l.is_lts(),
            OperatingSystem::Windows(w) => w.is_lts(),
        }
    }
    
    pub fn to_string(&self) -> Vec<String> {
        match &self.0 {
            OperatingSystem::Linux(os) => {
                os.to_string()
            },
            OperatingSystem::Windows(os) => {
                os.to_string()
            },
        }
    }
}

impl Linux {
    pub fn parse(label: &str) -> Result<Linux, String> {
        let linux = linux::Linux::try_from(label)?;

        Ok(Linux(linux))
    }

    pub fn vendor(&self) -> String {
        self.0.vendor()
    }

    pub fn product(&self) -> String {
        self.0.product()
    }

    pub fn release(&self) -> String {
        self.0.release()
    }
    
    pub fn is_enterprise(&self) -> bool {
        self.0.is_enterprise()
    }

    pub fn is_lts(&self) -> bool {
        self.0.is_lts()
    }
    
    pub fn to_string(&self) -> Vec<String> {
        self.0.to_string()
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

    pub fn release(&self) -> String {
        self.0.release()
    }
    
    pub fn is_enterprise(&self) -> bool {
        self.0.is_enterprise()
    }

    pub fn is_lts(&self) -> bool {
        self.0.is_lts()
    }
    
    pub fn to_string(&self) -> Vec<String> {
        self.0.to_string()
    }
}

impl TryFrom<&str> for OperatingSystem {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Ok(windows) = windows::Windows::try_from(value) {
            Ok(OperatingSystem::Windows(Windows(windows)))
        } else if let Ok(linux) = linux::Linux::try_from(value) {
            Ok(OperatingSystem::Linux(Linux(linux)))
        } else {
            Err(format!("{} \"{}\"", ERR_UNKNOWN_OS, value))
        }
    }
}

/*
impl std::fmt::Display for OS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.to_string())
    }
}
*/

/*
impl std::fmt::Display for OperatingSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OperatingSystem::Windows(os) => {
                write!(f, "{}", os.to_string())
            }
        }
    }
}
*/

/*
impl std::fmt::Display for Windows {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // todo: remove debug :?
        write!(f, "{:?}", self.0.to_string())
    }
}
*/
