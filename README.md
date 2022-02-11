# is-url
Check if a string is an URL.

## Installation

In your `Cargo.toml` add the following line after the `dependencies` field.

```
is-url = "1.0.0"
```

## Usage

This is an example usage someone might do.

```rust

use is_url::is_url;

fn main() {
  println!("{}", is_url("https://google.com")) // true
}
```