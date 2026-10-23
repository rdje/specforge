//! Python's `json` module as the ported gates observe it (`RUSTDOC-AND-XTASK.17`): `json.loads` into a value that
//! keeps key order, and `json.dumps` back out byte for byte. A gate that compared, printed or re-encoded what Python
//! parsed needs the same answers here, so this reproduces CPython 3.14's C scanner, error messages and positions
//! included, and its encoder's `indent`, `sort_keys`, `ensure_ascii` and `separators`, with Python's float `repr`.

use std::cmp::Ordering;

/// A Python string: code points, which may include the lone surrogates `json.loads` keeps.
pub type PyStr = Vec<u32>;

/// The Python object `json.loads` builds.
#[derive(Debug, Clone)]
pub enum Py {
    /// `None`, from `null`.
    None,
    /// `True` or `False`.
    Bool(bool),
    /// An `int`, as its decimal digits (`-0` reads as `0`).
    Int(String),
    /// A `float`, from a number with a fraction or exponent, or `NaN`, `Infinity`, `-Infinity`.
    Float(f64),
    /// A `str`.
    Str(PyStr),
    /// A `list`.
    List(Vec<Py>),
    /// A `dict` in insertion order; a repeated key keeps its first position and takes the last value.
    Dict(Vec<(PyStr, Py)>),
}

/// Python's `sys.get_int_max_str_digits()` default, past which `int()` refuses a decimal string.
const INT_MAX_STR_DIGITS: usize = 4300;

/// The code points of a Rust string.
pub fn py_str(text: &str) -> PyStr {
    text.chars().map(u32::from).collect()
}

/// A Python string as Rust text; a lone surrogate prints as Python's `backslashreplace` writes it.
pub fn to_text(text: &[u32]) -> String {
    text.iter()
        .map(|code| match char::from_u32(*code) {
            Some(character) => character.to_string(),
            None => format!("\\u{code:04x}"),
        })
        .collect()
}

/// `json.loads(text)` for a `str`: the value, or the `ValueError` message Python raises.
pub fn loads(text: &str) -> Result<Py, String> {
    let characters: Vec<char> = text.chars().collect();
    if characters.first() == Some(&'\u{feff}') {
        return Err(decode_error(
            "Unexpected UTF-8 BOM (decode using utf-8-sig)",
            &characters,
            0,
        ));
    }
    let mut scanner = Scanner { text: &characters };
    let start = scanner.skip_whitespace(0);
    let (value, end) = match scanner.scan_once(start) {
        Ok(found) => found,
        Err(Failure::StopIteration(position)) => {
            return Err(decode_error("Expecting value", &characters, position));
        }
        Err(Failure::Decode(message, position)) => {
            return Err(decode_error(message, &characters, position));
        }
        Err(Failure::Value(message)) => return Err(message),
    };
    let end = scanner.skip_whitespace(end);
    if end != characters.len() {
        return Err(decode_error("Extra data", &characters, end));
    }
    Ok(value)
}

/// `json.loads(raw)` for `bytes`: UTF-8, its byte-order mark dropped as `detect_encoding` drops it.
pub fn loads_bytes(raw: &[u8]) -> Result<Py, String> {
    let raw = raw.strip_prefix(b"\xef\xbb\xbf").unwrap_or(raw);
    match std::str::from_utf8(raw) {
        Ok(text) => loads(text),
        Err(error) => Err(format!("'utf-8' codec can't decode bytes: {error}")),
    }
}

/// `JSONDecodeError`'s message: `msg: line L column C (char P)`, P counted in code points.
fn decode_error(message: &str, text: &[char], position: usize) -> String {
    let before = &text[..position.min(text.len())];
    let line = before
        .iter()
        .filter(|character| **character == '\n')
        .count()
        + 1;
    let column = match before.iter().rposition(|character| *character == '\n') {
        Some(newline) => position - newline,
        None => position + 1,
    };
    format!("{message}: line {line} column {column} (char {position})")
}

enum Failure {
    /// The scanner found no value at this position; the caller reports `Expecting value` there.
    StopIteration(usize),
    /// A `JSONDecodeError` with its message and position.
    Decode(&'static str, usize),
    /// Another `ValueError`, reported as its message alone.
    Value(String),
}

struct Scanner<'a> {
    text: &'a [char],
}

