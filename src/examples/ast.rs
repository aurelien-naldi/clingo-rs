use clingo::*;
use std::env;

pub struct OnStatementData<'a> {
    atom: ast::Atom,
    builder: Option<ProgramBuilder<'a>>,
}

impl<'a> AstStatementHandler for OnStatementData<'a> {
    // adds atom enable to all rule bodies
    fn on_statement<T>(&mut self, stm: &AstStatement<T>) -> bool {
        // pass through all statements that are not rules
        if stm.statement_type() != ast::StatementType::Rule {
            self.builder
                .as_mut()
                .unwrap()
                .add(stm)
                .expect("Failed to add statement to ProgramBuilder.");
            return true;
        }

        // copy the current rule body
        if let Ok(rule) = stm.rule() {
            let body = rule.body();
            let mut extended_body = std::vec::Vec::with_capacity(body.len() + 1);
            for e in body {
                extended_body.push(e.clone());
            }

            // create atom enable
            let lit = ast::Literal::from_atom(self.atom.location(), ast::Sign::None, &self.atom);
            // add atom enable to the rule body
            let y = ast::BodyLiteral::new(
                self.atom.location(),
                ast::Sign::None,
                ast::BodyLiteralType::Literal,
                &lit,
            );
            extended_body.push(y);

            // initialize the rule
            let head = rule.head();
            let rule = ast::Rule::new(head, &extended_body);

            // initialize the statement
            let stm2 = rule.ast_statement(stm.location());

            // add the rewritten statement to the program
            self.builder
                .as_mut()
                .unwrap()
                .add(&stm2)
                .expect("Failed to add statement to ProgramBuilder.");
            return true;
        }
        false
    }
}

fn print_model(model: &Model) {
    // retrieve the symbols in the model
    let atoms = model
        .symbols(&ShowType::SHOWN)
        .expect("Failed to retrieve symbols in the model.");

    print!("Model:");

    for atom in atoms {
        // retrieve and print the symbol's string
        print!(" {}", atom.to_string().unwrap());
    }
    println!();
}

fn solve(ctl: &mut Control) {
    // get a solve handle
    let mut handle = ctl
        .solve(&SolveMode::YIELD, &[])
        .expect("Failed retrieving solve handle.");

    // loop over all models
    loop {
        handle.resume().expect("Failed resume on solve handle.");
        match handle.model() {
            // print the model
            Ok(Some(model)) => print_model(model),
            // stop if there are no more models
            Ok(None) => break,
            Err(e) => panic!("Error: {}", e.as_fail()),
        }
    }

    // close the solve handle
    handle
        .get()
        .expect("Failed to get result from solve handle.");
    handle.close().expect("Failed to close solve handle.");
}

fn main() {
    // collect clingo options from the command line
    let options = env::args().skip(1).collect();

    let mut ctl = Control::new(options).expect("Failed creating Control.");

    let sym = Symbol::create_id("enable", true).unwrap();

    {
        // get the program builder
        let builder = ctl.program_builder().ok();

        // initialize the location
        let location = Location::new("<rewrite>", "<rewrite>", 0, 0, 0, 0).unwrap();

        // initilize atom to add
        let atom = ast::Atom::from_symbol(location, sym);

        let mut data = OnStatementData {
            atom: atom,
            builder: builder,
        };

        // get the AST of the program
        parse_program("a :- not b. b :- not a.", &mut data)
            .expect("Failed to parse logic program.");

        // add the external statement: #external enable.
        let ext = ast::External::new(atom, &[]);

        let stm = ext.ast_statement(location);
        data.builder
            .as_mut()
            .unwrap()
            .add(&stm)
            .expect("Failed to add statement to ProgramBuilder.");

        // finish building a program
        data.builder
            .take()
            .unwrap()
            .end()
            .expect("Failed to finish building a program.");
    }

    // ground the base part
    let part = Part::new("base", &[]).unwrap();
    let parts = vec![part];
    ctl.ground(&parts)
        .expect("Failed to ground a logic program.");

    // solve with external enable = false
    println!("Solving with enable = false...");
    solve(&mut ctl);

    // solve with external enable = true
    println!("Solving with enable = true...");
    ctl.assign_external(&sym, TruthValue::True)
        .expect("Failed to assign #external enable true.");
    solve(&mut ctl);

    // solve with external enable = false
    println!("Solving with enable = false...");
    ctl.assign_external(&sym, TruthValue::False)
        .expect("Failed to assign #external enable false.");
    solve(&mut ctl);
}
