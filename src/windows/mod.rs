mod windows_7;
mod windows_8;
mod windows_10;
mod windows_11;
mod windows_vista;
mod windows_xp;

use windows_7::Windows7;
use windows_8::Windows8;
use windows_10::Windows10;
use windows_11::Windows11;
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
        } else {
            Err(format!("Not a windows: {}", value))
        }
    }
}
