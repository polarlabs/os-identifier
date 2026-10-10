#[test]
fn test_ubuntu_24_04_generic_1() {
    use os_identifier::Linux;

    let linux = Linux::parse("Ubuntu 24.04 LTS (Noble Numbat)").unwrap();

    assert_eq!(linux.vendor(), "Canonical".to_string());
    assert_eq!(linux.product(), "Ubuntu Linux".to_string());
    assert_eq!(linux.to_string().first().unwrap(), "Ubuntu Linux 24.04 LTS");
}

#[test]
fn test_debian_gnu() {
    use os_identifier::Linux;

    let linux = Linux::parse("Debian GNU 12").unwrap();

    assert_eq!(linux.vendor(), "Debian".to_string());
    assert_eq!(linux.product(), "Debian Linux".to_string());
    assert_eq!(linux.to_string().first().unwrap(), "Debian Linux 12");
}

#[test]
fn test_debian_gnu_linux() {
    use os_identifier::Linux;

    let linux = Linux::parse("Debian GNU/Linux 13").unwrap();

    assert_eq!(linux.vendor(), "Debian".to_string());
    assert_eq!(linux.product(), "Debian Linux".to_string());
    assert_eq!(linux.to_string().first().unwrap(), "Debian Linux 13");
}
