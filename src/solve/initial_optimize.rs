use super::*;
use crate::model::Model;
use crate::Args;
use smt_sb::SatResult;

pub fn resolve_initial_optimize(
    model: &Model,
    _pretty: &mut d_stuff::Pretty,
    args: &Args,
) -> Response {
    //----- Algo -----
    let log_file = match &args.log_folder {
        Some(folder) => Some(format!("{}/log.smt", folder)),
        None => None,
    };

    let mut best_solution: Option<Solution> = None;

    loop {
        let mut solver = Solver::new(model, log_file.clone());
        solver.create_truncated(0);
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
                    None => return Response::NoSolution(0),
                }
            }
            SatResult::Sat => {
                let solution = Solution::from_solver(&mut solver, false);
                solver.exit();
                best_solution = Some(solution);
            }
        }
    }
}
