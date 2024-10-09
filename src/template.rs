use std::collections::HashMap;
use std::convert::Into;
use std::fmt::{self, Display, Formatter};

pub struct Template<'a, 'b>(pub String, pub &'a HashMap<&'b str, toml::Value>);

impl<'a, 'b> Into<String> for Template<'a, 'b> {
    fn into(self) -> String {
        format!("{}", &self)
    }
}

enum ParseMode {
    Literal,
    Template(String),
}

impl<'a, 'b> Display for Template<'a, 'b> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut mode = ParseMode::Literal;

        // look at each character one at a time
        for c in self.0.chars() {
            // if we're reading a template name
            if let ParseMode::Template(ref mut name) = mode {
                // and we reach the end of the template
                if c == '}' {
                    // and the name is in the field table
                    if let Some(value) = self.1.get(name.as_str()) {
                        // write it
                        if let toml::Value::String(value_str) = value {
                            write!(f, "{}", value_str)?;
                        }
                        else {
                            write!(f, "{}", value)?;
                        }
                    }
                    // switch back to literal mode
                    mode = ParseMode::Literal;
                }
                // and we haven't yet reached the end
                else {
                    // add the character to the field name
                    name.push(c);
                }
            }
            // if we're in literal mode but detect a field-start token
            else if c == '{' {
                // switch to template mode
                mode = ParseMode::Template(String::new());
            }
            // if we're in literal mode
            else {
                // write the character
                write!(f, "{}", c)?;
            }
        }

        Ok(())
    }
}
