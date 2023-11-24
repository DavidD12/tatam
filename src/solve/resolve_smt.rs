use super::*;
use crate::model::Model;
use crate::search::*;
use std::time::Instant;

pub fn resolve_smt<'a>(
    model: &Model,
    pretty: &mut d_stuff::Pretty,
    verbose: u8,
    threads: u32,
) -> Response {
    let mut inner_model = model.clone();
    // Propagate
    inner_model.propagate_expr();
    // Flatten
    inner_model.flatten_ltl();

    if verbose >= 3 {
        pretty.add(inner_model.to_debug_entry());
        pretty.print();
    } else if verbose >= 2 {
        pretty.add(inner_model.to_entry());
        pretty.print();
    }

    // Solve
    match model.search().path() {
        PathType::Initial => resolve_initial(&inner_model, pretty, verbose, threads),
        PathType::Sequence(tn) => todo!(),
        PathType::Truncated(tn) => resolve_unicity(
            &inner_model,
            pretty,
            verbose,
            threads,
            tn,
            true,
            false,
            false,
        ),
        PathType::Infinite(tn) => resolve_unicity(
            &inner_model,
            pretty,
            verbose,
            threads,
            tn,
            false,
            false,
            true,
        ),
        PathType::Finite(tn) => resolve_unicity(
            &inner_model,
            pretty,
            verbose,
            threads,
            tn,
            false,
            true,
            false,
        ),
    }
    // Display solution
    // TODO
}

pub fn resolve_unicity(
    model: &Model,
    pretty: &mut d_stuff::Pretty,
    verbose: u8,
    threads: u32,
    tn: TransitionNumber,
    truncated: bool,
    finite: bool,
    infinite: bool,
) -> Response {
    let cfg = z3::Config::new();
    // cfg.set_param_value("auto_config", "true");

    let ctx = z3::Context::new(&cfg);
    // Solver
    let solver = match model.search().search() {
        SearchType::Solve => {
            let mut params = z3::Params::new(&ctx);

            let propagate = z3::Tactic::new(&ctx, "propagate-values");
            let simplify = z3::Tactic::new(&ctx, "simplify");
            let solve_eqs = z3::Tactic::new(&ctx, "solve-eqs");
            let then_t = propagate.and_then(&simplify).and_then(&solve_eqs);
            let repeat = z3::Tactic::repeat(&ctx, &then_t, 0);
            let smt_t = z3::Tactic::new(&ctx, "smt");
            let tatic = repeat.and_then(&smt_t);
            println!("-----> {}", tatic.to_string());
            params.set_symbol(
                "tactic.smt",
                // "(then (repeat (then propagate-values simplify solve-eqs)) smt)",
                tatic.to_string(),
            );

            let solver = z3::Solver::new(&ctx);
            solver.set_params(&params);

            Z3Solver::Solve(solver)
        }
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
    let mut smt = Smt::empty(model, &cfg, &ctx, &solver);

    //----- Algo -----
    let mut transitions = tn.min();
    smt.initialize(transitions, true);

    loop {
        #[cfg(debug_assertions)]
        {
            println!(
                "========================= {} transition =========================",
                transitions
            );
        }

        smt.solver_push();
        #[cfg(debug_assertions)]
        {
            println!("------------------------- Push -------------------------");
            println!("{}", smt.solver());
        }

        // LTL Possible path semantic
        smt.add_future_semantic();

        #[cfg(debug_assertions)]
        {
            println!("------------------------- Future ? -------------------------");
            println!("{}", smt.solver());
        }

        {
            let mut file = std::fs::File::create("future.smt").unwrap();
            let mut text = format!("{}", smt.solver());
            text += "(check-sat)\n";
            text += "(get-model)\n";
            use std::io::Write;
            file.write_all(text.as_bytes()).unwrap();
        }
        let start_time = Instant::now();

        // ---------- Check if Has Next ----------
        match solver.check() {
            z3::SatResult::Unknown => return Response::Unknown,
            z3::SatResult::Unsat => return Response::NoSolution,
            z3::SatResult::Sat => {
                let end_time = Instant::now();
                let duration = end_time.duration_since(start_time);
                println!("Future Temps d'exécution : {} sec", duration.as_secs_f64());

                // Display Log
            }
        }

        // ---------- Infinite ----------
        if infinite {
            smt.solver_pop();
            smt.solver_push();

            smt.add_loop_semantic();

            #[cfg(debug_assertions)]
            {
                println!("------------------------- Infinite ? -------------------------");
                println!("{}", smt.solver());
            }

            {
                let mut file = std::fs::File::create("infinite.smt").unwrap();
                let mut text = format!("{}", smt.solver());
                text += "(check-sat)\n";
                text += "(get-model)\n";
                use std::io::Write;
                file.write_all(text.as_bytes()).unwrap();
            }

            let start_time = Instant::now();

            match solver.check() {
                z3::SatResult::Unknown => return Response::Unknown,
                z3::SatResult::Sat => {
                    let end_time = Instant::now();
                    let duration = end_time.duration_since(start_time);
                    println!(
                        "infinite Temps d'exécution : {} sec",
                        duration.as_secs_f64()
                    );

                    let z3_model = solver.get_model().unwrap();
                    if verbose >= 3 {
                        pretty.add(smt.z3_model_to_entry());
                        pretty.print();
                    }
                    let solution = Solution::new(&smt, &z3_model);
                    return Response::Solution(solution);
                }
                z3::SatResult::Unsat => {}
            }
        }

        // ---------- Truncated or Finite ----------
        if truncated || finite {
            smt.solver_pop();
            smt.solver_push();

            // println!("------------------------- Pop -------------------------");
            // println!("{}", smt.solver());

            smt.set_finite_semantic();

            #[cfg(debug_assertions)]
            {
                println!("------------------------- Truncated ? -------------------------");
                println!("{}", smt.solver());
            }

            match solver.check() {
                z3::SatResult::Unknown => return Response::Unknown,
                z3::SatResult::Sat => {
                    if truncated {
                        let z3_model = solver.get_model().unwrap();
                        if verbose >= 3 {
                            pretty.add(smt.z3_model_to_entry());
                            pretty.print();
                        }
                        let solution = Solution::new(&smt, &z3_model);
                        return Response::Solution(solution);
                    }
                    if finite {
                        todo!()
                    }
                }
                z3::SatResult::Unsat => {}
            }

            if finite {
                todo!();
            }
        }

        transitions += 1;

        // Bound reached
        if let Some(max) = tn.max() {
            if transitions > max {
                return Response::BoundReached;
            }
        }

        smt.solver_pop();
        // println!("------------------------- Pop -------------------------");
        // println!("{}", smt.solver());

        smt.add_transition();
        // println!("------------------------- Next Transition -------------------------");
        // println!("{}", smt.solver());
    }
}
