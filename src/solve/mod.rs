pub mod response;
pub use response::*;

pub mod solution;
pub use solution::*;

pub mod resolve;
pub use resolve::*;

pub mod initial;
pub use initial::*;

pub mod initial_optimize;
pub use initial_optimize::*;

pub mod sequence;
pub use sequence::*;

pub mod sequence_optimize;
pub use sequence_optimize::*;

pub mod parallel;
pub use parallel::*;

pub mod parallel_optimize;
pub use parallel_optimize::*;

pub mod incremental;
pub use incremental::*;

pub mod solver;
pub use solver::*;

#[derive(Clone, Copy, Debug)]
enum ExecuteRequest {
    Truncated(usize),
    Infinite(usize),
    Finite(usize),
    Complete(usize),
}

#[derive(Clone, Debug)]
struct ExecuteResponse {
    request: ExecuteRequest,
    response: Response,
}

use std::sync::mpsc::Sender;
use threadpool::ThreadPool;

use crate::model::Model;
use crate::Args;
use crate::ToLang;
use smt_sb::SatResult;

fn bound_reached(tn: crate::search::TransitionNumber, transitions: usize) -> bool {
    if let Some(max) = tn.max() {
        return transitions > max;
    }
    return false;
}

fn execute_complete(
    model: &Model,
    transitions: usize,
    args: &Args,
    tx: &Sender<ExecuteResponse>,
    pool: &ThreadPool,
) {
    let tx = tx.clone();
    let model = model.clone();
    let file = log_file(args.log_folder.clone(), "complete", transitions);

    pool.execute(move || {
        let mut solver = Solver::new(&model, file);
        solver
            .add_comment(&format!("resolve_perf future + unicity k={}", transitions))
            .unwrap();
        solver.create_future(transitions);

        let result = solver.check();

        match result {
            SatResult::Unknown => {
                solver.exit();
                tx.send(ExecuteResponse {
                    request: ExecuteRequest::Complete(transitions),
                    response: Response::Unknown,
                })
                .unwrap();
            }
            SatResult::Unsat => {
                solver.exit();
                tx.send(ExecuteResponse {
                    request: ExecuteRequest::Complete(transitions),
                    response: Response::NoSolution(transitions),
                })
                .unwrap();
            }
            SatResult::Sat => {
                solver.exit();
                tx.send(ExecuteResponse {
                    request: ExecuteRequest::Complete(transitions),
                    response: Response::Unknown,
                })
                .unwrap();
            }
        }
    });
}

fn execute_truncated(
    model: &Model,
    transitions: usize,
    args: &Args,
    tx: &Sender<ExecuteResponse>,
    pool: &ThreadPool,
) {
    let tx = tx.clone();
    let model = model.clone();
    let file = log_file(args.log_folder.clone(), "truncated", transitions);

    pool.execute(move || {
        let mut solver = Solver::new(&model, file);

        solver
            .add_comment(&format!("resolve_perf truncated k={}", transitions))
            .unwrap();
        solver.create_truncated(transitions);

        let result = solver.check();

        match result {
            SatResult::Unknown => {
                solver.exit();
                tx.send(ExecuteResponse {
                    request: ExecuteRequest::Truncated(transitions),
                    response: Response::Unknown,
                })
                .unwrap();
            }
            SatResult::Unsat => {
                solver.exit();
                tx.send(ExecuteResponse {
                    request: ExecuteRequest::Truncated(transitions),
                    response: Response::Unknown,
                })
                .unwrap();
            }
            SatResult::Sat => {
                let solution = Solution::from_solver(&mut solver, false);
                solver.exit();
                tx.send(ExecuteResponse {
                    request: ExecuteRequest::Truncated(transitions),
                    response: Response::Solution(solution),
                })
                .unwrap();
            }
        }
    });
}

