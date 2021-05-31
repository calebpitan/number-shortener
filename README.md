# Rust Number Shortener

A Rust crate for making dumb numbers appear smart and if they already look smart, then smarter!

Basically: A number shortener that goes just like the number of followers you might have on Twitter.

This library, maybe, doesn't do much but it's worth having around especially when you need to format some very large number and display it in a very succinct and easy to read manner.

## Usage

The library exports a `shorten_number` function that does the math and wraps whatever result in a `ShortNumber` struct to allow easy access to other desired functionalities.

```rust
extern crate number_shortener;
use number_shortener::{shorten_number, ShortNumber};

fn main() {
    let large_number = 350450550;
    let short_number: ShortNumber = shorten_number(large_number as f32);
    println!("Short Number is: {}", short_number.to_string());

    // Prints: "Short Number is: 350.4M"
}
```

## Ordering and Equality

`ShortNumber`s can be ordered. This means the `>`, `>=`, `<=`, `<` comparisons works on them.

```rust
extern crate number_shortener;
use number_shortener::{shorten_number};

fn main() {
    let number = 350450550_f32;
    let other_number = 350450_f32;
    let short_number = shorten_number(number);
    let other_short_number = shorten_number(other_number);
    assert_eq!(short_number > other_short_number, true);
}
```

They can also be tested for equality, hence can be hashed.

```rust
extern crate number_shortener;
use number_shortener::{shorten_number};

fn main() {
    let number = 350450_f32;
    let other_number = 350450_f32;
    let short_number = shorten_number(number);
    let other_short_number = shorten_number(other_number);
    assert_eq!(short_number == other_short_number, true);
}
```

## What about ShortNumber?

A `ShortNumber` is a struct representation of the just shortened number.

```rust
struct ShortNumber {
    value: f32,
    power: f32,
    pub prefix: Prefix // an enum
}
```

### The `Prefix` enum

Prefixes can also be ordered and tested for equality.

```rust
pub enum Prefix { Kilo, Mega, Giga, Tera, Peta, Unknown }
```

As you already know, the `Prefix::Peta` > `Prefix::Tera` > `Prefix::Giga` > `Prefix::Mega` > `Prefix::Kilo` > `Prefix::Unknown`.

|       Prefix      |       Value (<code>Base<sup>Power</sup></code>)       |
|-------------------|-------------------------------------------------------|
|       Kilo        |       10<sup>3</sup>                                  |
|       Mega        |       10<sup>6</sup>                                  |
|       Giga        |       10<sup>9</sup>                                  |
|       Tera        |       10<sup>12</sup>                                 |
|       Peta        |       10<sup>15</sup>                                 |
|       Unknown     |       10<sup>0</sup> - 10<sup>-(1...n)</sup>          |

### API

**ShortNumber::new**

Constructs a new `ShortNumber`. Takes the representation of a number as float and the power of the number too, from which the original value of the number can be re-constructed.

```rust
extern crate number_shortener;
use number_shortener::{ShortNumber};

let short_number = ShortNumber::new(120.21, 3.0); 
```

**ShortNumber::get_value**

Get the value of the `ShortNumber`.

```rust
extern crate number_shortener;
use number_shortener::{ShortNumber};

let short_number = ShortNumber::new(120.21, 3.0);
let (short_value, power) = short_number.get_value();
assert_eq!(short_value, 120.21);
assert_eq!(power, 3.0);

// to get the original value
// use number_shortener::{BASE};
// let original_number = short_value * BASE.powf(power);
```

**ShortNumber::get_original_value**

Get the value of the `ShortNumber`.

```rust
extern crate number_shortener;
use number_shortener::{ShortNumber};

let short_number = ShortNumber::new(120.21, 3.0);
let original_value = short_number.get_original_value();
assert_eq!(original_value, 120210);
```

**ShortNumber::get_prefix**

Get the string prefix for the `ShortNumber`.

```rust
extern crate number_shortener;
use number_shortener::{ShortNumber, Prefix};

let short_number = ShortNumber::new(120.21, 3.0);
let prefix = short_number.get_prefix();
assert_eq!(prefix, "K");
assert_eq!(short_number.prefix, Prefix::Kilo);

// Unknown prefix
let short_number = ShortNumber::new(120.21, 0.0);
let prefix = short_number.get_prefix();
assert_eq!(prefix, "");
assert_eq!(short_number.prefix, Prefix::Unknown);
```

**ShortNumber::to_string**

Get the formatted string representation for the `ShortNumber`.

```rust
extern crate number_shortener;
use number_shortener::{ShortNumber, Prefix};

let short_number = ShortNumber::new(120.21, 3.0);
let formatted = short_number.to_string();
assert_eq!(formatted, "120.2K");

// Unknown prefix
let short_number = ShortNumber::new(120.0, 0.0);
let formatted = short_number.to_string();
assert_eq!(formatted, "120.0");
```

## License

This library is licensed under the [MIT License](https://github.com/calebpitan/number-shortener/tree/master/LICENSE), Copyright (c) 2020 Caleb Adepitan.