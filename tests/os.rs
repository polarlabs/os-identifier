#[test]
fn test_windows_11_endoflife_1() {
    use os_identifier::OS;

    let windows = OS::parse("windows-11-24h2-e");
    assert!(windows.is_ok());

    let windows = windows.unwrap();
    let canonical_names = windows.to_string();
    assert!(canonical_names.contains(&String::from("Microsoft Windows 11 Enterprise 24H2")));
}

#[test]
fn test_windows_11_endoflife_2() {
    use os_identifier::OS;

    let os = OS::parse("windows-11-24h2-e-lts");
    assert!(os.is_ok());

    let os = os.unwrap();
    assert_eq!(os.vendor(), "Microsoft".to_string());
    assert_eq!(os.product(), "Windows 11".to_string());
    assert_eq!(os.release(), "24H2".to_string());

    assert!(os.is_enterprise());
    assert!(os.is_lts());
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
fn test_windows_11_generic_2() {
    use os_identifier::OS;

    let os = OS::parse("Windows 11 Enterprise Edition (Build 26100) (64 Bit) GA (General Availability)").unwrap();

    assert_eq!(os.vendor(), "Microsoft".to_string());
    assert_eq!(os.product(), "Windows 11".to_string());
    assert_eq!(os.release(), "24H2".to_string());

    assert!(os.is_enterprise());
    assert!(!os.is_lts());
}

#[test]
fn test_windows_server_generic_1() {
    use os_identifier::OS;

    let windows = OS::parse("Windows Server 2019 Server Standard Edition (Build 17763) (64 Bit)");
    assert!(windows.is_ok());

    let windows = windows.unwrap();
    let canonical_names = windows.to_string();
    assert!(canonical_names.contains(&String::from("Microsoft Windows Server 2019 Standard")));
}

#[test]
fn test_windows_server_generic_2() {
    use os_identifier::OS;

    let windows = OS::parse("Windows Server Build 26100");
    assert!(windows.is_ok());

    let windows = windows.unwrap();
    let canonical_names = windows.to_string();
    assert!(canonical_names.contains(&String::from("Microsoft Windows Server 2025 Standard")));
    assert!(canonical_names.contains(&String::from("Microsoft Windows Server 2025 Datacenter")));
}

#[test]
fn test_debian_13_endoflife_1() {
    use os_identifier::OS;

    let linux = OS::parse("debian-13");
    assert!(linux.is_ok());

    let linux = linux.unwrap();
    let canonical_names = linux.to_string();
    assert!(canonical_names.contains(&String::from("Debian Linux 13")));
}

#[test]
fn test_rhel_9_7_endoflife_1() {
    use os_identifier::OS;

    let linux = OS::parse("rhel-9.7");
    assert!(linux.is_ok());

    let linux = linux.unwrap();
    let canonical_names = linux.to_string();
    assert!(canonical_names.contains(&String::from("Red Hat Enterprise Linux 9.7")));
}

#[test]
fn test_ubuntu_24_04_endoflife_1() {
    use os_identifier::OS;

    let linux = OS::parse("ubuntu-24.04");
    assert!(linux.is_ok());

    let linux = linux.unwrap();
    let canonical_names = linux.to_string();
    assert!(canonical_names.contains(&String::from("Ubuntu Linux 24.04 LTS")));
}

#[test]
fn test_ubuntu_25_04_endoflife_1() {
    use os_identifier::OS;

    let linux = OS::parse("ubuntu-25.04");
    assert!(linux.is_ok());

    let linux = linux.unwrap();
    let canonical_names = linux.to_string();
    assert!(canonical_names.contains(&String::from("Ubuntu Linux 25.04")));
}

#[test]
fn test_ubuntu_24_04_generic_1() {
    use os_identifier::OS;

    let os = OS::parse("Ubuntu 24.04 LTS (Noble Numbat)");
    assert!(os.is_ok());

    let os = os.unwrap();
    assert_eq!(os.vendor(), "Canonical".to_string());
    assert_eq!(os.product(), "Ubuntu Linux".to_string());
    assert_eq!(os.release(), "24.04".to_string());

    assert!(os.is_enterprise());
    assert!(os.is_lts());
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
