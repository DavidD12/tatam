use std::sync::mpsc::channel;
use threadpool::ThreadPool;

use super::*;
use crate::model::Model;
use crate::search::*;
use crate::Args;

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
    let mut running = 0;

    if complete {
        #[cfg(debug_assertions)]
        {
            println!(">>> execute complete {} <<<", transitions);
        }
        execute_complete(&model, transitions, args, &tx, &pool);
        complete_ruinning = true;
        running += 1;
    }

    loop {
        // -------------------- Bound Reached --------------------
        if bound_reached(tn, transitions) {
            if running == 0 {
                return Response::BoundReached;
            }
        } else {
            // -------------------- Send Jobs --------------------
            #[cfg(debug_assertions)]
            {
                println!(
                    "===> running = {} | pool = {}+{} / {}",
                    running,
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
                        break;
                    }
                }
                if truncated {
                    #[cfg(debug_assertions)]
                    {
                        println!(">>> execute truncated {} <<<", transitions);
                    }
                    execute_truncated(model, transitions, args, &tx, &pool);
                    running += 1;
                }
                if infinite && transitions > 0 {
                    #[cfg(debug_assertions)]
                    {
                        println!(">>> execute infinite {} <<<", transitions);
                    }
                    execute_infinite(model, transitions, args, &tx, &pool);
                    running += 1;
                }
                if finite {
                    #[cfg(debug_assertions)]
                    {
                        println!(">>> execute finite {} <<<", transitions);
                    }
                    execute_finite(model, transitions, args, &tx, &pool);
                    running += 1;
                }
                if complete && !complete_ruinning {
                    #[cfg(debug_assertions)]
                    {
                        println!(">>> execute complete {} <<<", transitions);
                    }
                    execute_complete(model, transitions, args, &tx, &pool);
                    complete_ruinning = true;
                    running += 1;
                }
                transitions += 1;

                #[cfg(debug_assertions)]
                {
                    println!(
                        "===> running = {} | pool = {}+{} / {}",
                        running,
                        pool.queued_count(),
                        pool.active_count(),
                        pool.max_count()
                    );
                }
            }
        }

        // -------------------- Read Response --------------------
        let msg = rx.recv().unwrap();
        #[cfg(debug_assertions)]
        {
            println!("------------ Response ------------",);
            println!("request: {:?}", msg.request);
        }
        running -= 1;

        match msg.response {
            Response::NoSolution(_) => return msg.response,
            Response::Unknown => {
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
                            running += 1;
                        }
                    }
                }
            }
            Response::BoundReached => return msg.response,
            Response::Solution(_) => return msg.response,
            _ => panic!("TODO"),
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
    let mut running = 0;

    loop {
        // -------------------- Bound Reached --------------------
        if bound_reached(tn, transitions) {
            if running == 0 {
                return Response::BoundReached;
            }
        } else {
            // -------------------- Send Jobs --------------------
            #[cfg(debug_assertions)]
            {
                println!(
                    "===> running = {} | pool = {}+{} / {}",
                    running,
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
                        break;
                    }
                }

                #[cfg(debug_assertions)]
                {
                    println!(">>> execute complete {} <<<", transitions);
                }
                execute_complete(model, transitions, args, &tx, &pool);
                transitions += 1;
                running += 1;

                #[cfg(debug_assertions)]
                {
                    println!(
                        "===> running = {} | pool = {}+{} / {}",
                        running,
                        pool.queued_count(),
                        pool.active_count(),
                        pool.max_count()
                    );
                }
            }
        }

        // -------------------- Read Response --------------------
        let msg = rx.recv().unwrap();
        #[cfg(debug_assertions)]
        {
            println!("------------ Response ------------",);
            println!("request: {:?}", msg.request);
        }
        running -= 1;

        match msg.response {
            Response::NoSolution(_) => return msg.response,
            Response::Unknown => {
                #[cfg(debug_assertions)]
                {
                    println!("response: no solution");
                }
            }
            Response::BoundReached => return msg.response,
            Response::Solution(_) => return msg.response,
            _ => panic!("TODO"),
        }
    }
}
