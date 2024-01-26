use super::*;
use crate::common::*;
use crate::expr::*;
use crate::model::*;
use crate::typing::*;
use smt_sb::*;

pub struct Solver<'a> {
    model: &'a Model,
    transitions: usize,
    with_loop: bool,
    smt: SmtBridge,
}

impl<'a> Solver<'a> {
    pub fn new(model: &'a Model, log_file: Option<String>) -> Self {
        let mut smt = SmtBridge::new("z3", vec!["-in"], log_file).unwrap();
        smt.set_option("print-success", "false").unwrap();
        //
        Self {
            model,
            transitions: 0,
            with_loop: false,
            smt,
        }
    }

    pub fn exit(&mut self) {
        self.smt.exit().unwrap();
    }

    pub fn model(&self) -> &Model {
        self.model
    }

    pub fn transitions(&self) -> usize {
        self.transitions
    }

    pub fn states(&self) -> usize {
        self.transitions + 1
    }

    pub fn add_comment(&mut self, comment: &str) -> std::io::Result<()> {
        self.smt.add_comment(comment)
    }

    //------------------------- Sort -------------------------

    fn to_sort(&self, typ: &Type) -> String {
        match typ {
            Type::Bool => "Bool".to_string(),
            Type::Int => "Int".to_string(),
            Type::Real => "Real".to_string(),
            Type::IntInterval(_, _) => "Int".to_string(),
            Type::Interval(_) => "Int".to_string(),

            Type::Enumerate(id) => self.model.get(*id).unwrap().name().to_string(),

            Type::Undefined => panic!(),
            Type::Unresolved(_, _) => panic!(),
            Type::Function(_, _) => panic!(),
        }
    }

    //------------------------- Naming -------------------------

    pub fn enumerate_name(e: &Enumerate) -> String {
        e.name().to_string()
    }
    pub fn enumerate_name_from_id(&self, id: EnumerateId) -> String {
        let e = self.model.get(id).unwrap();
        Self::enumerate_name(e)
    }

    pub fn cst_dec_name(dec: &Declaration) -> String {
        dec.name().to_string()
    }
    pub fn cst_dec_name_from_id(&self, id: DeclarationId) -> String {
        let dec = self.model.get(id).unwrap();
        Self::cst_dec_name(dec)
    }

    pub fn cst_fun_name(fun: &FunDec) -> String {
        fun.name().to_string()
    }
    pub fn cst_fun_name_from_id(&self, id: FunDecId) -> String {
        let fun = self.model.get(id).unwrap();
        Self::cst_fun_name(fun)
    }

    pub fn var_dec_name(dec: &Declaration, state: usize) -> String {
        format!("{}__{}", dec.name(), state)
    }
    pub fn var_dec_name_from_id(&self, id: DeclarationId, state: usize) -> String {
        let dec = self.model.get(id).unwrap();
        Self::var_dec_name(dec, state)
    }

    pub fn var_fun_name(fun: &FunDec, state: usize) -> String {
        format!("{}__{}", fun.name(), state)
    }
    pub fn var_fun_name_from_id(&self, id: FunDecId, state: usize) -> String {
        let fun = self.model.get(id).unwrap();
        Self::var_fun_name(fun, state)
    }

    pub fn ltl_var_name(v: &LTLVariable, state: usize) -> String {
        format!("{}__{}", v.name(), state)
    }
    pub fn ltl_var_name_from_id(&self, id: LTLVariableId, state: usize) -> String {
        let v = self.model.get(id).unwrap();
        Self::ltl_var_name(v, state)
    }

    pub fn loop_name(state: usize) -> String {
        format!("_l_{}", state)
    }

    //------------------------- Enum Declaration -------------------------

    fn declare_enumerate(&mut self, enumerate: &Enumerate) {
        let elements = enumerate
            .elements()
            .iter()
            .map(|e| e.name())
            .collect::<Vec<_>>();
        self.smt
            .declare_enumeration(enumerate.name(), &elements)
            .unwrap();
    }

    fn declare_enumerates(&mut self) {
        for e in self.model.enumerates().iter() {
            self.declare_enumerate(e);
        }
    }

    //------------------------- Cst Declaration -------------------------

    fn declare_dec_cst(&mut self, dec: &Declaration) {
        let name = &Self::cst_dec_name(dec);
        let typ = dec.get_type(self.model);
        let sort = self.to_sort(&typ);
        self.smt.declare_const(&name, &sort).unwrap();
        if let Type::IntInterval(min, max) = typ {
            self.smt.assert(&format!("(>= {} {})", name, min)).unwrap();
            self.smt.assert(&format!("(<= {} {})", name, max)).unwrap();
        }
    }

    fn declare_dec_csts(&mut self) {
        for d in self.model.declarations().iter() {
            if d.is_constant() {
                self.declare_dec_cst(d);
            }
        }
    }

    //------------------------- Cst Function -------------------------

