use std::hash::Hasher;
use std::hash::Hash;
use std::cmp::Ordering;

mod prefix;

use prefix::{ get_prefixes, get_prefixes_enum };
pub use prefix::Prefix;

pub const LEAST_EXPONENT: f32 = 3_f32;
pub const BASE: f32 = 10_f32;


#[derive(Debug, Copy, Clone)]
pub struct ShortNumber {
    value: f32,
    power: f32,
    pub prefix: Prefix
}

impl ShortNumber {
    pub fn new(value: f32, power: f32) -> Self {
        let prefix = *get_prefixes_enum().get(&(power as i32)).unwrap_or(&Prefix::Unknown);

        ShortNumber{ value, power, prefix }
    }

    /// Get the value of the `ShortNumber` as a tuple of the `significant_value`
    /// after shortening and the `power` a combination of which can regenerate
    /// the approximate original value.
    /// 
    /// # Examples
    /// 
    /// ```
    /// extern crate number_shortener;
    /// use number_shortener::{ShortNumber, BASE};
    /// 
    /// let short_number = ShortNumber::new(1.0, 3.0);
    /// let (value, power) = short_number.get_value();
    /// let approx_original_value = value * BASE.powf(power);
    /// 
    /// assert_eq!(approx_original_value, 1000.0);
    /// assert_eq!(value, 1.0);
    /// assert_eq!(power, 3.0);
    /// ```
    #[inline]
    pub fn get_value(&self) -> (f32, f32) { (self.value, self.power) }

    /// Get the original numeric value before shortening.
    /// 
    /// # Examples
    /// 
    /// ```
    /// extern crate number_shortener;
    /// use number_shortener::ShortNumber;
    /// 
    /// let short_number = ShortNumber::new(1.0, 3.0);
    /// assert_eq!(short_number.get_original_value(), 1000.0);
    /// ```
    #[inline]
    pub fn get_original_value(&self) -> f32 { self.value * BASE.powf(self.power) }

    /// Get the string prefix for the `power` of the `significant_value`
    /// after shortening.
    /// 
    /// # Examples
    /// 
    /// ```
    /// extern crate number_shortener;
    /// use number_shortener::ShortNumber;
    /// 
    /// let short_number = ShortNumber::new(1.0, 3.0);
    /// assert_eq!(short_number.get_prefix(), "K"); // Kilo (1 * 10^3)
    /// ```
    /// 
    /// ```
    /// extern crate number_shortener;
    /// use number_shortener::ShortNumber;
    /// 
    /// let short_number = ShortNumber::new(123.45, 6.0);
    /// assert_eq!(short_number.get_prefix(), "M"); // Mega (1 * 10^6)
    /// ```
    pub fn get_prefix(&self) -> &str {
        let prefixes = get_prefixes();
        let prefix = *prefixes.get(&(self.power as i32)).unwrap_or(&"");
        prefix
    }

    /// Convert this `ShortNumber` to its string representation.
    /// 
    /// This appends the `prefix` to the `significant_value` and returns it 
    /// with at least 1 digit after the decimal.
    /// 
    /// If the number is less than `BASE` (`10`) to the power of `LEAST_EXPONENT` (`3`) i.e `1000`
    /// it returns the number as a string without appending any prefix.
    /// 
    /// # Examples
    /// 
    /// ```
    /// extern crate number_shortener;
    /// use number_shortener::ShortNumber;
    /// 
    /// let short_number = ShortNumber::new(1.0, 3.0);
    /// assert_eq!(short_number.to_string(), "1.0K");
    /// ```
    /// 
    /// ## Precision
    /// 
    /// ```
    /// extern crate number_shortener;
    /// use number_shortener::ShortNumber;
    /// 
    /// let short_number = ShortNumber::new(123.45, 6.0);
    /// assert_eq!(short_number.to_string(), "123.4M");
    /// ```
    /// 
    /// ## Number less than a kilo
    /// 
    /// ```
    /// extern crate number_shortener;
    /// use number_shortener::ShortNumber;
    /// 
    /// let short_number = ShortNumber::new(123.0, 0.0);
    /// assert_eq!(short_number.to_string(), "123.0");
    /// ```
    pub fn to_string(&self) -> String {
        let prefix = self.get_prefix();

        if self.get_original_value() < BASE.powf(LEAST_EXPONENT) {
            return format!("{:.1}", self.value);
        }

        format!("{:.1}{prefix}", self.value, prefix = prefix)
    }
}

impl PartialEq for ShortNumber {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.power == other.power &&
            self.prefix == other.prefix 
    }
}

impl Eq for ShortNumber { }

impl PartialOrd for ShortNumber {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.get_original_value().partial_cmp(&other.get_original_value())
    }
}

impl Ord for ShortNumber {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.get_original_value() > other.get_original_value() {
            Ordering::Greater
        } else if self.get_original_value() < other.get_original_value() {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
}

impl Hash for ShortNumber {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self.value as usize).hash(state);
        (self.power as usize).hash(state);
        self.prefix.hash(state);
    }
}