impl Scanner<'_> {
    fn at(&self, index: usize) -> Option<char> {
        self.text.get(index).copied()
    }

    fn skip_whitespace(&self, mut index: usize) -> usize {
        while matches!(self.at(index), Some(' ' | '\t' | '\n' | '\r')) {
            index += 1;
        }
        index
    }

    fn matches(&self, index: usize, word: &str) -> bool {
        word.chars()
            .enumerate()
            .all(|(offset, character)| self.at(index + offset) == Some(character))
    }

    fn scan_once(&mut self, index: usize) -> Result<(Py, usize), Failure> {
        let Some(character) = self.at(index) else {
            return Err(Failure::StopIteration(index));
        };
        match character {
            '"' => {
                return self
                    .scan_string(index + 1)
                    .map(|(text, end)| (Py::Str(text), end));
            }
            '{' => return self.parse_object(index + 1),
            '[' => return self.parse_array(index + 1),
            'n' if self.matches(index, "null") => return Ok((Py::None, index + 4)),
            't' if self.matches(index, "true") => return Ok((Py::Bool(true), index + 4)),
            'f' if self.matches(index, "false") => return Ok((Py::Bool(false), index + 5)),
            'N' if self.matches(index, "NaN") => return Ok((Py::Float(f64::NAN), index + 3)),
            'I' if self.matches(index, "Infinity") => {
                return Ok((Py::Float(f64::INFINITY), index + 8));
            }
            '-' if self.matches(index, "-Infinity") => {
                return Ok((Py::Float(f64::NEG_INFINITY), index + 9));
            }
            _ => {}
        }
        self.match_number(index)
    }

    /// The C scanner's `_match_number_unicode`: an optional `-`, `0` or a digit run, then a `.` with digits and an
    /// exponent, each taken only when complete.
    fn match_number(&self, start: usize) -> Result<(Py, usize), Failure> {
        let digit = |index: usize| {
            self.at(index)
                .is_some_and(|character| character.is_ascii_digit())
        };
        let last = self.text.len().wrapping_sub(1);
        let mut index = start;
        if self.at(index) == Some('-') {
            index += 1;
            if index > last || self.text.is_empty() {
                return Err(Failure::StopIteration(start));
            }
        }
        match self.at(index) {
            Some('1'..='9') => {
                index += 1;
                while digit(index) {
                    index += 1;
                }
            }
            Some('0') => index += 1,
            _ => return Err(Failure::StopIteration(start)),
        }
        let mut is_float = false;
        if index < last && self.at(index) == Some('.') && digit(index + 1) {
            is_float = true;
            index += 2;
            while digit(index) {
                index += 1;
            }
        }
        if index < last && matches!(self.at(index), Some('e' | 'E')) {
            let exponent = index;
            index += 1;
            if index < last && matches!(self.at(index), Some('-' | '+')) {
                index += 1;
            }
            while digit(index) {
                index += 1;
            }
            if digit(index - 1) {
                is_float = true;
            } else {
                index = exponent;
            }
        }
        let lexeme: String = self.text[start..index].iter().collect();
        if is_float {
            return Ok((Py::Float(lexeme.parse().unwrap_or(f64::NAN)), index));
        }
        let digits = lexeme.trim_start_matches('-');
        if digits.len() > INT_MAX_STR_DIGITS {
            return Err(Failure::Value(format!(
                "Exceeds the limit ({INT_MAX_STR_DIGITS} digits) for integer string conversion: value has {} digits; use sys.set_int_max_str_digits() to increase the limit",
                digits.len()
            )));
        }
        let int = if digits == "0" {
            "0".to_string()
        } else {
            lexeme
        };
        Ok((Py::Int(int), index))
    }

    /// The C scanner's `scanstring_unicode`, from the character after the opening quote.
    fn scan_string(&self, mut end: usize) -> Result<(PyStr, usize), Failure> {
        let begin = end - 1;
        let length = self.text.len();
        let mut out = PyStr::new();
        loop {
            let mut next = end;
            let mut found = None;
            while next < length {
                let character = self.text[next];
                if character == '"' || character == '\\' {
                    found = Some(character);
                    break;
                }
                if u32::from(character) <= 0x1f {
                    return Err(Failure::Decode("Invalid control character at", next));
                }
                next += 1;
            }
            out.extend(
                self.text[end..next]
                    .iter()
                    .map(|character| u32::from(*character)),
            );
            match found {
                Some('"') => return Ok((out, next + 1)),
                Some(_) => {}
                None => return Err(Failure::Decode("Unterminated string starting at", begin)),
            }
            next += 1;
            let Some(escape) = self.at(next) else {
                return Err(Failure::Decode("Unterminated string starting at", begin));
            };
            if escape != 'u' {
                end = next + 1;
                let decoded = match escape {
                    '"' => '"',
                    '\\' => '\\',
                    '/' => '/',
                    'b' => '\u{8}',
                    'f' => '\u{c}',
                    'n' => '\n',
                    'r' => '\r',
                    't' => '\t',
                    _ => return Err(Failure::Decode("Invalid \\escape", end - 2)),
                };
                out.push(u32::from(decoded));
                continue;
            }
            next += 1;
            end = next + 4;
            if end > length {
                return Err(Failure::Decode("Invalid \\uXXXX escape", next - 1));
            }
            let mut code = self
                .hex4(next)
                .ok_or(Failure::Decode("Invalid \\uXXXX escape", end - 5))?;
            if (0xd800..0xdc00).contains(&code)
                && end + 6 < length
                && self.text[end] == '\\'
                && self.text[end + 1] == 'u'
            {
                let low = self
                    .hex4(end + 2)
                    .ok_or(Failure::Decode("Invalid \\uXXXX escape", end + 1))?;
                if (0xdc00..0xe000).contains(&low) {
                    code = 0x10000 + ((code - 0xd800) << 10) + (low - 0xdc00);
                    end += 6;
                }
            }
            out.push(code);
        }
    }

    fn hex4(&self, start: usize) -> Option<u32> {
        (start..start + 4).try_fold(0u32, |code, index| {
            Some((code << 4) | self.at(index)?.to_digit(16)?)
        })
    }

    fn parse_object(&mut self, start: usize) -> Result<(Py, usize), Failure> {
        let mut pairs: Vec<(PyStr, Py)> = Vec::new();
        let mut index = self.skip_whitespace(start);
        if self.at(index) != Some('}') {
            loop {
                if self.at(index) != Some('"') {
                    return Err(Failure::Decode(
                        "Expecting property name enclosed in double quotes",
                        index,
                    ));
                }
                let (key, next) = self.scan_string(index + 1)?;
                index = self.skip_whitespace(next);
                if self.at(index) != Some(':') {
                    return Err(Failure::Decode("Expecting ':' delimiter", index));
                }
                index = self.skip_whitespace(index + 1);
                let (value, next) = self.scan_once(index)?;
                match pairs.iter_mut().find(|(existing, _)| *existing == key) {
                    Some(pair) => pair.1 = value,
                    None => pairs.push((key, value)),
                }
                index = self.skip_whitespace(next);
                if self.at(index) == Some('}') {
                    break;
                }
                if self.at(index) != Some(',') {
                    return Err(Failure::Decode("Expecting ',' delimiter", index));
                }
                let comma = index;
                index = self.skip_whitespace(index + 1);
                if self.at(index) == Some('}') {
                    return Err(Failure::Decode(
                        "Illegal trailing comma before end of object",
                        comma,
                    ));
                }
            }
        }
        Ok((Py::Dict(pairs), index + 1))
    }

    fn parse_array(&mut self, start: usize) -> Result<(Py, usize), Failure> {
        let mut items = Vec::new();
        let mut index = self.skip_whitespace(start);
        if self.at(index) != Some(']') {
            loop {
                let (value, next) = self.scan_once(index)?;
                items.push(value);
                index = self.skip_whitespace(next);
                if self.at(index) == Some(']') {
                    break;
                }
                if self.at(index) != Some(',') {
                    return Err(Failure::Decode("Expecting ',' delimiter", index));
                }
                let comma = index;
                index = self.skip_whitespace(index + 1);
                if self.at(index) == Some(']') {
                    return Err(Failure::Decode(
                        "Illegal trailing comma before end of array",
                        comma,
                    ));
                }
            }
        }
        Ok((Py::List(items), index + 1))
    }
}

