
extern crate libc;
extern crate clingo_sys;

use libc::c_int;
use libc::c_char;
use std::ffi::CStr;
use std::ffi::CString;

use clingo_sys::*;
pub use clingo_sys::{clingo_ast_term__bindgen_ty_1, clingo_program_builder_t, clingo_location,
                     clingo_ast_term_t, clingo_ast_term_type, clingo_ast_term_type_t,
                     clingo_propagator, clingo_solve_event_callback_t, clingo_solve_event_type_t,
                     clingo_solve_handle_t, clingo_solve_result_bitset_t,
                     clingo_solve_mode_bitset_t, clingo_solve_mode, clingo_show_type_bitset_t,
                     clingo_show_type, clingo_logger_t, clingo_literal_t, clingo_id_t};


pub fn safe_clingo_version() -> (i32, i32, i32) {
    let mut major = 0;
    let mut minor = 0;
    let mut revision = 0;
    unsafe { clingo_version(&mut major, &mut minor, &mut revision) };

    (major, minor, revision)
}
pub struct ClingoPart<'a> {
    pub name: CString,
    pub params: &'a [clingo_symbol_t],
}
fn from_clingo_part(spart: &ClingoPart) -> clingo_part {
    clingo_part {
        name: spart.name.as_ptr(),
        params: spart.params.as_ptr(),
        size: spart.params.len(),
    }
}
pub fn safe_clingo_error_code() -> clingo_error_t {
    unsafe { clingo_error_code() }
}
pub fn safe_clingo_error_message() -> &'static str {

    let c_buf: *const c_char = unsafe { clingo_error_message() };
    if c_buf.is_null() {
        return "";
    } else {
        let c_str = unsafe { CStr::from_ptr(c_buf) };
        return c_str.to_str().unwrap();
    }
}
pub fn safe_clingo_set_error(code: clingo_error_t, message: &str) {

    let m2 = CString::new(message).unwrap().as_ptr();
    unsafe {
        clingo_set_error(code, m2);
    }
}
pub fn safe_clingo_symbol_to_string(symbol: clingo_symbol_t) -> std::option::Option<CString> {

    let mut size: usize = 0;
    let err = unsafe { clingo_symbol_to_string_size(symbol, &mut size) };
    if !err {
        None
    } else {
        let a1 = vec![1; size];
        let string = unsafe { CString::from_vec_unchecked(a1) };
        let err = unsafe { clingo_symbol_to_string(symbol, string.as_ptr() as *mut c_char, size) };
        if !err { None } else { Some(string) }
    }

}
pub fn safe_clingo_symbol_create_number(number: c_int) -> clingo_symbol_t {

    let mut symbol = 0 as clingo_symbol_t;
    unsafe {
        clingo_symbol_create_number(number, &mut symbol);
    }
    symbol
}
pub fn safe_clingo_symbol_create_id(name: &str,
                                    positive: bool)
                                    -> std::option::Option<clingo_symbol_t> {

    let mut symbol = 0 as clingo_symbol_t;
    let err = unsafe {
        clingo_symbol_create_id(CString::new(name).unwrap().as_ptr(), positive, &mut symbol)
    };
    if !err { None } else { Some(symbol) }
}
pub fn safe_clingo_symbol_create_function(name: &str,
                                          arguments: &[clingo_symbol_t],
                                          positive: bool)
                                          -> std::option::Option<clingo_symbol_t> {

    let mut symbol = 0 as clingo_symbol_t;
    let err = unsafe {
        clingo_symbol_create_function(CString::new(name).unwrap().as_ptr(),
                                      arguments.as_ptr(),
                                      arguments.len(),
                                      positive,
                                      &mut symbol)
    };
    if !err { None } else { Some(symbol) }
}
pub fn safe_clingo_symbol_number(symbol: clingo_symbol_t) -> Option<c_int> {

    let mut number = 0;
    let err = unsafe { clingo_symbol_number(symbol, &mut number) };
    if !err { None } else { Some(number) }
}
pub fn safe_clingo_symbol_hash(symbol: clingo_symbol_t) -> usize {
    unsafe { clingo_symbol_hash(symbol) }
}
pub fn safe_clingo_symbol_arguments(symbol: clingo_symbol_t)
                                    -> std::option::Option<Vec<clingo_symbol_t>> {

    let mut a_ptr = std::ptr::null() as *const clingo_symbol_t;
    let mut size: usize = 0;
    let err = unsafe { clingo_symbol_arguments(symbol, &mut a_ptr, &mut size) };
    if !err {
        None
    } else {
        let mut a1 = Vec::<clingo_symbol_t>::with_capacity(size);
        for _ in 0..size {
            let nsymbol = unsafe { *a_ptr };
            a1.push(nsymbol);
        }
        Some(a1)
    }

}
pub fn safe_clingo_symbol_is_equal_to(a: clingo_symbol_t, b: clingo_symbol_t) -> bool {
    unsafe { clingo_symbol_is_equal_to(a, b) }
}
pub fn safe_clingo_symbol_is_less_than(a: clingo_symbol_t, b: clingo_symbol_t) -> bool {
    unsafe { clingo_symbol_is_less_than(a, b) }
}
pub fn safe_clingo_parse_program(program: &str,
                                 callback: clingo_ast_callback_t,
                                 callback_data: *mut ::std::os::raw::c_void,
                                 logger: clingo_logger_t,
                                 logger_data: *mut ::std::os::raw::c_void,
                                 message_limit: ::std::os::raw::c_uint)
                                 -> bool {
    unsafe {

        let mprogram = CString::new(program).unwrap();
        clingo_parse_program(mprogram.as_ptr(),
                             callback,
                             callback_data,
                             logger,
                             logger_data,
                             message_limit)
    }
}
pub fn new_clingo_control<'a>(arguments: std::env::Args,
                              logger: clingo_logger_t,
                              logger_data: *mut ::std::os::raw::c_void,
                              message_limit: ::std::os::raw::c_uint)
                              -> std::option::Option<&'a mut ClingoControl> {
    let arguments_size = arguments.len() - 1;
    // create a vector of zero terminated strings
    let args = arguments
        .map(|arg| CString::new(arg).unwrap())
        .collect::<Vec<CString>>();
    // drop first element
    let (_, tail) = args.split_first().unwrap();
    // convert the strings to raw pointers
    let c_args = tail.iter()
        .map(|arg| arg.as_ptr())
        .collect::<Vec<*const c_char>>();

    let mut ctl = std::ptr::null_mut();

    let err = unsafe {
        clingo_control_new(c_args.as_ptr(),
                           arguments_size,
                           logger,
                           logger_data,
                           message_limit,
                           &mut ctl)
    };
    if !err {
        None
    } else {
        unsafe { Some(&mut *(ctl as *mut ClingoControl)) }
    }

}