    fn declare_fun_cst(&mut self, fun: &FunDec) {
        let name = Self::cst_fun_name(fun);
        let params = fun
            .parameters()
            .iter()
            .map(|p| self.to_sort(&p.get_type(self.model)).to_string())
            .collect::<Vec<_>>();
        let params = params.iter().map(|p| p.as_str()).collect::<Vec<_>>();
        let typ = fun.return_type().get_type(self.model);
        let sort = self.to_sort(&typ);
        self.smt.declare_fun(&name, &params, &sort).unwrap();
        if let Type::IntInterval(min, max) = typ {
            let fun_params: Vec<Expr> = fun.parameters().iter().map(|p| p.clone().into()).collect();
            let fun_app = Expr::apply(fun.id(), fun_params);
            let min_e = fun_app.clone().ge(min.into());
            let max_e = fun_app.le(max.into());
            let e = Expr::and(vec![min_e, max_e]);
            let e = QtOperator::Forall.new(fun.parameters().clone(), e).into();
            let smt = self.to_smt(&e, 0);
            self.smt.assert(&smt).unwrap();
        }
    }

    fn declare_fun_csts(&mut self) {
        for fun in self.model.fun_decs().iter() {
            if fun.is_constant() {
                self.declare_fun_cst(fun);
            }
        }
    }

    //------------------------- Var Declaration -------------------------

    fn declare_dec_var(&mut self, dec: &Declaration, state: usize) {
        let name = Self::var_dec_name(dec, state);
        let typ = dec.get_type(self.model);
        let sort = self.to_sort(&typ);
        self.smt.declare_const(&name, &sort).unwrap();
        if let Type::IntInterval(min, max) = typ {
            self.smt.assert(&format!("(>= {} {})", name, min)).unwrap();
            self.smt.assert(&format!("(<= {} {})", name, max)).unwrap();
        }
    }

    fn declare_dec_vars(&mut self, state: usize) {
        for d in self.model.declarations().iter() {
            if !d.is_constant() {
                self.declare_dec_var(d, state);
            }
        }
    }

    //------------------------- Var Function -------------------------

    fn declare_fun_var(&mut self, fun: &FunDec, state: usize) {
        let name = Self::var_fun_name(fun, state);
        let params = fun
            .parameters()
            .iter()
            .map(|p| self.to_sort(&p.get_type(self.model)).to_string())
            .collect::<Vec<_>>();
        let params = params.iter().map(|p| p.as_str()).collect::<Vec<_>>();
        let typ = fun.return_type().get_type(self.model);
        let sort = self.to_sort(&typ);
        self.smt.declare_fun(&name, &params, &sort).unwrap();
        if let Type::IntInterval(min, max) = typ {
            let fun_params: Vec<Expr> = fun.parameters().iter().map(|p| p.clone().into()).collect();
            let fun_app = Expr::apply(fun.id(), fun_params);
            let min_e = fun_app.clone().ge(min.into());
            let max_e = fun_app.le(max.into());
            let e = Expr::and(vec![min_e, max_e]);
            let e = QtOperator::Forall.new(fun.parameters().clone(), e).into();
            let smt = self.to_smt(&e, state);
            self.smt.assert(&smt).unwrap();
        }
    }

    fn declare_fun_vars(&mut self, state: usize) {
        for fun in self.model.fun_decs().iter() {
            if !fun.is_constant() {
                self.declare_fun_var(fun, state);
            }
        }
    }

    //------------------------- State Unicity -------------------------

    fn add_state_diff(&mut self, first: usize, second: usize) {
        let mut disj = "(or".to_string();
        for id in self.model.var_declaration_ids() {
            // v[first] != v[second]
            let v_first = self.var_dec_name_from_id(id, first);
            let v_second = self.var_dec_name_from_id(id, second);
            let e = format!("(not (= {} {}))", v_first, v_second);
            disj += &format!(" {}", e);
        }
        for var in self.model.ltl_variables() {
            if !var.is_loop() {
                // v[first] != v[second]
                let v_first = Self::ltl_var_name(var, first);
                let v_second = Self::ltl_var_name(var, second);
                let e = format!("(not (= {} {}))", v_first, v_second);
                disj += &format!(" {}", e);
            }
        }
        for id in self.model.var_function_ids() {
            let fun = self.model.get(id).unwrap();
            let params = fun
                .parameters()
                .iter()
                .map(|p| p.clone().into())
                .collect::<Vec<_>>();
            let e_first = Expr::apply(id, params.clone()).state(first);
            let e_second = Expr::apply(id, params).state(second);
            let e = Expr::exists(fun.parameters().clone(), e_first.ne(e_second));
            let e = &self.to_smt(&e, 0);
            disj += &format!(" {}", e);
        }
        disj += ")";
        self.smt.assert(&disj).unwrap();
    }

    fn add_state_unicity_with_previous(&mut self, state: usize) {
        for first in 0..state {
            self.add_state_diff(first, state);
        }
    }

    fn state_equality(&self, first: usize, second: usize) -> String {
        let mut conj = "(and true".to_string();
        for id in self.model.var_declaration_ids() {
            // v[first] = v[second]
            let v_first = self.var_dec_name_from_id(id, first);
            let v_second = self.var_dec_name_from_id(id, second);
            let e = format!("(= {} {})", v_first, v_second);
            conj += &format!(" {}", e);
        }
        for id in self.model.var_function_ids() {
            let fun = self.model.get(id).unwrap();
            let params = fun
                .parameters()
                .iter()
                .map(|p| p.clone().into())
                .collect::<Vec<_>>();
            let e_first = Expr::apply(id, params.clone()).state(first);
            let e_second = Expr::apply(id, params).state(second);
            let e = Expr::forall(fun.parameters().clone(), e_first.eq(e_second));
            let e = &self.to_smt(&e, 0);
            conj += &format!(" {}", e);
        }
        for var in self.model.ltl_variables() {
            if !var.is_loop() {
                // v[first] != v[second]
                let v_first = Self::ltl_var_name(var, first);
                let v_second = Self::ltl_var_name(var, second);
                let e = format!("(= {} {})", v_first, v_second);
                conj += &format!(" {}", e);
            }
        }
        conj += ")";
        conj
    }

