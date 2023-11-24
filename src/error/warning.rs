use crate::common::*;
use crate::model::*;
use d_stuff::*;

pub enum Warning {
    UnboundedDec(Vec<DeclarationId>),
    UnboundedFun(Vec<FunDecId>),
}

impl Warning {
    pub fn to_entry(&self, model: &Model) -> d_stuff::Entry {
        match self {
            Warning::UnboundedDec(v) => {
                let mut messages = vec![];

                for dec in v {
                    let dec = model.get(*dec).unwrap();
                    messages.push(Message::new(
                        Some(d_stuff::Text::new(
                            format!("'{}'", dec.name()),
                            termion::style::Reset.to_string(),
                            termion::color::LightBlue.fg_str(),
                        )),
                        match dec.position() {
                            Some(pos) => d_stuff::Text::new(
                                format!("{}", pos),
                                termion::style::Reset.to_string(),
                                termion::color::Cyan.fg_str(),
                            ),
                            None => d_stuff::Text::new(
                                "",
                                termion::style::Reset.to_string(),
                                termion::color::Cyan.fg_str(),
                            ),
                        },
                    ));
                }

                d_stuff::Entry::new(
                    d_stuff::Status::Info,
                    d_stuff::Text::new(
                        "Unbounded",
                        termion::style::Bold.to_string(),
                        termion::color::Blue.fg_str(),
                    ),
                    Some(d_stuff::Text::new(
                        "Warning",
                        termion::style::Reset.to_string(),
                        termion::color::Red.fg_str(),
                    )),
                    messages,
                )
            }
            Warning::UnboundedFun(_) => todo!(),
        }
    }
}
