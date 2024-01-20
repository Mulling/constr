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

println!("{}", constants::to_str(2));  // "bar means something else"
println!("{}", constants::to_str!(FOO)); // "foo means something"
```
