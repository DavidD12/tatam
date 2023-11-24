use super::*;
use crate::common::*;
use crate::expr::*;
use crate::model::*;
use crate::typing::*;
use std::collections::HashMap;
use z3::ast::Ast;

#[derive(Clone)]
pub struct Smt<'a> {
    model: &'a Model,
    transitions: usize,
    unicity: bool,
    // has_loop: bool,
    //
    _cfg: &'a z3::Config,
    ctx: &'a z3::Context,
    solver: &'a Z3Solver<'a>,
    // Enum
    enum_sort: HashMap<EnumerateId, z3::Sort<'a>>,
    enum_elts: HashMap<EnumerateElementId, z3::ast::Datatype<'a>>,
    // Constant
    enum_cst: HashMap<DeclarationId, z3::ast::Datatype<'a>>,
    bool_cst: HashMap<DeclarationId, z3::ast::Bool<'a>>,
    int_cst: HashMap<DeclarationId, z3::ast::Int<'a>>,
    real_cst: HashMap<DeclarationId, z3::ast::Real<'a>>,
    // Variables
    enum_var: HashMap<DeclarationId, Vec<z3::ast::Datatype<'a>>>,
    bool_var: HashMap<DeclarationId, Vec<z3::ast::Bool<'a>>>,
    int_var: HashMap<DeclarationId, Vec<z3::ast::Int<'a>>>,
    real_var: HashMap<DeclarationId, Vec<z3::ast::Real<'a>>>,
    // LTL
    ltl_var: HashMap<LTLVariableId, Vec<z3::ast::Bool<'a>>>,
    // Loop
    loops: Vec<z3::ast::Bool<'a>>,
}

impl<'a> Smt<'a> {
    pub fn empty(
        model: &'a Model,
        cfg: &'a z3::Config,
        ctx: &'a z3::Context,
        solver: &'a Z3Solver<'a>,
    ) -> Self {
        Self {
            model,
            transitions: 0,
            unicity: false,
            _cfg: cfg,
            ctx,
            solver,
            enum_sort: HashMap::new(),
            enum_elts: HashMap::new(),
            enum_cst: HashMap::new(),
            bool_cst: HashMap::new(),
            int_cst: HashMap::new(),
            real_cst: HashMap::new(),
            enum_var: HashMap::new(),
            bool_var: HashMap::new(),
            int_var: HashMap::new(),
            real_var: HashMap::new(),
            ltl_var: HashMap::new(),
            loops: Vec::new(),
        }
    }

    //------------------------- -------------------------

    pub fn model(&self) -> &Model {
        self.model
    }

    pub fn transitions(&self) -> usize {
        self.transitions
    }

    pub fn states(&self) -> usize {
        self.transitions + 1
        //     if self.has_loop {
        //         self.transitions + 2
        //     } else {
        //         self.transitions + 1
        //     }
    }

    // pub fn has_loop(&self) -> bool {
    //     self.has_loop
    // }