    //------------------------- Loop -------------------------

    fn declare_loop(&mut self) {
        let mut loops = vec![];
        for state in 0..self.transitions {
            let name = Self::loop_name(state);
            self.smt.declare_const(&name, "Bool").unwrap();
            loops.push(name);
        }
        let mut e = "((_ pbeq 1".to_string();
        for _ in 0..loops.len() {
            e += " 1";
        }
        e += ")";
        for l in loops.iter() {
            e += &format!(" {}", l);
        }
        e += ")";
        self.smt.assert(&e).unwrap()
    }

    fn define_loop(&mut self) {
        for state in 0..self.transitions {
            // l_id = state_equality(id, last)
            let l = Self::loop_name(state);
            let e = self.state_equality(state, self.states() - 1);
            let phi = format!("(= {} {})", l, e);
            self.smt.assert(&phi).unwrap()
        }
    }

    pub fn get_loop_index(&mut self) -> Option<usize> {
        if self.with_loop {
            for state in 0..self.transitions {
                let l = Self::loop_name(state);
                let e = self.smt.eval(&l).unwrap();
                if e == "true" {
                    return Some(state);
                }
            }
            panic!()
        } else {
            None
        }
    }

    //------------------------- Init -------------------------

    fn define_init(&mut self, init: &Initial) {
        let x = self.to_smt(init.expr(), 0);
        self.smt.assert(&x).unwrap()
    }

    fn define_inits(&mut self) {
        for i in self.model.initials() {
            self.define_init(i);
        }
    }

    //------------------------- Inv -------------------------

    fn define_invariant(&mut self, inv: &Invariant, state: usize) {
        let x = self.to_smt(inv.expr(), state);
        self.smt.assert(&x).unwrap();
    }

    fn define_invariants(&mut self, state: usize) {
        for i in self.model.invariants() {
            self.define_invariant(i, state);
        }
    }

    //------------------------- Trans -------------------------

    fn define_transitions(&mut self, state: usize) {
        let mut v = vec![];
        for t in self.model.transitions() {
            v.push(t.expr().clone());
        }
        let len = v.len();
        if len == 1 {
            let e = &v[0];
            self.smt.assert(&self.to_smt(&e, state)).unwrap();
        } else if len > 1 {
            let e = Expr::or(v);
            self.smt.assert(&self.to_smt(&e, state)).unwrap();
        } else {
            self.smt.assert("false").unwrap();
        }
    }

    //------------------------- LTL Variable -------------------------

    fn declare_ltl_var(&mut self, var: &LTLVariable, state: usize) {
        let name = Self::ltl_var_name(var, state);
        if state == 0 {
            self.smt.add_comment(&var.to_lang(self.model)).unwrap();
        }
        self.smt.declare_const(&name, "Bool").unwrap();
    }

    fn declare_ltl_non_loop_vars(&mut self, state: usize) {
        for v in self.model.ltl_variables().iter() {
            if !v.is_loop() {
                self.declare_ltl_var(v, state);
            }
        }
    }

    fn declare_ltl_loop_vars(&mut self, state: usize) {
        for v in self.model.ltl_variables().iter() {
            if v.is_loop() {
                self.declare_ltl_var(v, state);
            }
        }
    }

