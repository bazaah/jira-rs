use {
    serde::{Serialize, Serializer},
    smol_str::SmolStr,
    std::{
        fmt,
        iter::{Extend, FromIterator},
    },
};

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct CommaDelimited {
    buffer: String,
}

impl CommaDelimited {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn as_str(&self) -> &str {
        self.buffer.as_str()
    }

    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    pub fn append(&mut self, elem: impl AsRef<Element>) {
        self.write_delimited(elem.as_ref())
    }

    pub fn append_elements<I, T>(&mut self, iter: I)
    where
        I: Iterator<Item = T>,
        T: Into<Element>,
    {
        self.extend(iter.map(Into::into))
    }

    pub fn with<F>(self, f: F) -> Self
    where
        F: FnOnce(&mut Self),
    {
        let mut this = self;
        f(&mut this);
        this
    }

    fn write_delimited(&mut self, e: &Element) {
        use fmt::Write;
        if !self.buffer.is_empty() {
            self.buffer
                .write_fmt(format_args!(",{}", e))
                .expect("Display impl should never return an error")
        } else {
            self.buffer
                .write_fmt(format_args!("{}", e))
                .expect("Display impl should never return an error")
        }
    }
}

impl<T> From<T> for CommaDelimited where T: Into<Element> {
    fn from(elem: T) -> Self {
        Self::new().with(|this| this.append(elem.into()))
    }
}

impl Serialize for CommaDelimited {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.buffer.as_str())
    }
}

impl<A> FromIterator<A> for CommaDelimited
where
    A: AsRef<Element>,
{
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        iter.into_iter().fold(Self::default(), |acc, elem| {
            acc.with(|this: &mut Self| this.append(elem))
        })
    }
}

impl<A> Extend<A> for CommaDelimited
where
    A: AsRef<Element>,
{
    fn extend<T: IntoIterator<Item = A>>(&mut self, iter: T) {
        for elem in iter.into_iter() {
            self.append(elem)
        }
    }
}

#[derive(Debug, Clone)]
pub enum Element {
    Text(SmolStr),
    Number(Number),
    Boolean(bool),
}

impl Element {
    pub fn from_float(f: f64) -> Option<Self> {
        Some(Self::Number(Number::from_float(f)?))
    }
}

impl AsRef<Element> for Element {
    fn as_ref(&self) -> &Element {
        self
    }
}

impl fmt::Display for Element {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use Element::{Boolean, Number, Text};
        match self {
            Text(s) => fmt::Display::fmt(s.as_str(), fmt),
            Number(n) => fmt::Display::fmt(&n, fmt),
            Boolean(b) => fmt::Display::fmt(&b, fmt),
        }
    }
}

impl Serialize for Element {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use Element::{Boolean, Number, Text};
        match self {
            Text(s) => serializer.serialize_str(s.as_str()),
            Number(n) => n.serialize(serializer),
            Boolean(b) => serializer.serialize_bool(*b),
        }
    }
}

impl From<&str> for Element {
    fn from(s: &str) -> Self {
        Self::Text(SmolStr::new(s))
    }
}

impl From<String> for Element {
    fn from(s: String) -> Self {
        Self::Text(SmolStr::new(s))
    }
}

impl From<usize> for Element {
    fn from(n: usize) -> Self {
        Self::Number(Number::from(n))
    }
}

impl From<u8> for Element {
    fn from(n: u8) -> Self {
        Self::Number(Number::from(n))
    }
}

impl From<u16> for Element {
    fn from(n: u16) -> Self {
        Self::Number(Number::from(n))
    }
}

impl From<u32> for Element {
    fn from(n: u32) -> Self {
        Self::Number(Number::from(n))
    }
}

impl From<u64> for Element {
    fn from(n: u64) -> Self {
        Self::Number(Number::from(n))
    }
}

impl From<isize> for Element {
    fn from(n: isize) -> Self {
        Self::Number(Number::from(n))
    }
}

impl From<i8> for Element {
    fn from(n: i8) -> Self {
        Self::Number(Number::from(n))
    }
}

impl From<i16> for Element {
    fn from(n: i16) -> Self {
        Self::Number(Number::from(n))
    }
}

impl From<i32> for Element {
    fn from(n: i32) -> Self {
        Self::Number(Number::from(n))
    }
}

impl From<i64> for Element {
    fn from(n: i64) -> Self {
        Self::Number(Number::from(n))
    }
}

