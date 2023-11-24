use crate::common::*;
use crate::expr::*;
use crate::model::*;

#[derive(Clone)]
pub struct Combine<T: Clone> {
    elements: Vec<Vec<T>>,
    index: Vec<usize>,
}

impl<T: Clone> Combine<T> {
    pub fn new(elements: Vec<Vec<T>>) -> Self {
        let index = elements.iter().map(|_| 0).collect();
        Self { elements, index }
    }

    pub fn index(&self) -> &Vec<usize> {
        &self.index
    }

    pub fn values(&self) -> Vec<T> {
        self.elements
            .iter()
            .zip(self.index.iter())
            .map(|(v, i)| v[*i].clone())
            .collect()
    }

    pub fn step(&mut self) -> bool {
        for i in 0..self.index.len() {
            let n = self.index[i];
            if n < self.elements[i].len() - 1 {
                self.index[i] += 1;
                return true;
            }
            if i != self.index.len() - 1 {
                for j in 0..=i {
                    self.index[j] = 0;
                }
            }
        }
        false
    }

    pub fn next(&mut self) -> Option<Vec<T>> {
        if self.step() {
            Some(self.values())
        } else {
            None
        }
    }
}

pub fn combine_all(model: &Model, parameters: &Vec<Parameter>, expr: &Expr) -> Vec<Expr> {
    let params_all = parameters
        .iter()
        .map(|p| p.get_type(model).all(model))
        .collect();
    let params_exp = parameters
        .iter()
        .map(|p| Expr::new(Expression::Parameter(p.clone()), None))
        .collect::<Vec<_>>();
    let mut combine = Combine::new(params_all);
    let mut v = Vec::new();
    loop {
        let values = combine.values();
        let all = params_exp.clone().into_iter().zip(values.clone()).collect();
        let e = expr.substitute_all(all);
        v.push(e);
        if !combine.step() {
            break;
        }
    }
    v
}