    fn define_ltl_var(&mut self, var: &LTLVariable, state: usize) {
        let v = Self::ltl_var_name(var, state);

        match var.expr().expression() {
            Expression::LTLunary(op, kid) => match op {
                LTLUnaryOperator::X => {
                    // v[s] = kid[s+1]
                    let kid_next = self.to_smt(&kid, state + 1);
                    let phi = format!("(= {} {})", v, kid_next);
                    self.smt.assert(&phi).unwrap();
                }
                LTLUnaryOperator::F => {
                    // v[s] = kid[s] or v[s+1]
                    let kid = self.to_smt(&kid, state);
                    let v_next = self.to_smt(&var.id().into(), state + 1);
                    let phi = format!("(= {} (or {} {}))", v, kid, v_next);
                    self.smt.assert(&phi).unwrap();
                }
                LTLUnaryOperator::G => {
                    // v[s] = kid[s] and v[s+1]
                    let kid = self.to_smt(&kid, state);
                    let v_next = self.to_smt(&var.id().into(), state + 1);
                    let phi = format!("(= {} (and {} {}))", v, kid, v_next);
                    self.smt.assert(&phi).unwrap();
                }
                LTLUnaryOperator::_F_ => {
                    // v[s] = kid[s] or v[s+1]
                    let kid = self.to_smt(&kid, state);
                    let v_next = self.to_smt(&var.id().into(), state + 1);
                    let phi = format!("(= {} (or {} {}))", v, kid, v_next);
                    self.smt.assert(&phi).unwrap();
                }
                LTLUnaryOperator::_G_ => {
                    // v[s] = kid[s] and v[s+1]
                    let kid = self.to_smt(&kid, state);
                    let v_next = self.to_smt(&var.id().into(), state + 1);
                    let phi = format!("(= {} (and {} {}))", v, kid, v_next);
                    self.smt.assert(&phi).unwrap();
                }
            },
            Expression::LTLbinary(left, op, right) => match op {
                LTLBinaryOperator::U => {
                    // v[s] = right[s] or (left[s] and v[s+1])
                    let left = self.to_smt(&left, state);
                    let right = self.to_smt(&right, state);
                    let v_next = self.to_smt(&var.id().into(), state + 1);
                    let phi = format!("(= {} (or {} (and {} {})))", v, right, left, v_next);
                    self.smt.assert(&phi).unwrap();
                }
                LTLBinaryOperator::R => {
                    // v[s] = right[s] and (left[s] or v[s+1])
                    let left = self.to_smt(&left, state);
                    let right = self.to_smt(&right, state);
                    let v_next = self.to_smt(&var.id().into(), state + 1);
                    let phi = format!("(= {} (and {} (or {} {})))", v, right, left, v_next);
                    self.smt.assert(&phi).unwrap();
                }
                LTLBinaryOperator::_U_ => {
                    // v[s] = right[s] or (left[s] and v[s+1])
                    let left = self.to_smt(&left, state);
                    let right = self.to_smt(&right, state);
                    let v_next = self.to_smt(&var.id().into(), state + 1);
                    let phi = format!("(= {} (or {} (and {} {})))", v, right, left, v_next);
                    self.smt.assert(&phi).unwrap();
                }
                LTLBinaryOperator::_R_ => {
                    // v[s] = right[s] and (left[s] or v[s+1])
                    let left = self.to_smt(&left, state);
                    let right = self.to_smt(&right, state);
                    let v_next = self.to_smt(&var.id().into(), state + 1);
                    let phi = format!("(= {} (and {} (or {} {})))", v, right, left, v_next);
                    self.smt.assert(&phi).unwrap();
                }
            },
            _ => panic!(),
        }
    }

    // fn define_ltl_var_future(&mut self, var: &LTLVariable, state: usize) {
    //     let v = Self::ltl_var_name(var, state);
    //     match var.expr().expression() {
    //         Expression::LTLunary(op, kid) => match op {
    //             LTLUnaryOperator::X => {
    //                 // v[s] = kid[s+1]
    //                 let kid_next = self.to_smt(&kid, state + 1);
    //                 let phi = format!("(= {} {})", v, kid_next);
    //                 self.smt.assert(&phi).unwrap();
    //             }
    //             LTLUnaryOperator::F => {
    //                 // v[s] = kid[s] or v[s+1]
    //                 let kid = self.to_smt(&kid, state);
    //                 let v_next = self.to_smt(&var.id().into(), state + 1);
    //                 let phi = format!("(= {} (or {} {}))", v, kid, v_next);
    //                 self.smt.assert(&phi).unwrap();
    //             }
    //             LTLUnaryOperator::G => {
    //                 // v[s] = kid[s]
    //                 let kid = self.to_smt(&kid, state);
    //                 let phi = format!("(= {} {})", v, kid);
    //                 self.smt.assert(&phi).unwrap();
    //             }
    //             LTLUnaryOperator::_F_ => panic!(),
    //             LTLUnaryOperator::_G_ => panic!(),
    //         },
    //         Expression::LTLbinary(left, op, right) => match op {
    //             LTLBinaryOperator::U => {
    //                 // v[s] = right[s] or (left[s] and v[s+1])
    //                 let left = self.to_smt(&left, state);
    //                 let right = self.to_smt(&right, state);
    //                 let v_next = self.to_smt(&var.id().into(), state + 1);
    //                 let phi = format!("(= {} (or {} (and {} {})))", v, right, left, v_next);
    //                 self.smt.assert(&phi).unwrap();
    //             }
    //             LTLBinaryOperator::R => {
    //                 // v[s] = right[s]
    //                 let right = self.to_smt(&right, state);
    //                 let phi = format!("(= {} {})", v, right);
    //                 self.smt.assert(&phi).unwrap();
    //             }
    //             LTLBinaryOperator::_U_ => panic!(),
    //             LTLBinaryOperator::_R_ => panic!(),
    //         },
    //         _ => panic!(),
    //     }
    // }

    fn define_ltl_var_finite(&mut self, var: &LTLVariable, state: usize) {
        let v = Self::ltl_var_name(var, state);
        match var.expr().expression() {
            Expression::LTLunary(op, kid) => match op {
                LTLUnaryOperator::X => {
                    // v[s] = false
                    let phi = format!("(= {} false)", v);
                    self.smt.assert(&phi).unwrap();
                }
                LTLUnaryOperator::F => {
                    // v[s] = kid[s]
                    let kid = self.to_smt(&kid, state);
                    let phi = format!("(= {} {})", v, kid);
                    self.smt.assert(&phi).unwrap();
                }
                LTLUnaryOperator::G => {
                    // v[s] = kid[s]
                    let kid = self.to_smt(&kid, state);
                    let phi = format!("(= {} {})", v, kid);
                    self.smt.assert(&phi).unwrap();
                }
                LTLUnaryOperator::_F_ => panic!(),
                LTLUnaryOperator::_G_ => panic!(),
            },
            Expression::LTLbinary(_, op, right) => match op {
                LTLBinaryOperator::U => {
                    // v[s] = right[s]
                    let right = self.to_smt(&right, state);
                    let phi = format!("(= {} {})", v, right);
                    self.smt.assert(&phi).unwrap();
                }
                LTLBinaryOperator::R => {
                    // v[s] = right[s]
                    let right = self.to_smt(&right, state);
                    let phi = format!("(= {} {})", v, right);
                    self.smt.assert(&phi).unwrap();
                }
                LTLBinaryOperator::_U_ => panic!(),
                LTLBinaryOperator::_R_ => panic!(),
            },
            _ => panic!(),
        }
    }

