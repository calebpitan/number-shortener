mod short_number;
pub use short_number::{ ShortNumber, LEAST_EXPONENT, BASE, Prefix };

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(super::shorten_number(120450.0).to_string(), "120.4K");
        assert_eq!(super::shorten_number(120.0).to_string(), "120.0");
    }

    #[test]
    fn order_prefix() {
        assert_eq!(super::Prefix::Peta > super::Prefix::Tera, true);
        assert_eq!(super::Prefix::Tera > super::Prefix::Giga, true);
        assert_eq!(super::Prefix::Giga > super::Prefix::Mega, true);
        assert_eq!(super::Prefix::Mega > super::Prefix::Kilo, true);
        assert_eq!(super::Prefix::Kilo > super::Prefix::Unknown, true);
    }
}

pub fn shorten_number(num: f32) -> ShortNumber {
    let exponent = num.log10().floor();
    let power = exponent - (exponent % LEAST_EXPONENT);
    let significant = num / BASE.powf(power);

    ShortNumber::new(significant, power)
}
