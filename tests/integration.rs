use constr::constr;

// TODO: also generate a macro to_str!(FOO);

#[constr(to_str(u8))]
mod foo {
    #[to("test")]
    pub const FOO: usize = 0x08;
}

#[test]
fn constr_valid() {
    println!("{}", to_str(10));
}
