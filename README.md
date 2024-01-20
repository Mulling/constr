# constr

Something like this:

```rust
#[constr(to_str(u8))]
mod constants {
    #[to("foo means something")]
    const FOO: usize = 1;
    #[to("bar means something else")]
    const BAR: usize = 2;
}

println!("{}", constants::to_str(2));  // "foo means something"
println!("{}", constants::to_str!(1)); // "some meaning to foo"
```
