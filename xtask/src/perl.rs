//! Perl's view of a value `JSON::PP` decoded, as far as the ported Perl gates observe it (`RUSTDOC-AND-XTASK.13.d`,
//! `.13.e`): how a scalar stringifies, which values are references, what is true, and how long `encode_utf8` makes a
//! byte string.

use crate::pyjson::{self, Py};

/// A number as Perl prints it (`%.15g`): fifteen significant digits, scientific below 1e-4 or from 1e15.
pub fn number(number: f64) -> String {
    if number.is_infinite() {
        return if number > 0.0 { "Inf" } else { "-Inf" }.to_string();
    }
    if number.is_nan() {
        return "NaN".to_string();
    }
    if number == 0.0 {
        return "0".to_string();
    }
    let exponent = number.abs().log10().floor() as i32;
    if (-4..15).contains(&exponent) {
        let decimals = (14 - exponent).max(0) as usize;
        let fixed = format!("{number:.decimals$}");
        if fixed.contains('.') {
            fixed
                .trim_end_matches('0')
                .trim_end_matches('.')
                .to_string()
        } else {
            fixed
        }
    } else {
        let scientific = format!("{number:.14e}");
        let (mantissa, power) = scientific.split_once('e').expect("an exponent");
        let mantissa = mantissa.trim_end_matches('0').trim_end_matches('.');
        let power: i32 = power.parse().expect("a power");
        format!(
            "{mantissa}e{}{:02}",
            if power < 0 { '-' } else { '+' },
            power.abs()
        )
    }
}

/// Whether `ref()` is true of the decoded value: arrays, objects, and `JSON::PP`'s blessed booleans.
pub fn is_reference(value: &Py) -> bool {
    matches!(value, Py::List(_) | Py::Dict(_) | Py::Bool(_))
}

/// `"$value"` for a defined value that is not a reference: strings as themselves, numbers as Perl prints them,
/// booleans as `1` and `0`; `None` for `undef` (`null` or absent).
pub fn scalar(value: Option<&Py>) -> Option<String> {
    match value? {
        Py::None => None,
        Py::Bool(flag) => Some(if *flag { "1" } else { "0" }.to_string()),
        Py::Int(digits) => Some(digits.clone()),
        Py::Float(float) => Some(number(*float)),
        Py::Str(text) => Some(pyjson::to_text(text)),
        Py::List(_) | Py::Dict(_) => Some(String::new()),
    }
}

/// The value as a Perl number: what `+ 0` or a numeric comparison makes of it.
pub fn numeric(value: Option<&Py>) -> f64 {
    match value {
        Some(Py::Int(digits)) => digits.parse().unwrap_or(0.0),
        Some(Py::Float(float)) => *float,
        Some(Py::Bool(flag)) => f64::from(u8::from(*flag)),
        Some(Py::Str(text)) => leading_number(&pyjson::to_text(text)),
        _ => 0.0,
    }
}

/// Perl's numification of a string: its leading decimal number, or 0.
fn leading_number(text: &str) -> f64 {
    let text = text.trim_start();
    let mut end = 0;
    let bytes = text.as_bytes();
    if end < bytes.len() && (bytes[end] == b'-' || bytes[end] == b'+') {
        end += 1;
    }
    while end < bytes.len() && (bytes[end].is_ascii_digit() || bytes[end] == b'.') {
        end += 1;
    }
    text[..end].parse().unwrap_or(0.0)
}

/// Perl's truthiness: `undef`, `""`, `"0"`, zero and `JSON::PP::false` are false; references are true.
pub fn truthy(value: Option<&Py>) -> bool {
    match value {
        None | Some(Py::None) => false,
        Some(Py::Bool(flag)) => *flag,
        Some(Py::List(_) | Py::Dict(_)) => true,
        Some(other) => {
            let text = scalar(Some(other)).unwrap_or_default();
            !text.is_empty() && text != "0"
        }
    }
}

/// `length(encode_utf8($bytes))` for a byte string read raw: each byte at or above 0x80 is a Latin-1 character,
/// which UTF-8 writes in two bytes.
pub fn utf8_length(bytes: &[u8]) -> usize {
    bytes.len() + bytes.iter().filter(|byte| **byte >= 0x80).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numbers_print_as_perl_prints_them() {
        for (value, perl) in [
            (1.0, "1"),
            (0.5, "0.5"),
            (1e20, "1e+20"),
            (1e15, "1e+15"),
            (123456789012345.0, "123456789012345"),
            (0.0001, "0.0001"),
            (0.00001, "1e-05"),
        ] {
            assert_eq!(number(value), perl, "{value}");
        }
    }

    #[test]
    fn truth_references_and_lengths_follow_perl() {
        let parse = |text: &str| pyjson::loads(text).expect("value");
        assert!(
            !truthy(Some(&parse("\"0\"")))
                && !truthy(Some(&parse("false")))
                && truthy(Some(&parse("[]")))
        );
        assert!(is_reference(&parse("true")) && !is_reference(&parse("1")));
        assert_eq!(scalar(Some(&parse("1.0"))).as_deref(), Some("1"));
        assert_eq!(numeric(Some(&parse("\"12abc\""))), 12.0);
        assert_eq!(utf8_length("a—".as_bytes()), 1 + 3 + 3);
    }
}
