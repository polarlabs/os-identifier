pub(crate) mod windows;

pub struct GenericLabel<'a> {
    #[allow(dead_code)]
    raw: &'a str,
}

impl<'a> From<&'a str> for GenericLabel<'a> {
    fn from(value: &'a str) -> Self {
            GenericLabel {
                raw: value,
            }
    }
}