    fn define_ltl_var_loop(&mut self, var: &LTLVariable, state: usize) {
        let v = Self::ltl_var_name(var, state);
        match var.expr().expression() {
            Expression::LTLunary(op, kid) => match op {
                LTLUnaryOperator::X => {
                    // v[s] = Or_i (l_i and kid[i+1])
                    let mut disj = "(or".to_string();
                    for state in 0..self.transitions {
                        let l = Self::loop_name(state);
                        let kid_next = self.to_smt(kid, state + 1);
                        let e = format!(" (and {} {})", l, kid_next);
                        disj += &e;
                    }
                    disj += ")";
                    self.smt.assert(&format!("(= {} {})", v, disj)).unwrap();
                }
                LTLUnaryOperator::F => {
                    // v[s] = Or_i (l_i and _F_(kid)[i])
                    let mut disj = "(or".to_string();
                    for state in 0..self.transitions {
                        let l = Self::loop_name(state);
                        let f = LTLUnaryOperator::_F_.new(*kid.clone()).into();
                        let f = self.model.get_ltl_expr(f);
                        let f = self.to_smt(&f, state);
                        let e = format!(" (and {} {})", l, f);
                        disj += &e;
                    }
                    disj += ")";
                    self.smt.assert(&format!("(= {} {})", v, disj)).unwrap();
                }
                LTLUnaryOperator::G => {
                    // v[s] = Or_i (l_i and _G_(kid)[i])
                    let mut disj = "(or".to_string();
                    for state in 0..self.transitions {
                        let l = Self::loop_name(state);
                        let f: Expr = LTLUnaryOperator::_G_.new(*kid.clone()).into();
                        let f = self.model.get_ltl_expr(f);
                        let f = self.to_smt(&f, state);
                        let e = format!(" (and {} {})", l, f);
                        disj += &e;
                    }
                    disj += ")";
                    self.smt.assert(&format!("(= {} {})", v, disj)).unwrap();
                }
                LTLUnaryOperator::_F_ => {
                    // v[s] = false
                    self.smt.assert(&format!("(= {} false)", v)).unwrap();
                }
                LTLUnaryOperator::_G_ => {
                    // v[s] = true
                    self.smt.assert(&format!("(= {} true)", v)).unwrap();
                }
            },
            Expression::LTLbinary(left, op, right) => match op {
                LTLBinaryOperator::U => {
                    // v[s] = Or_i (l_i and _U_(left, right)[i])
                    let mut disj = "(or".to_string();
                    for state in 0..self.transitions {
                        let l = Self::loop_name(state);
                        let f = LTLBinaryOperator::_U_
                            .new(*left.clone(), *right.clone())
                            .into();
                        let f = self.model.get_ltl_expr(f);
                        let f = self.to_smt(&f, state);
                        let e = format!(" (and {} {})", l, f);
                        disj += &e;
                    }
                    disj += ")";
                    self.smt.assert(&format!("(= {} {})", v, disj)).unwrap();
                }
                LTLBinaryOperator::R => {
                    // v[s] = Or_i (l_i and _R_(left, right)[i])
                    let mut disj = "(or".to_string();
                    for state in 0..self.transitions {
                        let l = Self::loop_name(state);
                        let f = LTLBinaryOperator::_R_
                            .new(*left.clone(), *right.clone())
                            .into();
                        let f = self.model.get_ltl_expr(f);
                        let f = self.to_smt(&f, state);
                        let e = format!(" (and {} {})", l, f);
                        disj += &e;
                    }
                    disj += ")";
                    self.smt.assert(&format!("(= {} {})", v, disj)).unwrap();
                }
                LTLBinaryOperator::_U_ => {
                    // v[s] = false
                    self.smt.assert(&format!("(= {} false)", v)).unwrap();
                }
                LTLBinaryOperator::_R_ => {
                    // v[s] = true
                    self.smt.assert(&format!("(= {} true)", v)).unwrap();
                }
            },
            _ => panic!(),
        }
    }

    fn define_ltl_non_loop_vars(&mut self, state: usize) {
        for v in self.model.ltl_variables().iter() {
            if !v.is_loop() {
                self.define_ltl_var(v, state);
            }
        }
    }

    fn define_ltl_loop_vars(&mut self, state: usize) {
        for v in self.model.ltl_variables().iter() {
            if v.is_loop() {
                self.define_ltl_var(v, state);
            }
        }
    }

    // fn define_ltl_vars_future(&mut self, state: usize) {
    //     for v in self.model.ltl_variables().iter() {
    //         if !v.is_loop() {
    //             self.define_ltl_var_future(v, state);
    //         }
    //     }
    // }

    fn define_ltl_vars_finite(&mut self, state: usize) {
        for v in self.model.ltl_variables().iter() {
            if !v.is_loop() {
                self.define_ltl_var_finite(v, state);
            }
        }
    }

    fn define_ltl_vars_loop(&mut self, state: usize) {
        for v in self.model.ltl_variables().iter() {
            self.define_ltl_var_loop(v, state);
        }
    }

