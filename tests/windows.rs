#[test]
fn test_windows_11() {
    use os_identifier::Windows;

    let windows = Windows::try_from("11-24h2-e");
    assert!(windows.is_ok());

    let windows = windows.unwrap();
    let canonical_names = windows.to_string();
    assert!(canonical_names.contains(&String::from("Microsoft Windows 11 Enterprise 24H2")));
}
