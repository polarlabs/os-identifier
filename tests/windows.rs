#[test]
fn test_windows_7() {
    use os_identifier::Windows;

    let windows = Windows::try_from("7-sp1");
    assert!(windows.is_ok());

    let windows = windows.unwrap();
    let canonical_names = windows.to_string();
    assert!(canonical_names.contains(&String::from("Microsoft Windows 7 Ultimate SP1")));
}

#[test]
fn test_windows_8() {
    use os_identifier::Windows;

    let windows = Windows::try_from("8.1");
    assert!(windows.is_ok());

    let windows = windows.unwrap();
    let canonical_names = windows.to_string();
    assert!(canonical_names.contains(&String::from("Microsoft Windows 8.1 Professional")));
}

#[test]
fn test_windows_10() {
    use os_identifier::Windows;

    let windows = Windows::try_from("10-1809-w");
    assert!(windows.is_ok());

    let windows = windows.unwrap();
    let canonical_names = windows.to_string();
    assert!(canonical_names.contains(&String::from("Microsoft Windows 10 Pro 1809")));
}

#[test]
fn test_windows_10_1507_iot() {
    use os_identifier::Windows;

    let windows = Windows::try_from("10-1507-iot");
    assert!(windows.is_ok());

    let windows = windows.unwrap();
    let canonical_names = windows.to_string();
    assert!(canonical_names.contains(&String::from("Microsoft Windows 10 IoT Core 1507")));
}

#[test]
fn test_windows_11() {
    use os_identifier::Windows;

    let windows = Windows::try_from("11-24h2-e");
    assert!(windows.is_ok());

    let windows = windows.unwrap();
    let canonical_names = windows.to_string();
    assert!(canonical_names.contains(&String::from("Microsoft Windows 11 Enterprise 24H2")));
}

#[test]
fn test_windows_vista() {
    use os_identifier::Windows;

    let windows = Windows::try_from("6-sp2");
    assert!(windows.is_ok());

    let windows = windows.unwrap();
    let canonical_names = windows.to_string();
    println!("Debug: {:?}", canonical_names);
    assert!(canonical_names.contains(&String::from("Microsoft Windows Vista Ultimate 64-bit SP2")));
}

#[test]
fn test_windows_xp() {
    use os_identifier::Windows;

    let windows = Windows::try_from("5-sp3");
    assert!(windows.is_ok());

    let windows = windows.unwrap();
    let canonical_names = windows.to_string();
    assert!(canonical_names.contains(&String::from("Microsoft Windows XP Professional x64 SP3")));
}