fn execute_infinite(
    model: &Model,
    transitions: usize,
    args: &Args,
    tx: &Sender<ExecuteResponse>,
    pool: &ThreadPool,
) {
    let tx = tx.clone();
    let model = model.clone();
    let file = log_file(args.log_folder.clone(), "infinite", transitions);

    pool.execute(move || {
        let mut solver = Solver::new(&model, file);
        solver
            .add_comment(&format!("resolve_perf infinte k={}", transitions))
            .unwrap();
        solver.create_infinite(transitions);

        let result = solver.check();

        match result {
            SatResult::Unknown => {
                solver.exit();
                tx.send(ExecuteResponse {
                    request: ExecuteRequest::Infinite(transitions),
                    response: Response::Unknown,
                })
                .unwrap();
            }
            SatResult::Unsat => {
                solver.exit();
                tx.send(ExecuteResponse {
                    request: ExecuteRequest::Infinite(transitions),
                    response: Response::Unknown,
                })
                .unwrap()
            }
            SatResult::Sat => {
                let solution = Solution::from_solver(&mut solver, false);
                solver.exit();
                tx.send(ExecuteResponse {
                    request: ExecuteRequest::Infinite(transitions),
                    response: Response::Solution(solution),
                })
                .unwrap();
            }
        }
    });
}

fn execute_finite(
    model: &Model,
    transitions: usize,
    args: &Args,
    tx: &Sender<ExecuteResponse>,
    pool: &ThreadPool,
) {
    let tx = tx.clone();
    let model = model.clone();
    let log_folder = args.log_folder.clone();
    // let file = log_file(log_folder, "finite", transitions);

    pool.execute(move || {
        loop {
            let mut solutions: Vec<Solution> = Vec::new();
            let mut solver = Solver::new(
                &model,
                log_file_n(log_folder.clone(), "finite", transitions, solutions.len()),
            );

            solver
                .add_comment(&format!("resolve_perf finite k={}", transitions))
                .unwrap();
            for (i, solution) in solutions.iter().enumerate() {
                solver
                    .add_comment(&format!("previous solution {}: ", i))
                    .unwrap();
                solver
                    .add_comment(&format!("{}", solution.to_lang(&model)))
                    .unwrap();
            }
            solver.create_finite(transitions, &solutions);
            let result = solver.check();

            match result {
                SatResult::Unknown => {
                    solver.exit();
                    tx.send(ExecuteResponse {
                        request: ExecuteRequest::Finite(transitions),
                        response: Response::Unknown,
                    })
                    .unwrap();
                    break;
                }
                SatResult::Unsat => {
                    solver.exit();
                    tx.send(ExecuteResponse {
                        request: ExecuteRequest::Finite(transitions),
                        response: Response::Unknown,
                    })
                    .unwrap();
                    break;
                }
                SatResult::Sat => {
                    let solution = Solution::from_solver(&mut solver, true);
                    solver.exit();

                    // Check if is_finite
                    let mut solver = Solver::new(
                        &model,
                        log_file_n(
                            log_folder.clone(),
                            "is_finite",
                            transitions,
                            solutions.len(),
                        ),
                    );

                    solver
                        .add_comment(&format!("resolve_perf check finite k={}", transitions))
                        .unwrap();
                    for (i, solution) in solutions.iter().enumerate() {
                        solver
                            .add_comment(&format!("previous solution {}: ", i))
                            .unwrap();
                        solver
                            .add_comment(&format!("{}", solution.to_lang(&model)))
                            .unwrap();
                    }
                    solver
                        .add_comment(&format!("current solution:\n{}", solution.to_lang(&model)))
                        .unwrap();

                    solver.create_finite_future(transitions + 1, &solution);

                    let result = solver.check();

                    match result {
                        SatResult::Unknown => {
                            solver.exit();
                            tx.send(ExecuteResponse {
                                request: ExecuteRequest::Infinite(transitions),
                                response: Response::Unknown,
                            })
                            .unwrap();
                            break;
                        }
                        SatResult::Unsat => {
                            solver.exit();
                            tx.send(ExecuteResponse {
                                request: ExecuteRequest::Infinite(transitions),
                                response: Response::Solution(solution),
                            })
                            .unwrap();
                            break;
                        }
                        SatResult::Sat => {
                            solver.exit();
                            solutions.push(solution);
                        }
                    }
                }
            }
        }
    });
}