pub struct ClingoControl(clingo_control_t);
impl Drop for ClingoControl {
    fn drop(&mut self) {
        unsafe {
            let ClingoControl(ref mut control) = *self;
            clingo_control_free(control)
        }

    }
}
impl ClingoControl {
    //     pub fn clingo_control_load(control: *mut ClingoControl, file: *const c_char) -> u8;
    pub fn add(&mut self, name: &str, parameters: Vec<&str>, program: &str) -> bool {

        let mname = CString::new(name).unwrap();
        let mprogram = CString::new(program).unwrap();
        let parameters_size = parameters.len();
        // create a vector of zero terminated strings
        let args = parameters
            .into_iter()
            .map(|arg| CString::new(arg).unwrap())
            .collect::<Vec<CString>>();
        // convert the strings to raw pointers
        let c_args = args.iter()
            .map(|arg| arg.as_ptr())
            .collect::<Vec<*const c_char>>();

        unsafe {
            let ClingoControl(ref mut control) = *self;
            clingo_control_add(control,
                               mname.as_ptr(),
                               c_args.as_ptr(),
                               parameters_size,
                               mprogram.as_ptr())
        }
    }
    pub fn ground(&mut self,
                  sparts: Vec<ClingoPart>,
                  ground_callback: clingo_ground_callback_t,
                  ground_callback_data: *mut ::std::os::raw::c_void)
                  -> bool {

        let parts = sparts
            .iter()
            .map(|arg| from_clingo_part(arg))
            .collect::<Vec<clingo_part>>();
        let parts_size = parts.len();

        unsafe {
            let ClingoControl(ref mut control) = *self;
            clingo_control_ground(control,
                                  parts.as_ptr(),
                                  parts_size,
                                  ground_callback,
                                  ground_callback_data)
        }
    }
    pub fn solve(&mut self,
                 mode: clingo_solve_mode_bitset_t,
                 assumptions: Vec<clingo_symbolic_literal_t>,
                 notify: clingo_solve_event_callback_t,
                 data: *mut ::std::os::raw::c_void)
                 -> Option<&mut ClingoSolveHandle> {
        unsafe {
            let ClingoControl(ref mut control) = *self;

            let mut handle = std::ptr::null_mut() as *mut clingo_solve_handle_t;
            let err = clingo_control_solve(control,
                                           mode,
                                           assumptions.as_ptr(),
                                           assumptions.len(),
                                           notify,
                                           data,
                                           &mut handle);
            if !err {
                None
            } else {
                Some(&mut *(handle as *mut ClingoSolveHandle))
            }

        }
    }

    //     pub fn clingo_control_cleanup(control: *mut ClingoControl) -> u8;
    //     pub fn clingo_control_assign_external(control: *mut ClingoControl,
    //                                           atom: clingo_symbol_t,
    //                                           value: clingo_truth_value_t)
    //                                           -> u8;
    //     pub fn clingo_control_release_external(control: *mut ClingoControl,
    //                                            atom: clingo_symbol_t)
    //                                            -> u8;

