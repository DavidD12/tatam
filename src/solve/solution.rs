use super::*;
use crate::common::*;
use crate::expr::*;
use crate::model::*;
use crate::typing::*;
use std::collections::HashMap;

pub struct Solution {
    pub states: usize,
    pub loop_index: Option<usize>,
    pub cst_dec: HashMap<DeclarationId, Option<Expr>>,
    pub var_dec: HashMap<DeclarationId, Vec<Option<Expr>>>,
}

impl Solution {
    // pub fn eval_into_expr(
    //     expr: &Expr,
    //     state: usize,
    //     smt: &Smt,
    //     z3_model: &z3::Model,
    //     completion: bool,
    // ) -> Option<Expr> {
    //     let t = expr.get_type(smt.model());
    //     match t {
    //         crate::typing::typ::Type::Enumerate(_) => {
    //             let e = smt.to_datatype(expr, state);
    //             if let Some(eval) = z3_model.eval(&e, completion) {
    //                 let eval_string = eval.to_string();
    //                 if e.to_string() != eval_string {
    //                     let elt: Option<&EnumerateElement> = smt.model().from_name(&eval_string);
    //                     if let Some(elt) = elt {
    //                         return Some(elt.into());
    //                     }
    //                 }
    //             }
    //             None
    //         }
    //         crate::typing::typ::Type::Bool => {
    //             let e = smt.to_bool(expr, state);
    //             if let Some(eval) = z3_model.eval(&e, completion) {
    //                 if e.to_string() != eval.to_string() {
    //                     if let Some(value) = eval.as_bool() {
    //                         return Some(value.into());
    //                     }
    //                 }
    //             }
    //             None
    //         }
    //         crate::typing::typ::Type::Int => {
    //             let e = smt.to_int(expr, state);
    //             if let Some(eval) = z3_model.eval(&e, completion) {
    //                 if e.to_string() != eval.to_string() {
    //                     if let Some(value) = eval.as_i64() {
    //                         return Some(value.into());
    //                     }
    //                 }
    //             }
    //             None
    //         }
    //         crate::typing::typ::Type::IntInterval(_, _) => {
    //             let e = smt.to_int(expr, state);
    //             if let Some(eval) = z3_model.eval(&e, completion) {
    //                 if e.to_string() != eval.to_string() {
    //                     if let Some(value) = eval.as_i64() {
    //                         return Some(value.into());
    //                     }
    //                 }
    //             }
    //             None
    //         }
    //         crate::typing::typ::Type::Real => {
    //             let e = smt.to_real(expr, state);
    //             if let Some(eval) = z3_model.eval(&e, completion) {
    //                 if e.to_string() != eval.to_string() {
    //                     if let Some(value) = eval.as_real() {
    //                         return Some(value.into());
    //                     }
    //                 }
    //             }
    //             None
    //         }

    //         crate::typing::typ::Type::Interval(_) => panic!(),
    //         crate::typing::typ::Type::Function(_, _) => panic!(),
    //         crate::typing::typ::Type::Unresolved(_, _) => panic!(),
    //         crate::typing::typ::Type::Undefined => panic!(),
    //     }
    // }

    // pub fn new(smt: &Smt, z3_model: &z3::Model) -> Self {
    //     let loop_index = smt.get_loop_index(z3_model);
    //     // Constantes
    //     let mut cst_dec = HashMap::new();
    //     for id in smt.model().cst_declaration_ids() {
    //         let eval = Self::eval_into_expr(&id.into(), 0, smt, z3_model, false);
    //         cst_dec.insert(id, eval);
    //     }

    //     // Variables / States
    //     let mut var_dec = HashMap::new();
    //     let list = smt.model().var_declaration_ids();

    //     for id in list.iter() {
    //         var_dec.insert(*id, vec![]);
    //     }

    //     for id in list.into_iter() {
    //         let v = var_dec.get_mut(&id).unwrap();
    //         for state in 0..smt.states() {
    //             let eval = Self::eval_into_expr(&id.into(), state, smt, z3_model, false);
    //             v.push(eval);
    //         }
    //     }

    //     Self {
    //         states: smt.states(),
    //         loop_index,
    //         cst_dec,
    //         var_dec,
    //     }
    // }

    pub fn get_default_value(typ: &Type) -> Expr {
        match typ {
            Type::Enumerate(_) => todo!(),
            Type::Bool => false.into(),
            Type::Int => 0.into(),
            Type::IntInterval(min, _) => (*min).into(),
            Type::Real => 0.into(),
            //
            Type::Undefined => panic!(),
            Type::Unresolved(_, _) => panic!(),
            Type::Interval(_) => panic!(),
            Type::Function(_, _) => panic!(),
        }
    }

    pub fn from_solver(solver: &mut Solver, complete: bool) -> Self {
        let loop_index = solver.get_loop_index();
        // Constantes
        let mut cst_dec = HashMap::new();
        for id in solver.model().cst_declaration_ids() {
            let eval = solver.eval(&id.into(), 0);
            if complete && eval.is_none() {
                let dec = solver.model().get(id).unwrap();
                let eval = Self::get_default_value(&dec.get_type(solver.model()));
                cst_dec.insert(id, Some(eval));
            } else {
                cst_dec.insert(id, eval);
            }
        }

        // Variables / States
        let mut var_dec = HashMap::new();
        let list = solver.model().var_declaration_ids();

        for id in list.iter() {
            var_dec.insert(*id, vec![]);
        }

        for id in list.into_iter() {
            let v = var_dec.get_mut(&id).unwrap();
            for state in 0..solver.states() {
                let eval = solver.eval(&id.into(), state);
                if complete && eval.is_none() {
                    let dec = solver.model().get(id).unwrap();
                    let eval = Self::get_default_value(&dec.get_type(solver.model()));
                    v.push(Some(eval));
                } else {
                    v.push(eval);
                }
            }
        }

        Self {
            states: solver.states(),
            loop_index,
            cst_dec,
            var_dec,
        }
    }
}

//------------------------- To Lang -------------------------

impl ToLang for Solution {
    fn to_lang(&self, model: &Model) -> String {
        let mut res = "".to_string();

        // Constantes
        for (id, value) in self.cst_dec.iter() {
            if let Some(value) = value {
                // Avoid Pop added in inner_model
                if let Some(dec) = model.get(*id) {
                    res += &format!("{} = {}\n", dec.to_lang(model), value.to_lang(model));
                }
            }
        }
        // Variables / States
        for state in 0..self.states {
            res += &format!("---------- State {} ----------\n", state);
            for (id, v) in self.var_dec.iter() {
                let dec = model.get(*id).unwrap();
                if let Some(value) = &v[state] {
                    res += &format!("{} = {}\n", dec.to_lang(model), value.to_lang(model));
                }
            }
        }
        // Loop
        match self.loop_index {
            Some(index) => res += &format!(">>>>>>>>>> Loop {} <<<<<<<<<<\n", index),
            None => {}
        }
        res
    }
}
