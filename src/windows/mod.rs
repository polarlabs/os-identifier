use windows_7::Windows7;
use windows_8::Windows8;
use windows_10::Windows10;
use windows_11::Windows11;
use windows_vista::WindowsVista;
use windows_xp::WindowsXP;

mod windows_7;

mod windows_8;

mod windows_10;

mod windows_11;

mod windows_vista;

mod windows_xp;

#[derive(Debug)]
pub enum Windows {
    Windows7(Windows7),
    Windows8(Windows8),
    Windows10(Windows10),
    Windows11(Windows11),
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
            Windows::WindowsVista(windows) => windows.to_string(),
            Windows::WindowsXP(windows) => windows.to_string(),
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
        } else {
            Err(format!("Not a windows: {}", value))
        }
    }
}