    //------------------------- Property -------------------------

    fn add_property(&mut self) {
        let opt: Option<&Declaration> = self.model.from_name("prop");
        if let Some(prop) = opt {
            self.smt
                .add_comment("---------- Add Property ----------")
                .unwrap();
            let expr = prop.id().into();
            let e = self.to_smt(&expr, 0);
            self.smt.assert(&e).unwrap();
        }
        // TODO: add path semantic: finite/truncated/sequence or loop
    }

    //------------------------- Expr -------------------------

    pub fn to_smt(&self, expr: &Expr, state: usize) -> String {
        match expr.expression() {
            Expression::Bool(value) => format!("{}", value),
            Expression::Int(value) => format!("{}", value),
            Expression::Real(_) => todo!(),

            Expression::PrefixUnary(op, kid) => match op {
                PrefixUnaryOperator::Not => format!("(not {})", self.to_smt(kid, state)),
                PrefixUnaryOperator::Neg => format!("(- {})", self.to_smt(kid, state)),
            },
            Expression::Binary(left, op, right) => {
                let left = self.to_smt(left, state);
                let right = self.to_smt(right, state);
                match op {
                    BinaryOperator::Eq => format!("(= {} {})", left, right),
                    BinaryOperator::Ne => format!("(not (= {} {}))", left, right),
                    BinaryOperator::Lt => format!("(< {} {})", left, right),
                    BinaryOperator::Le => format!("(<= {} {})", left, right),
                    BinaryOperator::Ge => format!("(>= {} {})", left, right),
                    BinaryOperator::Gt => format!("(> {} {})", left, right),
                    BinaryOperator::Implies => format!("(=> {} {})", left, right),
                    BinaryOperator::Min => {
                        format!("(ite (<= {} {}) {} {})", left, right, left, right)
                    }
                    BinaryOperator::Max => {
                        format!("(ite (>= {} {}) {} {})", left, right, left, right)
                    }
                }
            }
            Expression::Nary(op, list) => {
                let mut l = "".to_string();
                for e in list.iter() {
                    let e = self.to_smt(e, state);
                    l += &format!(" {}", e);
                }
                match op {
                    NaryOperator::And => format!("(and{})", l),
                    NaryOperator::Or => format!("(or{})", l),
                    NaryOperator::Add => format!("(+{})", l),
                    NaryOperator::Sub => format!("(-{})", l),
                    NaryOperator::Mul => format!("(*{})", l),
                }
            }
            Expression::EnumerateElement(id) => self.model.get(*id).unwrap().name().to_string(),
            Expression::Declaration(id) => {
                let dec = self.model.get(*id).unwrap();
                if dec.is_constant() {
                    Self::cst_dec_name(dec)
                } else {
                    Self::var_dec_name(dec, state)
                }
            }
            Expression::Definition(_) => panic!(),
            Expression::FunDec(_) => todo!(),
            Expression::FunDef(_) => todo!(),
            Expression::Parameter(_) => todo!(),
            Expression::Apply(fun, params) => match fun.expression() {
                Expression::FunDec(id) => {
                    let f = self.model.get(*id).unwrap();
                    let mut res = if f.is_constant() {
                        Self::cst_fun_name(f)
                    } else {
                        Self::var_fun_name(f, state)
                    };
                    for p in params.iter() {
                        res += &format!(" {}", self.to_smt(p, state));
                    }
                    format!("({})", res)
                }
                _ => panic!(),
            },
            Expression::As(kid, typ, default) => {
                if let Type::IntInterval(min, max) = typ {
                    let k = self.to_smt(kid, state);
                    let d = self.to_smt(default, state);
                    format!(
                        "(ite (and (>= {} {}) (<= {} {})) {} {})",
                        k, min, k, max, k, d
                    )
                } else {
                    panic!()
                }
            }
            //
            Expression::Following(kid) => self.to_smt(kid, state + 1),
            Expression::State(expr, state_expr, default) => {
                let state_index = match state_expr.state() {
                    State::First => state_expr.shift(),
                    State::Current => (state as isize) + state_expr.shift(),
                    State::Last => self.transitions as isize,
                };
                if state_index >= 0 && state_index <= self.transitions as isize {
                    self.to_smt(expr, state_index as usize)
                } else {
                    let default = default.as_ref().unwrap();
                    self.to_smt(default, state)
                }
            }
            Expression::Scope(l, e) => {
                let mut v = vec![];
                for dec in self.model.declarations() {
                    let x: Expr = Expression::Declaration(dec.id()).into();
                    if !l.iter().any(|y| x.is_same(y)) {
                        let expr = Expression::Binary(
                            Box::new(x.clone()),
                            BinaryOperator::Eq,
                            Box::new(Expression::Following(Box::new(x.clone())).into()),
                        );
                        v.push(expr.into());
                    }
                }
                v.push(*e.clone());
                let expr = Expression::Nary(NaryOperator::And, v).into();
                self.to_smt(&expr, state)
            }
            //
            Expression::IfThenElse(c, t, list, e) => {
                let c = self.to_smt(c, state);
                let t = self.to_smt(t, state);
                let l = list
                    .iter()
                    .map(|(c, e)| (self.to_smt(c, state), self.to_smt(e, state)))
                    .collect::<Vec<_>>();
                let e = self.to_smt(e, state);
                let mut res = e;
                for (x, y) in l.iter().rev() {
                    res = format!("(ite {} {} {})", x, y, res);
                }
                res = format!("(ite {} {} {})", c, t, res);
                res
            }
            Expression::Quantifier(op, params, e) => {
                let exprs = Expr::combine_all(self.model, params, e);
                let mut res = "(".to_string();
                res += match op {
                    QtOperator::Forall => "and",
                    QtOperator::Exists => "or",
                    QtOperator::Sum => "+",
                    QtOperator::Prod => "*",
                    QtOperator::Min => todo!(),
                    QtOperator::Max => todo!(),
                };
                for e in exprs.iter() {
                    res += &format!(" {}", self.to_smt(e, state));
                }
                res + ")"
            }
            //
            Expression::LTLunary(_, _) => panic!(),
            Expression::LTLbinary(_, _, _) => panic!(),
            Expression::LTLVariable(id) => Self::ltl_var_name_from_id(self, *id, state),
            //
            Expression::Unresolved(_) => panic!(),
        }
    }

