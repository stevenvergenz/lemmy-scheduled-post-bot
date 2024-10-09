use std::fmt::{self, Display, Formatter};

pub struct OptionString<'a>(&'a Option<String>);
impl<'a> Display for OptionString<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "'{}'", self.0.as_ref().map(|s| s.as_str()).unwrap_or(""))
    }
}

pub fn fos(str: &Option<String>) -> OptionString {
    OptionString(str)
}
