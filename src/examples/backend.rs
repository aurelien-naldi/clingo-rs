extern crate clingo;

use std::env;
use clingo::*;


fn print_model(model: &mut ClingoModel) {

    // retrieve the symbols in the model
    let atoms = model
        .symbols(
            clingo_show_type::clingo_show_type_shown as clingo_show_type_bitset_t,
        )
        .expect("Failed to retrieve symbols in the model");

    print!("Model:");

    for atom in atoms {
        // retrieve and print the symbol's string
        print!(" {}", atom.to_string().unwrap());
    }
    println!("");
}

fn solve(ctl: &mut ClingoControl) {

    let solve_mode = clingo_solve_mode::clingo_solve_mode_yield as clingo_solve_mode_bitset_t;
    let assumptions = vec![];
    let solve_event_callback = None;
    let data = std::ptr::null_mut();

    // get a solve handle
    let handle = ctl.solve(solve_mode, assumptions, solve_event_callback, data)
        .expect("Failed retrieving solve handle");

    // loop over all models
    loop {
        handle.resume().expect("Failed resume on solve handle.");
        match handle.model() {
            // stop if there are no more models
            None => break,
            // print the model
            Some(model) => print_model(model),
        }
    }

    // close the solve handle
    let _result = handle.get();
    handle.close().expect("Failed to close solve handle");
}

fn main() {

    // collect clingo options from the command line
    let options = env::args().skip(1).collect();

    // create a control object and pass command line arguments
    let logger = None;
    let logger_data = std::ptr::null_mut();
    let mut ctl = ClingoControl::new(options, logger, logger_data, 20)
        .expect("Failed creating clingo_control");

    // add a logic program to the base part
    let parameters: Vec<&str> = Vec::new();
    ctl.add("base", parameters, "{a; b; c}.").expect(
        "Failed to add a logic program",
    );

    // ground the base part
    let part = ClingoPart::new_part("base", &[]);
    let parts = vec![part];
    let ground_callback = None;
    let ground_callback_data = std::ptr::null_mut();
    ctl.ground(parts, ground_callback, ground_callback_data)
        .expect("Failed to ground a logic program");

    let atom_strings = ["a", "b", "c"];
    // get the ids of atoms a, b, and c
    let mut atom_ids = Vec::new();
    {
        // get symbolic atoms
        let atoms = ctl.symbolic_atoms().unwrap();

        for atom in atom_strings.iter() {
            let symbol = ClingoSymbol::create_id(atom, true).unwrap();
            let atom_it = atoms.find(symbol).unwrap();

            // get the atom's id
            let lit = atoms.literal(atom_it).unwrap();
            atom_ids.push(lit);
        }
    }

    {
        // get the backend
        let backend = ctl.backend().unwrap();

        // add an additional atom (called d below)
        let atom_d = backend.add_atom().unwrap();

        // add rule: d :- a, b.
        let head = vec![atom_d];
        let body = vec![atom_ids[0], atom_ids[1]];
        backend.rule(false, &head, &body).expect(
            "Failed to add a rule to the program.",
        );

        // add rule: :- not d, c.
        let head = vec![];
        let body = vec![ClingoLiteral::UNSAFE_from(atom_d).negate(), atom_ids[2]];

        backend.rule(false, &head, &body).expect(
            "Failed to add a rule to the program.",
        );
    }

    // solve
    solve(ctl);
}