/// The options of one `json.dumps` call.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Encoder {
    /// `indent`: `None` writes one line; a count puts each item on its own line, indented that much per level.
    pub indent: Option<usize>,
    /// `sort_keys`.
    pub sort_keys: bool,
    /// `ensure_ascii`: escape every character outside printable ASCII.
    pub ensure_ascii: bool,
    /// The first of `separators`, between items.
    pub item_separator: &'static str,
    /// The second of `separators`, between a key and its value.
    pub key_separator: &'static str,
}

/// `json.dumps(value, …)` under `encoder`, or `None` where Python's later `.encode()` would fail on a lone
/// surrogate written raw (`ensure_ascii=False`).
pub fn dumps(value: &Py, encoder: &Encoder) -> Option<String> {
    let mut out = String::new();
    write_value(value, encoder, 0, &mut out).then_some(out)
}

fn write_value(value: &Py, encoder: &Encoder, level: usize, out: &mut String) -> bool {
    match value {
        Py::None => out.push_str("null"),
        Py::Bool(true) => out.push_str("true"),
        Py::Bool(false) => out.push_str("false"),
        Py::Int(digits) => out.push_str(digits),
        Py::Float(number) => out.push_str(&float_json(*number)),
        Py::Str(text) => return write_string(text, encoder.ensure_ascii, out),
        Py::List(items) if items.is_empty() => out.push_str("[]"),
        Py::Dict(pairs) if pairs.is_empty() => out.push_str("{}"),
        Py::List(items) => {
            out.push('[');
            let separator = open_level(encoder, level, out);
            for (position, item) in items.iter().enumerate() {
                if position > 0 {
                    out.push_str(&separator);
                }
                if !write_value(item, encoder, level + 1, out) {
                    return false;
                }
            }
            close_level(encoder, level, out);
            out.push(']');
        }
        Py::Dict(pairs) => {
            let mut ordered: Vec<&(PyStr, Py)> = pairs.iter().collect();
            if encoder.sort_keys {
                ordered.sort_by(|left, right| left.0.cmp(&right.0));
            }
            out.push('{');
            let separator = open_level(encoder, level, out);
            for (position, (key, item)) in ordered.into_iter().enumerate() {
                if position > 0 {
                    out.push_str(&separator);
                }
                if !write_string(key, encoder.ensure_ascii, out) {
                    return false;
                }
                out.push_str(encoder.key_separator);
                if !write_value(item, encoder, level + 1, out) {
                    return false;
                }
            }
            close_level(encoder, level, out);
            out.push('}');
        }
    }
    true
}

