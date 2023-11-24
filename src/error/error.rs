use crate::expr::Expr;
use crate::parser::Position;
use crate::typing::Type;
use lalrpop_util::lexer::Token;
use lalrpop_util::ParseError;
use line_col::LineColLookup;

pub enum Error {
    File {
        filename: String,
        message: String,
    },
    Parse {
        message: String,
        token: Option<String>,
        position: Option<Position>,
        expected: Vec<String>,
    },
    Interval {
        name: String,
        position: Option<Position>,
    },
    Duplicate {
        name: String,
        first: Option<Position>,
        second: Option<Position>,
    },
    Resolve {
        category: String,
        name: String,
        position: Option<Position>,
    },
    Type {
        expr: Expr,
        typ: Type,
        expected: Vec<Type>,
    },
    Time {
        message: String,
        name: String,
        position: Option<Position>,
        expr: Expr,
    },
    Bounded {
        name: String,
        position: Option<Position>,
    },
}

impl Error {
    pub fn new_parse(
        file: &str,
        lookup: &LineColLookup,
        error: ParseError<usize, Token, &str>,
    ) -> Self {
        match error {
            ParseError::InvalidToken { location } => Self::Parse {
                message: "Invalid Token".into(),
                token: None,
                position: Some(Position::new(file, lookup, location)),
                expected: Vec::new(),
            },
            ParseError::UnrecognizedEOF { location, expected } => Self::Parse {
                message: "Unreconized EOF".into(),
                token: None,
                position: Some(Position::new(file, lookup, location)),
                expected,
            },
            ParseError::UnrecognizedToken { token, expected } => Self::Parse {
                message: "Unreconized Token".into(),
                token: Some(token.1.to_string()),
                position: Some(Position::new(file, lookup, token.0)),
                expected,
            },
            ParseError::ExtraToken { token } => Self::Parse {
                message: "Extra Token".into(),
                token: Some(token.1.to_string()),
                position: Some(Position::new(file, lookup, token.0)),
                expected: Vec::new(),
            },
            ParseError::User { error } => Self::Parse {
                message: "Parse Error".into(),
                token: Some(error.to_string()),
                position: None,
                expected: Vec::new(),
            },
        }
    }
}
