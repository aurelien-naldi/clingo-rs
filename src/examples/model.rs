use std::env;
extern crate clingo;
use clingo::*;
use std::ffi::CString;


fn error_main() {
    let error_message = safe_clingo_error_message();
    println!("{}", error_message);
    safe_clingo_error_code();
}

fn print_model(model: &mut ClingoModel, label: &str, show: clingo_show_type_bitset_t) {

    print!("{}:", label);

    // retrieve the symbols in the model
    let atoms = model.symbols(show)
        .expect("Failed to retrieve symbols in the model");

    for atom in atoms {
        // retrieve and print the symbol's string
        let atom_string = safe_clingo_symbol_to_string(atom).unwrap();
        print!(" {}", atom_string.to_str().unwrap());
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
        if !handle.resume() {
            return error_main();
        }
        if let Some(model) = handle.model() {

            // get model type
            let model_type = model.model_type().unwrap();

            let mut type_string = "";
            match model_type {
                0 => type_string = "Stable model",
                1 => type_string = "Brave consequences", 
                2 => type_string = "Cautious consequences",
                _ => {}
            };

            // get running number of model
            let number = model.number().unwrap();

            println!("{}: {}", type_string, number);

            print_model(model,
                        "  shown",
                        clingo_show_type::clingo_show_type_shown as clingo_show_type_bitset_t);
            print_model(model,
                        "  atoms",
                        clingo_show_type::clingo_show_type_atoms as clingo_show_type_bitset_t);
            print_model(model,
                        "  terms",
                        clingo_show_type::clingo_show_type_terms as clingo_show_type_bitset_t);
            print_model(model,
                            " ~atoms",
                            (clingo_show_type::clingo_show_type_complement as clingo_show_type_bitset_t +
                            clingo_show_type::clingo_show_type_atoms as clingo_show_type_bitset_t));

        } else {
            // stop if there are no more models
            break;
        }
    }

    // close the solve handle
    let _result = handle.get().expect("Failed to get solve handle");
    handle.close();
}

fn main() {
    // create a control object and pass command line arguments
    let logger = None;
    let logger_data = std::ptr::null_mut();
    let mut ctl = new_clingo_control(env::args(), logger, logger_data, 20)
        .expect("Failed creating clingo_control");

    // add a logic program to the base part
    let parameters: Vec<&str> = Vec::new();
    let err = ctl.add("base", parameters, "1 {a; b} 1. #show c : b. #show a/0.");
    if !err {
        return error_main();
    }

    // ground the base part
    let part = ClingoPart {
        name: CString::new("base").unwrap(),
        params: &[],
    };
    let parts = vec![part];
    let ground_callback = None;
    let ground_callback_data = std::ptr::null_mut();
    let err = ctl.ground(parts, ground_callback, ground_callback_data);
    if !err {
        return error_main();
    }

    // solve
    let _solve_result = solve(ctl);

}