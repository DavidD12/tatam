pub mod position;
pub use position::*;

pub mod parser;
pub use parser::*;

lalrpop_mod!(grammar, "/parser/grammar.rs");

use crate::error::Error;
use crate::model::Model;
use line_col::LineColLookup;

#[derive(Debug)]
pub struct Identifier {
    pub name: String,
    pub position: Position,
}

impl Identifier {
    pub fn new(file: &str, lookup: &LineColLookup, name: &str, offset: usize) -> Self {
        let name = name.into();
        let position = Position::new(file, lookup, offset);
        Self { name, position }
    }
}

pub fn parse_file(model: &mut Model, file: &str) -> Result<(), Error> {
    let mut parser = Parser::new(model);
    parser.add(file);

    loop {
        match parser.next() {
            None => return Ok(()),
            Some(file) => match std::fs::read_to_string(&file) {
                Ok(input) => {
                    let lookup = LineColLookup::new(&input);
                    match grammar::ModelParser::new().parse(&lookup, &mut parser, &input) {
                        Ok(_) => {}
                        Err(e) => return Err(Error::new_parse(&file, &lookup, e)),
                    }
                }
                Err(e) => {
                    let e = Error::File {
                        filename: file,
                        message: format!("{:?}", e),
                    };
                    return Err(e);
                }
            },
        }
    }
}
