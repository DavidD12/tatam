use std::sync::mpsc::{channel, Sender};
use threadpool::ThreadPool;

use super::*;
use crate::model::Model;
use crate::search::*;
use crate::Args;
use crate::ToLang;
use smt_sb::SatResult;

#[derive(Clone, Debug)]
struct ExecuteResponse {
    request: ExecuteRequest,
    response: Option<Response>,
}

#[derive(Clone, Copy, Debug)]
enum ExecuteRequest {
    Truncated(usize),
    Infinite(usize),
    Finite(usize),
    Complete(usize),
}

pub fn resolve_parallel(
    model: &Model,
    _pretty: &mut d_stuff::Pretty,
    args: &Args,
    infinite: bool,
    truncated: bool,
    finite: bool,
    complete: bool,
    tn: TransitionNumber,
    pool_size: usize,
) -> Response {
    let pool = ThreadPool::new(pool_size);
    let (tx, rx) = channel();

    let mut transitions = tn.min();
    let mut complete_ruinning = false;

    if complete {
        #[cfg(debug_assertions)]
        {
            println!(">>> execute complete {} <<<", transitions);
        }
        execute_complete(&model, transitions, args, &tx, &pool);
        complete_ruinning = true;
    }

    loop {
        // -------------------- Bound Reached --------------------
        if let Some(max) = tn.max() {
            if transitions > max {
                pool.join();
                return Response::BoundReached;
            }
        }

        // -------------------- Send Jobs --------------------
        #[cfg(debug_assertions)]
        {
            println!(
                "===> pool = {}+{} / {}",
                pool.queued_count(),
                pool.active_count(),
                pool.max_count()
            );
        }

        while pool.queued_count() + pool.active_count() < pool.max_count() {
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

            if let Some(max) = tn.max() {
                if transitions > max {
                    pool.join();
                    return Response::BoundReached;
                }
            }
            if truncated {
                #[cfg(debug_assertions)]
                {
                    println!(">>> execute truncated {} <<<", transitions);
                }
                execute_truncated(model, transitions, args, &tx, &pool)
            }
            if infinite && transitions > 0 {
                #[cfg(debug_assertions)]
                {
                    println!(">>> execute infinite {} <<<", transitions);
                }
                execute_infinite(model, transitions, args, &tx, &pool);
            }
            if finite {
                #[cfg(debug_assertions)]
                {
                    println!(">>> execute finite {} <<<", transitions);
                }
                execute_finite(model, transitions, args, &tx, &pool);
            }
            if complete && !complete_ruinning {
                #[cfg(debug_assertions)]
                {
                    println!(">>> execute complete {} <<<", transitions);
                }
                execute_complete(model, transitions, args, &tx, &pool);
                complete_ruinning = true;
            }
            transitions += 1;

            #[cfg(debug_assertions)]
            {
                println!(
                    "---> pool = {}+{} / {}",
                    pool.queued_count(),
                    pool.active_count(),
                    pool.max_count()
                );
            }
        }

        // -------------------- Read Response --------------------
        let msg = rx.recv().unwrap();
        #[cfg(debug_assertions)]
        {
            println!("------------ Response ------------",);
            println!("request: {:?}", msg.request);
        }
        match msg.response {
            Some(solution) => {
                pool.join();
                return solution;
            }
            _ => {
                #[cfg(debug_assertions)]
                {
                    println!("response: no solution");
                }
                if complete {
                    if let ExecuteRequest::Complete(k) = msg.request {
                        if k < transitions {
                            #[cfg(debug_assertions)]
                            {
                                println!(">>> execute complete {} <<<", transitions);
                            }
                            execute_complete(&model, transitions, args, &tx, &pool);
                            complete_ruinning = false;
                        }
                    }
                }
            }
        }
    }
}

pub fn resolve_parallel_complete(
    model: &Model,
    _pretty: &mut d_stuff::Pretty,
    args: &Args,
    tn: TransitionNumber,
    pool_size: usize,
) -> Response {
    let pool = ThreadPool::new(pool_size);
    let (tx, rx) = channel();

    let mut transitions = tn.min();

    loop {
        // -------------------- Bound Reached --------------------
        if let Some(max) = tn.max() {
            if transitions > max {
                pool.join();
                return Response::BoundReached;
            }
        }

        // -------------------- Send Jobs --------------------
        #[cfg(debug_assertions)]
        {
            println!(
                "===> pool = {}+{} / {}",
                pool.queued_count(),
                pool.active_count(),
                pool.max_count()
            );
        }

        while pool.queued_count() + pool.active_count() < pool.max_count() {
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

            if let Some(max) = tn.max() {
                if transitions > max {
                    pool.join();
                    return Response::BoundReached;
                }
            }

            #[cfg(debug_assertions)]
            {
                println!(">>> execute truncated {} <<<", transitions);
            }
            execute_complete(model, transitions, args, &tx, &pool);
            transitions += 1;

            #[cfg(debug_assertions)]
            {
                println!(
                    "---> pool = {}+{} / {}",
                    pool.queued_count(),
                    pool.active_count(),
                    pool.max_count()
                );
            }
        }

        // -------------------- Read Response --------------------
        let msg = rx.recv().unwrap();
        #[cfg(debug_assertions)]
        {
            println!("------------ Response ------------",);
            println!("request: {:?}", msg.request);
        }
        match msg.response {
            Some(solution) => {
                pool.join();
                return solution;
            }
            _ => {
                #[cfg(debug_assertions)]
                {
                    println!("response: no solution");
                }
            }
        }
    }
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
                    response: None,
                })
                .unwrap();
            }
            SatResult::Unsat => {
                solver.exit();
                tx.send(ExecuteResponse {
                    request: ExecuteRequest::Complete(transitions),
                    response: Some(Response::NoSolution(transitions)),
                })
                .unwrap();
            }
            SatResult::Sat => {
                solver.exit();
                tx.send(ExecuteResponse {
                    request: ExecuteRequest::Complete(transitions),
                    response: None,
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
                    response: None,
                })
                .unwrap();
            }
            SatResult::Unsat => {
                solver.exit();
                tx.send(ExecuteResponse {
                    request: ExecuteRequest::Truncated(transitions),
                    response: None,
                })
                .unwrap();
            }
            SatResult::Sat => {
                let solution = Solution::from_solver(&mut solver, false);
                solver.exit();
                tx.send(ExecuteResponse {
                    request: ExecuteRequest::Truncated(transitions),
                    response: Some(Response::Solution(solution)),
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
                    response: None,
                })
                .unwrap();
            }
            SatResult::Unsat => {
                solver.exit();
                tx.send(ExecuteResponse {
                    request: ExecuteRequest::Infinite(transitions),
                    response: None,
                })
                .unwrap()
            }
            SatResult::Sat => {
                let solution = Solution::from_solver(&mut solver, false);
                solver.exit();
                tx.send(ExecuteResponse {
                    request: ExecuteRequest::Infinite(transitions),
                    response: Some(Response::Solution(solution)),
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
                        response: None,
                    })
                    .unwrap();
                    break;
                }
                SatResult::Unsat => {
                    solver.exit();
                    tx.send(ExecuteResponse {
                        request: ExecuteRequest::Finite(transitions),
                        response: None,
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
                                response: None,
                            })
                            .unwrap();
                            break;
                        }
                        SatResult::Unsat => {
                            solver.exit();
                            tx.send(ExecuteResponse {
                                request: ExecuteRequest::Infinite(transitions),
                                response: Some(Response::Solution(solution)),
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
