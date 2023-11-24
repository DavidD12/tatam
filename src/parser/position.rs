use line_col::LineColLookup;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Position {
    pub file: String,
    pub line: usize,
    pub column: usize,
}

impl Position {
    pub fn new<S: Into<String>>(file: S, lookup: &LineColLookup, offset: usize) -> Self {
        let file = file.into();
        let (line, column) = lookup.get(offset);
        Self { file, line, column }
    }

    pub fn to_text(&self) -> d_stuff::Text {
        d_stuff::Text::new(
            format!("{}", self),
            termion::style::Reset.to_string(),
            termion::color::Cyan.fg_str(),
        )
    }

    pub fn to_message(&self) -> d_stuff::Message {
        d_stuff::Message::new(
            Some(d_stuff::Text::new(
                "File",
                termion::style::Reset.to_string(),
                termion::color::White.fg_str(),
            )),
            self.to_text(),
        )
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:{}", self.file, self.line, self.column)
    }
}
