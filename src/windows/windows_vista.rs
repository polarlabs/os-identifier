//
// https://learn.microsoft.com/lifecycle/products/windows-vista
//
#[derive(Debug)]
pub(crate) struct WindowsVista {
    vendor: String,
    product: String,
    release: Release,
    editions: Editions,
}

impl TryFrom<&str> for WindowsVista {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = value.split('-').collect();

        // Ensure at least 2 parts are present
        if parts.len() < 2 {
            Err(String::from("This is not a Windows Vista."))
        } else {
            if parts[0] == "6" {
                let release = Release::try_from(parts[1])?;

                Ok(
                    WindowsVista {
                        vendor: "Microsoft".to_string(),
                        product: "Windows Vista".to_string(),
                        release,
                        editions: Editions::all(),
                    }
                )
            } else {
                Err(String::from("This is not a Windows Vista."))
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
    // Service Pack 2
    SP2,
}

impl TryFrom<&str> for Release {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_uppercase().as_str() {
            "SP1" => Ok(Release::SP1),
            "SP2" => Ok(Release::SP2),
            _ => Err(String::from("This is not a Windows Vista Release.")),
        }
    }
}

impl std::fmt::Display for Release {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = match self {
            Release::GA => "",
            Release::SP1 => "SP1",
            Release::SP2 => "SP2",
        };

        write!(f, "{}", out.to_string())
    }
}

#[derive(Debug)]
struct Editions(Vec<Edition>);

impl Editions {
    fn all() -> Self {
        Editions(vec![
            Edition::Business,
            Edition::BusinessN,
            Edition::BusinessN64bit,
            Edition::Enterprise,
            Edition::Enterprise64bit,
            Edition::EnterpriseX64,
            Edition::HomeBasic,
            Edition::HomeBasic64bit,
            Edition::HomeBasicN,
            Edition::HomeBasicN64bit,
            Edition::HomePremium,
            Edition::HomePremium64bit,
            Edition::Starter,
            Edition::Ultimate,
            Edition::Ultimate64bit,
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
    Business,
    BusinessN,
    BusinessN64bit,
    Enterprise,
    Enterprise64bit,
    EnterpriseX64,
    HomeBasic,
    HomeBasic64bit,
    HomeBasicN,
    HomeBasicN64bit,
    HomePremium,
    HomePremium64bit,
    Starter,
    Ultimate,
    Ultimate64bit,
}

impl std::fmt::Display for Edition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = match self {
            Edition::Business => "Business",
            Edition::BusinessN => "Business N",
            Edition::BusinessN64bit => "Business N 64-bit",
            Edition::Enterprise => "Enterprise",
            Edition::Enterprise64bit => "Enterprise 64-bit",
            Edition::EnterpriseX64 => "Enterprise X64",
            Edition::HomeBasic => "Home Basic",
            Edition::HomeBasic64bit => "Home Basic 64-bit",
            Edition::HomeBasicN => "Home Basic N",
            Edition::HomeBasicN64bit => "Home Basic N 64-bit",
            Edition::HomePremium => "Home Premium",
            Edition::HomePremium64bit => "Home Premium 64-bit",
            Edition::Starter => "Starter",
            Edition::Ultimate => "Ultimate",
            Edition::Ultimate64bit => "Ultimate 64-bit",
        };

        write!(f, "{}", out.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_string() {
        let label = WindowsVista::try_from("6-sp2").unwrap();

        assert_eq!(label.vendor, "Microsoft".to_string());
        assert_eq!(label.product, "Windows Vista".to_string());
        assert_eq!(label.release.to_string(), "SP2".to_string());

        assert_eq!(label.editions.len(), Editions::all().len());
    }
}