    pub fn ctx(&self) -> &'a z3::Context {
        self.ctx
    }

    pub fn solver(&self) -> &Z3Solver {
        &self.solver
    }

    //-------------------------  -------------------------

    pub fn enum_elt(&self, id: EnumerateElementId) -> &z3::ast::Datatype<'a> {
        self.enum_elts.get(&id).unwrap()
    }

    pub fn enum_cst(&self, id: DeclarationId) -> &z3::ast::Datatype<'a> {
        self.enum_cst.get(&id).unwrap()
    }

    pub fn bool_cst(&self, id: DeclarationId) -> &z3::ast::Bool<'a> {
        self.bool_cst.get(&id).unwrap()
    }

    pub fn int_cst(&self, id: DeclarationId) -> &z3::ast::Int<'a> {
        self.int_cst.get(&id).unwrap()
    }

    pub fn real_cst(&self, id: DeclarationId) -> &z3::ast::Real<'a> {
        self.real_cst.get(&id).unwrap()
    }

    pub fn enum_var(&self, id: DeclarationId, state: usize) -> &z3::ast::Datatype<'a> {
        let v = self.enum_var.get(&id).unwrap();
        v.get(state).unwrap()
    }

    pub fn bool_var(&self, id: DeclarationId, state: usize) -> &z3::ast::Bool<'a> {
        let v = self.bool_var.get(&id).unwrap();
        v.get(state).unwrap()
    }

    pub fn int_var(&self, id: DeclarationId, state: usize) -> &z3::ast::Int<'a> {
        let v = self.int_var.get(&id).unwrap();
        v.get(state).unwrap()
    }

    pub fn real_var(&self, id: DeclarationId, state: usize) -> &z3::ast::Real<'a> {
        let v = self.real_var.get(&id).unwrap();
        v.get(state).unwrap()
    }

    pub fn enum_dec(&self, id: DeclarationId, state: usize) -> &z3::ast::Datatype<'a> {
        let d = self.model.get(id).unwrap();
        if d.is_constant() {
            self.enum_cst(id)
        } else {
            self.enum_var(id, state)
        }
    }

    pub fn bool_dec(&self, id: DeclarationId, state: usize) -> &z3::ast::Bool<'a> {
        let d = self.model.get(id).unwrap();
        if d.is_constant() {
            self.bool_cst(id)
        } else {
            self.bool_var(id, state)
        }
    }

    pub fn int_dec(&self, id: DeclarationId, state: usize) -> &z3::ast::Int<'a> {
        let d = self.model.get(id).unwrap();
        if d.is_constant() {
            self.int_cst(id)
        } else {
            self.int_var(id, state)
        }
    }

    pub fn real_dec(&self, id: DeclarationId, state: usize) -> &z3::ast::Real<'a> {
        let d = self.model.get(id).unwrap();
        if d.is_constant() {
            self.real_cst(id)
        } else {
            self.real_var(id, state)
        }
    }

    pub fn ltl_var(&self, id: LTLVariableId, state: usize) -> &z3::ast::Bool<'a> {
        let v = self.ltl_var.get(&id).unwrap();
        v.get(state).unwrap()
    }

    pub fn name_state(name: &str, state: usize) -> String {
        format!("{}__{}", name, state)
    }

    //------------------------- Sort -------------------------

    fn to_sort(&self, typ: &Type) -> z3::Sort<'a> {
        match typ {
            Type::Bool => z3::Sort::bool(self.ctx),
            Type::Int => z3::Sort::int(self.ctx),
            Type::Real => z3::Sort::real(self.ctx),
            Type::Enumerate(id) => self.enum_sort[id].clone(),
            Type::Interval(_) => z3::Sort::int(self.ctx),
            Type::IntInterval(_, _) => z3::Sort::int(self.ctx),
            //
            Type::Function(_, _) => panic!(),
            Type::Undefined => panic!(),
            Type::Unresolved(_, _) => panic!(),
        }
    }

    //------------------------- Enum Declaration -------------------------

    fn declare_enumerate(&mut self, enumerate: &Enumerate) {
        let elts = enumerate.elements();
        let names = elts.iter().map(|e| e.name().into()).collect::<Vec<_>>();
        let (sort, consts, _testers) =
            z3::Sort::enumeration(self.ctx, enumerate.name().into(), &names);
        // Sort
        self.enum_sort.insert(enumerate.id(), sort);
        // Elt
        for (elt, f) in elts.iter().zip(consts.into_iter()) {
            self.enum_elts
                .insert(elt.id(), f.apply(&[]).as_datatype().unwrap());
        }
    }

    fn declare_enumerates(&mut self) {
        for e in self.model.enumerates() {
            self.declare_enumerate(e);
        }
    }

    //------------------------- Cst Declaration -------------------------

    fn declare_cst(&mut self, dec: &Declaration) {
        match dec.get_type(self.model) {
            Type::Enumerate(id) => {
                let sort = &self.enum_sort[&id];
                let x = z3::ast::Datatype::new_const(self.ctx, dec.name(), sort);
                self.enum_cst.insert(dec.id(), x);
            }
            Type::Bool => {
                let x = z3::ast::Bool::new_const(self.ctx, dec.name());
                self.bool_cst.insert(dec.id(), x);
            }
            Type::Int => {
                let x = z3::ast::Int::new_const(self.ctx, dec.name());
                self.int_cst.insert(dec.id(), x);
            }
            Type::Real => {
                let x = z3::ast::Real::new_const(self.ctx, dec.name());
                self.real_cst.insert(dec.id(), x);
            }
            Type::IntInterval(min, max) => {
                let x = z3::ast::Int::new_const(self.ctx, dec.name());
                self.int_cst.insert(dec.id(), x);
                let x = self.int_cst(dec.id());
                self.solver
                    .assert(&x.ge(&z3::ast::Int::from_i64(self.ctx, min as i64)));
                self.solver
                    .assert(&x.le(&z3::ast::Int::from_i64(self.ctx, max as i64)));
            }
            //
            Type::Undefined => panic!(),
            Type::Unresolved(_, _) => panic!(),
            Type::Function(_, _) => panic!(),
            Type::Interval(_) => panic!(),
        }
    }

    fn declare_csts(&mut self) {
        for d in self.model.declarations().iter() {
            if d.is_constant() {
                self.declare_cst(d);
            }
        }
    }

    //------------------------- Var Declaration -------------------------

    fn declare_var(&mut self, dec: &Declaration, state: usize) {
        let name = Self::name_state(dec.name(), state);

        match dec.get_type(self.model) {
            Type::Enumerate(id) => {
                let sort = &self.enum_sort[&id];
                let x = z3::ast::Datatype::new_const(self.ctx, name, sort);
                if state == 0 {
                    self.enum_var.insert(dec.id(), vec![]);
                }
                let v = self.enum_var.get_mut(&dec.id()).unwrap();
                let len = v.len();
                if len == state {
                    v.push(x);
                } else if len != state + 1 {
                    panic!();
                }
            }
            Type::Bool => {
                let x = z3::ast::Bool::new_const(self.ctx, name);
                if state == 0 {
                    self.bool_var.insert(dec.id(), vec![]);
                }
                let v = self.bool_var.get_mut(&dec.id()).unwrap();
                let len = v.len();
                if len == state {
                    v.push(x);
                } else if len != state + 1 {
                    panic!();
                }
            }
            Type::Int => {
                let x = z3::ast::Int::new_const(self.ctx, name);
                if state == 0 {
                    self.int_var.insert(dec.id(), vec![]);
                }
                let v = self.int_var.get_mut(&dec.id()).unwrap();
                let len = v.len();
                if len == state {
                    v.push(x);
                } else if len != state + 1 {
                    panic!();
                }
            }
            Type::Real => {
                let x = z3::ast::Real::new_const(self.ctx, name);
                if state == 0 {
                    self.real_var.insert(dec.id(), vec![]);
                }
                let v = self.real_var.get_mut(&dec.id()).unwrap();
                let len = v.len();
                if len == state {
                    v.push(x);
                } else if len != state + 1 {
                    panic!();
                }
            }
            Type::IntInterval(min, max) => {
                let x = z3::ast::Int::new_const(self.ctx, name);
                if state == 0 {
                    self.int_var.insert(dec.id(), vec![]);
                }
                let v = self.int_var.get_mut(&dec.id()).unwrap();
                let len = v.len();
                if len == state {
                    v.push(x);
                    let x = self.int_var(dec.id(), state);
                    self.solver
                        .assert(&x.ge(&z3::ast::Int::from_i64(self.ctx, min as i64)));
                    self.solver
                        .assert(&x.le(&z3::ast::Int::from_i64(self.ctx, max as i64)));
                } else if len != state + 1 {
                    panic!();
                }
            }
            //
            Type::Undefined => panic!(),
            Type::Unresolved(_, _) => panic!(),
            Type::Function(_, _) => panic!(),
            Type::Interval(_) => panic!(),
        }
    }

    fn declare_vars(&mut self, state: usize) {
        for d in self.model.declarations().iter() {
            if !d.is_constant() {
                self.declare_var(d, state);
            }
        }
    }

    //------------------------- Init -------------------------

    fn define_init(&mut self, init: &Initial) {
        let x = self.to_bool(init.expr(), 0);
        self.solver.assert(&x)
    }

    fn define_inits(&mut self) {
        for i in self.model.initials() {
            self.define_init(i);
        }
    }

    //------------------------- Inv -------------------------

    fn define_invariant(&mut self, inv: &Invariant, state: usize) {
        let x = self.to_bool(inv.expr(), state);
        self.solver.assert(&x);
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
            self.solver.assert(&self.to_bool(&e, state));
        } else if len > 1 {
            let e = Expr::or(v);
            self.solver.assert(&self.to_bool(&e, state));
        }
    }

    //------------------------- LTL Variable -------------------------

    fn declare_ltl_var(&mut self, var: &LTLVariable, state: usize) {
        let name = Self::name_state(&var.name(), state);

        let x = z3::ast::Bool::new_const(self.ctx, name);
        if state == 0 {
            self.ltl_var.insert(var.id(), vec![]);
        }
        let v = self.ltl_var.get_mut(&var.id()).unwrap();
        let len = v.len();
        if len == state {
            v.push(x);
        } else if len != state + 1 {
            panic!();
        }
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
        let v = self.to_bool(&var.id().into(), state);
        let e = match var.expr().expression() {
            Expression::LTLunary(op, kid) => match op {
                LTLUnaryOperator::X => {
                    // v[s] = kid[s+1]
                    let kid_next = self.to_bool(&kid, state + 1);
                    v._eq(&kid_next)
                }
                LTLUnaryOperator::F => {
                    // v[s] = kid[s] or v[s+1]
                    let kid = self.to_bool(&kid, state);
                    let v_next = self.to_bool(&var.id().into(), state + 1);
                    let phi = z3::ast::Bool::or(self.ctx, &[&kid, &v_next]);
                    v._eq(&phi)
                }
                LTLUnaryOperator::G => {
                    // v[s] = kid[s] and v[s+1]
                    let kid = self.to_bool(&kid, state);
                    let v_next = self.to_bool(&var.id().into(), state + 1);
                    let phi = z3::ast::Bool::and(self.ctx, &[&kid, &v_next]);
                    v._eq(&phi)
                }
                LTLUnaryOperator::_F_ => {
                    // v[s] = kid[s] or v[s+1]
                    let kid = self.to_bool(&kid, state);
                    let v_next = self.to_bool(&var.id().into(), state + 1);
                    let phi = z3::ast::Bool::or(self.ctx, &[&kid, &v_next]);
                    v._eq(&phi)
                }
                LTLUnaryOperator::_G_ => {
                    // v[s] = kid[s] and v[s+1]
                    let kid = self.to_bool(&kid, state);
                    let v_next = self.to_bool(&var.id().into(), state + 1);
                    let phi = z3::ast::Bool::and(self.ctx, &[&kid, &v_next]);
                    v._eq(&phi)
                }
            },
            Expression::LTLbinary(left, op, right) => match op {
                LTLBinaryOperator::U => {
                    // v[s] = right[s] or (left[s] and v[s+1])
                    let left = self.to_bool(&left, state);
                    let right = self.to_bool(&right, state);
                    let v_next = self.to_bool(&var.id().into(), state + 1);
                    let sub = z3::ast::Bool::and(self.ctx, &[&left, &v_next]);
                    let phi = z3::ast::Bool::or(self.ctx, &[&right, &sub]);
                    v._eq(&phi)
                }
                LTLBinaryOperator::R => {
                    // v[s] = right[s] and (left[s] or v[s+1])
                    let left = self.to_bool(&left, state);
                    let right = self.to_bool(&right, state);
                    let v_next = self.to_bool(&var.id().into(), state + 1);
                    let sub = z3::ast::Bool::or(self.ctx, &[&left, &v_next]);
                    let phi = z3::ast::Bool::and(self.ctx, &[&right, &sub]);
                    v._eq(&phi)
                }
                LTLBinaryOperator::_U_ => {
                    // v[s] = right[s] or (left[s] and v[s+1])
                    let left = self.to_bool(&left, state);
                    let right = self.to_bool(&right, state);
                    let v_next = self.to_bool(&var.id().into(), state + 1);
                    let sub = z3::ast::Bool::and(self.ctx, &[&left, &v_next]);
                    let phi = z3::ast::Bool::or(self.ctx, &[&right, &sub]);
                    v._eq(&phi)
                }
                LTLBinaryOperator::_R_ => {
                    // v[s] = right[s] and (left[s] or v[s+1])
                    let left = self.to_bool(&left, state);
                    let right = self.to_bool(&right, state);
                    let v_next = self.to_bool(&var.id().into(), state + 1);
                    let sub = z3::ast::Bool::or(self.ctx, &[&left, &v_next]);
                    let phi = z3::ast::Bool::and(self.ctx, &[&right, &sub]);
                    v._eq(&phi)
                }
            },
            _ => panic!(),
        };
        self.solver.assert(&e);
    }

    fn define_ltl_var_future(&mut self, var: &LTLVariable, state: usize) {
        let v = self.to_bool(&var.id().into(), state);
        let e = match var.expr().expression() {
            Expression::LTLunary(op, kid) => match op {
                LTLUnaryOperator::X => {
                    // v[s] = kid[s+1]
                    let kid_next = self.to_bool(&kid, state + 1);
                    v._eq(&kid_next)
                }
                LTLUnaryOperator::F => {
                    // v[s] = kid[s] or v[s+1]
                    let kid = self.to_bool(&kid, state);
                    let v_next = self.to_bool(&var.id().into(), state + 1);
                    let phi = z3::ast::Bool::or(self.ctx, &[&kid, &v_next]);
                    v._eq(&phi)
                }
                LTLUnaryOperator::G => {
                    // v[s] = kid[s]
                    let kid = self.to_bool(&kid, state);
                    v._eq(&kid)
                }
                LTLUnaryOperator::_F_ => panic!(),
                LTLUnaryOperator::_G_ => panic!(),
            },
            Expression::LTLbinary(left, op, right) => match op {
                LTLBinaryOperator::U => {
                    // v[s] = right[s] or (left[s] and v[s+1])
                    let left = self.to_bool(&left, state);
                    let right = self.to_bool(&right, state);
                    let v_next = self.to_bool(&var.id().into(), state + 1);
                    let sub = z3::ast::Bool::and(self.ctx, &[&left, &v_next]);
                    let phi = z3::ast::Bool::or(self.ctx, &[&right, &sub]);
                    v._eq(&phi)
                }
                LTLBinaryOperator::R => {
                    // v[s] = right[s]
                    let right = self.to_bool(&right, state);
                    v._eq(&right)
                }
                LTLBinaryOperator::_U_ => panic!(),
                LTLBinaryOperator::_R_ => panic!(),
            },
            _ => panic!(),
        };
        self.solver.assert(&e);
    }

    fn define_ltl_var_finite(&mut self, var: &LTLVariable, state: usize) {
        let v = self.to_bool(&var.id().into(), state);
        let e = match var.expr().expression() {
            Expression::LTLunary(op, kid) => match op {
                LTLUnaryOperator::X => {
                    // v[s] = false
                    v._eq(&z3::ast::Bool::from_bool(self.ctx, false))
                }
                LTLUnaryOperator::F => {
                    // v[s] = kid[s]
                    let kid = self.to_bool(&kid, state);
                    v._eq(&kid)
                }
                LTLUnaryOperator::G => {
                    // v[s] = kid[s]
                    let kid = self.to_bool(&kid, state);
                    v._eq(&kid)
                }
                LTLUnaryOperator::_F_ => panic!(),
                LTLUnaryOperator::_G_ => panic!(),
            },
            Expression::LTLbinary(_, op, right) => match op {
                LTLBinaryOperator::U => {
                    // v[s] = right[s]
                    let right = self.to_bool(&right, state);
                    v._eq(&right)
                }
                LTLBinaryOperator::R => {
                    // v[s] = right[s]
                    let right = self.to_bool(&right, state);
                    v._eq(&right)
                }
                LTLBinaryOperator::_U_ => panic!(),
                LTLBinaryOperator::_R_ => panic!(),
            },
            _ => panic!(),
        };
        self.solver.assert(&e);
    }

    fn define_ltl_var_loop(&mut self, var: &LTLVariable, state: usize) {
        let v = self.to_bool(&var.id().into(), state);
        match var.expr().expression() {
            Expression::LTLunary(op, kid) => match op {
                LTLUnaryOperator::X => {
                    // v[s] = Or_i (l_i and kid[i+1])
                    let mut list = vec![];
                    for (i, l) in self.loops.iter().enumerate() {
                        let e = self.to_bool(kid, i + 1);
                        let e = z3::ast::Bool::and(self.ctx, &[l, &e]);
                        list.push(e);
                    }
                    let list = list.iter().collect::<Vec<_>>();
                    let e = z3::ast::Bool::or(self.ctx, &list);
                    self.solver.assert(&v._eq(&e));
                }
                LTLUnaryOperator::F => {
                    // v[s] = Or_i (l_i and _F_(kid)[i])
                    let mut list = vec![];
                    for (i, l) in self.loops.iter().enumerate() {
                        let e = LTLUnaryOperator::_F_.new(*kid.clone()).into();
                        let e = self.model.get_ltl_expr(e);
                        let e = self.to_bool(&e, i);
                        let e = z3::ast::Bool::and(self.ctx, &[l, &e]);
                        list.push(e);
                    }
                    let list = list.iter().collect::<Vec<_>>();
                    let e = z3::ast::Bool::or(self.ctx, &list);
                    self.solver.assert(&v._eq(&e));
                }
                LTLUnaryOperator::G => {
                    // v[s] = Or_i (l_i and _G_(kid)[i])
                    let mut list = vec![];
                    for (i, l) in self.loops.iter().enumerate() {
                        let e = LTLUnaryOperator::_G_.new(*kid.clone()).into();
                        let e = self.model.get_ltl_expr(e);
                        let e = self.to_bool(&e, i);
                        let e = z3::ast::Bool::and(self.ctx, &[l, &e]);
                        list.push(e);
                    }
                    let list = list.iter().collect::<Vec<_>>();
                    let e = z3::ast::Bool::or(self.ctx, &list);
                    self.solver.assert(&v._eq(&e));
                }
                LTLUnaryOperator::_F_ => {
                    // v[s] = false
                    self.solver
                        .assert(&v._eq(&z3::ast::Bool::from_bool(self.ctx, false)));
                }
                LTLUnaryOperator::_G_ => {
                    // v[s] = true
                    self.solver
                        .assert(&v._eq(&z3::ast::Bool::from_bool(self.ctx, true)));
                }
            },
            Expression::LTLbinary(left, op, right) => match op {
                LTLBinaryOperator::U => {
                    // v[s] = Or_i (l_i and _U_(left, right)[i])
                    let mut list = vec![];
                    for (i, l) in self.loops.iter().enumerate() {
                        let e = LTLBinaryOperator::_U_
                            .new(*left.clone(), *right.clone())
                            .into();
                        let e = self.model.get_ltl_expr(e);
                        let e = self.to_bool(&e, i);
                        let e = z3::ast::Bool::and(self.ctx, &[l, &e]);
                        list.push(e);
                    }
                    let list = list.iter().collect::<Vec<_>>();
                    let e = z3::ast::Bool::or(self.ctx, &list);
                    self.solver.assert(&v._eq(&e));
                }
                LTLBinaryOperator::R => {
                    // v[s] = Or_i (l_i and _R_(left, right)[i])
                    let mut list = vec![];
                    for (i, l) in self.loops.iter().enumerate() {
                        let e = LTLBinaryOperator::_R_
                            .new(*left.clone(), *right.clone())
                            .into();
                        let e = self.model.get_ltl_expr(e);
                        let e = self.to_bool(&e, i);
                        let e = z3::ast::Bool::and(self.ctx, &[l, &e]);
                        list.push(e);
                    }
                    let list = list.iter().collect::<Vec<_>>();
                    let e = z3::ast::Bool::or(self.ctx, &list);
                    self.solver.assert(&v._eq(&e));
                }
                LTLBinaryOperator::_U_ => {
                    // v[s] = false
                    self.solver
                        .assert(&v._eq(&z3::ast::Bool::from_bool(self.ctx, false)));
                }
                LTLBinaryOperator::_R_ => {
                    // v[s] = true
                    self.solver
                        .assert(&v._eq(&z3::ast::Bool::from_bool(self.ctx, true)));
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

    fn define_ltl_vars_future(&mut self, state: usize) {
        for v in self.model.ltl_variables().iter() {
            if !v.is_loop() {
                self.define_ltl_var_future(v, state);
            }
        }
    }

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

    //------------------------- State Unicity -------------------------

    fn add_state_diff(&mut self, first: usize, second: usize) {
        let mut v = vec![];
        for id in self.model.var_declaration_ids() {
            let d: Expr = id.into();
            let x = d.clone().state(first);
            let y = d.state(second);
            let e = x.ne(y);
            v.push(e);
        }
        for var in self.model.ltl_variables() {
            if !var.is_loop() {
                let var: Expr = var.id().into();
                let x = var.clone().state(first);
                let y = var.state(second);
                let e = x.ne(y);
                v.push(e);
            }
        }
        let e = Expr::or(v);
        self.solver.assert(&self.to_bool(&e, 0));
    }

    fn add_state_unicity_with_previous(&mut self, state: usize) {
        for first in 0..state {
            self.add_state_diff(first, state);
        }
    }

    fn state_equality(&self, first: usize, second: usize) -> Expr {
        let mut v = vec![];
        for id in self.model.var_declaration_ids() {
            let d: Expr = id.into();
            let x = d.clone().state(first);
            let y = d.state(second);
            let e = x.eq(y);
            v.push(e);
        }
        for var in self.model.ltl_variables() {
            if !var.is_loop() {
                let var: Expr = var.id().into();
                let x = var.clone().state(first);
                let y = var.state(second);
                let e = x.eq(y);
                v.push(e);
            }
        }
        Expr::and(v)
    }

    //------------------------- Loop -------------------------

    fn declare_loop(&mut self) {
        for state in 0..=self.transitions {
            let l = z3::ast::Bool::new_const(self.ctx, format!("_l_{}", state));
            if self.loops.len() <= state {
                self.loops.push(l);
            }
        }
        let values = &self.loops.iter().map(|x| (x, 1)).collect::<Vec<_>>();
        self.solver
            .assert(&z3::ast::Bool::pb_eq(self.ctx, values, 1));
    }

    fn define_loop(&mut self) {
        for state in 0..=self.transitions {
            let l = &self.loops[state];
            let e = self.state_equality(state, self.states());
            let e = self.to_bool(&e, 0);
            self.solver.assert(&l._eq(&e));
        }
    }

    pub fn get_loop_index(&self, z3_model: &z3::Model) -> Option<usize> {
        if self.loops.is_empty() {
            None
        } else {
            for (i, l) in self.loops.iter().enumerate() {
                let value = z3_model.eval(l, true).unwrap().as_bool().unwrap();
                if value {
                    return Some(i);
                }
            }
            None
        }
    }

    //------------------------- Solution -------------------------

    pub fn add_solution(&mut self, solution: &Solution) {
        for (id, expr) in solution.cst_dec.iter() {
            if let Some(expr) = expr {
                let dec: Expr = (*id).into();
                let e = dec.eq(expr.clone());
                self.solver.assert(&self.to_bool(&e, 0));
            }
        }
        for (id, v) in solution.var_dec.iter() {
            for state in 0..solution.states {
                if let Some(expr) = &v[state] {
                    let dec: Expr = (*id).into();
                    let e = dec.eq(expr.clone());
                    self.solver.assert(&self.to_bool(&e, state));
                }
            }
        }
    }

    pub fn remove_solution(&mut self, solution: &Solution) {
        let mut conj = vec![];
        for (id, expr) in solution.cst_dec.iter() {
            if let Some(expr) = expr {
                let dec: Expr = (*id).into();
                let e = dec.eq(expr.clone());
                conj.push(self.to_bool(&e, 0));
            }
        }
        for (id, v) in solution.var_dec.iter() {
            for state in 0..solution.states {
                if let Some(expr) = &v[state] {
                    let dec: Expr = (*id).into();
                    let e = dec.eq(expr.clone());
                    conj.push(self.to_bool(&e, state));
                }
            }
        }
        let a = conj.iter().collect::<Vec<_>>();
        let e = z3::ast::Bool::and(self.ctx, &a);
        self.solver.assert(&e.not());
    }

    //------------------------- Property -------------------------

    fn add_property(&mut self) {
        let opt: Option<&Declaration> = self.model.from_name("prop");
        if let Some(prop) = opt {
            let expr = prop.id().into();
            let e = self.to_bool(&expr, 0);
            self.solver.assert(&e);
        }
        // TODO: add path semantic: finite/truncated/sequence or loop
    }

    //------------------------- Initialize -------------------------

    pub fn initialize(&mut self, transitions: usize, unicity: bool) {
        self.transitions = transitions;
        self.unicity = unicity;

        // Enum
        self.declare_enumerates();
        // Cst
        self.declare_csts();

        for state in 0..self.states() {
            // Var
            self.declare_vars(state);
            // LTL Variables
            self.declare_ltl_non_loop_vars(state);
        }

        // Init
        self.define_inits();

        for state in 0..self.states() - 1 {
            // Invariants
            self.define_invariants(state);
            // Transition
            self.define_transitions(state);
        }

        // Unicity
        if self.unicity {
            for state in 1..self.states() {
                self.add_state_unicity_with_previous(state);
            }
        }
        // LTL Variables: classical semantic until last
        for state in 0..self.states() - 1 {
            self.define_ltl_non_loop_vars(state);
        }

        // Property
        self.add_property();
    }

    //------------------------- Incremental -------------------------

    pub fn solver_push(&self) {
        self.solver.push();
    }

    pub fn solver_pop(&self) {
        self.solver.pop();
    }

    pub fn add_transition(&mut self) {
        self.transitions += 1;
        let last_state = self.states() - 1;
        // Var
        self.declare_vars(last_state);
        // LTL Variables
        self.declare_ltl_non_loop_vars(last_state);

        // Invariant
        self.define_invariants(last_state);
        // Transition
        self.define_transitions(last_state - 1);
        //
        // Unicity
        if self.unicity {
            self.add_state_unicity_with_previous(last_state);
        }
        // LTL Variables: until last
        self.define_ltl_non_loop_vars(last_state - 1);
    }

    //------------------------- Future -------------------------

    pub fn add_future_semantic(&mut self) {
        let future_state = self.states();

        self.declare_vars(future_state);
        self.declare_ltl_non_loop_vars(future_state);
        self.define_ltl_vars_future(future_state - 1);
    }

    //------------------------- Finite -------------------------

    pub fn set_finite_semantic(&mut self) {
        let last_state = self.states() - 1;
        self.define_ltl_vars_finite(last_state);
    }

    //------------------------- Infinite -------------------------

    pub fn add_loop_semantic(&mut self) {
        let loop_state = self.states();

        self.declare_vars(loop_state);
        self.declare_ltl_non_loop_vars(loop_state);
        for state in 0..=loop_state {
            self.declare_ltl_loop_vars(state);
        }
        self.declare_loop();

        // Invariant
        self.define_invariants(loop_state);
        // Transition
        self.define_transitions(loop_state - 1);

        self.define_ltl_non_loop_vars(loop_state - 1);

        self.define_loop();
        for state in 0..loop_state {
            self.define_ltl_loop_vars(state);
        }
        self.define_ltl_vars_loop(loop_state); // loop semantic
    }

    //------------------------- To Entry -------------------------

    pub fn solver_to_entry(&self) -> d_stuff::Entry {
        d_stuff::Entry::new(
            d_stuff::Status::Info,
            d_stuff::Text::new(
                "SMT Model",
                termion::style::Bold.to_string(),
                termion::color::Blue.fg_str(),
            ),
            None,
            vec![d_stuff::Message::new(
                None,
                d_stuff::Text::new(
                    format!("{}", self.solver),
                    termion::style::Reset.to_string(),
                    termion::color::White.fg_str(),
                ),
            )],
        )
    }

    pub fn z3_model_to_entry(&self) -> d_stuff::Entry {
        d_stuff::Entry::new(
            d_stuff::Status::Info,
            d_stuff::Text::new(
                "SMT Z3_Model",
                termion::style::Bold.to_string(),
                termion::color::Blue.fg_str(),
            ),
            None,
            vec![d_stuff::Message::new(
                None,
                d_stuff::Text::new(
                    format!("{}", self.solver.get_model().unwrap()),
                    termion::style::Reset.to_string(),
                    termion::color::White.fg_str(),
                ),
            )],
        )
    }
}
