pub struct Cursor<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> Cursor<'a> {
    pub fn new(input: &'a str) -> Self {
        Cursor { input, pos: 0 }
    }

    pub(crate) fn bump(&mut self) -> char {
        let char = self.input.chars().nth(self.pos).unwrap_or('\0');
        self.pos += 1;
        char
    }

    pub(crate) fn first(&self) -> char {
        self.input.chars().nth(self.pos).unwrap_or('\0')
    }

    pub(crate) fn second(&self) -> char {
        self.input.chars().nth(self.pos+1).unwrap_or('\0')
    }

    pub(crate) fn is_eof(&self) -> bool {
        self.pos >= self.input.len()
    }
}

#[cfg(test)]
mod tests {
    use super::Cursor;

    #[test]
    fn test_new_cursor() {
        let input = "Hello, world!";
        let cursor = Cursor::new(input);
        assert_eq!(cursor.input, input);
        assert_eq!(cursor.pos, 0);
    }

    #[test]
    fn test_bump() {
        let mut cursor = Cursor::new("abc");
        assert_eq!(cursor.bump(), 'a');
        assert_eq!(cursor.bump(), 'b');
        assert_eq!(cursor.bump(), 'c');
        assert_eq!(cursor.bump(), '\0'); // EOF
    }

    #[test]
    fn test_peek() {
        let cursor = Cursor::new("abc");
        assert_eq!(cursor.first(), 'a');
        // `peek` should not advance the position
        assert_eq!(cursor.pos, 0);
    }

    #[test]
    fn test_is_eof() {
        let mut cursor = Cursor::new("a");
        assert!(!cursor.is_eof());
        cursor.bump();
        assert!(cursor.is_eof());
    }
}
