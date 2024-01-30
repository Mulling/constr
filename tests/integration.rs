use constr::constr;

// TODO: also generate a macro to_str!(FOO);

#[constr(to_str(u8))]
pub mod foo {
    #[to("foo")]
    pub const FOO: usize = 0x01;

    #[to("bar")]
    pub const BAR: usize = 0x02;

    #[to("baz")]
    pub const BAZ: usize = 0x03;
}

#[test]
fn constr_valid() {
    println!("{}", foo::to_str(10));
}