    pub fn register_propagator(&mut self,
                               propagator: *const clingo_propagator_t,
                               data: *mut ::std::os::raw::c_void,
                               sequential: bool)
                               -> bool {
        let ClingoControl(ref mut control) = *self;
        unsafe { clingo_control_register_propagator(control, propagator, data, sequential) }
    }
    pub fn statistics(&mut self) -> std::option::Option<&mut ClingoStatistics> {
        unsafe {
            let ClingoControl(ref mut control) = *self;
            let mut stat = std::ptr::null_mut() as *mut clingo_statistics_t;
            let err = clingo_control_statistics(control, &mut stat);
            if !err {
                None
            } else {
                Some(&mut *(stat as *mut ClingoStatistics))
            }
        }
    }
    //     pub fn clingo_control_interrupt(control: *mut ClingoControl);
    pub fn configuration(&mut self) -> std::option::Option<&mut ClingoConfiguration> {
        unsafe {
            let ClingoControl(ref mut control) = *self;
            let mut conf = std::ptr::null_mut() as *mut clingo_configuration_t;
            let err = clingo_control_configuration(control, &mut conf);
            if !err {
                None
            } else {
                Some(&mut *(conf as *mut ClingoConfiguration))
            }
        }
    }
    //     pub fn clingo_control_use_enumeration_assumption(control: *mut ClingoControl,
    //                                                      enable: u8)
    //                                                     -> u8;
    //     pub fn clingo_control_get_const(control: *mut ClingoControl,
    //                                     name: *const c_char,
    //                                     symbol: *mut clingo_symbol_t)
    //                                    -> u8;
    //     pub fn clingo_control_has_const(control: *mut ClingoControl,
    //                                     name: *const c_char,
    //                                     exists: *mut u8)
    //                                    -> u8;
    pub fn symbolic_atoms(&mut self) -> std::option::Option<&mut ClingoSymbolicAtoms> {
        unsafe {
            let ClingoControl(ref mut control) = *self;
            let mut atoms = std::ptr::null_mut() as *mut clingo_symbolic_atoms_t;
            let err = clingo_control_symbolic_atoms(control, &mut atoms);
            if !err {
                None
            } else {
                Some(&mut *(atoms as *mut ClingoSymbolicAtoms))
            }
        }
    }
    pub fn theory_atoms(&mut self) -> std::option::Option<&mut ClingoTheoryAtoms> {
        unsafe {
            let ClingoControl(ref mut control) = *self;
            let mut atoms = std::ptr::null_mut() as *mut clingo_theory_atoms_t;
            let err = clingo_control_theory_atoms(control, &mut atoms);
            if !err {
                None
            } else {
                Some(&mut *(atoms as *mut ClingoTheoryAtoms))
            }
        }
    }
    pub fn backend(&mut self) -> std::option::Option<&mut ClingoBackend> {
        unsafe {
            let ClingoControl(ref mut control) = *self;
            let mut backend = std::ptr::null_mut() as *mut clingo_backend_t;
            let err = clingo_control_backend(control, &mut backend);
            if !err {
                None
            } else {
                Some(&mut *(backend as *mut ClingoBackend))
            }
        }
    }
    pub fn program_builder(&mut self) -> std::option::Option<&mut ClingoProgramBuilder> {
        unsafe {
            let ClingoControl(ref mut control) = *self;
            let mut builder = std::ptr::null_mut() as *mut clingo_program_builder_t;
            let err = clingo_control_program_builder(control, &mut builder);
            if !err {
                None
            } else {
                Some(&mut *(builder as *mut ClingoProgramBuilder))
            }
        }
    }
}

pub struct ClingoProgramBuilder(clingo_program_builder_t);
impl ClingoProgramBuilder {
    pub fn begin(&mut self) -> bool {
        unsafe {
            let ClingoProgramBuilder(ref mut builder) = *self;
            clingo_program_builder_begin(builder)
        }
    }
}

pub struct ClingoConfiguration(clingo_configuration_t);
impl ClingoConfiguration {
    pub fn configuration_root(&mut self) -> std::option::Option<clingo_id_t> {
        unsafe {
            let ClingoConfiguration(ref mut conf) = *self;
            let mut root_key = 0 as clingo_id_t;
            let err = clingo_configuration_root(conf, &mut root_key);
            if !err { None } else { Some(root_key) }
        }
    }
    //     pub fn clingo_configuration_type(configuration: *mut ClingoConfiguration,
    //                                      key: clingo_id_t,
    //                                      type_: *mut clingo_configuration_type_bitset_t)
    //                                      -> u8;
    //     pub fn clingo_configuration_description(configuration: *mut ClingoConfiguration,
    //                                             key: clingo_id_t,
    //                                             description: *mut *const c_char)
    //                                             -> u8;
    //     pub fn clingo_configuration_array_size(configuration: *mut ClingoConfiguration,
    //                                            key: clingo_id_t,
    //                                            size: *mut size_t)
    //                                            -> u8;
    pub fn configuration_array_at(&mut self,
                                  key: clingo_id_t,
                                  offset: usize)
                                  -> std::option::Option<clingo_id_t> {
        unsafe {
            let ClingoConfiguration(ref mut conf) = *self;
            let mut nkey = 0 as clingo_id_t;
            let err = clingo_configuration_array_at(conf, key, offset, &mut nkey);
            if !err { None } else { Some(nkey) }
        }
    }
    //     pub fn clingo_configuration_map_size(configuration: *mut ClingoConfiguration,
    //                                          key: clingo_id_t,
    //                                          size: *mut size_t)
    //                                          -> u8;
    //     pub fn clingo_configuration_map_subkey_name(configuration: *mut ClingoConfiguration,
    //                                                 key: clingo_id_t,
    //                                                 offset: size_t,
    //                                                 name: *mut *const c_char)
    //                                                 -> u8;
    pub fn configuration_map_at(&mut self,
                                key: clingo_id_t,
                                name: &str)
                                -> std::option::Option<clingo_id_t> {
        unsafe {
            let ClingoConfiguration(ref mut conf) = *self;
            let mut nkey = 0 as clingo_id_t;
            let err = clingo_configuration_map_at(conf,
                                                  key,
                                                  CString::new(name).unwrap().as_ptr(),
                                                  &mut nkey);

            if !err { None } else { Some(nkey) }
        }
    }
    //     pub fn clingo_configuration_value_is_assigned(configuration: *mut ClingoConfiguration,
    //                                                   key: clingo_id_t,
    //                                                   assigned: *mut u8)
    //                                                   -> u8;
    //     pub fn clingo_configuration_value_get_size(configuration: *mut ClingoConfiguration,
    //                                                key: clingo_id_t,
    //                                                size: *mut size_t)
    //                                                -> u8;
    //     pub fn clingo_configuration_value_get(configuration: *mut ClingoConfiguration,
    //                                           key: clingo_id_t,
    //                                           value: *mut c_char,
    //                                           size: size_t)
    //                                           -> u8;
    pub fn configuration_value_set(&mut self, key: clingo_id_t, value: &str) -> bool {
        unsafe {
            let ClingoConfiguration(ref mut conf) = *self;
            clingo_configuration_value_set(conf, key, CString::new(value).unwrap().as_ptr())
        }
    }
}

