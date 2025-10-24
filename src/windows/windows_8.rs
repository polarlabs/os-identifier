use crate::windows::windows_7::Windows7;

//
// https://learn.microsoft.com/lifecycle/products/windows-8
// https://learn.microsoft.com/lifecycle/products/windows-81
//
#[derive(Debug)]
pub(crate) struct Windows8 {
    vendor: String,
    product: String,
    editions: Editions,
}

impl TryFrom<&str> for Windows8 {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "8" => {
                Ok(
                    Windows8 {
                        vendor: "Microsoft".to_string(),
                        product: "Windows 8".to_string(),
                        editions: Editions::all(),
                    }
                )
            },
            "8.1" => {
                Ok(
                    Windows8 {
                        vendor: "Microsoft".to_string(),
                        product: "Windows 8.1".to_string(),
                        editions: Editions::all(),
                    }
                )
            }
            _ => {
                Err(String::from("This is not a Windows 8."))
            }
        }
    }
}

#[derive(Debug)]
struct Editions(Vec<Edition>);

impl Editions {
    fn all() -> Self {
        Editions(vec![
            Edition::Enterprise,
            Edition::EnterpriseN,
            Edition::N,
            Edition::ProWithMediaCenter,
            Edition::Professional,
            Edition::ProfessionalN,
            Edition::SL
        ])
    }

    fn contains(&self, edition: Edition) -> bool {
        self.0.contains(&edition)
    }

    fn len(&self) -> usize {
        self.0.len()
    }
}

#[derive(PartialEq, Debug)]
enum Edition {
    Enterprise,
    EnterpriseN,
    N,
    ProWithMediaCenter,
    Professional,
    ProfessionalN,
    SL,
}

impl std::fmt::Display for Edition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = match self {
            Edition::Enterprise => "Enterprise",
            Edition::EnterpriseN => "Enterprise N",
            Edition::N => "N",
            Edition::ProWithMediaCenter => "Pro with Media Center",
            Edition::Professional => "Professional",
            Edition::ProfessionalN => "Professional N",
            Edition::SL => "SL"
        };

        write!(f, "{}", out.to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::windows::windows_xp::WindowsXP;
    use super::*;

    #[test]
    fn test_from_string_8() {
        let label = Windows8::try_from("8").unwrap();

        assert_eq!(label.vendor, "Microsoft".to_string());
        assert_eq!(label.product, "Windows 8".to_string());

        assert_eq!(label.editions.len(), Editions::all().len());
    }

    #[test]
    fn test_from_string_81() {
        let label = Windows8::try_from("8.1").unwrap();

        assert_eq!(label.vendor, "Microsoft".to_string());
        assert_eq!(label.product, "Windows 8.1".to_string());

        assert_eq!(label.editions.len(), Editions::all().len());
    }
}