/// Open a container's items: with an indent, a newline at the next level; returns the separator between items.
fn open_level(encoder: &Encoder, level: usize, out: &mut String) -> String {
    match encoder.indent {
        Some(indent) => {
            let newline = format!("\n{}", " ".repeat(indent * (level + 1)));
            out.push_str(&newline);
            format!("{}{newline}", encoder.item_separator)
        }
        None => encoder.item_separator.to_string(),
    }
}

fn close_level(encoder: &Encoder, level: usize, out: &mut String) {
    if let Some(indent) = encoder.indent {
        out.push('\n');
        out.push_str(&" ".repeat(indent * level));
    }
}

/// A string as `py_encode_basestring_ascii` or `py_encode_basestring` writes it.
fn write_string(text: &[u32], ensure_ascii: bool, out: &mut String) -> bool {
    out.push('"');
    for &code in text {
        match code {
            0x22 => out.push_str("\\\""),
            0x5c => out.push_str("\\\\"),
            0x0a => out.push_str("\\n"),
            0x0d => out.push_str("\\r"),
            0x09 => out.push_str("\\t"),
            0x08 => out.push_str("\\b"),
            0x0c => out.push_str("\\f"),
            0x00..=0x1f => out.push_str(&format!("\\u{code:04x}")),
            0x20..=0x7e => out.push(char::from_u32(code).expect("ASCII")),
            _ if ensure_ascii => {
                if code >= 0x10000 {
                    let value = code - 0x10000;
                    out.push_str(&format!(
                        "\\u{:04x}\\u{:04x}",
                        0xd800 + (value >> 10),
                        0xdc00 + (value & 0x3ff)
                    ));
                } else {
                    out.push_str(&format!("\\u{code:04x}"));
                }
            }
            _ => match char::from_u32(code) {
                Some(character) => out.push(character),
                None => return false,
            },
        }
    }
    out.push('"');
    true
}

/// A float as `json.dumps` writes it: `NaN`, `Infinity` and `-Infinity` for the non-finite, otherwise `repr`.
fn float_json(number: f64) -> String {
    if number.is_nan() {
        "NaN".to_string()
    } else if number.is_infinite() {
        if number > 0.0 {
            "Infinity"
        } else {
            "-Infinity"
        }
        .to_string()
    } else {
        float_repr(number)
    }
}

/// Python's `repr(float)`: the shortest digits that round-trip, in fixed notation unless the decimal exponent is
/// below -4 or above 16, where it is scientific with a signed, at least two-digit exponent.
pub fn float_repr(number: f64) -> String {
    if number.is_nan() {
        return "nan".to_string();
    }
    if number.is_infinite() {
        return if number > 0.0 { "inf" } else { "-inf" }.to_string();
    }
    let sign = if number.is_sign_negative() { "-" } else { "" };
    let scientific = format!("{:e}", number.abs());
    let (mantissa, exponent) = scientific
        .split_once('e')
        .expect("LowerExp writes an exponent");
    let digits: String = mantissa
        .chars()
        .filter(|character| *character != '.')
        .collect();
    let point = exponent.parse::<i32>().expect("a decimal exponent") + 1;
    let count = digits.len() as i32;
    let body = if point <= -4 || point > 16 {
        let rest = &digits[1..];
        let exponent = point - 1;
        format!(
            "{}{}{}e{}{:02}",
            &digits[..1],
            if rest.is_empty() { "" } else { "." },
            rest,
            if exponent < 0 { '-' } else { '+' },
            exponent.abs()
        )
    } else if point <= 0 {
        format!("0.{}{digits}", "0".repeat((-point) as usize))
    } else if point >= count {
        format!("{digits}{}.0", "0".repeat((point - count) as usize))
    } else {
        format!(
            "{}.{}",
            &digits[..point as usize],
            &digits[point as usize..]
        )
    };
    format!("{sign}{body}")
}

