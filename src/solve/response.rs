use super::*;
use crate::common::*;
use crate::model::*;

#[derive(Clone, Debug)]
pub enum Response {
    NoSolution(usize),
    Unknown,
    BoundReached,
    Solution(Solution),
}

//------------------------- To Lang -------------------------

impl ToLang for Response {
    fn to_lang(&self, model: &Model) -> String {
        match self {
            Response::NoSolution(k) => format!("no solution k={}", k),
            Response::Unknown => "unknown".to_string(),
            Response::BoundReached => "bound reached".to_string(),
            Response::Solution(solution) => format!("one solution:\n{}", solution.to_lang(model)),
        }
    }
}

//------------------------- To Entry -------------------------

impl ToEntry for Response {
    fn to_entry(&self, model: &Model) -> d_stuff::Entry {
        match self {
            Response::NoSolution(k) => d_stuff::Entry::new(
                d_stuff::Status::Failure,
                d_stuff::Text::new(
                    "Solve ",
                    termion::style::Bold.to_string(),
                    termion::color::Blue.fg_str(),
                ),
                Some(d_stuff::Text::new(
                    format!("UNSAT k={}", k),
                    termion::style::Reset.to_string(),
                    termion::color::Red.fg_str(),
                )),
                vec![],
            ),
            Response::Unknown => d_stuff::Entry::new(
                d_stuff::Status::Question,
                d_stuff::Text::new(
                    "Solve ",
                    termion::style::Bold.to_string(),
                    termion::color::Blue.fg_str(),
                ),
                Some(d_stuff::Text::new(
                    "UNKNOWN",
                    termion::style::Reset.to_string(),
                    termion::color::Red.fg_str(),
                )),
                vec![],
            ),
            Response::BoundReached => d_stuff::Entry::new(
                d_stuff::Status::Question,
                d_stuff::Text::new(
                    "Solve ",
                    termion::style::Bold.to_string(),
                    termion::color::Blue.fg_str(),
                ),
                Some(d_stuff::Text::new(
                    "BOUND REACHED",
                    termion::style::Reset.to_string(),
                    termion::color::Red.fg_str(),
                )),
                vec![],
            ),
            Response::Solution(solution) => d_stuff::Entry::new(
                d_stuff::Status::Success,
                d_stuff::Text::new(
                    "Solve ",
                    termion::style::Bold.to_string(),
                    termion::color::Blue.fg_str(),
                ),
                Some(d_stuff::Text::new(
                    "SAT",
                    termion::style::Reset.to_string(),
                    termion::color::Green.fg_str(),
                )),
                vec![d_stuff::Message::new(
                    None,
                    d_stuff::Text::new(
                        solution.to_lang(model),
                        termion::style::Reset.to_string(),
                        termion::color::White.fg_str(),
                    ),
                )],
            ),
        }
    }
}
