use super::*;
use crate::common::*;
use crate::model::*;
use crate::search::*;

pub fn resolve_finite(
    model: &Model,
    pretty: &mut d_stuff::Pretty,
    verbose: u8,
    threads: u32,
    tn: TransitionNumber,
) -> Response {
    let mut transitions = tn.min();
    loop {
        if let Some(max) = tn.max() {
            if transitions > max {
                return Response::BoundReached;
            }
        }
        let cfg = z3::Config::new();
        let ctx = z3::Context::new(&cfg);
        // Solver
        let solver = match model.search().search() {
            SearchType::Solve => Z3Solver::Solve(z3::Solver::new(&ctx)),
            SearchType::Optimize {
                minimize: _,
                objective: _,
                bound: _,
            } => Z3Solver::Optimize(z3::Optimize::new(&ctx)),
        };
        // Parallel
        if threads > 1 {
            solver.set_threads(&ctx, threads);
        }
        // SMT
        let mut smt = Smt::new(model, transitions, false, &cfg, &ctx, &solver);
        //
        let mut opt_sol = None;
        match resolve_finite_transitions_first(&mut smt, pretty, verbose, transitions) {
            Response::NoSolution => {}
            Response::Unknown => return Response::Unknown,
            Response::BoundReached => panic!(),
            Response::Solution(sol) => {
                opt_sol = Some(sol);
            }
        }
        // All Solutions
        while opt_sol.is_some() {
            let solution = opt_sol.unwrap();
            if verbose >= 2 {
                let entry = d_stuff::Entry::new(
                    d_stuff::Status::Success,
                    d_stuff::Text::new(
                        format!("Current Solution {}", transitions),
                        termion::style::Bold.to_string(),
                        termion::color::Blue.fg_str(),
                    ),
                    None,
                    vec![d_stuff::Message::new(
                        None,
                        d_stuff::Text::new(
                            solution.to_lang(model),
                            termion::style::Reset.to_string(),
                            termion::color::White.fg_str(),
                        ),
                    )],
                );
                pretty.add(entry);
                pretty.print();
            }
            if is_finite_path(model, pretty, verbose, threads, transitions, &solution) {
                return Response::Solution(solution);
            }
            match resolve_finite_transitions_next(&mut smt, &solution, pretty, verbose, transitions)
            {
                Response::NoSolution => opt_sol = None,
                Response::Unknown => return Response::Unknown,
                Response::BoundReached => panic!(),
                Response::Solution(sol) => opt_sol = Some(sol),
            }
        }

        transitions += 1;
    }
}

pub fn resolve_finite_transitions_first(
    smt: &mut Smt,
    pretty: &mut d_stuff::Pretty,
    verbose: u8,
    transitions: usize,
) -> Response {
    // Initialize
    smt.initialize();
    if verbose >= 3 {
        pretty.add(smt.solver_to_entry());
        pretty.print();
    }
    smt.add_state_unicity();

    match smt.solver().check() {
        z3::SatResult::Unknown => Response::Unknown,
        z3::SatResult::Sat => {
            let z3_model = smt.solver().get_model().unwrap();
            if verbose >= 3 {
                pretty.add(smt.z3_model_to_entry());
                pretty.print();
            }
            let solution = Solution::new(&smt, &z3_model);
            Response::Solution(solution)
        }
        z3::SatResult::Unsat => {
            if verbose >= 2 {
                let entry = d_stuff::Entry::new(
                    d_stuff::Status::Question,
                    d_stuff::Text::new(
                        "Solve Finite First",
                        termion::style::Bold.to_string(),
                        termion::color::Blue.fg_str(),
                    ),
                    Some(d_stuff::Text::new(
                        format!("no solution with {} transitions", transitions),
                        termion::style::Reset.to_string(),
                        termion::color::Yellow.fg_str(),
                    )),
                    vec![],
                );
                pretty.add(entry);
                pretty.print();
            }
            Response::NoSolution
        }
    }
}

pub fn resolve_finite_transitions_next(
    smt: &mut Smt,
    solution: &Solution,
    pretty: &mut d_stuff::Pretty,
    verbose: u8,
    transitions: usize,
) -> Response {
    smt.remove_solution(solution);

    match smt.solver().check() {
        z3::SatResult::Unknown => Response::Unknown,
        z3::SatResult::Sat => {
            let z3_model = smt.solver().get_model().unwrap();
            if verbose >= 3 {
                pretty.add(smt.z3_model_to_entry());
                pretty.print();
            }
            let solution = Solution::new(&smt, &z3_model);
            Response::Solution(solution)
        }
        z3::SatResult::Unsat => {
            if verbose >= 2 {
                let entry = d_stuff::Entry::new(
                    d_stuff::Status::Question,
                    d_stuff::Text::new(
                        format!("Solve Finite Next {}", transitions),
                        termion::style::Bold.to_string(),
                        termion::color::Blue.fg_str(),
                    ),
                    Some(d_stuff::Text::new(
                        format!("no more solution {}", transitions),
                        termion::style::Reset.to_string(),
                        termion::color::Yellow.fg_str(),
                    )),
                    vec![],
                );
                pretty.add(entry);
                pretty.print();
            }
            Response::NoSolution
        }
    }
}

fn is_finite_path(
    model: &Model,
    pretty: &mut d_stuff::Pretty,
    verbose: u8,
    threads: u32,
    transitions: usize,
    solution: &Solution,
) -> bool {
    let cfg = z3::Config::new();
    let ctx = z3::Context::new(&cfg);
    // Solver
    let solver = match model.search().search() {
        SearchType::Solve => Z3Solver::Solve(z3::Solver::new(&ctx)),
        SearchType::Optimize {
            minimize: _,
            objective: _,
            bound: _,
        } => Z3Solver::Optimize(z3::Optimize::new(&ctx)),
    };
    // Parallel
    if threads > 1 {
        solver.set_threads(&ctx, threads);
    }
    // SMT
    let mut smt = Smt::new(model, transitions + 1, false, &cfg, &ctx, &solver);
    // Initialize
    smt.initialize();
    if verbose >= 3 {
        let entry = d_stuff::Entry::new(
            d_stuff::Status::Question,
            d_stuff::Text::new(
                "Solve Finite Path",
                termion::style::Bold.to_string(),
                termion::color::Blue.fg_str(),
            ),
            None,
            vec![],
        );
        pretty.add(entry);
        pretty.add(smt.solver_to_entry());
        pretty.print();
    }
    smt.add_state_unicity_until_last();
    smt.add_solution(solution);

    match solver.check() {
        z3::SatResult::Unknown => {
            // println!("Unknown");
            false
        }
        z3::SatResult::Sat => {
            // println!("Sat");
            false
        }
        z3::SatResult::Unsat => {
            // println!("Unsat");
            true
        }
    }
}
