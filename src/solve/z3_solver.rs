pub enum Z3Solver<'a> {
    Solve(z3::Solver<'a>),
    Optimize(z3::Optimize<'a>),
}

impl<'a> Z3Solver<'a> {
    pub fn assert(&self, ast: &z3::ast::Bool<'a>) {
        match self {
            Z3Solver::Solve(solver) => solver.assert(ast),
            Z3Solver::Optimize(solver) => solver.assert(ast),
        }
    }

    pub fn set_threads(&self, ctx: &'a z3::Context, threads: u32) {
        match self {
            Z3Solver::Solve(solver) => {
                let mut params = z3::Params::new(ctx);
                params.set_u32("threads", threads);
                solver.set_params(&params);
            }
            Z3Solver::Optimize(_) => {}
        }
    }

    pub fn check(&self) -> z3::SatResult {
        match self {
            Z3Solver::Solve(solver) => solver.check(),
            Z3Solver::Optimize(solver) => solver.check(&[]),
        }
    }

    pub fn get_model(&self) -> Option<z3::Model> {
        match self {
            Z3Solver::Solve(solver) => solver.get_model(),
            Z3Solver::Optimize(solver) => solver.get_model(),
        }
    }

    pub fn minimize_int(&self, expr: &z3::ast::Int) {
        match self {
            Z3Solver::Solve(_) => {}
            Z3Solver::Optimize(solver) => solver.minimize(expr),
        }
    }
    pub fn maximize_int(&self, expr: &z3::ast::Int) {
        match self {
            Z3Solver::Solve(_) => {}
            Z3Solver::Optimize(solver) => solver.maximize(expr),
        }
    }

    pub fn minimize_real(&self, expr: &z3::ast::Real) {
        match self {
            Z3Solver::Solve(_) => {}
            Z3Solver::Optimize(solver) => solver.minimize(expr),
        }
    }
    pub fn maximize_real(&self, expr: &z3::ast::Real) {
        match self {
            Z3Solver::Solve(_) => {}
            Z3Solver::Optimize(solver) => solver.maximize(expr),
        }
    }

    pub fn push(&self) {
        match self {
            Z3Solver::Solve(solver) => solver.push(),
            Z3Solver::Optimize(solver) => solver.push(),
        }
    }

    pub fn pop(&self) {
        match self {
            Z3Solver::Solve(solver) => solver.pop(1),
            Z3Solver::Optimize(solver) => solver.pop(),
        }
    }
}

impl<'a> std::fmt::Display for Z3Solver<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Z3Solver::Solve(solver) => write!(f, "{}", solver),
            Z3Solver::Optimize(solver) => write!(f, "{}", solver),
        }
    }
}