impl From<bool> for Element {
    fn from(b: bool) -> Self {
        Self::Boolean(b)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Number {
    // Zero or positive
    Positive(u64),
    // Always negative
    Negative(i64),
    // Always a normal number
    Float(f64),
}

impl Number {
    /// Check with the given float is a valid number
    pub fn valid_float(f: f64) -> bool {
        f.is_finite()
    }

    pub fn from_float(f: f64) -> Option<Self> {
        if Self::valid_float(f) {
            Some(Self::Float(f))
        } else {
            None
        }
    }
}

impl Default for Number {
    fn default() -> Self {
        Self::Positive(0)
    }
}

impl fmt::Display for Number {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use Number::{Float, Negative, Positive};
        match self {
            Positive(u) => fmt::Display::fmt(&u, fmt),
            Negative(i) => fmt::Display::fmt(&i, fmt),
            Float(f) => fmt::Display::fmt(&f, fmt),
        }
    }
}

impl Serialize for Number {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use Number::{Float, Negative, Positive};
        match self {
            Positive(u) => serializer.serialize_u64(*u),
            Negative(i) => serializer.serialize_i64(*i),
            Float(f) => serializer.serialize_f64(*f),
        }
    }
}

impl From<usize> for Number {
    fn from(n: usize) -> Self {
        Self::Positive(n as u64)
    }
}

impl From<u8> for Number {
    fn from(n: u8) -> Self {
        Self::Positive(n as u64)
    }
}

impl From<u16> for Number {
    fn from(n: u16) -> Self {
        Self::Positive(n as u64)
    }
}

impl From<u32> for Number {
    fn from(n: u32) -> Self {
        Self::Positive(n as u64)
    }
}

impl From<u64> for Number {
    fn from(n: u64) -> Self {
        Self::Positive(n)
    }
}

impl From<isize> for Number {
    fn from(n: isize) -> Self {
        Self::Negative(n as i64)
    }
}

impl From<i8> for Number {
    fn from(n: i8) -> Self {
        Self::Negative(n as i64)
    }
}

impl From<i16> for Number {
    fn from(n: i16) -> Self {
        Self::Negative(n as i64)
    }
}

impl From<i32> for Number {
    fn from(n: i32) -> Self {
        Self::Negative(n as i64)
    }
}

impl From<i64> for Number {
    fn from(n: i64) -> Self {
        Self::Negative(n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn single() {
        let delim = CommaDelimited::from("hello");

        assert_eq!(delim.as_str(), "hello");
    }

    #[test]
    fn multiple() {
        let items: &[Element] = &["hello".into(), 42.into(), false.into(), "world".into()];
        let delim = CommaDelimited::from_iter(items);

        assert_eq!(delim.as_str(), "hello,42,false,world");
    }

    #[test]
    fn urlencoded_none() {
        let s = TestSer::new(CommaDelimited::default(), None);
        let req = generate(s).build().expect("a valid request");

        assert_eq!(req.url().query(), None);
    }

    #[test]
    fn urlencoded_single() {
        let s = TestSer::new(CommaDelimited::from(42), None);
        let req = generate(s).build().expect("a valid request");
        let query = req.url().query().expect("a non-empty query");

        assert_eq!(query, "key=42");
    }

    #[test]
    fn urlencoded_delimited() {
        let items: &[Element] = &["hello".into(), 42.into(), false.into(), "world".into()];
        let s = TestSer::new(CommaDelimited::from_iter(items), None);
        let req = generate(s).build().expect("a valid request");
        let query = req.url().query().expect("a non-empty query");

        assert_eq!(query, "key=hello%2C42%2Cfalse%2Cworld");
    }

    #[test]
    fn urlencoded_multiple() {
        let items: &[Element] = &["hello".into(), 42.into(), false.into(), "world".into()];
        let s = TestSer::new(CommaDelimited::from_iter(items), CommaDelimited::from("another"));
        let req = generate(s).build().expect("a valid request");
        let query = req.url().query().expect("a non-empty query");

        assert_eq!(query, "key=hello%2C42%2Cfalse%2Cworld&opt=another");
    }

    #[derive(Serialize)]
    struct TestSer {
        #[serde(skip_serializing_if = "CommaDelimited::is_empty")]
        key: CommaDelimited,
        #[serde(skip_serializing_if = "Option::is_none")]
        opt: Option<CommaDelimited>
    }

    impl TestSer {
        fn new(key: CommaDelimited, opt: impl Into<Option<CommaDelimited>>) -> Self {
            Self { key, opt: opt.into() }
        }
    }

    fn generate(s: impl Serialize) -> reqwest::RequestBuilder {
        reqwest::Client::new().get("http://localhost").query(&s)
    }

}