pub struct ClingoBackend(clingo_backend_t);
impl ClingoBackend {
    pub fn rule(&mut self,
                choice: bool,
                head_vector: &Vec<clingo_atom_t>,
                body_vector: &Vec<clingo_literal_t>)
                -> bool {

        let head = head_vector.as_ptr();
        let head_size = head_vector.len();

        let body = body_vector.as_ptr();
        let body_size = body_vector.len();
        unsafe {
            let ClingoBackend(ref mut backend) = *self;
            clingo_backend_rule(backend, choice, head, head_size, body, body_size)
        }
    }
    //     pub fn clingo_backend_weight_rule(backend: *mut ClingoBackend,
    //                                       choice: u8,
    //                                       head: *const clingo_atom_t,
    //                                       head_size: size_t,
    //                                       lower_bound: clingo_weight_t,
    //                                       body: *const clingo_weighted_literal_t,
    //                                       body_size: size_t)
    //                                       -> u8;
    //     pub fn clingo_backend_minimize(backend: *mut ClingoBackend,
    //                                    priority: clingo_weight_t,
    //                                    literals: *const clingo_weighted_literal_t,
    //                                    size: size_t)
    //                                    -> u8;
    //     pub fn clingo_backend_project(backend: *mut ClingoBackend,
    //                                   atoms: *const clingo_atom_t,
    //                                   size: size_t)
    //                                   -> u8;
    //     pub fn clingo_backend_external(backend: *mut ClingoBackend,
    //                                    atom: clingo_atom_t,
    //                                    type_: clingo_external_type_t)
    //                                    -> u8;
    pub fn assume(&mut self, literals: *const clingo_literal_t, size: usize) -> bool {

        unsafe {
            let ClingoBackend(ref mut backend) = *self;
            clingo_backend_assume(backend, literals, size)
        }
    }
    //     pub fn clingo_backend_heuristic(backend: *mut ClingoBackend,
    //                                     atom: clingo_atom_t,
    //                                     type_: clingo_heuristic_type_t,
    //                                     bias: c_int,
    //                                     priority: ::std::os::raw::c_uint,
    //                                     condition: *const clingo_literal_t,
    //                                     size: size_t)
    //                                     -> u8;
    //     pub fn clingo_backend_acyc_edge(backend: *mut ClingoBackend,
    //                                     node_u: c_int,
    //                                     node_v: c_int,
    //                                     condition: *const clingo_literal_t,
    //                                     size: size_t)
    //                                     -> u8;
    pub fn add_atom(&mut self) -> std::option::Option<clingo_atom_t> {
        unsafe {
            let ClingoBackend(ref mut backend) = *self;
            let mut atom = 0 as clingo_atom_t;
            let err = clingo_backend_add_atom(backend, &mut atom);
            if !err { None } else { Some(atom) }
        }
    }
}

pub struct ClingoStatistics(clingo_statistics_t);
impl ClingoStatistics {
    pub fn statistics_root(&mut self) -> std::option::Option<u64> {
        unsafe {
            let ClingoStatistics(ref mut stats) = *self;
            let mut root_key = 0 as u64;
            let err = clingo_statistics_root(stats, &mut root_key);
            if !err { None } else { Some(root_key) }
        }
    }
    pub fn statistics_type(&mut self, key: u64) -> std::option::Option<clingo_statistics_type_t> {
        unsafe {
            let ClingoStatistics(ref mut stats) = *self;
            let mut stype = 0 as clingo_statistics_type_t;
            let err = clingo_statistics_type(stats, key, &mut stype);
            if !err { None } else { Some(stype) }
        }
    }
    pub fn statistics_array_size(&mut self, key: u64) -> std::option::Option<usize> {
        unsafe {
            let ClingoStatistics(ref mut stats) = *self;
            let mut size = 0 as usize;
            let err = clingo_statistics_array_size(stats, key, &mut size);
            if !err { None } else { Some(size) }
        }
    }
    pub fn statistics_array_at(&mut self, key: u64, offset: usize) -> std::option::Option<u64> {
        unsafe {
            let ClingoStatistics(ref mut stats) = *self;
            let mut subkey = 0 as u64;
            let err = clingo_statistics_array_at(stats, key, offset, &mut subkey);
            if !err { None } else { Some(subkey) }
        }
    }
    pub fn statistics_map_size(&mut self, key: u64) -> std::option::Option<usize> {
        unsafe {
            let ClingoStatistics(ref mut stats) = *self;
            let mut size = 0 as usize;
            let err = clingo_statistics_map_size(stats, key, &mut size);
            if !err { None } else { Some(size) }
        }
    }
    pub fn statistics_map_subkey_name<'a>(&mut self,
                                          key: u64,
                                          offset: usize)
                                          -> std::option::Option<&'a str> {
        unsafe {
            let ClingoStatistics(ref mut stats) = *self;
            let mut name = std::ptr::null() as *const c_char;

            let err = clingo_statistics_map_subkey_name(stats, key, offset, &mut name);
            if !err {
                None
            } else {
                Some(CStr::from_ptr(name).to_str().unwrap())
            }
        }
    }
    pub fn statistics_map_at(&mut self, key: u64, name: &str) -> std::option::Option<u64> {
        unsafe {
            let ClingoStatistics(ref mut stats) = *self;
            let mut subkey = 0 as u64;
            let err = clingo_statistics_map_at(stats,
                                               key,
                                               CString::new(name).unwrap().as_ptr(),
                                               &mut subkey);
            if !err { None } else { Some(subkey) }
        }
    }
    pub fn statistics_value_get(&mut self, key: u64) -> std::option::Option<f64> {
        unsafe {
            let ClingoStatistics(ref mut stats) = *self;
            let mut value = 0.0 as f64;
            let err = clingo_statistics_value_get(stats, key, &mut value);
            if !err { None } else { Some(value) }
        }
    }
}

