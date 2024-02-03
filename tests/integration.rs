use constr::constr;

#[constr(to_str(u8))]
pub mod foo {
    #[str = "foo"]
    pub const FOO: usize = 0x01;

    #[str = "bar"]
    pub const BAR: usize = 0x02;

    #[str = "baz"]
    pub const BAZ: usize = 0x03;

    #[str = "TODO: This shoudl fail as we already have 0x03"]
    pub const SHOULD_FAIL: usize = 0x03;

    // FIXME: should be IGNORED
    // pub const IGNORED: u8 = 0x00;
}

#[test]
fn test_mod_foo() {
    assert_eq!(to_str(3), "baz");
    assert_eq!(to_str(2), "bar");
    assert_eq!(to_str(1), "foo");
}
