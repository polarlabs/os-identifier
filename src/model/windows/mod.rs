mod windows_7;
mod windows_8;

pub(crate) mod windows_10;
pub(crate) use windows_10::Windows10;

pub(crate) mod windows_11;
pub(crate) use windows_11::Windows11;

mod windows_vista;
mod windows_xp;

mod windows_server_1709ff;

mod windows_2000;

mod windows_server_2003;

mod windows_server_2008;

mod windows_server_2008_r2;

mod windows_server_2012;

mod windows_server_2012_r2;

mod windows_server_2016;

pub(crate) mod windows_server_2019ff;
pub(crate) use windows_server_2019ff::WindowsServer2019ff;

use windows_7::Windows7;
use windows_8::Windows8;
use windows_2000::Windows2000;
use windows_server_1709ff::WindowsServer1709ff;
use windows_server_2003::WindowsServer2003;
use windows_server_2008::WindowsServer2008;
use windows_server_2008_r2::WindowsServer2008R2;
use windows_server_2012::WindowsServer2012;
use windows_server_2012_r2::WindowsServer2012R2;
use windows_server_2016::WindowsServer2016;
use windows_vista::WindowsVista;
use windows_xp::WindowsXP;

#[derive(Debug)]
pub(crate) enum Windows {
    Windows7(Windows7),
    Windows8(Windows8),
    Windows10(Windows10),
    Windows11(Windows11),
    Windows2000(Windows2000),
    WindowsServer1709ff(WindowsServer1709ff),
    WindowsServer2003(WindowsServer2003),
    WindowsServer2008(WindowsServer2008),
    WindowsServer2008R2(WindowsServer2008R2),
    WindowsServer2012(WindowsServer2012),
    WindowsServer2012R2(WindowsServer2012R2),
    WindowsServer2016(WindowsServer2016),
    WindowsServer2019ff(WindowsServer2019ff),
    WindowsVista(WindowsVista),
    WindowsXP(WindowsXP),
}

impl Windows {
    pub fn to_string(&self) -> Vec<String> {
        match self {
            Windows::Windows7(windows) => windows.to_string(),
            Windows::Windows8(windows) => windows.to_string(),
            Windows::Windows10(windows) => windows.to_string(),
            Windows::Windows11(windows) => windows.to_string(),
            Windows::Windows2000(windows) => windows.to_string(),
            Windows::WindowsServer1709ff(windows) => windows.to_string(),
            Windows::WindowsServer2003(windows) => windows.to_string(),
            Windows::WindowsServer2008(windows) => windows.to_string(),
            Windows::WindowsServer2008R2(windows) => windows.to_string(),
            Windows::WindowsServer2012(windows) => windows.to_string(),
            Windows::WindowsServer2012R2(windows) => windows.to_string(),
            Windows::WindowsServer2016(windows) => windows.to_string(),
            Windows::WindowsServer2019ff(windows) => windows.to_string(),
            Windows::WindowsVista(windows) => windows.to_string(),
            Windows::WindowsXP(windows) => windows.to_string(),
        }
    }

    pub fn vendor(&self) -> String {
        match self {
            Windows::Windows7(w) => String::from(w.vendor()),
            Windows::Windows8(w) => String::from(w.vendor()),
            Windows::Windows10(w) => String::from(w.vendor()),
            Windows::Windows11(w) => String::from(w.vendor()),
            Windows::Windows2000(w) => String::from(w.vendor()),
            Windows::WindowsServer1709ff(w) => String::from(w.vendor()),
            Windows::WindowsServer2003(w) => String::from(w.vendor()),
            Windows::WindowsServer2008(w) => String::from(w.vendor()),
            Windows::WindowsServer2008R2(w) => String::from(w.vendor()),
            Windows::WindowsServer2012(w) => String::from(w.vendor()),
            Windows::WindowsServer2012R2(w) => String::from(w.vendor()),
            Windows::WindowsServer2016(w) => String::from(w.vendor()),
            Windows::WindowsServer2019ff(w) => String::from(w.vendor()),
            Windows::WindowsVista(w) => String::from(w.vendor()),
            Windows::WindowsXP(w) => String::from(w.vendor()),
        }
    }

    pub fn product(&self) -> String {
        match self {
            Windows::Windows7(w) => String::from(w.product()),
            Windows::Windows8(w) => String::from(w.product()),
            Windows::Windows10(w) => String::from(w.product()),
            Windows::Windows11(w) => String::from(w.product()),
            Windows::Windows2000(w) => String::from(w.product()),
            Windows::WindowsServer1709ff(w) => String::from(w.product()),
            Windows::WindowsServer2003(w) => String::from(w.product()),
            Windows::WindowsServer2008(w) => String::from(w.product()),
            Windows::WindowsServer2008R2(w) => String::from(w.product()),
            Windows::WindowsServer2012(w) => String::from(w.product()),
            Windows::WindowsServer2012R2(w) => String::from(w.product()),
            Windows::WindowsServer2016(w) => String::from(w.product()),
            Windows::WindowsServer2019ff(w) => String::from(w.product()),
            Windows::WindowsVista(w) => String::from(w.product()),
            Windows::WindowsXP(w) => String::from(w.product()),
        }
    }

