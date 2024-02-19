use super::*;
use crate::model::Model;
use crate::search::*;
use crate::Args;
use crate::ToLang;
use smt_sb::SatResult;

pub fn resolve_incremental(
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

    let mut solver = Solver::new(
        model,
        log_file(args.log_folder.clone(), "incremanetal", transitions),
    );
    solver.create_path(transitions);
    // solver.apply_tactic();
    solver.push();

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
            solver
                .add_comment(&format!("incremental truncated k={}", transitions))
                .unwrap();
            solver.set_truncated();

            let result = solver.check();

            match result {
                SatResult::Unknown => {
                    solver.exit();
                    return Response::Unknown;
                }
                SatResult::Unsat => {
                    solver.pop();
                    solver.push();
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

            solver
                .add_comment(&format!("incremental infinte k={}", transitions))
                .unwrap();
            solver.set_infinite();

            let result = solver.check();

            match result {
                SatResult::Unknown => {
                    solver.exit();
                    return Response::Unknown;
                }
                SatResult::Unsat => {
                    solver.pop();
                    solver.push();
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
                solver
                    .add_comment(&format!("incremental finite k={}", transitions))
                    .unwrap();
                for (i, solution) in solutions.iter().enumerate() {
                    solver
                        .add_comment(&format!("previous solution {}: ", i))
                        .unwrap();
                    solver
                        .add_comment(&format!("{}", solution.to_lang(model)))
                        .unwrap();
                }
                solver.set_finite(&solutions);
                let result = solver.check();

                match result {
                    SatResult::Unknown => {
                        solver.exit();
                        return Response::Unknown;
                    }
                    SatResult::Unsat => {
                        solver.pop();
                        solver.push();
                        break;
                    }
                    SatResult::Sat => {
                        let solution = Solution::from_solver(&mut solver, true);
                        solver.exit();

                        solver
                            .add_comment(&format!("check finite k={}", transitions))
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

                        solver.set_finite_future(&solution);

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
                                solver.pop();
                                solver.push();
                                solver.decrement_path();
                                solutions.push(solution);
                            }
                        }
                    }
                }
            }
        }

        // -------------------- Complete/Future --------------------
        if complete {
            solver
                .add_comment(&format!("incremental future + unicity k={}", transitions))
                .unwrap();
            solver.set_future();

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
                    solver.pop();
                    solver.push();
                }
            }
        }

        transitions += 1;
        solver.increment_path();
        // solver.apply_tactic();
        solver.push();
    }
}