    //-------------------------  -------------------------

    pub fn create_states(&mut self, number: usize) {
        self.smt
            .add_comment("---------- Constant ----------")
            .unwrap();
        self.declare_dec_csts();
        self.declare_fun_csts();

        for state in 0..number {
            self.smt
                .add_comment(&format!("---------- State {} ----------", state))
                .unwrap();
            // Var
            self.declare_dec_vars(state);
            self.declare_fun_vars(state);
            // LTL Variables
            self.declare_ltl_non_loop_vars(state);
        }

        // Init
        self.smt.add_comment("---------- Init ----------").unwrap();
        self.define_inits();

        // Invariants
        for state in 0..self.states() {
            self.smt
                .add_comment(&format!("---------- Invariant {} ----------", state))
                .unwrap();
            self.define_invariants(state);
        }
    }

    //------------------------- Initialize -------------------------

    /**
     * create trace without the last state:
     * - declare constants
     * - declare vars for all states (last included) with non_loop_ltl_vars
     * - define init
     * - define invariants
     * - define all transition (included last)
     * - add unicity if needed
     * - define ltl semantic until last (excluded)
     */
    pub fn create_path(&mut self, transitions: usize) {
        self.transitions = transitions;
        // Enum
        self.smt
            .add_comment("---------- Enumerate ----------")
            .unwrap();
        self.declare_enumerates();
        // Cst
        self.smt
            .add_comment("---------- Constant ----------")
            .unwrap();
        self.declare_dec_csts();
        self.declare_fun_csts();

        for state in 0..self.states() {
            self.smt
                .add_comment(&format!("---------- State {} ----------", state))
                .unwrap();
            // Var
            self.declare_dec_vars(state);
            self.declare_fun_vars(state);
            // LTL Variables
            self.declare_ltl_non_loop_vars(state);
        }

        // Init
        self.smt.add_comment("---------- Init ----------").unwrap();
        self.define_inits();

        // Invariants
        for state in 0..self.states() {
            self.smt
                .add_comment(&format!("---------- Invariant {} ----------", state))
                .unwrap();
            self.define_invariants(state);
        }

        // Transition
        for state in 0..self.states() - 1 {
            self.smt
                .add_comment(&format!("---------- Transition {} ----------", state))
                .unwrap();
            self.define_transitions(state);
        }

        // LTL Variables: classical semantic until last
        for state in 0..self.states() - 1 {
            self.smt
                .add_comment(&format!("---------- LTL {} ----------", state))
                .unwrap();
            self.define_ltl_non_loop_vars(state);
        }
    }

    pub fn add_unicity(&mut self) {
        for state in 1..self.states() {
            self.smt
                .add_comment(&format!("---------- Unicity {} ----------", state))
                .unwrap();
            self.add_state_unicity_with_previous(state);
        }
    }

    pub fn add_last_ltl_semantic(&mut self) {
        let future_state = self.states();

        self.smt
            .add_comment(&format!(
                "---------- declare LTL future last {} ----------",
                future_state
            ))
            .unwrap();
        self.declare_ltl_non_loop_vars(future_state);

        let state = future_state - 1;
        self.smt
            .add_comment(&format!("---------- LTL {} ----------", state))
            .unwrap();
        self.define_ltl_non_loop_vars(state);
    }

    pub fn add_last_finite_semantic(&mut self) {
        // LTL finite
        let last_state = self.states() - 1;
        self.smt
            .add_comment(&format!("---------- LTL finite {} ----------", last_state))
            .unwrap();
        self.define_ltl_vars_finite(last_state);
    }

    pub fn add_last_infinite_semantic(&mut self) {
        self.with_loop = true;
        //
        let loop_state = self.states() - 1;

        // declare LTL_loop
        for state in 0..=loop_state {
            self.smt
                .add_comment(&format!("---------- LTL loop vars {} ----------", state))
                .unwrap();
            self.declare_ltl_loop_vars(state);
        }
        // declare & define loop (l_i)
        self.smt
            .add_comment("---------- declare loop ----------")
            .unwrap();
        self.declare_loop();
        self.define_loop();

        // define LTL_loop
        for state in 0..loop_state {
            self.smt
                .add_comment(&format!("---------- define loop vars {} ----------", state))
                .unwrap();
            self.define_ltl_loop_vars(state);
        }
        // define LTL infinte semantic on last
        self.smt
            .add_comment(&format!(
                "---------- LTL infinite {} ----------",
                loop_state
            ))
            .unwrap();
        self.define_ltl_vars_loop(loop_state); // loop semantic
    }

