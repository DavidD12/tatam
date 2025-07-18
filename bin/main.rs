use clap::Parser;
use tatam::{common::*, load_file, model::*, solve::*, Args};

fn main() {
    let mut pretty = d_stuff::Pretty::new();

    let args: Args = Args::parse();

    let mut model = Model::empty();

    match load_file(&mut pretty, &mut model, &args.file, args.verbose) {
        Ok(_) => {
            if args.verbose >= 3 {
                pretty.add(model.to_debug_entry());
                pretty.print();
            } else if args.verbose >= 2 {
                pretty.add(model.to_entry());
                pretty.print();
            }
            //
            let response = resolve(&mut model, &mut pretty, &args);
            if args.verbose > 0 {
                pretty.add(response.to_entry(&model));
                pretty.print();
            } else {
                println!("{}", response.to_lang(&model));
            }
        }
        Err(e) => {
            if args.verbose > 0 {
                pretty.add(e.to_entry(&model));
                pretty.print();
            }
        }
    }
}
