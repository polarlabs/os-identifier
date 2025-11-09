use crate::model;

pub struct Windows(model::Windows);

impl Windows {
    pub fn parse(label: &str) -> Result<Windows, String> {
        let windows = model::Windows::try_from(label)?;

        Ok(Windows(windows))
    }
}

impl std::fmt::Display for Windows {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // todo: remove debug :?
        write!(f, "{:?}", self.0.to_string())
    }
}
