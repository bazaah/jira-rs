use {
    smol_str::SmolStr,
    std::{borrow::Borrow, fmt, ops::Deref},
};

/// Small str focused implementation of Cow, using `SmolStr`
/// as the owned type. Used internally as representation of
/// the many small str components in many of the structures
pub enum SmolCow<'a, B>
where
    B: 'a + ToSmol + ?Sized,
{
    Borrowed(&'a B),
    Owned(<B as ToSmol>::Owned),
}

impl<B> SmolCow<'_, B>
where
    B: ToSmol + ?Sized,
{
    pub fn to_smol(self) -> <B as ToSmol>::Owned {
        match self {
            Self::Borrowed(b) => b.to_smol(),
            Self::Owned(o) => o,
        }
    }

    pub fn as_mut(&mut self) -> &mut <B as ToSmol>::Owned {
        match self {
            Self::Borrowed(b) => {
                *self = Self::Owned(b.to_smol());
                match *self {
                    Self::Borrowed(_) => unreachable!(),
                    Self::Owned(ref mut o) => o,
                }
            }
            Self::Owned(ref mut o) => o,
        }
    }

    pub fn is_owned(&self) -> bool {
        !self.is_borrowed()
    }

    pub fn is_borrowed(&self) -> bool {
        match self {
            Self::Borrowed(_) => true,
            Self::Owned(_) => false,
        }
    }
}

impl<'a> SmolCow<'a, str> {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Borrowed(b) => b,
            Self::Owned(o) => o.as_str(),
        }
    }
}

impl<'a> From<&'a str> for SmolCow<'a, str> {
    fn from(s: &'a str) -> Self {
        SmolCow::Borrowed(s)
    }
}

impl<'a> From<&'a String> for SmolCow<'a, str> {
    fn from(s: &'a String) -> Self {
        SmolCow::Borrowed(s.as_str())
    }
}

impl From<SmolStr> for SmolCow<'_, str> {
    fn from(s: SmolStr) -> Self {
        Self::Owned(s)
    }
}

impl Clone for SmolCow<'_, str> {
    fn clone(&self) -> Self {
        self.to_owned()
    }
}

impl Default for SmolCow<'_, str> {
    fn default() -> Self {
        Self::Owned(SmolStr::default())
    }
}

impl fmt::Debug for SmolCow<'_, str> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Borrowed(ref b) => fmt::Debug::fmt(b, f),
            Self::Owned(ref o) => fmt::Debug::fmt(o, f),
        }
    }
}

impl Deref for SmolCow<'_, str> {
    type Target = str;

    fn deref(&self) -> &str {
        match *self {
            Self::Borrowed(borrowed) => borrowed,
            Self::Owned(ref owned) => owned.borrow(),
        }
    }
}

impl AsRef<str> for SmolCow<'_, str> {
    fn as_ref(&self) -> &str {
        self
    }
}

impl Borrow<str> for SmolCow<'_, str> {
    fn borrow(&self) -> &str {
        &*self
    }
}

pub trait ToSmol {
    type Owned: Sized;
    fn to_smol(&self) -> Self::Owned;
}

impl ToSmol for str {
    type Owned = SmolStr;
    fn to_smol(&self) -> Self::Owned {
        SmolStr::new(self)
    }
}
