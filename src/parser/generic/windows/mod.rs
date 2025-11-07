mod windows_11;
pub(crate) use windows_11::Windows11Parser;

include!(concat!(env!("OUT_DIR"), "/build_to_release_map.rs"));

fn resolve_build_to_release(build: &str) -> Result<String, String> {
    if let Some(release) = BUILD_TO_RELEASE_MAP.get(build) {
        Ok(release.get(0).unwrap().to_string())
    } else {
        Err(format!("Build '{}' does not exist.", build))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_to_release_1() {
        let release = resolve_build_to_release("26100");

        assert_eq!(release, Ok(String::from("24H2")));
    }
}
