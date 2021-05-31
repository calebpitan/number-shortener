use std::cmp::Ordering;
use std::collections::HashMap;

#[inline]
pub fn get_prefixes<'a>() -> HashMap<i32, &'a str> {
    [(3, "K"), (6, "M"), (9, "G"), (12, "T"), (15, "P")]
        .iter()
        .cloned()
        .collect()
}

#[inline]
pub fn get_prefixes_enum() -> HashMap<i32, Prefix> {
    [(3, Prefix::Kilo), (6, Prefix::Mega), (9, Prefix::Giga), (12, Prefix::Tera), (15, Prefix::Peta)]
        .iter()
        .cloned()
        .collect()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Prefix { Kilo, Mega, Giga, Tera, Peta, Unknown }

impl PartialOrd for Prefix {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        struct Weight { self_value: i32, other_value: i32 };
        let prefixes_enum = get_prefixes_enum();
        let weight = prefixes_enum.iter().fold(
            Weight { self_value: 0, other_value: 0 },
            |mut w, (size, pfx)| {
                if pfx == self { 
                    w.self_value = *size;
                    w
                } else if pfx == other {
                    w.other_value = *size;
                    w
                } else {
                    w
                }
            }
        );

        if weight.self_value > weight.other_value {
            Some(Ordering::Greater)
        } else if weight.self_value < weight.other_value {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Equal)
        }
    }
}

impl Ord for Prefix {
    fn cmp(&self, other: &Self) -> Ordering {
        struct Weight { self_value: i32, other_value: i32 };
        let prefixes_enum = get_prefixes_enum();
        let weight = prefixes_enum.iter().fold(
            Weight { self_value: 0, other_value: 0 },
            |mut w, (size, pfx)| {
                if pfx == self { 
                    w.self_value = *size;
                    w
                } else if pfx == other {
                    w.other_value = *size;
                    w
                } else {
                    w
                }
            }
        );

        if weight.self_value > weight.other_value {
            Ordering::Greater
        } else if weight.self_value < weight.other_value {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
}