    //------------------------- Future -------------------------

    pub fn create_future(&mut self, transitions: usize) {
        self.create_path(transitions);
        self.with_loop = false; // TODO: ????

        // last = LTL
        self.add_last_ltl_semantic();
        // Unicity
        self.add_unicity();
    }

    //------------------------- Truncated -------------------------

    pub fn create_truncated(&mut self, transitions: usize) {
        self.create_path(transitions);
        self.with_loop = false; // TODO: ????

        // last = Finite
        self.add_last_finite_semantic();
        // Property
        self.add_property();
    }

    //------------------------- Infinite -------------------------

    pub fn create_infinite(&mut self, transitions: usize) {
        self.create_path(transitions);

        // last = Infinite
        self.add_last_infinite_semantic();
        // Property
        self.add_property();
    }

    //------------------------- Finite -------------------------

    pub fn create_finite(&mut self, transitions: usize, solutions: &Vec<Solution>) {
        self.create_path(transitions);
        self.with_loop = false; // TODO: ????

        // last = Finite
        self.add_last_finite_semantic();
        // Property
        self.add_property();
        // Remove previous solutions
        self.remove_solutions(solutions);
    }

    //------------------------- Finite Check -------------------------

    pub fn create_finite_future(&mut self, transitions: usize, solution: &Solution) {
        self.create_path(transitions);
        self.with_loop = false; // TODO: ????

        // last = LTL
        self.add_last_ltl_semantic();
        // set solution
        self.set_solution(solution);
    }

    //-------------------------  -------------------------

    fn remove_solution(&mut self, solution: &Solution) {
        let mut conj = vec![];
        // Cst
        for (id, expr) in solution.cst_dec.iter() {
            if let Some(expr) = expr {
                let dec: Expr = (*id).into();
                let e = dec.eq(expr.clone());
                conj.push(e);
            }
        }
        // Var
        for (id, v) in solution.var_dec.iter() {
            for state in 0..solution.states {
                if let Some(expr) = &v[state] {
                    let dec: Expr = (*id).into();
                    let e = dec.state(state).eq(expr.clone());
                    conj.push(e);
                }
            }
        }
        let e = Expr::and(conj).not();
        self.smt.assert(&self.to_smt(&e, 0)).unwrap();
    }

    fn set_solution(&mut self, solution: &Solution) {
        // Cst
        for (id, opt) in solution.cst_dec.iter() {
            if let Some(expr) = opt {
                let dec: Expr = (*id).into();
                let e = dec.eq(expr.clone());
                self.smt.assert(&self.to_smt(&e, 0)).unwrap();
            }
        }
        // Var
        for (id, v) in solution.var_dec.iter() {
            for state in 0..solution.states {
                if let Some(expr) = &v[state] {
                    let dec: Expr = (*id).into();
                    let e = dec.eq(expr.clone());
                    self.smt.assert(&self.to_smt(&e, state)).unwrap();
                }
            }
        }
    }

    fn remove_solutions(&mut self, solutions: &Vec<Solution>) {
        for solution in solutions.iter() {
            self.remove_solution(solution);
        }
    }

    //------------------------- Incremental -------------------------

    pub fn push(&mut self) {
        // let tactic = "(repeat (then propagate-values simplify solve-eqs))";
        // let tactic = "(then propagate-values solve-eqs)";
        // self.smt.apply(tactic).unwrap();
        self.smt.push().unwrap();
    }

    pub fn pop(&mut self) {
        self.smt.pop().unwrap();
    }

    //------------------------- Solve -------------------------

    pub fn check(&mut self) -> SatResult {
        self.smt
            .add_comment("---------- Check Sat ----------")
            .unwrap();
        let tactic = "(then (repeat (then propagate-ineqs simplify propagate-values solve-eqs elim-uncnstr)) smt)";
        let res = self.smt.check_sat_using(tactic).unwrap();
        res
    }

    pub fn eval(&mut self, expr: &Expr, state: usize) -> Option<Expr> {
        let e = self.to_smt(expr, state).trim().to_string();
        let eval = self.smt.eval(&e).unwrap().trim().to_string();
        let eval = eval.replace(&['(', ')', ' '][..], "");
        // println!("> {} = {}", e, eval);
        if e == eval {
            None
        } else {
            Some(match expr.get_type(self.model) {
                crate::typing::typ::Type::Enumerate(id) => {
                    let enumerate = self.model.get(id).unwrap();
                    let element = enumerate.from_name(&eval).unwrap();
                    Expression::EnumerateElement(element.id()).into()
                }
                crate::typing::typ::Type::Bool => eval.parse::<bool>().unwrap().into(),
                crate::typing::typ::Type::Int => eval.parse::<i64>().unwrap().into(),
                crate::typing::typ::Type::Real => todo!(),
                crate::typing::typ::Type::IntInterval(_, _) => eval.parse::<i64>().unwrap().into(),
                //
                crate::typing::typ::Type::Undefined => panic!(),
                crate::typing::typ::Type::Unresolved(_, _) => panic!(),
                crate::typing::typ::Type::Interval(_) => panic!(),
                crate::typing::typ::Type::Function(_, _) => panic!(),
            })
        }
    }
}
