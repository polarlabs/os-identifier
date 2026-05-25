pub(crate) mod debian;
pub(crate) use debian::Debian;

pub(crate) mod oracle;
pub(crate) use oracle::OracleLinux;

pub(crate) mod rhel;
pub(crate) use rhel::RedHatEnterpriseLinux;

pub(crate) mod ubuntu;
pub(crate) use ubuntu::Ubuntu;

#[derive(Debug)]
pub(crate) enum Linux {
    Debian(Debian),
    OracleLinux(OracleLinux),
    RedHatEnterpriseLinux(RedHatEnterpriseLinux),
    Ubuntu(Ubuntu),
}

impl Linux {
    pub fn to_string(&self) -> Vec<String> {
        match self {
            Linux::Debian(linux) => linux.to_string(),
            Linux::OracleLinux(linux) => linux.to_string(),
            Linux::RedHatEnterpriseLinux(linux) => linux.to_string(),
            Linux::Ubuntu(linux) => linux.to_string(),
        }
    }

    pub fn vendor(&self) -> String {
        match self {
            Linux::Debian(linux) => String::from(linux.vendor()),
            Linux::OracleLinux(linux) => String::from(linux.vendor()),
            Linux::RedHatEnterpriseLinux(linux) => String::from(linux.vendor()),
            Linux::Ubuntu(linux) => String::from(linux.vendor()),
        }
    }

    pub fn product(&self) -> String {
        match self {
            Linux::Debian(linux) => String::from(linux.product()),
            Linux::OracleLinux(linux) => String::from(linux.product()),
            Linux::RedHatEnterpriseLinux(linux) => String::from(linux.product()),
            Linux::Ubuntu(linux) => String::from(linux.product()),
        }
    }

    pub fn release(&self) -> String {
        match self {
            Linux::Debian(linux) => String::from(linux.release()),
            Linux::OracleLinux(linux) => String::from(linux.release()),
            Linux::RedHatEnterpriseLinux(linux) => String::from(linux.release()),
            Linux::Ubuntu(linux) => String::from(linux.release()),
        }
    }
    
    pub fn is_enterprise(&self) -> bool {
        match self {
            Linux::Debian(linux) => linux.is_enterprise(),
            Linux::OracleLinux(linux) => linux.is_enterprise(),
            Linux::RedHatEnterpriseLinux(linux) => linux.is_enterprise(),
            Linux::Ubuntu(linux) => linux.is_enterprise(),
        }
    }

    pub fn is_lts(&self) -> bool {
        match self {
            Linux::Debian(linux) => linux.is_lts(),
            Linux::OracleLinux(linux) => linux.is_lts(),
            Linux::RedHatEnterpriseLinux(linux) => linux.is_lts(),
            Linux::Ubuntu(linux) => linux.is_lts(),
        }
    }
}

impl TryFrom<&str> for Linux {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Ok(linux) = Debian::try_from(value) {
            Ok(Linux::Debian(linux))
        } else if let Ok(linux) = OracleLinux::try_from(value) {
            Ok(Linux::OracleLinux(linux))
        } else if let Ok(linux) = RedHatEnterpriseLinux::try_from(value) {
            Ok(Linux::RedHatEnterpriseLinux(linux))
        } else if let Ok(linux) = Ubuntu::try_from(value) {
            Ok(Linux::Ubuntu(linux))
        } else {
            Err(format!("Not a linux: {}", value))
        }
    }
}
