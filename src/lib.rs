#[macro_use]
extern crate lalrpop_util;

pub mod common;
pub mod error;
pub mod expr;
pub mod model;
pub mod parser;
pub mod search;
pub mod solve;
pub mod typing;
pub use common::*;

use crate::search::*;
use crate::typing::*;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "tatam", about = "Transition And Theory Analysis Machine")]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// input file
    #[arg(short, long)]
    pub file: String,
    /// verbose level
    #[arg(short, long, default_value_t = 1)]
    pub verbose: u8,
    /// threads N (number of threads)
    #[arg(short, long, default_value_t = 1)]
    pub threads: u32,
    /// Incremental solver
    #[arg(short, long, default_value_t = false)]
    pub incremental: bool,
    // /// install vscode extensions in the destination folder
    // #[arg(long = "vs")]
    // pub vscode_extension: Option<String>,
    /// SMT Log Folder
    #[arg(short, long)]
    log_folder: Option<String>,
}

pub fn ok_entry<S: Into<String>>(title: S) -> d_stuff::Entry {
    let title = d_stuff::Text::new(
        title,
        termion::style::Bold.to_string(),
        termion::color::Blue.fg_str(),
    );
    let message = d_stuff::Text::new(
        "OK",
        termion::style::Reset.to_string(),
        termion::color::Green.fg_str(),
    );
    d_stuff::Entry::new(d_stuff::Status::Success, title, Some(message), vec![])
}

pub fn load_file(
    pretty: &mut d_stuff::Pretty,
    model: &mut model::Model,
    filename: &str,
    verbose: u8,
) -> Result<(), error::Error> {
    // Parsing
    match parser::parse_file(model, filename) {
        Ok(_) => {
            if verbose >= 2 {
                pretty.add(ok_entry("Parse"));
                pretty.print();
            }
        }
        Err(e) => return Err(e),
    }
    // Check Interval
    match model.check_intervals() {
        Ok(_) => {
            if verbose >= 2 {
                pretty.add(ok_entry("Interval"));
                pretty.print();
            }
        }
        Err(e) => return Err(e),
    }
    // Duplicate
    match model.check_unicity() {
        Ok(_) => {
            if verbose >= 2 {
                pretty.add(ok_entry("Unicity"));
                pretty.print();
            }
        }
        Err(e) => return Err(e),
    }
    // Resolve Type
    match model.resolve_type() {
        Ok(_) => {
            if verbose >= 2 {
                pretty.add(ok_entry("Resolve Type"));
                pretty.print();
            }
        }
        Err(e) => return Err(e),
    }

    // ------------------------- Expr -------------------------

    // resolve Expr
    match model.resolve_expr() {
        Ok(_) => {
            if verbose >= 2 {
                pretty.add(ok_entry("Resolve Expression"));
                pretty.print();
            }
        }
        Err(e) => return Err(e),
    }

    // Check Type
    match model.check_type() {
        Ok(_) => {
            if verbose >= 2 {
                pretty.add(ok_entry("Check Type"));
                pretty.print();
            }
        }
        Err(e) => return Err(e),
    }

    // Check Time
    match model.check_time() {
        Ok(_) => {
            if verbose >= 2 {
                pretty.add(ok_entry("Check Time"));
                pretty.print();
            }
        }
        Err(e) => return Err(e),
    }

    // ------------------------- Search -------------------------

    // Check Search
    match model.search().path_type() {
        PathType::Initial => {}
        PathType::Path {
            infinite,
            truncated: _,
            finite,
            complete,
        } => {
            // Check Bounded
            if infinite || complete || finite {
                match model.check_var_fun_bounded_paramters() {
                    Ok(_) => {
                        if verbose >= 2 {
                            pretty.add(ok_entry("Check Bounded Variable Function Parameters"));
                            pretty.print();
                        }
                    }
                    Err(e) => return Err(e),
                }
            }
            if finite {
                match model.check_cst_fun_bounded_paramters() {
                    Ok(_) => {
                        if verbose >= 2 {
                            pretty.add(ok_entry("Check Bounded Constant Function Parameters"));
                            pretty.print();
                        }
                    }
                    Err(e) => return Err(e),
                }
            }
        }
    }

    // // Check Cycle
    // match problem.check_cycle() {
    //     Ok(_) => {
    //         if verbose >= 2 {
    //             pretty.add(ok_entry("Cycle    "));
    //             pretty.print();
    //         }
    //     }
    //     Err(e) => return Err(e),
    // }
    // // Check Interval
    // match problem.check_interval() {
    //     Ok(_) => {
    //         if verbose >= 2 {
    //             pretty.add(ok_entry("Interval "));
    //             pretty.print();
    //         }
    //     }
    //     Err(e) => return Err(e),
    // }
    // // Check Bounded
    // match problem.check_bounded() {
    //     Ok(_) => {
    //         if verbose >= 2 {
    //             pretty.add(ok_entry("Bounded  "));
    //             pretty.print();
    //         }
    //     }
    //     Err(e) => return Err(e),
    // }
    // ------------------------- Preprocess ? -------------------------

    // // Check Empty
    // match problem.check_empty() {
    //     Ok(_) => {
    //         if verbose >= 2 {
    //             pretty.add(ok_entry("Empty    "));
    //             pretty.print();
    //         }
    //     }
    //     Err(e) => return Err(e),
    // }

    // // Check Parameter Size
    // match problem.check_parameter_size() {
    //     Ok(_) => {
    //         if verbose >= 2 {
    //             pretty.add(ok_entry("Parameter"));
    //             pretty.print();
    //         }
    //     }
    //     Err(e) => return Err(e),
    // }

    Ok(())
}