/// Python's `==` between two parsed values: `1 == 1.0 == True`, `NaN` equal to nothing, a `dict` equal to another
/// with the same keys and equal values in any order.
pub fn py_eq(left: &Py, right: &Py) -> bool {
    match (left, right) {
        (Py::None, Py::None) => true,
        (Py::Str(left), Py::Str(right)) => left == right,
        (Py::List(left), Py::List(right)) => {
            left.len() == right.len()
                && left
                    .iter()
                    .zip(right)
                    .all(|(left, right)| py_eq(left, right))
        }
        (Py::Dict(left), Py::Dict(right)) => {
            left.len() == right.len()
                && left.iter().all(|(key, value)| {
                    right
                        .iter()
                        .any(|(other, item)| other == key && py_eq(value, item))
                })
        }
        (left, right) => match (number_of(left), number_of(right)) {
            (Some(left), Some(right)) => compare_numbers(&left, &right) == Some(Ordering::Equal),
            _ => false,
        },
    }
}

enum Number {
    Int(String),
    Float(f64),
}

fn number_of(value: &Py) -> Option<Number> {
    match value {
        Py::Bool(flag) => Some(Number::Int(if *flag { "1" } else { "0" }.to_string())),
        Py::Int(digits) => Some(Number::Int(digits.clone())),
        Py::Float(number) => Some(Number::Float(*number)),
        _ => None,
    }
}

/// Exact comparison, as Python compares an `int` with a `float`: equal only when the float is that integer.
fn compare_numbers(left: &Number, right: &Number) -> Option<Ordering> {
    match (left, right) {
        (Number::Float(left), Number::Float(right)) => left.partial_cmp(right),
        (Number::Int(left), Number::Int(right)) => (left == right).then_some(Ordering::Equal),
        (Number::Int(int), Number::Float(float)) | (Number::Float(float), Number::Int(int)) => {
            let integral = float.is_finite() && float.fract() == 0.0;
            let digits = if *float == 0.0 {
                "0".to_string()
            } else {
                format!("{float:.0}")
            };
            (integral && digits == *int).then_some(Ordering::Equal)
        }
    }
}

/// Python's truthiness: `None`, `False`, zero, and empty strings and containers are false.
pub fn truthy(value: &Py) -> bool {
    match value {
        Py::None => false,
        Py::Bool(flag) => *flag,
        Py::Int(digits) => digits != "0",
        Py::Float(number) => *number != 0.0,
        Py::Str(text) => !text.is_empty(),
        Py::List(items) => !items.is_empty(),
        Py::Dict(pairs) => !pairs.is_empty(),
    }
}

/// `dict.get(key)` on a parsed `dict`; `None` for any other value or a missing key.
pub fn get<'a>(value: &'a Py, key: &str) -> Option<&'a Py> {
    let Py::Dict(pairs) = value else {
        return None;
    };
    let key = py_str(key);
    pairs
        .iter()
        .find(|(name, _)| *name == key)
        .map(|(_, item)| item)
}

/// Python's `repr()` of a parsed value.
pub fn repr(value: &Py) -> String {
    match value {
        Py::None => "None".to_string(),
        Py::Bool(true) => "True".to_string(),
        Py::Bool(false) => "False".to_string(),
        Py::Int(digits) => digits.clone(),
        Py::Float(number) => float_repr(*number),
        Py::Str(text) => string_repr(text),
        Py::List(items) => format!(
            "[{}]",
            items.iter().map(repr).collect::<Vec<_>>().join(", ")
        ),
        Py::Dict(pairs) => format!(
            "{{{}}}",
            pairs
                .iter()
                .map(|(key, item)| format!("{}: {}", string_repr(key), repr(item)))
                .collect::<Vec<_>>()
                .join(", ")
        ),
    }
}

/// Python's `str()` of a parsed value: a string as itself, anything else as its `repr()`.
pub fn str_of(value: &Py) -> String {
    match value {
        Py::Str(text) => to_text(text),
        other => repr(other),
    }
}

