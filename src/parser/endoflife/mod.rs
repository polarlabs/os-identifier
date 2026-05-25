pub(crate) mod linux;
pub(crate) mod windows;

use crate::util;

pub struct EndOfLifeLabel<'a> {
    #[allow(dead_code)]
    raw: &'a str,
    split: Vec<&'a str>,
}

impl EndOfLifeLabel<'_> {
    pub fn get(&self, index: usize) -> Option<&str> {
        self.split.get(index).map(|s| *s)
    }

    pub fn len(&self) -> usize {
        self.split.len()
    }
    
    pub fn starts_with(&self, value: &str) -> bool {
        self.raw.starts_with(value)
    }
    
    pub fn last(&self) -> Option<&str> {
        self.split.last().map(|s| *s)
    }
}

// Create an EndOfLifeLabel from an arbitrary string.
impl<'a> TryFrom<&'a str> for EndOfLifeLabel<'a> {
    type Error = String;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        if util::is_subdivided_by_dashes_only(value) {
            let split = value.split('-').collect();
            Ok(EndOfLifeLabel {
                raw: value,
                split,
            })
        } else {
            Err(String::from("Not an EOL name."))
        }
    }
}
