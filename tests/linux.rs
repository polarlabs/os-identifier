#[test]
fn test_ubuntu_24_04_generic_1() {
    use os_identifier::Linux;

    let linux = Linux::parse("Ubuntu 24.04 LTS (Noble Numbat)").unwrap();

    assert_eq!(linux.vendor(), "Canonical".to_string());
    assert_eq!(linux.product(), "Ubuntu Linux".to_string());
    assert_eq!(linux.to_string().get(0).unwrap(), "Ubuntu Linux 24.04 LTS");
}