    pub fn release(&self) -> String {
        match self {
            Windows::Windows7(w) => String::from(w.release()),
            Windows::Windows8(w) => String::from(w.release()),
            Windows::Windows10(w) => String::from(w.release()),
            Windows::Windows11(w) => String::from(w.release()),
            Windows::Windows2000(w) => String::from(w.release()),
            Windows::WindowsServer1709ff(w) => String::from(w.release()),
            Windows::WindowsServer2003(w) => String::from(w.release()),
            Windows::WindowsServer2008(w) => String::from(w.release()),
            Windows::WindowsServer2008R2(w) => String::from(w.release()),
            Windows::WindowsServer2012(w) => String::from(w.release()),
            Windows::WindowsServer2012R2(w) => String::from(w.release()),
            Windows::WindowsServer2016(w) => String::from(w.release()),
            Windows::WindowsServer2019ff(w) => String::from(w.release()),
            Windows::WindowsVista(w) => String::from(w.release()),
            Windows::WindowsXP(w) => String::from(w.release()),
        }
    }
    
    pub fn is_enterprise(&self) -> bool {
        match self {
            Windows::Windows7(w) => w.is_enterprise(),
            Windows::Windows8(w) => w.is_enterprise(),
            Windows::Windows10(w) => w.is_enterprise(),
            Windows::Windows11(w) => w.is_enterprise(),
            Windows::Windows2000(w) => w.is_enterprise(),
            Windows::WindowsServer1709ff(w) => w.is_enterprise(),
            Windows::WindowsServer2003(w) => w.is_enterprise(),
            Windows::WindowsServer2008(w) => w.is_enterprise(),
            Windows::WindowsServer2008R2(w) => w.is_enterprise(),
            Windows::WindowsServer2012(w) => w.is_enterprise(),
            Windows::WindowsServer2012R2(w) => w.is_enterprise(),
            Windows::WindowsServer2016(w) => w.is_enterprise(),
            Windows::WindowsServer2019ff(w) => w.is_enterprise(),
            Windows::WindowsVista(w) => w.is_enterprise(),
            Windows::WindowsXP(w) => w.is_enterprise(),
        }
    }

    pub fn is_lts(&self) -> bool {
        match self {
            Windows::Windows7(w) => w.is_lts(),
            Windows::Windows8(w) => w.is_lts(),
            Windows::Windows10(w) => w.is_lts(),
            Windows::Windows11(w) => w.is_lts(),
            Windows::Windows2000(w) => w.is_lts(),
            Windows::WindowsServer1709ff(w) => w.is_lts(),
            Windows::WindowsServer2003(w) => w.is_lts(),
            Windows::WindowsServer2008(w) => w.is_lts(),
            Windows::WindowsServer2008R2(w) => w.is_lts(),
            Windows::WindowsServer2012(w) => w.is_lts(),
            Windows::WindowsServer2012R2(w) => w.is_lts(),
            Windows::WindowsServer2016(w) => w.is_lts(),
            Windows::WindowsServer2019ff(w) => w.is_lts(),
            Windows::WindowsVista(w) => w.is_lts(),
            Windows::WindowsXP(w) => w.is_lts(),
        }
    }
}

impl TryFrom<&str> for Windows {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Ok(windows) = Windows11::try_from(value) {
            Ok(Windows::Windows11(windows))
        } else if let Ok(windows) = Windows10::try_from(value) {
            Ok(Windows::Windows10(windows))
        } else if let Ok(windows) = Windows8::try_from(value) {
            Ok(Windows::Windows8(windows))
        } else if let Ok(windows) = Windows7::try_from(value) {
            Ok(Windows::Windows7(windows))
        } else if let Ok(windows) = WindowsVista::try_from(value) {
            Ok(Windows::WindowsVista(windows))
        } else if let Ok(windows) = WindowsXP::try_from(value) {
            Ok(Windows::WindowsXP(windows))
        } else if let Ok(windows) = WindowsServer2019ff::try_from(value) {
            Ok(Windows::WindowsServer2019ff(windows))
        } else if let Ok(windows) = WindowsServer2016::try_from(value) {
            Ok(Windows::WindowsServer2016(windows))
        } else if let Ok(windows) = WindowsServer2012R2::try_from(value) {
            Ok(Windows::WindowsServer2012R2(windows))
        } else if let Ok(windows) = WindowsServer2012::try_from(value) {
            Ok(Windows::WindowsServer2012(windows))
        } else if let Ok(windows) = WindowsServer2008R2::try_from(value) {
            Ok(Windows::WindowsServer2008R2(windows))
        } else if let Ok(windows) = WindowsServer2008::try_from(value) {
            Ok(Windows::WindowsServer2008(windows))
        } else if let Ok(windows) = WindowsServer2003::try_from(value) {
            Ok(Windows::WindowsServer2003(windows))
        } else if let Ok(windows) = Windows2000::try_from(value) {
            Ok(Windows::Windows2000(windows))
        } else if let Ok(windows) = WindowsServer1709ff::try_from(value) {
            Ok(Windows::WindowsServer1709ff(windows))
        } else {
            Err(format!("Not a windows: {}", value))
        }
    }
}
