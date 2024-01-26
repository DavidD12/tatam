use super::*;
use crate::model::Model;
use crate::search::*;
use crate::Args;
use crate::ToLang;
use smt_sb::SatResult;

pub fn resolve_sequence_optimize(
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
    let mut best_solution: Option<Solution> = None;

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
                match &best_solution {
                    Some(solution) => return Response::Solution(solution.clone()),
                    None => return Response::BoundReached,
                }
            }
        }

        // -------------------- Truncated --------------------
        if truncated {
            #[cfg(debug_assertions)]
            {
                println!("========================= Truncated =========================",);
            }
            let mut solver = Solver::new(
                model,
                log_file(args.log_folder.clone(), "truncated", transitions),
            );

            solver
                .add_comment(&format!("resolve_perf truncated k={}", transitions))
                .unwrap();
            solver.create_truncated(transitions);
            solver.add_optimization();
            if let Some(solution) = &best_solution {
                solver.add_best_objective_constraint(solution.objective.as_ref().unwrap());
            }

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
                    best_solution = Some(solution);
                }
            }
        }

        // -------------------- Infinite --------------------
        if infinite && transitions > 0 {
            #[cfg(debug_assertions)]
            {
                println!("========================= Infinite =========================",);
            }

            let mut solver = Solver::new(
                model,
                log_file(args.log_folder.clone(), "inifinite", transitions),
            );

            solver
                .add_comment(&format!("resolve_perf infinte k={}", transitions))
                .unwrap();
            solver.create_infinite(transitions);
            solver.add_optimization();
            if let Some(solution) = &best_solution {
                solver.add_best_objective_constraint(solution.objective.as_ref().unwrap());
            }

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
                    best_solution = Some(solution);
                }
            }
        }

        // -------------------- Finite --------------------
        if finite {
            #[cfg(debug_assertions)]
            {
                println!("========================= Finite =========================",);
            }
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
                solver.add_optimization();
                if let Some(solution) = &best_solution {
                    solver.add_best_objective_constraint(solution.objective.as_ref().unwrap());
                }

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
                        solver.add_optimization();
                        if let Some(solution) = &best_solution {
                            solver.add_best_objective_constraint(
                                solution.objective.as_ref().unwrap(),
                            );
                        }

                        let result = solver.check();

                        match result {
                            SatResult::Unknown => {
                                solver.exit();
                                return Response::Unknown;
                            }
                            SatResult::Unsat => {
                                solver.exit();
                                best_solution = Some(solution);
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
        }

        // -------------------- Complete/Future --------------------
        if complete {
            #[cfg(debug_assertions)]
            {
                println!("========================= Complete =========================",);
            }
            let mut solver = Solver::new(
                model,
                log_file(args.log_folder.clone(), "complete", transitions),
            );

            solver
                .add_comment(&format!("resolve_perf future + unicity k={}", transitions))
                .unwrap();
            solver.create_future(transitions);
            solver.add_optimization();
            if let Some(solution) = &best_solution {
                solver.add_best_objective_constraint(solution.objective.as_ref().unwrap());
            }

            let result = solver.check();

            match result {
                SatResult::Unknown => {
                    solver.exit();
                    return Response::Unknown;
                }
                SatResult::Unsat => {
                    solver.exit();
                    match &best_solution {
                        Some(solution) => return Response::Solution(solution.clone()),
                        None => return Response::NoSolution(transitions),
                    }
                }
                SatResult::Sat => {
                    solver.exit();
                }
            }
        }

        transitions += 1;
    }
}
