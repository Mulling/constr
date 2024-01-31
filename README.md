# constr

```rust
#[constr(some_function(u8) -> Option)]
mod constants {
    #[to("foo means something")]
    const FOO: u8 = 1;
    #[to("bar means something else")]
    const BAR: u8 = 2;
}

println!("{}", constants::to_str(2).unwrap());  // prints "bar means something else"
println!("{}", constants::to_str(3).unwrap());  // panics
println!("{}", constants::to_str(1).unwrap());  // prints "foo means something"

expands to:

#[constr(some_function(u8) -> Option)]
mod constants {
    const FOO: u8 = 1;
    const BAR: u8 = 2;

    pub const fn some_function(arg: u8) -> Option<&'static str> {
        match arg {
            FOO => Some("foo means something"),
            BAR => Some("foo means something else"),
            _ => None

        }
    }
}

```
