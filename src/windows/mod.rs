mod windows_10;
mod windows_11;
mod windows_7;
mod windows_8;
mod windows_vista;
mod windows_xp;

//mod windows_server;
mod windows_2000;

mod windows_server_2003;

mod windows_server_2008;

mod windows_server_2008_r2;

mod windows_server_2012;

mod windows_server_2012_r2;

mod windows_server_2016;

mod windows_server_2019ff;

use windows_7::Windows7;
use windows_8::Windows8;
use windows_10::Windows10;
use windows_11::Windows11;
use windows_2000::Windows2000;
use windows_server_2003::WindowsServer2003;
use windows_server_2008::WindowsServer2008;
use windows_server_2008_r2::WindowsServer2008R2;
use windows_server_2012::WindowsServer2012;
use windows_server_2012_r2::WindowsServer2012R2;
use windows_server_2016::WindowsServer2016;
use windows_server_2019ff::WindowsServer2019ff;
use windows_vista::WindowsVista;
use windows_xp::WindowsXP;

#[derive(Debug)]
pub struct Windows(WindowsClientOS);

impl Windows {
    pub fn to_string(&self) -> Vec<String> {
        self.0.to_string()
    }
}

impl TryFrom<&str> for Windows {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let os = WindowsClientOS::try_from(value)?;
        Ok(Self(os))
    }
}

#[derive(Debug)]
enum WindowsClientOS {
    Windows7(Windows7),
    Windows8(Windows8),
    Windows10(Windows10),
    Windows11(Windows11),
    Windows2000(Windows2000),
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

impl WindowsClientOS {
    pub fn to_string(&self) -> Vec<String> {
        match self {
            WindowsClientOS::Windows7(windows) => windows.to_string(),
            WindowsClientOS::Windows8(windows) => windows.to_string(),
            WindowsClientOS::Windows10(windows) => windows.to_string(),
            WindowsClientOS::Windows11(windows) => windows.to_string(),
            WindowsClientOS::Windows2000(windows) => windows.to_string(),
            WindowsClientOS::WindowsServer2003(windows) => windows.to_string(),
            WindowsClientOS::WindowsServer2008(windows) => windows.to_string(),
            WindowsClientOS::WindowsServer2008R2(windows) => windows.to_string(),
            WindowsClientOS::WindowsServer2012(windows) => windows.to_string(),
            WindowsClientOS::WindowsServer2012R2(windows) => windows.to_string(),
            WindowsClientOS::WindowsServer2016(windows) => windows.to_string(),
            WindowsClientOS::WindowsServer2019ff(windows) => windows.to_string(),
            WindowsClientOS::WindowsVista(windows) => windows.to_string(),
            WindowsClientOS::WindowsXP(windows) => windows.to_string(),
        }
    }
}

impl TryFrom<&str> for WindowsClientOS {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Ok(windows) = Windows11::try_from(value) {
            Ok(WindowsClientOS::Windows11(windows))
        } else if let Ok(windows) = Windows10::try_from(value) {
            Ok(WindowsClientOS::Windows10(windows))
        } else if let Ok(windows) = Windows8::try_from(value) {
            Ok(WindowsClientOS::Windows8(windows))
        } else if let Ok(windows) = Windows7::try_from(value) {
            Ok(WindowsClientOS::Windows7(windows))
        } else if let Ok(windows) = WindowsVista::try_from(value) {
            Ok(WindowsClientOS::WindowsVista(windows))
        } else if let Ok(windows) = WindowsXP::try_from(value) {
            Ok(WindowsClientOS::WindowsXP(windows))
        } else if let Ok(windows) = WindowsServer2019ff::try_from(value) {
            Ok(WindowsClientOS::WindowsServer2019ff(windows))
        } else if let Ok(windows) = WindowsServer2016::try_from(value) {
            Ok(WindowsClientOS::WindowsServer2016(windows))
        } else if let Ok(windows) = WindowsServer2012R2::try_from(value) {
            Ok(WindowsClientOS::WindowsServer2012R2(windows))
        } else if let Ok(windows) = WindowsServer2012::try_from(value) {
            Ok(WindowsClientOS::WindowsServer2012(windows))
        } else if let Ok(windows) = WindowsServer2008R2::try_from(value) {
            Ok(WindowsClientOS::WindowsServer2008R2(windows))
        } else if let Ok(windows) = WindowsServer2008::try_from(value) {
            Ok(WindowsClientOS::WindowsServer2008(windows))
        } else if let Ok(windows) = WindowsServer2003::try_from(value) {
            Ok(WindowsClientOS::WindowsServer2003(windows))
        } else if let Ok(windows) = Windows2000::try_from(value) {
            Ok(WindowsClientOS::Windows2000(windows))
        } else {
            Err(format!("Not a windows: {}", value))
        }
    }
}
