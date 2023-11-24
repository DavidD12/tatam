extern crate z3;

use z3::{ast::Ast, *};

fn main() {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);

    let solver = Solver::new(&ctx);

    // Ajouter une assertion initiale
    let x = z3::ast::Int::new_const(&ctx, "x");
    solver.assert(&x._eq(&z3::ast::Int::from_i64(&ctx, 0)));
    solver.push();

    let y = z3::ast::Int::new_const(&ctx, "y");
    solver.assert(&y._eq(&z3::ast::Int::from_i64(&ctx, 10)));

    // Push le contexte et ajoute une nouvelle assertion
    solver.push();
    solver.assert(&x.gt(&y));

    // Vérifie que les assertions sont satisfaisables
    let result = solver.check();

    // Si les assertions sont satisfaisables, affiche un modèle
    if result == SatResult::Sat {
        let model = solver.get_model().unwrap();
        println!(
            "Solution : x = {}, y = {}",
            model.eval(&x, true).unwrap(),
            model.eval(&y, true).unwrap()
        );
    } else {
        println!("Aucune solution trouvée.");
    }

    // Pop le contexte et ajoute une nouvelle assertion
    solver.pop(2);
    solver.assert(&x.lt(&y));
    solver.assert(&x.lt(&z3::ast::Int::from_i64(&ctx, 10)));

    // Vérifie que les assertions sont satisfaisables
    let result = solver.check();

    // Si les assertions sont satisfaisables, affiche un modèle
    if result == SatResult::Sat {
        let model = solver.get_model().unwrap();
        println!(
            "Solution : x = {}, y = {}",
            model.eval(&x, true).unwrap(),
            model.eval(&y, true).unwrap()
        );
    } else {
        println!("Aucune solution trouvée.");
    }
}
