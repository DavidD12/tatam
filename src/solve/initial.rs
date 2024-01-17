use std::time::Instant;

use super::*;
use crate::model::Model;
use crate::Args;
use smt_sb::SatResult;

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
