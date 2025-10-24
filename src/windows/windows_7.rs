//
// https://learn.microsoft.com/lifecycle/products/windows-7
//
#[derive(Debug)]
pub(crate) struct Windows7 {
    vendor: String,
    product: String,
    release: Release,
    editions: Editions,
}

impl TryFrom<&str> for Windows7 {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = value.split('-').collect();

        // Ensure at least 2 parts are present
        if parts.len() < 2 {
            Err(String::from("This is not a Windows 7."))
        } else {
            if parts[0] == "7" {
                let release = Release::try_from(parts[1])?;

                Ok(
                    Windows7 {
                        vendor: "Microsoft".to_string(),
                        product: "Windows 7".to_string(),
                        release,
                        editions: Editions::all(),
                    }
                )
            } else {
                Err(String::from("This is not a Windows 7."))
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
    // Extended Security Update Year 1
    ESU1,
    // Extended Security Update Year 2
    ESU2,
    // Extended Security Update Year 3
    ESU3,
}

impl TryFrom<&str> for Release {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_uppercase().as_str() {
            "SP1" => Ok(Release::SP1),
            "ESU1" => Ok(Release::ESU1),
            "ESU2" => Ok(Release::ESU2),
            "ESU3" => Ok(Release::ESU3),
            _ => Err(String::from("This is not a Windows 7 Release.")),
        }
    }
}

impl std::fmt::Display for Release {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = match self {
            Release::GA => "",
            Release::SP1 => "SP1",
            Release::ESU1 => "ESU1",
            Release::ESU2 => "ESU2",
            Release::ESU3 => "ESU3",
        };

        write!(f, "{}", out.to_string())
    }
}

#[derive(Debug)]
struct Editions(Vec<Edition>);

impl Editions {
    fn all() -> Self {
        Editions(vec![
            Edition::Enterprise,
            Edition::EnterpriseN,
            Edition::HomeBasic,
            Edition::HomePremium,
            Edition::HomePremiumN,
            Edition::Professional,
            Edition::ProfessionalForEmbeddedSystems,
            Edition::ProfessionalN,
            Edition::Starter,
            Edition::StarterN,
            Edition::Ultimate,
            Edition::UltimateForEmbeddedSystems,
            Edition::UltimateN,
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
    HomeBasic,
    HomePremium,
    HomePremiumN,
    Professional,
    ProfessionalForEmbeddedSystems,
    ProfessionalN,
    Starter,
    StarterN,
    Ultimate,
    UltimateForEmbeddedSystems,
    UltimateN,
}

impl std::fmt::Display for Edition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = match self {
            Edition::Enterprise => "Enterprise",
            Edition::EnterpriseN => "Enterprise N",
            Edition::HomeBasic => "Home Basic",
            Edition::HomePremium => "Home Premium",
            Edition::HomePremiumN => "Home Premium N",
            Edition::Professional => "Professional",
            Edition::ProfessionalForEmbeddedSystems => "Professional for Embedded Systems",
            Edition::ProfessionalN => "Professional N",
            Edition::Starter => "Starter",
            Edition::StarterN => "Starter N",
            Edition::Ultimate => "Ultimate",
            Edition::UltimateForEmbeddedSystems => "Ultimate for Embedded Systems",
            Edition::UltimateN => "Ultimate N",
        };

        write!(f, "{}", out.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_string() {
        let label = Windows7::try_from("7-sp1").unwrap();

        assert_eq!(label.vendor, "Microsoft".to_string());
        assert_eq!(label.product, "Windows 7".to_string());
        assert_eq!(label.release.to_string(), "SP1".to_string());

        assert_eq!(label.editions.len(), Editions::all().len());
    }
}
