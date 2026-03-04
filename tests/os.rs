#[test]
fn test_windows_11_endoflife_1() {
    use os_identifier::OS;

    let windows = OS::parse("11-24h2-e");
    assert!(windows.is_ok());

    let windows = windows.unwrap();
    let canonical_names = windows.to_string();
    assert!(canonical_names.contains(&String::from("Microsoft Windows 11 Enterprise 24H2")));
}

#[test]
fn test_windows_11_generic_1() {
    use os_identifier::OS;

    let windows = OS::parse("Microsoft Windows 11 Enterprise 21H2");
    assert!(windows.is_ok());

    let windows = windows.unwrap();
    let canonical_names = windows.to_string();
    assert!(canonical_names.contains(&String::from("Microsoft Windows 11 Enterprise 21H2")));
}

#[test]
fn test_unknown_endoflife_1() {
    use os_identifier::OS;

    let windows = OS::parse("eol-1");
    assert!(windows.is_err());
}

#[test]
fn test_unknown_generic_1() {
    use os_identifier::OS;

    let windows = OS::parse("Ops unknown");
    assert!(windows.is_err());
}