pub struct ClingoSymbolicAtoms(clingo_symbolic_atoms_t);
impl ClingoSymbolicAtoms {
    pub fn begin(&mut self,
                 signature: *const clingo_signature_t)
                 -> std::option::Option<clingo_symbolic_atom_iterator_t> {
        unsafe {
            let ClingoSymbolicAtoms(ref mut atoms) = *self;
            let mut iterator = 0 as clingo_symbolic_atom_iterator_t;
            let err = clingo_symbolic_atoms_begin(atoms, signature, &mut iterator);
            if !err { None } else { Some(iterator) }
        }
    }
    pub fn end(&mut self) -> std::option::Option<clingo_symbolic_atom_iterator_t> {
        unsafe {
            let ClingoSymbolicAtoms(ref mut atoms) = *self;
            let mut iterator = 0 as clingo_symbolic_atom_iterator_t;
            let err = clingo_symbolic_atoms_end(atoms, &mut iterator);
            if !err { None } else { Some(iterator) }
        }
    }
    pub fn find(&mut self,
                symbol: clingo_symbol_t)
                -> std::option::Option<clingo_symbolic_atom_iterator_t> {
        unsafe {
            let ClingoSymbolicAtoms(ref mut atoms) = *self;
            let mut iterator = 0 as clingo_symbolic_atom_iterator_t;
            let err = clingo_symbolic_atoms_find(atoms, symbol, &mut iterator);
            if !err { None } else { Some(iterator) }
        }
    }
    pub fn iterator_is_equal_to(&mut self,
                                a: clingo_symbolic_atom_iterator_t,
                                b: clingo_symbolic_atom_iterator_t)
                                -> std::option::Option<bool> {
        unsafe {
            let ClingoSymbolicAtoms(ref mut atoms) = *self;
            let mut equal = false;
            let err = clingo_symbolic_atoms_iterator_is_equal_to(atoms, a, b, &mut equal);
            if !err { None } else { Some(equal) }
        }
    }
    pub fn symbol(&mut self,
                  iterator: clingo_symbolic_atom_iterator_t)
                  -> std::option::Option<clingo_symbol_t> {
        unsafe {
            let ClingoSymbolicAtoms(ref mut atoms) = *self;
            let mut symbol = 0 as clingo_symbol_t;
            let err = clingo_symbolic_atoms_symbol(atoms, iterator, &mut symbol);
            if !err { None } else { Some(symbol) }
        }
    }
    pub fn is_fact(&mut self,
                   iterator: clingo_symbolic_atom_iterator_t)
                   -> std::option::Option<bool> {
        unsafe {
            let ClingoSymbolicAtoms(ref mut atoms) = *self;
            let mut fact = false;
            let err = clingo_symbolic_atoms_is_fact(atoms, iterator, &mut fact);
            if !err { None } else { Some(fact) }
        }
    }
    pub fn is_external(&mut self,
                       iterator: clingo_symbolic_atom_iterator_t)
                       -> std::option::Option<bool> {
        unsafe {
            let ClingoSymbolicAtoms(ref mut atoms) = *self;
            let mut external = false;
            let err = clingo_symbolic_atoms_is_external(atoms, iterator, &mut external);
            if !err { None } else { Some(external) }
        }
    }
    pub fn literal(&mut self,
                   iterator: clingo_symbolic_atom_iterator_t)
                   -> std::option::Option<clingo_literal_t> {
        unsafe {
            let ClingoSymbolicAtoms(ref mut atoms) = *self;
            let mut literal = 0 as clingo_literal_t;
            let err = clingo_symbolic_atoms_literal(atoms, iterator, &mut literal);
            if !err { None } else { Some(literal) }
        }
    }
    //     pub fn clingo_symbolic_atoms_signatures_size(atoms: *mut ClingoSymbolicAtoms,
    //                                                  size: *mut size_t)
    //                                                  -> u8;
    //     pub fn clingo_symbolic_atoms_signatures(atoms: *mut ClingoSymbolicAtoms,
    //                                             signatures: *mut clingo_signature_t,
    //                                             size: size_t)
    //                                             -> u8;
    pub fn next(&mut self,
                iterator: clingo_symbolic_atom_iterator_t)
                -> std::option::Option<clingo_symbolic_atom_iterator_t> {
        unsafe {
            let ClingoSymbolicAtoms(ref mut atoms) = *self;
            let mut next = 0 as clingo_symbolic_atom_iterator_t;
            let err = clingo_symbolic_atoms_next(atoms, iterator, &mut next);
            if !err { None } else { Some(next) }
        }
    }
    //     pub fn clingo_symbolic_atoms_is_valid(atoms: *mut ClingoSymbolicAtoms,
    //                                           iterator: clingo_symbolic_atom_iterator_t,
    //                                           valid: *mut u8)
    //                                           -> u8;
}