/// Python's `repr()` of a string: single quotes unless it holds one and no double quote.
pub fn string_repr(text: &[u32]) -> String {
    let has_single = text.contains(&0x27);
    let quote = if has_single && !text.contains(&0x22) {
        '"'
    } else {
        '\''
    };
    let mut out = String::from(quote);
    for &code in text {
        match char::from_u32(code) {
            Some('\\') => out.push_str("\\\\"),
            Some('\n') => out.push_str("\\n"),
            Some('\r') => out.push_str("\\r"),
            Some('\t') => out.push_str("\\t"),
            Some(character) if character == quote => {
                out.push('\\');
                out.push(character);
            }
            Some(character)
                if (character as u32) < 0x20 || (0x7f..0xa1).contains(&(character as u32)) =>
            {
                out.push_str(&format!("\\x{:02x}", character as u32))
            }
            Some(character) => out.push(character),
            None => out.push_str(&format!("\\u{code:04x}")),
        }
    }
    out.push(quote);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    const COMPACT: Encoder = Encoder {
        indent: None,
        sort_keys: false,
        ensure_ascii: true,
        item_separator: ",",
        key_separator: ":",
    };

    /// `json.loads` on each input under CPython 3.14.7, re-encoded compactly or with its error message
    /// (`.project-data/tmp/rdx-17/truth.py`).
    const LOADS: &[(&str, Result<&str, &str>)] = &[
        ("", Err("Expecting value: line 1 column 1 (char 0)")),
        (" ", Err("Expecting value: line 1 column 2 (char 1)")),
        (
            "{",
            Err("Expecting property name enclosed in double quotes: line 1 column 2 (char 1)"),
        ),
        ("[", Err("Expecting value: line 1 column 2 (char 1)")),
        (
            "\"",
            Err("Unterminated string starting at: line 1 column 1 (char 0)"),
        ),
        (
            "\"abc",
            Err("Unterminated string starting at: line 1 column 1 (char 0)"),
        ),
        (
            "{\"a\"",
            Err("Expecting ':' delimiter: line 1 column 5 (char 4)"),
        ),
        ("{\"a\":", Err("Expecting value: line 1 column 6 (char 5)")),
        (
            "{\"a\": 1",
            Err("Expecting ',' delimiter: line 1 column 8 (char 7)"),
        ),
        (
            "{\"a\": 1,",
            Err("Expecting property name enclosed in double quotes: line 1 column 9 (char 8)"),
        ),
        (
            "{\"a\": 1,}",
            Err("Illegal trailing comma before end of object: line 1 column 8 (char 7)"),
        ),
        (
            "[1,]",
            Err("Illegal trailing comma before end of array: line 1 column 3 (char 2)"),
        ),
        (
            "[1 2]",
            Err("Expecting ',' delimiter: line 1 column 4 (char 3)"),
        ),
        (
            "{\"a\" 1}",
            Err("Expecting ':' delimiter: line 1 column 6 (char 5)"),
        ),
        (
            "{1: 2}",
            Err("Expecting property name enclosed in double quotes: line 1 column 2 (char 1)"),
        ),
        ("[1,", Err("Expecting value: line 1 column 4 (char 3)")),
        ("nul", Err("Expecting value: line 1 column 1 (char 0)")),
        ("tru", Err("Expecting value: line 1 column 1 (char 0)")),
        ("fals", Err("Expecting value: line 1 column 1 (char 0)")),
        ("-", Err("Expecting value: line 1 column 1 (char 0)")),
        ("-a", Err("Expecting value: line 1 column 1 (char 0)")),
        ("01", Err("Extra data: line 1 column 2 (char 1)")),
        ("1.", Err("Extra data: line 1 column 2 (char 1)")),
        ("1e", Err("Extra data: line 1 column 2 (char 1)")),
        ("1e+", Err("Extra data: line 1 column 2 (char 1)")),
        (".5", Err("Expecting value: line 1 column 1 (char 0)")),
        ("\"\\x\"", Err("Invalid \\escape: line 1 column 2 (char 1)")),
        (
            "\"\\u12\"",
            Err("Invalid \\uXXXX escape: line 1 column 3 (char 2)"),
        ),
        (
            "\"\\u12G4\"",
            Err("Invalid \\uXXXX escape: line 1 column 3 (char 2)"),
        ),
        (
            "\"\\ud800\\u12G4\"",
            Err("Invalid \\uXXXX escape: line 1 column 9 (char 8)"),
        ),
        (
            "\"a\nb\"",
            Err("Invalid control character at: line 1 column 3 (char 2)"),
        ),
        (
            "\"a\tb\"",
            Err("Invalid control character at: line 1 column 3 (char 2)"),
        ),
        ("[1] x", Err("Extra data: line 1 column 5 (char 4)")),
        ("{\"a\": 1} {", Err("Extra data: line 1 column 10 (char 9)")),
        (
            "\u{feff}{}",
            Err("Unexpected UTF-8 BOM (decode using utf-8-sig): line 1 column 1 (char 0)"),
        ),
        ("NaN x", Err("Extra data: line 1 column 5 (char 4)")),
        ("[-Infinity, NaN, Infinity]", Ok("[-Infinity,NaN,Infinity]")),
        ("{\"a\":1,\"a\":2}", Ok("{\"a\":2}")),
        ("\"\\ud83d\\ude00\"", Ok("\"\\ud83d\\ude00\"")),
        ("\"\\ud800\"", Ok("\"\\ud800\"")),
        ("\"\\udc00x\"", Ok("\"\\udc00x\"")),
        (
            "[[[]]",
            Err("Expecting ',' delimiter: line 1 column 6 (char 5)"),
        ),
        (
            "{\"a\": [1, {\"b\": }]}",
            Err("Expecting value: line 1 column 17 (char 16)"),
        ),
        (
            "  \n  {\"a\":\n  x}",
            Err("Expecting value: line 3 column 3 (char 13)"),
        ),
        (
            "\"\\u0041",
            Err("Unterminated string starting at: line 1 column 1 (char 0)"),
        ),
        (
            "\"\\",
            Err("Unterminated string starting at: line 1 column 1 (char 0)"),
        ),
        ("[1.5e400]", Ok("[Infinity]")),
        (
            "[-0, -0.0, 1E2, 1.50, 0.1, 123456789012345678901234567890]",
            Ok("[0,-0.0,100.0,1.5,0.1,123456789012345678901234567890]"),
        ),
        ("true", Ok("true")),
        ("null ", Ok("null")),
        (" false\n", Ok("false")),
        ("\"\\/\"", Ok("\"/\"")),
    ];

    #[test]
    fn loads_matches_python_on_every_case() {
        for (input, expected) in LOADS {
            let found = loads(input).map(|value| dumps(&value, &COMPACT).expect("encodable"));
            let found = found.as_deref().map_err(String::as_str);
            assert_eq!(found, *expected, "{input:?}");
        }
        let long = "1".repeat(4301);
        assert_eq!(
            loads(&long).unwrap_err(),
            "Exceeds the limit (4300 digits) for integer string conversion: value has 4301 digits; use sys.set_int_max_str_digits() to increase the limit"
        );
    }

    #[test]
    fn float_repr_matches_python() {
        for (number, expected) in [
            (0.1, "0.1"),
            (1e16, "1e+16"),
            (1e15, "1000000000000000.0"),
            (0.0001, "0.0001"),
            (0.00001, "1e-05"),
            (-0.0, "-0.0"),
            (1.5e-7, "1.5e-07"),
            (1e100, "1e+100"),
            (123.456, "123.456"),
            (2f64.powi(70), "1.1805916207174113e+21"),
            (1.0 / 3.0, "0.3333333333333333"),
            (5e-324, "5e-324"),
            (f64::MAX, "1.7976931348623157e+308"),
            (100.0, "100.0"),
            (1e22, "1e+22"),
            (12345678.9, "12345678.9"),
        ] {
            assert_eq!(float_repr(number), expected, "{number:e}");
        }
    }

    #[test]
    fn dumps_matches_python_across_its_options() {
        let value = loads("{\"b\": 1, \"a\": [1, 2.5, null, true, \"x\\u00e9\\ud83d\\ude00\\u007f\\u0001\"], \"k\": {\"z\": 1, \"y\": [{}]}}")
            .expect("value");
        let cases: [(Encoder, &str); 4] = [
            (
                Encoder {
                    indent: None,
                    sort_keys: true,
                    ensure_ascii: true,
                    item_separator: ",",
                    key_separator: ": ",
                },
                "{\"a\": [1,2.5,null,true,\"x\\u00e9\\ud83d\\ude00\\u007f\\u0001\"],\"b\": 1,\"k\": {\"y\": [{}],\"z\": 1}}",
            ),
            (
                Encoder {
                    indent: None,
                    sort_keys: false,
                    ensure_ascii: false,
                    item_separator: ", ",
                    key_separator: ": ",
                },
                "{\"b\": 1, \"a\": [1, 2.5, null, true, \"x\u{e9}\u{1f600}\u{7f}\\u0001\"], \"k\": {\"z\": 1, \"y\": [{}]}}",
            ),
            (
                Encoder {
                    indent: Some(2),
                    sort_keys: true,
                    ensure_ascii: true,
                    item_separator: ",",
                    key_separator: " : ",
                },
                "{\n  \"a\" : [\n    1,\n    2.5,\n    null,\n    true,\n    \"x\\u00e9\\ud83d\\ude00\\u007f\\u0001\"\n  ],\n  \"b\" : 1,\n  \"k\" : {\n    \"y\" : [\n      {}\n    ],\n    \"z\" : 1\n  }\n}",
            ),
            (
                Encoder {
                    indent: Some(3),
                    sort_keys: false,
                    ensure_ascii: true,
                    item_separator: ",",
                    key_separator: ": ",
                },
                "{\n   \"b\": 1,\n   \"a\": [\n      1,\n      2.5,\n      null,\n      true,\n      \"x\\u00e9\\ud83d\\ude00\\u007f\\u0001\"\n   ],\n   \"k\": {\n      \"z\": 1,\n      \"y\": [\n         {}\n      ]\n   }\n}",
            ),
        ];
        for (encoder, expected) in cases {
            assert_eq!(
                dumps(&value, &encoder).as_deref(),
                Some(expected),
                "{encoder:?}"
            );
        }
        let lone = loads("\"\\ud800\"").expect("lone surrogate");
        assert_eq!(dumps(&lone, &COMPACT).as_deref(), Some("\"\\ud800\""));
        assert_eq!(
            dumps(
                &lone,
                &Encoder {
                    ensure_ascii: false,
                    ..COMPACT
                }
            ),
            None
        );
    }

    /// FNV-1a over bytes, which the corpus writer computes the same way, so a long `dumps` output compares as one
    /// number.
    fn fnv1a(bytes: &[u8]) -> u64 {
        bytes.iter().fold(0xcbf2_9ce4_8422_2325, |hash, byte| {
            (hash ^ u64::from(*byte)).wrapping_mul(0x0000_0100_0000_01b3)
        })
    }

    /// The differential harness: with `XTASK_PYJSON_CORPUS` naming a file of JSON lines written by Python
    /// (`.project-data/tmp/rdx-17/corpus.py`), every case must agree. A `loads` case carries `input` and Python's
    /// `expected` outcome (`OK:` and the compact re-encoding, or `ERR:` and the message); a `dumps` case carries
    /// `input`, the `encoder` as `[indent, sort_keys, ensure_ascii, item_separator, key_separator]` and the FNV-1a of
    /// Python's output, or `ERR` when its `.encode()` fails. Run with `cargo test -p xtask -- --ignored`.
    #[test]
    #[ignore]
    fn agrees_with_python_on_a_corpus() {
        let Ok(path) = std::env::var("XTASK_PYJSON_CORPUS") else {
            return;
        };
        let corpus = std::fs::read_to_string(path).expect("corpus");
        let text = |value: &Py, key: &str| match get(value, key) {
            Some(Py::Str(text)) => to_text(text),
            other => panic!("{key}: {other:?}"),
        };
        let (mut cases, mut failures) = (0, Vec::new());
        for line in corpus.lines() {
            let case = loads(line).expect("corpus line");
            let input = text(&case, "input");
            let (expected, found) = match get(&case, "encoder") {
                None => {
                    let found = match loads(&input) {
                        Ok(value) => format!("OK:{}", dumps(&value, &COMPACT).expect("encodable")),
                        Err(message) => format!("ERR:{message}"),
                    };
                    (text(&case, "expected"), found)
                }
                Some(Py::List(options)) => {
                    let indent = match &options[0] {
                        Py::Int(digits) => Some(digits.parse().expect("indent")),
                        _ => None,
                    };
                    let separator = |index: usize| -> &'static str {
                        match &options[index] {
                            Py::Str(text) => Box::leak(to_text(text).into_boxed_str()),
                            other => panic!("separator {other:?}"),
                        }
                    };
                    let encoder = Encoder {
                        indent,
                        sort_keys: truthy(&options[1]),
                        ensure_ascii: truthy(&options[2]),
                        item_separator: separator(3),
                        key_separator: separator(4),
                    };
                    let value = loads(&input).expect("a dumps case parses");
                    let found = dumps(&value, &encoder)
                        .map_or("ERR".to_string(), |out| fnv1a(out.as_bytes()).to_string());
                    (text(&case, "expected"), found)
                }
                Some(other) => panic!("encoder {other:?}"),
            };
            cases += 1;
            if expected != found {
                failures.push(format!("{input:?}: python {expected:?}, rust {found:?}"));
            }
        }
        assert!(
            failures.is_empty(),
            "{} of {cases} disagree:\n{}",
            failures.len(),
            failures[..failures.len().min(20)].join("\n")
        );
        println!("pyjson: {cases} corpus cases agree with Python");
    }

    #[test]
    fn python_equality_truthiness_and_repr() {
        let parse = |text: &str| loads(text).expect("value");
        assert!(py_eq(&parse("1"), &parse("1.0")) && py_eq(&parse("1"), &parse("true")));
        assert!(!py_eq(&parse("\"1\""), &parse("1")) && !py_eq(&parse("NaN"), &parse("NaN")));
        assert!(!py_eq(
            &parse("9007199254740993"),
            &parse("9007199254740992.0")
        ));
        assert!(py_eq(&parse("0"), &parse("-0.0")));
        assert!(py_eq(
            &parse("{\"a\": 1, \"b\": [2]}"),
            &parse("{\"b\": [2.0], \"a\": true}")
        ));
        assert!(!truthy(&parse("0")) && !truthy(&parse("\"\"")) && truthy(&parse("[0]")));
        assert_eq!(
            repr(&parse("[null, true, \"it's\", 1.5, {\"k\": \"v\"}]")),
            "[None, True, \"it's\", 1.5, {'k': 'v'}]"
        );
        assert_eq!(str_of(&parse("\"plain\"")), "plain");
    }
}
