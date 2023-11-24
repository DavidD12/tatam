use super::*;
use crate::model::*;
use crate::search::*;

pub fn resolve_sequence(
    model: &Model,
    pretty: &mut d_stuff::Pretty,
    verbose: u8,
    threads: u32,
    tn: TransitionNumber,
) -> Response {
    let mut transitions = tn.min();
    loop {
        if let Some(max) = tn.max() {
            if transitions > max {
                return Response::BoundReached;
            }
        }
        let cfg = z3::Config::new();
        let ctx = z3::Context::new(&cfg);
        // Solver
        let solver = match model.search().search() {
            SearchType::Solve => Z3Solver::Solve(z3::Solver::new(&ctx)),
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
        let mut smt = Smt::new(model, transitions, false, &cfg, &ctx, &solver);
        // Initialize
        smt.initialize();
        if verbose >= 3 {
            pretty.add(smt.solver_to_entry());
            pretty.print();
        }

        match solver.check() {
            z3::SatResult::Unknown => return Response::Unknown,
            z3::SatResult::Sat => {
                let z3_model = solver.get_model().unwrap();
                if verbose >= 3 {
                    pretty.add(smt.z3_model_to_entry());
                    pretty.print();
                }
                let solution = Solution::new(&smt, &z3_model);
                return Response::Solution(solution);
            }
            z3::SatResult::Unsat => transitions += 1,
        }
    }
}
