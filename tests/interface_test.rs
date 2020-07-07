use ipasir_sys::*;
use std::ffi::{CStr, c_void};

mod ipasir_signature {
    use super::*;

    #[test]
    fn it_returns_the_name_and_version_of_the_sat_solver() {
        let c_buffer = unsafe { ipasir_signature() };
        let c_string = unsafe { CStr::from_ptr(c_buffer) };
        let signature = c_string.to_str().unwrap();

        assert_eq!(signature, "cadical-1.3.1");
    }
}

mod ipasir_init {
    use super::*;

    #[test]
    fn it_constructs_a_new_solver_and_returns_a_pointer_to_it() {
        let _solver: *mut c_void = unsafe { ipasir_init() };
    }
}

mod ipasir_release {
    use super::*;

    #[test]
    fn it_releases_the_solver_and_all_its_resources() {
        let solver = unsafe { ipasir_init() };

        unsafe { ipasir_release(solver) };
    }
}

mod ipasir_add {
    use super::*;

    #[test]
    fn it_adds_a_literal_to_the_current_clause_and_finalises_with_zero() {
        let solver = unsafe { ipasir_init() };

        unsafe { ipasir_add(solver, 1) };
        unsafe { ipasir_add(solver, -2) };
        unsafe { ipasir_add(solver, 0) };
    }
}

mod ipasir_assume {
    use super::*;

    #[test]
    fn it_adds_an_assumption_for_the_next_sat_search() {
        let solver = unsafe { ipasir_init() };

        unsafe { ipasir_assume(solver, 1) };
    }
}

mod ipasir_solve {
    use super::*;

    #[test]
    fn it_solves_the_formula_and_returns_10_for_sat_and_20_for_unsat() {
        let solver = unsafe { ipasir_init() };

        unsafe { ipasir_add(solver, 1) };
        unsafe { ipasir_add(solver, 0) };

        let sat_status = unsafe { ipasir_solve(solver) };
        assert_eq!(sat_status, 10);

        unsafe { ipasir_add(solver, -1) };
        unsafe { ipasir_add(solver, 0) };

        let unsat_status = unsafe { ipasir_solve(solver) };
        assert_eq!(unsat_status, 20);
    }

    // TODO interrupted
}

mod ipasir_val {
    use super::*;

    #[test]
    fn it_gets_the_truth_value_of_a_literal_in_the_satisfying_assigment() {
        let solver = unsafe { ipasir_init() };

        unsafe { ipasir_add(solver, 1) };
        unsafe { ipasir_add(solver, 0) };

        unsafe { ipasir_add(solver, -2) };
        unsafe { ipasir_add(solver, 0) };

        unsafe { ipasir_solve(solver) };

        let true_literal = unsafe { ipasir_val(solver, 1) };
        let false_literal = unsafe { ipasir_val(solver, 2) };

        assert_eq!(true_literal, 1);
        assert_eq!(false_literal, -2);
    }
}

mod ipasir_failed {
    use super::*;

    #[test]
    fn it_returns_1_if_the_assumption_caused_the_formula_to_be_unsat() {
        let solver = unsafe { ipasir_init() };

        unsafe { ipasir_add(solver, 1) };
        unsafe { ipasir_add(solver, 0) };

        unsafe { ipasir_assume(solver, -1) };

        let unsat_status = unsafe { ipasir_solve(solver) };
        assert_eq!(unsat_status, 20);

        let caused_unsat = unsafe { ipasir_failed(solver, -1) };
        assert_eq!(caused_unsat, 1);
    }

    #[test]
    fn it_returns_0_if_the_assumption_did_not_cause_the_formula_to_be_unsat() {
        let solver = unsafe { ipasir_init() };

        unsafe { ipasir_add(solver, 1) };
        unsafe { ipasir_add(solver, 0) };

        unsafe { ipasir_add(solver, -1) };
        unsafe { ipasir_add(solver, 0) };

        unsafe { ipasir_assume(solver, 2) };

        let unsat_status = unsafe { ipasir_solve(solver) };
        assert_eq!(unsat_status, 20);

        let caused_unsat = unsafe { ipasir_failed(solver, 2) };
        assert_eq!(caused_unsat, 0);
    }

    #[test]
    fn it_returns_0_if_the_polarity_of_the_literal_does_not_match_the_assumption() {
        // This case is slightly ambigious
        // See: https://github.com/biotomas/ipasir/issues/9

        let solver = unsafe { ipasir_init() };

        unsafe { ipasir_add(solver, 1) };
        unsafe { ipasir_add(solver, 0) };

        unsafe { ipasir_assume(solver, -1) };

        let unsat_status = unsafe { ipasir_solve(solver) };
        assert_eq!(unsat_status, 20);

        let caused_unsat = unsafe { ipasir_failed(solver, 1) }; // not -1
        assert_eq!(caused_unsat, 0);
    }
}

mod ipasir_set_terminate {
    use super::*;

    #[test]
    fn it_sets_a_callback_that_determines_whether_the_solver_should_terminate() {
        let solver = unsafe { ipasir_init() };

        // This can be anything and is passed through to the callback.
        // It is nothing to do with the state machine in the IPASIR spec.
        let state = 123 as *mut c_void;

        unsafe { ipasir_set_terminate(solver, state, Some(callback)) };
    }

    extern "C" fn callback(_state: *mut c_void) -> i32 {
        0
    }
}

// Cadical doesn't support this IPASIR function so disable the test.
#[cfg(feature = "ipasir_set_learn")]
mod ipasir_set_learn {
    use super::*;

    #[test]
    fn it_sets_a_callback_that_receives_learned_clauses_up_to_a_given_length() {
        let solver = unsafe { ipasir_init() };

        let state = 123 as *mut c_void;
        let max_length = 3;

        unsafe { ipasir_set_learn(solver, state, max_length, Some(callback)) };
    }

    extern "C" fn callback(_state: *mut c_void, _clause: *mut i32) { }
}