pub struct ClingoTheoryAtoms(clingo_theory_atoms_t);
impl ClingoTheoryAtoms {
    //     pub fn clingo_theory_atoms_term_type(atoms: *mut ClingoTheoryAtoms,
    //                                          term: clingo_id_t,
    //                                          type_: *mut clingo_theory_term_type_t)
    //                                          -> u8;
    //     pub fn clingo_theory_atoms_term_number(atoms: *mut ClingoTheoryAtoms,
    //                                            term: clingo_id_t,
    //                                            number: *mut c_int)
    //                                            -> u8;
    pub fn term_name<'a>(&mut self, term: clingo_id_t) -> std::option::Option<&'a str> {
        unsafe {
            let ClingoTheoryAtoms(ref mut atoms) = *self;
            let mut char_ptr = std::ptr::null() as *const c_char;
            let err = clingo_theory_atoms_term_name(atoms, term, &mut char_ptr);
            if !err {
                None
            } else {
                let c_str = CStr::from_ptr(char_ptr);
                Some(c_str.to_str().unwrap())
            }
        }
    }
    //     pub fn clingo_theory_atoms_term_arguments(atoms: *mut ClingoTheoryAtoms,
    //                                               term: clingo_id_t,
    //                                               arguments: *mut *const clingo_id_t,
    //                                               size: *mut size_t)
    //                                               -> u8;
    //     pub fn clingo_theory_atoms_term_to_string_size(atoms: *mut ClingoTheoryAtoms,
    //                                                    term: clingo_id_t,
    //                                                    size: *mut size_t)
    //                                                    -> u8;
    //     pub fn clingo_theory_atoms_term_to_string(atoms: *mut ClingoTheoryAtoms,
    //                                               term: clingo_id_t,
    //                                               string: *mut c_char,
    //                                               size: size_t)
    //                                               -> u8;
    //     pub fn clingo_theory_atoms_element_tuple(atoms: *mut ClingoTheoryAtoms,
    //                                              element: clingo_id_t,
    //                                              tuple: *mut *const clingo_id_t,
    //                                              size: *mut size_t)
    //                                              -> u8;
    //     pub fn clingo_theory_atoms_element_condition(atoms: *mut ClingoTheoryAtoms,
    //                                                  element: clingo_id_t,
    //                                                  condition: *mut *const clingo_literal_t,
    //                                                  size: *mut size_t)
    //                                                  -> u8;
    //     pub fn clingo_theory_atoms_element_condition_id(atoms: *mut ClingoTheoryAtoms,
    //                                                     element: clingo_id_t,
    //                                                     condition: *mut clingo_literal_t)
    //                                                     -> u8;
    //     pub fn clingo_theory_atoms_element_to_string_size(atoms: *mut ClingoTheoryAtoms,
    //                                                       element: clingo_id_t,
    //                                                       size: *mut size_t)
    //                                                       -> u8;
    //     pub fn clingo_theory_atoms_element_to_string(atoms: *mut ClingoTheoryAtoms,
    //                                                  element: clingo_id_t,
    //                                                  string: *mut c_char,
    //                                                  size: size_t)
    //                                                  -> u8;
    pub fn size(&mut self) -> std::option::Option<usize> {
        unsafe {
            let ClingoTheoryAtoms(ref mut atoms) = *self;
            let mut size = 0 as usize;
            let err = clingo_theory_atoms_size(atoms, &mut size);
            if !err { None } else { Some(size) }
        }
    }
    pub fn atom_term(&mut self, atom: clingo_id_t) -> std::option::Option<clingo_id_t> {
        unsafe {
            let ClingoTheoryAtoms(ref mut atoms) = *self;
            let mut term = 0 as clingo_id_t;
            let err = clingo_theory_atoms_atom_term(atoms, atom, &mut term);
            if !err { None } else { Some(term) }
        }
    }
    //     pub fn clingo_theory_atoms_atom_elements(atoms: *mut ClingoTheoryAtoms,
    //                                              atom: clingo_id_t,
    //                                              elements: *mut *const clingo_id_t,
    //                                              size: *mut size_t)
    //                                              -> u8;
    pub fn atom_has_guard(&mut self, atom: clingo_id_t) -> std::option::Option<bool> {
        unsafe {
            let ClingoTheoryAtoms(ref mut atoms) = *self;
            let mut has_guard = false;
            let err = clingo_theory_atoms_atom_has_guard(atoms, atom, &mut has_guard);
            if !err { None } else { Some(has_guard) }
        }
    }
    //     pub fn clingo_theory_atoms_atom_guard(atoms: *mut ClingoTheoryAtoms,
    //                                           atom: clingo_id_t,
    //                                           connective: *mut *const c_char,
    //                                           term: *mut clingo_id_t)
    //                                           -> u8;
    pub fn atom_literal(&mut self, atom: clingo_id_t) -> std::option::Option<clingo_literal_t> {
        unsafe {
            let ClingoTheoryAtoms(ref mut atoms) = *self;
            let mut literal = 0 as clingo_literal_t;
            let err = clingo_theory_atoms_atom_literal(atoms, atom, &mut literal);
            if !err { None } else { Some(literal) }
        }
    }
    //     pub fn clingo_theory_atoms_atom_to_string_size(atoms: *mut ClingoTheoryAtoms,
    //                                                    atom: clingo_id_t,
    //                                                    size: *mut size_t)
    //                                                    -> u8;
    //     pub fn clingo_theory_atoms_atom_to_string(atoms: *mut ClingoTheoryAtoms,
    //                                               atom: clingo_id_t,
    //                                               string: *mut c_char,
    //                                               size: size_t)
    //                                               -> u8;
}

