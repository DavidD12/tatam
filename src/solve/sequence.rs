use super::*;
use crate::model::Model;
use crate::search::*;
use crate::Args;
use crate::ToLang;
use smt_sb::SatResult;

pub fn resolve_sequence(
    model: &Model,
    _pretty: &mut d_stuff::Pretty,
    args: &Args,
    infinite: bool,
    truncated: bool,
    finite: bool,
    complete: bool,
    tn: TransitionNumber,
) -> Response {
    //----- Algo -----
    let mut transitions = tn.min();

    loop {
        #[cfg(debug_assertions)]
        {
            println!(
                "========================= {} transition =========================",
                transitions
            );
        }

        if args.verbose > 2 {
            println!(
                "========================= {} transition =========================",
                transitions
            );
        }

        // -------------------- Bound Reached --------------------
        if let Some(max) = tn.max() {
            if transitions > max {
                return Response::BoundReached;
            }
        }

        // -------------------- Truncated --------------------
        if truncated {
            let mut solver = Solver::new(
                model,
                log_file(args.log_folder.clone(), "truncated", transitions),
            );

            solver
                .add_comment(&format!("resolve_perf truncated k={}", transitions))
                .unwrap();
            solver.create_truncated(transitions);

            let result = solver.check();

            match result {
                SatResult::Unknown => {
                    solver.exit();
                    return Response::Unknown;
                }
                SatResult::Unsat => {
                    solver.exit();
                }
                SatResult::Sat => {
                    let solution = Solution::from_solver(&mut solver, false);
                    solver.exit();
                    return Response::Solution(solution);
                }
            }
        }

        // -------------------- Infinite --------------------
        if infinite && transitions > 0 {
            // ---------- Infinite ----------

            let mut solver = Solver::new(
                model,
                log_file(args.log_folder.clone(), "inifinite", transitions),
            );

            solver
                .add_comment(&format!("resolve_perf infinte k={}", transitions))
                .unwrap();
            solver.create_infinite(transitions);

            let result = solver.check();

            match result {
                SatResult::Unknown => {
                    solver.exit();
                    return Response::Unknown;
                }
                SatResult::Unsat => {
                    solver.exit();
                }
                SatResult::Sat => {
                    let solution = Solution::from_solver(&mut solver, false);
                    solver.exit();
                    return Response::Solution(solution);
                }
            }
        }

        // -------------------- Finite --------------------
        if finite {
            let mut solutions: Vec<Solution> = Vec::new();

            loop {
                let mut solver = Solver::new(
                    model,
                    log_file_n(
                        args.log_folder.clone(),
                        "finite",
                        transitions,
                        solutions.len(),
                    ),
                );

                solver
                    .add_comment(&format!("resolve_perf finite k={}", transitions))
                    .unwrap();
                for (i, solution) in solutions.iter().enumerate() {
                    solver
                        .add_comment(&format!("previous solution {}: ", i))
                        .unwrap();
                    solver
                        .add_comment(&format!("{}", solution.to_lang(model)))
                        .unwrap();
                }
                solver.create_finite(transitions, &solutions);
                let result = solver.check();

                match result {
                    SatResult::Unknown => {
                        solver.exit();
                        return Response::Unknown;
                    }
                    SatResult::Unsat => {
                        solver.exit();
                        break;
                    }
                    SatResult::Sat => {
                        let solution = Solution::from_solver(&mut solver, true);
                        solver.exit();

                        // Check if is_finite
                        let mut solver = Solver::new(
                            model,
                            log_file_n(
                                args.log_folder.clone(),
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
                                .add_comment(&format!("{}", solution.to_lang(model)))
                                .unwrap();
                        }
                        solver
                            .add_comment(&format!("current solution:\n{}", solution.to_lang(model)))
                            .unwrap();

                        solver.create_finite_future(transitions + 1, &solution);

                        let result = solver.check();

                        match result {
                            SatResult::Unknown => {
                                solver.exit();
                                return Response::Unknown;
                            }
                            SatResult::Unsat => {
                                solver.exit();
                                return Response::Solution(solution);
                            }
                            SatResult::Sat => {
                                solver.exit();
                                solutions.push(solution);
                            }
                        }
                    }
                }
            }
        }

        // -------------------- Complete/Future --------------------
        if complete {
            let mut solver = Solver::new(
                model,
                log_file(args.log_folder.clone(), "complete", transitions),
            );

            solver
                .add_comment(&format!("resolve_perf future + unicity k={}", transitions))
                .unwrap();
            solver.create_future(transitions);

            let result = solver.check();

            match result {
                SatResult::Unknown => {
                    solver.exit();
                    return Response::Unknown;
                }
                SatResult::Unsat => {
                    solver.exit();
                    return Response::NoSolution(transitions);
                }
                SatResult::Sat => {
                    solver.exit();
                    // Display ?
                }
            }
        }

        transitions += 1;
    }
}
