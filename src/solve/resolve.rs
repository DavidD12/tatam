use std::time::Instant;

use super::*;
use crate::model::Model;
use crate::search::*;
use crate::Args;
use crate::ToLang;

pub fn resolve<'a>(model: &Model, pretty: &mut d_stuff::Pretty, args: &Args) -> Response {
    let mut model = model.clone();
    // Propagate
    model.propagate_expr();
    // Flatten
    model.flatten_ltl();

    // Solve
    match model.search().path_type() {
        PathType::Initial => resolve_initial(&model, pretty, args),
        PathType::Path {
            infinite,
            truncated,
            finite,
            complete,
        } => resolve_perf(
            &model,
            pretty,
            args,
            infinite,
            truncated,
            finite,
            complete,
            model.search().transitions(),
        ),
    }
    // Display solution
    // TODO
}

pub fn resolve_initial(model: &Model, _pretty: &mut d_stuff::Pretty, args: &Args) -> Response {
    //----- Algo -----
    let log_file = match &args.log_folder {
        Some(folder) => Some(format!("{}/log.smt", folder)),
        None => None,
    };

    let start_time = Instant::now();

    let mut solver = Solver::new(model, log_file);
    solver.create_truncated(0);
    let finish_time = Instant::now();

    let result = solver.check();

    let duration = finish_time.duration_since(start_time).as_secs_f64();

    #[cfg(debug_assertions)]
    {
        println!("> initial : {:.3}", duration);
    }

    match result {
        SatResult::Unknown => {
            solver.exit();
            return Response::Unknown;
        }
        SatResult::Unsat => {
            solver.exit();
            return Response::NoSolution(0);
        }
        SatResult::Sat => {
            let solution = Solution::from_solver(&mut solver, false);
            solver.exit();
            return Response::Solution(solution);
        }
    }
}

fn next_log_file(args: &Args, count: &mut usize) -> Option<String> {
    match &args.log_folder {
        Some(folder) => {
            let opt = Some(format!("{}/log_{}.smt", folder, *count));
            *count += 1;
            opt
        }
        None => None,
    }
}