pub struct ClingoModel(clingo_model_t);
impl ClingoModel {
    pub fn model_type(&mut self) -> std::option::Option<clingo_model_type_t> {
        unsafe {
            let ClingoModel(ref mut model) = *self;
            let mut mtype = 0 as clingo_model_type_t;
            let err = clingo_model_type(model, &mut mtype);
            if !err { None } else { Some(mtype) }
        }
    }
    pub fn number(&mut self) -> std::option::Option<u64> {
        unsafe {
            let ClingoModel(ref mut model) = *self;
            let mut number = 0;
            let err = clingo_model_number(model, &mut number);
            if !err { None } else { Some(number) }
        }
    }
    //     pub fn clingo_model_symbols_size(model: *mut ClingoModel,
    //                                      show: clingo_show_type_bitset_t,
    //                                      size: *mut size_t)
    //                                      -> u8;
    pub fn symbols(&mut self,
                   show: clingo_show_type_bitset_t)
                   -> std::option::Option<Vec<clingo_symbol_t>> {
        let ClingoModel(ref mut model) = *self;
        let mut size: usize = 0;
        let size_p = &mut size as *mut usize;

        let err = unsafe { clingo_model_symbols_size(model, show, size_p) };
        if !err {
            None
        } else {
            let mut a1 = Vec::<clingo_symbol_t>::with_capacity(size);
            let slice = a1.as_mut_slice();
            let symbols = slice.as_ptr() as *mut clingo_symbol_t;
            let err = unsafe { clingo_model_symbols(model, show, symbols, size) };
            if !err {
                None
            } else {
                let res = unsafe { Vec::from_raw_parts(symbols, size, size) };
                Some(res)
            }
        }

    }
    //     pub fn clingo_model_contains(model: *mut ClingoModel,
    //                                  atom: clingo_symbol_t,
    //                                  contained: *mut u8)
    //                                  -> u8;
    //     pub fn clingo_model_cost_size(model: *mut ClingoModel, size: *mut size_t) -> u8;
    //     pub fn clingo_model_cost(model: *mut ClingoModel, costs: *mut int64_t, size: size_t) -> u8;
    //     pub fn clingo_model_optimality_proven(model: *mut ClingoModel, proven: *mut u8) -> u8;
    //     pub fn clingo_model_context(model: *mut ClingoModel,
    //                                 control: *mut *mut ClingoSolveControl)
    //                                 -> u8;
}

// impl clingo_symbol_t {
//     pub fn clingo_symbol_create_number(number: c_int, symbol: *mut clingo_symbol_t);
//     pub fn clingo_symbol_create_supremum(symbol: *mut clingo_symbol_t);
//     pub fn clingo_symbol_create_infimum(symbol: *mut clingo_symbol_t);
//     pub fn clingo_symbol_create_string(string: *const c_char, symbol: *mut clingo_symbol_t) -> u8;
//     pub fn clingo_symbol_create_id(name: *const c_char,
//                                    positive: u8,
//                                    symbol: *mut clingo_symbol_t)
//                                    -> u8;
//     pub fn clingo_symbol_create_function(name: *const c_char,
//                                          arguments: *const clingo_symbol_t,
//                                          arguments_size: size_t,
//                                          positive: u8,
//                                          symbol: *mut clingo_symbol_t)
//                                          -> u8;
//     pub fn clingo_symbol_number(symbol: clingo_symbol_t, number: *mut c_int) -> u8;
//     pub fn clingo_symbol_name(symbol: clingo_symbol_t, name: *mut *const c_char) -> u8;
//     pub fn clingo_symbol_string(symbol: clingo_symbol_t, string: *mut *const c_char) -> u8;
//     pub fn clingo_symbol_is_positive(symbol: clingo_symbol_t, positive: *mut u8) -> u8;
//     pub fn clingo_symbol_is_negative(symbol: clingo_symbol_t, negative: *mut u8) -> u8;
//     pub fn clingo_symbol_arguments(symbol: clingo_symbol_t,
//                                    arguments: *mut *const clingo_symbol_t,
//                                    arguments_size: *mut size_t)
//                                    -> u8;
//     pub fn clingo_symbol_type(symbol: clingo_symbol_t) -> clingo_symbol_type_t;
//     pub fn clingo_symbol_to_string_size(symbol: clingo_symbol_t, size: *mut size_t) -> u8;
//     pub fn to_string(&mut self) -> std::result::Result<CString, u8> {
//
//         let mut size: usize = 0;
//         let size_p = &mut size as *mut usize;
//         unsafe {
//             let err1 = clingo_symbol_to_string_size(self, size_p);
//             if err1 == 0 {
//                 Err(err1)
//             } else {
//                 let a1 = vec![1; size];
//                 let string = CString::from_vec_unchecked(a1);
//
//                 let err2 = clingo_symbol_to_string(self, string.as_ptr() as *mut c_char, size);
//                 if err2 == 0 {
//                     Err(err2)
//                 } else {
//                     Ok(string)
//                 }
//             }
//         }
//     }
//     pub fn clingo_symbol_is_equal_to(a: clingo_symbol_t, b: clingo_symbol_t) -> u8;
//     pub fn clingo_symbol_is_less_than(a: clingo_symbol_t, b: clingo_symbol_t) -> u8;
//     pub fn clingo_symbol_hash(symbol: clingo_symbol_t) -> size_t;
// }

