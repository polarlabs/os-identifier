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
fn test_windows_11() {
    use os_identifier::Windows;

    let windows = Windows::try_from("11-24h2-e");
    assert!(windows.is_ok());

    let windows = windows.unwrap();
    let canonical_names = windows.to_string();
    assert!(canonical_names.contains(&String::from("Microsoft Windows 11 Enterprise 24H2")));
}

#[test]
fn test_windows_2000() {
    use os_identifier::Windows;

    let windows = Windows::try_from("2000-sp1");
    assert!(windows.is_ok());

    let windows = windows.unwrap();
    let canonical_names = windows.to_string();
    assert!(canonical_names.contains(&String::from("Microsoft Windows 2000 Server SP1")));
}

#[test]
fn test_windows_server_2003() {
    use os_identifier::Windows;

    let windows = Windows::try_from("2003-sp1");
    assert!(windows.is_ok());

    let windows = windows.unwrap();
    let canonical_names = windows.to_string();
    assert!(canonical_names.contains(&String::from("Microsoft Windows Server 2003 Web SP1")));
}

#[test]
fn test_windows_server_2008() {
    use os_identifier::Windows;

    let windows = Windows::try_from("2008-sp2");
    assert!(windows.is_ok());

    let windows = windows.unwrap();
    let canonical_names = windows.to_string();
    assert!(canonical_names.contains(&String::from("Microsoft Windows Server 2008 Foundation SP2")));
}

#[test]
fn test_windows_server_2008_r2() {
    use os_identifier::Windows;

    let windows = Windows::try_from("2008-r2-sp1");
    assert!(windows.is_ok());

    let windows = windows.unwrap();
    let canonical_names = windows.to_string();
    assert!(canonical_names.contains(&String::from("Microsoft Windows Server 2008 R2 HPC SP1")));
}

#[test]
fn test_windows_server_2012() {
    use os_identifier::Windows;

    let windows = Windows::try_from("2012");
    assert!(windows.is_ok());

    let windows = windows.unwrap();
    let canonical_names = windows.to_string();
    assert!(canonical_names.contains(&String::from("Microsoft Windows Server 2012 Essentials")));
}

#[test]
fn test_windows_server_2012_r2() {
    use os_identifier::Windows;

    let windows = Windows::try_from("2012-r2");
    assert!(windows.is_ok());

    let windows = windows.unwrap();
    let canonical_names = windows.to_string();
    assert!(canonical_names.contains(&String::from("Microsoft Windows Server 2012 R2 Foundation")));
}

#[test]
fn test_windows_server_2016() {
    use os_identifier::Windows;

    let windows = Windows::try_from("2016");
    assert!(windows.is_ok());

    let windows = windows.unwrap();
    let canonical_names = windows.to_string();
    assert!(canonical_names.contains(&String::from("Microsoft Windows Server 2016 Datacenter")));
}

#[test]
fn test_windows_server_2019() {
    use os_identifier::Windows;

    let windows = Windows::try_from("2019");
    assert!(windows.is_ok());

    let windows = windows.unwrap();
    let canonical_names = windows.to_string();
    assert!(canonical_names.contains(&String::from("Microsoft Windows Server 2019 Datacenter")));
}

#[test]
fn test_windows_server_2022() {
    use os_identifier::Windows;

    let windows = Windows::try_from("2022");
    assert!(windows.is_ok());

    let windows = windows.unwrap();
    let canonical_names = windows.to_string();
    assert!(canonical_names.contains(&String::from("Microsoft Windows Server 2022 Standard")));
}

#[test]
fn test_windows_server_2025() {
    use os_identifier::Windows;

    let windows = Windows::try_from("2025");
    assert!(windows.is_ok());

    let windows = windows.unwrap();
    let canonical_names = windows.to_string();
    assert!(canonical_names.contains(&String::from("Microsoft Windows Server 2025 Datacenter")));
}

#[test]
fn test_windows_vista() {
    use os_identifier::Windows;

    let windows = Windows::try_from("6-sp2");
    assert!(windows.is_ok());

    let windows = windows.unwrap();
    let canonical_names = windows.to_string();
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