pub fn resolve_perf(
    model: &Model,
    _pretty: &mut d_stuff::Pretty,
    args: &Args,
    infinite: bool,
    truncated: bool,
    finite: bool,
    complete: bool,
    tn: TransitionNumber,
) -> Response {
    let mut log_count: usize = 0;
    //----- Algo -----
    let mut transitions = tn.min();

    let mut future_duration: f64 = 0.;
    let mut future_transitions: usize = 0;
    let mut path_duration: f64 = 0.;

    loop {
        #[cfg(debug_assertions)]
        {
            println!(
                "========================= {} transition =========================",
                transitions
            );
        }

        // -------------------- Complete/Future --------------------
        if complete && future_duration <= path_duration {
            let start_time = Instant::now();

            let mut solver = Solver::new(model, next_log_file(args, &mut log_count));
            solver
                .add_comment(&format!("resolve_perf future + unicity k={}", transitions))
                .unwrap();
            solver.create_future(transitions);

            let result = solver.check();

            let finish_time = Instant::now();
            let duration = finish_time.duration_since(start_time).as_secs_f64();
            future_duration += duration;
            future_transitions = transitions;

            #[cfg(debug_assertions)]
            {
                println!(
                    "> future {} : {:.3} / {:.3} = {:?}",
                    transitions, duration, future_duration, result
                );
            }

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

        // -------------------- Truncated --------------------
        if truncated {
            let start_time = Instant::now();

            let mut solver = Solver::new(model, next_log_file(args, &mut log_count));
            solver
                .add_comment(&format!("resolve_perf truncated k={}", transitions))
                .unwrap();
            solver.create_truncated(transitions);
            let finish_time = Instant::now();

            let result = solver.check();

            let duration = finish_time.duration_since(start_time).as_secs_f64();
            path_duration += duration;

            #[cfg(debug_assertions)]
            {
                println!(
                    "> truncated {} : {:.3} / {:.3}",
                    transitions, duration, path_duration
                );
            }

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
            let start_time = Instant::now();

            let mut solver = Solver::new(model, next_log_file(args, &mut log_count));
            solver
                .add_comment(&format!("resolve_perf infinte k={}", transitions))
                .unwrap();
            solver.create_infinite(transitions);
            let finish_time = Instant::now();

            let result = solver.check();

            let duration = finish_time.duration_since(start_time).as_secs_f64();
            path_duration += duration;

            #[cfg(debug_assertions)]
            {
                println!(
                    "> infinite {} : {:.3} / {:.3}",
                    transitions, duration, path_duration
                );
            }

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
            let start_time = Instant::now();
            let mut solutions: Vec<Solution> = Vec::new();

            loop {
                // println!("==========");
                // for sol in solutions.iter() {
                //     println!("----------");
                //     println!("{}", sol.to_lang(model));
                // }
                // println!("==========");
                let mut solver = Solver::new(model, next_log_file(args, &mut log_count));
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
                // println!("solution result = {:?}", result);

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
                        // println!("new solution:");
                        // println!("{}", solution.to_lang(model));
                        solver.exit();

                        // Check if is_finite
                        let mut solver = Solver::new(model, next_log_file(args, &mut log_count));
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
                        // println!("has next result = {:?}", result);

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

            let finish_time = Instant::now();
            let duration = finish_time.duration_since(start_time).as_secs_f64();
            path_duration += duration;

            #[cfg(debug_assertions)]
            {
                println!(
                    "> finite {} : {:.3} / {:.3}",
                    transitions, duration, path_duration
                );
            }
        }

        // -------------------- Bound Reached --------------------
        if let Some(max) = tn.max() {
            if transitions >= max {
                // -------------------- Future --------------------
                if complete && future_transitions < transitions {
                    let start_time = Instant::now();
                    let mut solver = Solver::new(model, next_log_file(args, &mut log_count));
                    solver
                        .add_comment(&format!("resolve_perf bound reached k={}", transitions))
                        .unwrap();

                    solver.create_future(transitions);

                    let result = solver.check();

                    let finish_time = Instant::now();
                    let duration = finish_time.duration_since(start_time).as_secs_f64();
                    future_duration += duration;

                    #[cfg(debug_assertions)]
                    {
                        println!(
                            "> future {} : {:.3} / {:.3}",
                            transitions, duration, future_duration
                        );
                    }

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

                return Response::BoundReached;
            }
        }

        transitions += 1;
    }
}

/*
pub fn resolve_sequence(
    model: &Model,
    pretty: &mut d_stuff::Pretty,
    tn: TransitionNumber,
) -> Response {
    //----- Algo -----
    let mut transitions = tn.min();
    let mut log = None;

    #[cfg(debug_assertions)]
    {
        log = Some("log.smt".to_string());
    }

    loop {
        #[cfg(debug_assertions)]
        {
            println!(
                "========================= {} transition =========================",
                transitions
            );
        }

        {
            // ---------- Future ----------

            let mut solver = Solver::new(model, log);
            // Parallel
            // if threads > 1 {
            //     todo!()
            // }
            solver.create_future(transitions, false);

            match solver.check() {
                SatResult::Unknown => {
                    solver.exit();
                    return Response::Unknown;
                }
                SatResult::Unsat => {
                    solver.exit();
                    return Response::NoSolution;
                }
                SatResult::Sat => {
                    solver.exit();
                    // Display ?
                }
            }
        }

        {
            // ---------- Infinite ----------
            let mut solver = Solver::new(model, log);
            // Parallel
            // if threads > 1 {
            //     todo!()
            // }
            solver.create_truncated(transitions, false);

            match solver.check() {
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

        transitions += 1;

        // Bound reached
        if let Some(max) = tn.max() {
            if transitions > max {
                return Response::BoundReached;
            }
        }
    }
}

pub fn resolve_truncated(
    model: &Model,
    pretty: &mut d_stuff::Pretty,
    tn: TransitionNumber,
) -> Response {
    //----- Algo -----
    let mut transitions = tn.min();
    let mut log = None;

    #[cfg(debug_assertions)]
    {
        log = Some("log.smt");
    }

    loop {
        #[cfg(debug_assertions)]
        {
            println!(
                "========================= {} transition =========================",
                transitions
            );
        }

        {
            // ---------- Future ----------

            let mut solver = Solver::new(model, log);
            // Parallel
            // if threads > 1 {
            //     todo!()
            // }
            solver.create_future(transitions, true);

            match solver.check() {
                SatResult::Unknown => {
                    solver.exit();
                    return Response::Unknown;
                }
                SatResult::Unsat => {
                    solver.exit();
                    return Response::NoSolution;
                }
                SatResult::Sat => {
                    solver.exit();
                    // Display ?
                }
            }
        }

        {
            // ---------- Infinite ----------
            let mut solver = Solver::new(model, log);
            // Parallel
            // if threads > 1 {
            //     todo!()
            // }
            solver.create_truncated(transitions, true);

            match solver.check() {
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

        transitions += 1;

        // Bound reached
        if let Some(max) = tn.max() {
            if transitions > max {
                return Response::BoundReached;
            }
        }
    }
}

pub fn resolve_infinite(
    model: &Model,
    pretty: &mut d_stuff::Pretty,
    tn: TransitionNumber,
) -> Response {
    //----- Algo -----
    let mut transitions = tn.min();
    let mut log = None;

    #[cfg(debug_assertions)]
    {
        log = Some("log.smt");
    }

    loop {
        #[cfg(debug_assertions)]
        {
            println!(
                "========================= {} transition =========================",
                transitions
            );
        }

        {
            // ---------- Future ----------

            let mut solver = Solver::new(model, log);
            // Parallel
            // if threads > 1 {
            //     todo!()
            // }
            solver.create_future(transitions, true);

            match solver.check() {
                SatResult::Unknown => {
                    solver.exit();
                    return Response::Unknown;
                }
                SatResult::Unsat => {
                    solver.exit();
                    return Response::NoSolution;
                }
                SatResult::Sat => {
                    solver.exit();
                    // Display ?
                }
            }
        }

        {
            // ---------- Infinite ----------
            let mut solver = Solver::new(model, log);
            // Parallel
            // if threads > 1 {
            //     todo!()
            // }
            solver.create_infinite(transitions, true);

            match solver.check() {
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

        transitions += 1;

        // Bound reached
        if let Some(max) = tn.max() {
            if transitions > max {
                return Response::BoundReached;
            }
        }
    }
}

pub fn resolve_infinite_perf(
    model: &Model,
    pretty: &mut d_stuff::Pretty,
    tn: TransitionNumber,
) -> Response {
    //----- Algo -----
    let mut transitions = tn.min();
    let mut log = None;

    let mut future_duration: f64 = 0.;
    let mut future_transitions: usize = 0;
    let mut loop_duration: f64 = 0.;

    #[cfg(debug_assertions)]
    {
        log = Some("log.smt");
    }

    loop {
        #[cfg(debug_assertions)]
        {
            println!(
                "========================= {} transition =========================",
                transitions
            );
        }

        if future_duration <= loop_duration {
            // ---------- Future ----------
            let start_time = Instant::now();

            let mut solver = Solver::new(model, log);
            solver.create_future(transitions, true);

            let result = solver.check();

            let finish_time = Instant::now();
            future_duration += finish_time.duration_since(start_time).as_secs_f64();
            future_transitions = transitions;

            #[cfg(debug_assertions)]
            {
                println!(
                    "> future {} : {} = {:?}",
                    transitions, future_duration, result
                );
            }

            match result {
                SatResult::Unknown => {
                    solver.exit();
                    return Response::Unknown;
                }
                SatResult::Unsat => {
                    solver.exit();
                    return Response::NoSolution;
                }
                SatResult::Sat => {
                    solver.exit();
                    // Display ?
                }
            }
        }

        {
            // ---------- Infinite ----------
            let start_time = Instant::now();

            let mut solver = Solver::new(model, log);
            solver.create_infinite(transitions, false);
            let finish_time = Instant::now();

            loop_duration += finish_time.duration_since(start_time).as_secs_f64();
            #[cfg(debug_assertions)]
            {
                println!("> loop {} : {}", transitions, loop_duration);
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
                    return Response::Solution(solution);
                }
            }
        }

        // Bound reached
        if let Some(max) = tn.max() {
            if transitions >= max {
                // ---------- Future ----------

                if future_transitions < transitions {
                    let start_time = Instant::now();
                    let mut solver = Solver::new(model, log);
                    solver.create_future(transitions, true);

                    let result = solver.check();

                    let finish_time = Instant::now();
                    future_duration = finish_time.duration_since(start_time).as_secs_f64();

                    #[cfg(debug_assertions)]
                    {
                        println!("> future {} : {}", transitions, future_duration);
                    }

                    match result {
                        SatResult::Unknown => {
                            solver.exit();
                            return Response::Unknown;
                        }
                        SatResult::Unsat => {
                            solver.exit();
                            return Response::NoSolution;
                        }
                        SatResult::Sat => {
                            solver.exit();
                            // Display ?
                        }
                    }
                }

                return Response::BoundReached;
            }
        }

        transitions += 1;
    }
}
*/

/*
pub fn resolve_infinite_incremental(
    model: &Model,
    pretty: &mut d_stuff::Pretty,
    tn: TransitionNumber,
) -> Response {
    //----- Algo -----
    let mut transitions = tn.min();
    let mut log = None;

    #[cfg(debug_assertions)]
    {
        log = Some("log.smt");
    }

    // ---------- Future ----------

    let mut solver = Solver::new(model, log);
    // Parallel
    // if threads > 1 {
    //     todo!()
    // }
    solver.initialize(transitions, true);

    loop {
        #[cfg(debug_assertions)]
        {
            println!(
                "========================= {} transition =========================",
                transitions
            );
        }

        solver.push();
        solver.inc_future();

        match solver.check() {
            SatResult::Unknown => {
                solver.exit();
                return Response::Unknown;
            }
            SatResult::Unsat => {
                solver.exit();
                return Response::NoSolution;
            }
            SatResult::Sat => {
                // Display ?
            }
        }

        solver.pop();
        solver.push();

        // ---------- Infinite ----------
        solver.inc_infinite();

        match solver.check() {
            SatResult::Unknown => {
                solver.exit();
                return Response::Unknown;
            }
            SatResult::Unsat => {
                //
            }
            SatResult::Sat => {
                let solution = Solution::from_solver(&mut solver, false);
                solver.exit();
                return Response::Solution(solution);
            }
        }

        transitions += 1;

        // Bound reached
        if let Some(max) = tn.max() {
            if transitions > max {
                return Response::BoundReached;
            }
        }

        solver.pop();
        solver.inc_transition(true);
    }
}
*/
