# factorialfunction
Function named factorial() takes input of positive integer and shows its factorial
If a positive integer is not entered, the function will keep on taking an input until a positive integer is entered.
To reduce the size, u32 variables are used hence you can enter from 0 ~ 12 to get factorial.
Function will return the factorial of the value entered by you.

To use this library you have to add following line in [dependencies] section of Cargo.toml

factorialfunction = "0.1.0"

In `src/main.rs` you can use like this:

```
use factorialfunction;
fn main() {
    println!("Hello, world!");
    factorialfunction::factorial();
}
```
now `cargo run` for results