pub struct ClingoSolveControl(clingo_solve_control_t);
impl ClingoSolveControl {
    pub fn add_clause(&mut self, clause: *const clingo_literal_t) -> bool {

        unsafe {
            let ClingoSolveControl(ref mut control) = *self;
            let size = 0; //TODO: comute size of clause
            clingo_solve_control_add_clause(control, clause, size)
        }
    }
}

pub struct ClingoPropagateControl(clingo_propagate_control_t);
impl ClingoPropagateControl {
    pub fn thread_id(&mut self) -> clingo_id_t {
        unsafe {
            let ClingoPropagateControl(ref mut control) = *self;
            clingo_propagate_control_thread_id(control)
        }
    }
    //     pub fn clingo_propagate_control_assignment(control: *mut ClingoPropagateControl)
    //                                                -> *mut clingo_assignment_t;
    pub fn add_clause(&mut self,
                      clause: &[clingo_literal_t],
                      type_: clingo_clause_type_t)
                      -> std::option::Option<bool> {
        unsafe {
            let ClingoPropagateControl(ref mut control) = *self;
            let size = 0; //TODO: compute size of claus
            let mut result = false;
            let err = clingo_propagate_control_add_clause(control,
                                                          clause.as_ptr(),
                                                          size,
                                                          type_,
                                                          &mut result);
            if !err { None } else { Some(result) }
        }
    }
    pub fn propagate(&mut self) -> std::option::Option<bool> {
        unsafe {
            let ClingoPropagateControl(ref mut control) = *self;
            let mut result = false;
            let err = clingo_propagate_control_propagate(control, &mut result);
            if !err { None } else { Some(result) }
        }
    }
}

pub struct ClingoPropagateInit(clingo_propagate_init_t);
impl ClingoPropagateInit {
    //     pub fn clingo_propagate_init_solver_literal(init: *mut ClingoPropagateInit,
    //                                                 aspif_literal: clingo_literal_t,
    //                                                 solver_literal: *mut clingo_literal_t)
    //                                                 -> u8;
    //     pub fn clingo_propagate_init_add_watch(init: *mut ClingoPropagateInit,
    //                                            solver_literal: clingo_literal_t)
    //                                            -> u8;
    //     pub fn clingo_propagate_init_symbolic_atoms(init: *mut ClingoPropagateInit,
    //                                                 atoms: *mut *mut ClingoSymbolicAtoms)
    //                                                 -> u8;
    //     pub fn c_lingo_propagate_init_theory_atoms(init: *mut ClingoPropagateInit,
    //                                               atoms: *mut *mut ClingoTheoryAtoms)
    //                                               -> u8;
    pub fn number_of_threads(&mut self) -> c_int {
        unsafe {
            let ClingoPropagateInit(ref mut init) = *self;
            let ret = clingo_propagate_init_number_of_threads(init);
            (ret as c_int)
        }
    }
}
pub struct ClingoSolveHandle(clingo_solve_handle);
impl ClingoSolveHandle {
    /// Get the next solve result.
    ///
    /// Blocks until the result is ready.
    /// When yielding partial solve results can be obtained, i.e.,
    /// when a model is ready, the result will be satisfiable but neither the search exhausted nor the optimality proven.
    ///
    /// @param[in] handle the target
    /// @param[out] result the solve result
    /// @return whether the call was successful; might set one of the following error codes:
    /// - ::clingo_error_bad_alloc
    /// - ::clingo_error_runtime if solving fails
    pub fn get(&mut self) -> Option<clingo_solve_result_bitset_t> {
        let ClingoSolveHandle(ref mut handle) = *self;
        let mut result = 0;
        let err = unsafe { clingo_solve_handle_get(handle, &mut result) };
        if !err { None } else { Some(result) }
    }

    /// Get the next model (or zero if there are no more models).
    ///
    /// @param[in] handle the target
    /// @param[out] model the model (it is NULL if there are no more models)
    /// @return whether the call was successful; might set one of the following error codes:
    /// - ::clingo_error_bad_alloc
    /// - ::clingo_error_runtime if solving fails

    pub fn model(&mut self) -> Option<&mut ClingoModel> {
        let ClingoSolveHandle(ref mut handle) = *self;
        let mut model = std::ptr::null_mut() as *mut clingo_model_t;
        unsafe {
            let err = clingo_solve_handle_model(handle, &mut model);
            if !err {
                None
            } else {
                Some(&mut *(model as *mut ClingoModel))
            }
        }
    }

    /// Discards the last model and starts the search for the next one.
    ///
    /// If the search has been started asynchronously, this function continues the search in the background.
    ///
    /// @note This function does not block.
    ///
    /// @return whether the call was successful; might set one of the following error codes:
    /// - ::clingo_error_bad_alloc
    /// - ::clingo_error_runtime if solving fails

    pub fn resume(&mut self) -> bool {
        let ClingoSolveHandle(ref mut handle) = *self;
        unsafe { clingo_solve_handle_resume(handle) }
    }
    /// Stops the running search and releases the handle.
    ///
    /// Blocks until the search is stopped (as if an implicit cancel was called before the handle is released).
    ///
    /// @param[in] handle the target
    /// @return whether the call was successful; might set one of the following error codes:
    /// - ::clingo_error_bad_alloc
    /// - ::clingo_error_runtime if solving fails
    pub fn close(&mut self) -> bool {
        let ClingoSolveHandle(ref mut handle) = *self;
        unsafe { clingo_solve_handle_close(handle) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn version_test() {
        let (ma, mi, re) = safe_clingo_version();
        assert!(ma == 5);
        assert!(mi == 0);
        assert!(re == 0);
    }
}
