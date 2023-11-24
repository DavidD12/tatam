use super::error::*;
use crate::model::Model;
use crate::*;
use d_stuff::*;

//------------------------- To Entry -------------------------

pub fn expected_to_message(expected: &Vec<String>) -> Message {
    let title = Text::new(
        "Expexted",
        termion::style::Reset.to_string(),
        termion::color::White.fg_str(),
    );

    let mut s = "".to_string();
    if let Some((first, others)) = expected.split_first() {
        s.push_str(first);
        for x in others {
            s.push_str(&format!(" {}", x));
        }
    }
    let message = Text::new(
        s,
        termion::style::Reset.to_string(),
        termion::color::LightBlue.fg_str(),
    );
    Message::new(Some(title), message)
}

impl ToEntry for Error {
    fn to_entry(&self, model: &Model) -> Entry {
        match self {
            Error::File { filename, message } => Entry::new(
                Status::Failure,
                Text::new(
                    "File",
                    termion::style::Bold.to_string(),
                    termion::color::Blue.fg_str(),
                ),
                Some(Text::new(
                    "ERROR",
                    termion::style::Reset.to_string(),
                    termion::color::Red.fg_str(),
                )),
                vec![
                    Message::new(
                        Some(Text::new(
                            "Cannot Read File",
                            termion::style::Reset.to_string(),
                            termion::color::Red.fg_str(),
                        )),
                        Text::new(
                            filename,
                            termion::style::Reset.to_string(),
                            termion::color::Cyan.fg_str(),
                        ),
                    ),
                    Message::new(
                        Some(Text::new(
                            "Message",
                            termion::style::Reset.to_string(),
                            termion::color::White.fg_str(),
                        )),
                        Text::new(
                            message,
                            termion::style::Reset.to_string(),
                            termion::color::LightBlue.fg_str(),
                        ),
                    ),
                ],
            ),
            Error::Parse {
                message,
                token,
                position,
                expected,
            } => {
                let mut messages = vec![];

                let title = Text::new(
                    message,
                    termion::style::Reset.to_string(),
                    termion::color::Red.fg_str(),
                );
                if let Some(token) = token {
                    messages.push(Message::new(
                        Some(title),
                        Text::new(
                            format!("'{}'", token),
                            termion::style::Reset.to_string(),
                            termion::color::LightBlue.fg_str(),
                        ),
                    ))
                } else {
                    messages.push(Message::new(None, title));
                }
                if let Some(position) = position {
                    messages.push(position.to_message());
                }
                if !expected.is_empty() {
                    messages.push(expected_to_message(expected));
                }

                Entry::new(
                    Status::Failure,
                    Text::new(
                        "Parse",
                        termion::style::Bold.to_string(),
                        termion::color::Blue.fg_str(),
                    ),
                    Some(Text::new(
                        "ERROR",
                        termion::style::Reset.to_string(),
                        termion::color::Red.fg_str(),
                    )),
                    messages,
                )
            }
            Error::Interval { name, position } => {
                let mut messages = vec![];

                messages.push(Message::new(
                    Some(Text::new(
                        "Malformed Interval",
                        termion::style::Reset.to_string(),
                        termion::color::Red.fg_str(),
                    )),
                    Text::new(
                        format!("'{}'", name),
                        termion::style::Reset.to_string(),
                        termion::color::LightBlue.fg_str(),
                    ),
                ));
                if let Some(position) = position {
                    messages.push(position.to_message());
                }

                Entry::new(
                    Status::Failure,
                    Text::new(
                        "Interval",
                        termion::style::Bold.to_string(),
                        termion::color::Blue.fg_str(),
                    ),
                    Some(Text::new(
                        "ERROR",
                        termion::style::Reset.to_string(),
                        termion::color::Red.fg_str(),
                    )),
                    messages,
                )
            }
            Error::Duplicate {
                name,
                first,
                second,
            } => {
                let mut messages = vec![];

                messages.push(Message::new(
                    Some(Text::new(
                        "Defined Twice",
                        termion::style::Reset.to_string(),
                        termion::color::Red.fg_str(),
                    )),
                    Text::new(
                        format!("'{}'", name),
                        termion::style::Reset.to_string(),
                        termion::color::LightBlue.fg_str(),
                    ),
                ));
                if let Some(position) = first {
                    messages.push(position.to_message());
                }
                if let Some(position) = second {
                    messages.push(position.to_message());
                }

                Entry::new(
                    Status::Failure,
                    Text::new(
                        "Unicity",
                        termion::style::Bold.to_string(),
                        termion::color::Blue.fg_str(),
                    ),
                    Some(Text::new(
                        "ERROR",
                        termion::style::Reset.to_string(),
                        termion::color::Red.fg_str(),
                    )),
                    messages,
                )
            }
            Error::Resolve {
                category,
                name,
                position,
            } => {
                let mut messages = vec![];

                messages.push(Message::new(
                    Some(d_stuff::Text::new(
                        format!("Undefined {}", category),
                        termion::style::Reset.to_string(),
                        termion::color::Red.fg_str(),
                    )),
                    d_stuff::Text::new(
                        format!("'{}'", name),
                        termion::style::Reset.to_string(),
                        termion::color::LightBlue.fg_str(),
                    ),
                ));
                if let Some(position) = position {
                    messages.push(position.to_message());
                }

                d_stuff::Entry::new(
                    d_stuff::Status::Failure,
                    d_stuff::Text::new(
                        "Resolve",
                        termion::style::Bold.to_string(),
                        termion::color::Blue.fg_str(),
                    ),
                    Some(d_stuff::Text::new(
                        "ERROR",
                        termion::style::Reset.to_string(),
                        termion::color::Red.fg_str(),
                    )),
                    messages,
                )
            }
            Error::Type {
                expr,
                typ,
                expected,
            } => {
                let mut messages = vec![];

                messages.push(d_stuff::Message::new(
                    Some(d_stuff::Text::new(
                        "Type Error",
                        termion::style::Reset.to_string(),
                        termion::color::Red.fg_str(),
                    )),
                    d_stuff::Text::new(
                        format!("'{}'", expr.to_lang(model)),
                        termion::style::Reset.to_string(),
                        termion::color::LightBlue.fg_str(),
                    ),
                ));

                if let Some(position) = expr.position() {
                    messages.push(position.to_message());
                }

                messages.push(d_stuff::Message::new(
                    Some(d_stuff::Text::new(
                        "Type",
                        termion::style::Reset.to_string(),
                        termion::color::White.fg_str(),
                    )),
                    d_stuff::Text::new(
                        typ.to_lang(model),
                        termion::style::Reset.to_string(),
                        termion::color::LightBlue.fg_str(),
                    ),
                ));

                if !expected.is_empty() {
                    messages.push(expected_to_message(
                        &expected.iter().map(|t| t.to_lang(model)).collect(),
                    ));
                }

                d_stuff::Entry::new(
                    d_stuff::Status::Failure,
                    d_stuff::Text::new(
                        "Type",
                        termion::style::Bold.to_string(),
                        termion::color::Blue.fg_str(),
                    ),
                    Some(d_stuff::Text::new(
                        "ERROR",
                        termion::style::Reset.to_string(),
                        termion::color::Red.fg_str(),
                    )),
                    messages,
                )
            }
            Error::Time {
                message,
                name,
                position,
                expr,
            } => {
                let mut messages = vec![];

                messages.push(d_stuff::Message::new(
                    Some(d_stuff::Text::new(
                        message,
                        termion::style::Reset.to_string(),
                        termion::color::Red.fg_str(),
                    )),
                    d_stuff::Text::new(
                        name,
                        termion::style::Reset.to_string(),
                        termion::color::LightBlue.fg_str(),
                    ),
                ));

                if let Some(position) = position {
                    messages.push(position.to_message());
                }

                messages.push(d_stuff::Message::new(
                    Some(d_stuff::Text::new(
                        "Expr",
                        termion::style::Reset.to_string(),
                        termion::color::White.fg_str(),
                    )),
                    d_stuff::Text::new(
                        expr.to_lang(model),
                        termion::style::Reset.to_string(),
                        termion::color::LightBlue.fg_str(),
                    ),
                ));

                if let Some(position) = expr.position() {
                    messages.push(position.to_message());
                }

                d_stuff::Entry::new(
                    d_stuff::Status::Failure,
                    d_stuff::Text::new(
                        "Time",
                        termion::style::Bold.to_string(),
                        termion::color::Blue.fg_str(),
                    ),
                    Some(d_stuff::Text::new(
                        "ERROR",
                        termion::style::Reset.to_string(),
                        termion::color::Red.fg_str(),
                    )),
                    messages,
                )
            }
            Error::Bounded { name, position } => {
                let mut messages = vec![];

                messages.push(Message::new(
                    Some(d_stuff::Text::new(
                        "Ubounded Type",
                        termion::style::Reset.to_string(),
                        termion::color::Red.fg_str(),
                    )),
                    d_stuff::Text::new(
                        format!("'{}'", name),
                        termion::style::Reset.to_string(),
                        termion::color::LightBlue.fg_str(),
                    ),
                ));
                if let Some(position) = position {
                    messages.push(position.to_message());
                }

                d_stuff::Entry::new(
                    d_stuff::Status::Failure,
                    d_stuff::Text::new(
                        "Bounded",
                        termion::style::Bold.to_string(),
                        termion::color::Blue.fg_str(),
                    ),
                    Some(d_stuff::Text::new(
                        "ERROR",
                        termion::style::Reset.to_string(),
                        termion::color::Red.fg_str(),
                    )),
                    messages,
                )
            }
        }
    }
}
