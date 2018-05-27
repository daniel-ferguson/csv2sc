#[derive(Debug, PartialEq)]
pub enum Field<'a> {
    Text(&'a str),
    Float(f64),
    Int(i64),
}

impl<'a> From<&'a str> for Field<'a> {
    fn from(text: &str) -> Field {
        if text.len() == 0 {
            return Field::Text(text);
        };

        let bytes = text.as_bytes();

        if (bytes[0] == b'-' && bytes[1] >= b'0' && bytes[1] <= b'9')
            || (bytes[0] >= b'0' && bytes[0] <= b'9')
        {
            if text.contains(".") {
                match text.parse::<f64>() {
                    Ok(num) => Field::Float(num),
                    Err(_) => Field::Text(text),
                }
            } else {
                match text.parse::<i64>() {
                    Ok(num) => Field::Int(num),
                    Err(_) => Field::Text(text),
                }
            }
        } else {
            Field::Text(text)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Field;

    #[test]
    fn float() {
        let f: Field = "2.34".into();
        assert_eq!(f, Field::Float(2.34));
    }

    #[test]
    fn int() {
        let f: Field = "22".into();
        assert_eq!(f, Field::Int(22));

        let f: Field = "-22".into();
        assert_eq!(f, Field::Int(-22));
    }

    #[test]
    fn text() {
        let f: Field = "story about doggos".into();
        assert_eq!(f, Field::Text("story about doggos"));
    }
}
