use crate::windows::windows_7::Windows7;

//
// https://learn.microsoft.com/lifecycle/products/windows-xp
//
#[derive(Debug)]
pub(crate) struct WindowsXP {
    vendor: String,
    product: String,
    release: Release,
    editions: Editions,
}

impl WindowsXP {
    pub(super) fn to_string(&self) -> Vec<String> {
        let out = self.editions.0
            .iter()
            .map(|edition| format!("{} {} {edition} {}", self.vendor, self.product, self.release))
            .collect();

        out
    }
}

impl TryFrom<&str> for WindowsXP {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = value.split('-').collect();

        // Ensure at least 2 parts are present
        if parts.len() < 2 {
            Err(String::from("This is not a Windows XP."))
        } else {
            if parts[0] == "5" {
                let release = Release::try_from(parts[1])?;

                Ok(
                    WindowsXP {
                        vendor: "Microsoft".to_string(),
                        product: "Windows XP".to_string(),
                        release,
                        editions: Editions::all(),
                    }
                )
            } else {
                Err(String::from("This is not a Windows XP."))
            }
        }
    }
}

#[derive(Debug)]
enum Release {
    // Original Release
    GA,
    // Service Pack 1
    SP1,
    // Service Pack 1a
    SP1a,
    // Service Pack 2
    SP2,
    // Service Pack 3
    SP3,
}

impl TryFrom<&str> for Release {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_uppercase().as_str() {
            "SP1" => Ok(Release::SP1),
            "SP1a" => Ok(Release::SP1a),
            "SP2" => Ok(Release::SP2),
            "SP3" => Ok(Release::SP3),
            _ => Err(String::from("This is not a Windows XP Release.")),
        }
    }
}

impl std::fmt::Display for Release {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = match self {
            Release::GA => "",
            Release::SP1 => "SP1",
            Release::SP1a => "SP1a",
            Release::SP2 => "SP2",
            Release::SP3 => "SP3",
        };

        write!(f, "{}", out.to_string())
    }
}

#[derive(Debug)]
struct Editions(Vec<Edition>);

impl Editions {
    fn all() -> Self {
        Editions(vec![
            Edition::Home,
            Edition::Professional,
            Edition::ProfessionalForEmbeddedSystems,
            Edition::ProfessionalX64,
            Edition::Starter,
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
    Home,
    Professional,
    ProfessionalForEmbeddedSystems,
    ProfessionalX64,
    Starter,
}

impl std::fmt::Display for Edition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = match self {
            Edition::Home => "Home",
            Edition::Professional => "Professional",
            Edition::ProfessionalForEmbeddedSystems => "Professional for Embedded Systems",
            Edition::ProfessionalX64 => "Professional x64",
            Edition::Starter => "Starter"
        };

        write!(f, "{}", out.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_string() {
        let label = WindowsXP::try_from("5-sp3").unwrap();

        assert_eq!(label.vendor, "Microsoft".to_string());
        assert_eq!(label.product, "Windows XP".to_string());
        assert_eq!(label.release.to_string(), "SP3".to_string());

        assert_eq!(label.editions.len(), Editions::all().len());
    }
}
