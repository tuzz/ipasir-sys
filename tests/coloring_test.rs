use ipasir_sys::*;

#[test]
fn it_can_incrementally_solve_a_coloring_problem() {
    let a_is_white = 1;
    let a_is_black = 2;

    let b_is_white = 3;
    let b_is_black = 4;

    unsafe {
        let solver = ipasir_init();

        // a is white or black
        ipasir_add(solver, a_is_white);
        ipasir_add(solver, a_is_black);
        ipasir_add(solver, 0);

        // b is white or black
        ipasir_add(solver, b_is_white);
        ipasir_add(solver, b_is_black);
        ipasir_add(solver, 0);

        // both a and b cannot be white
        ipasir_add(solver, -a_is_white);
        ipasir_add(solver, -b_is_white);
        ipasir_add(solver, 0);

        // both a and b cannot be black
        ipasir_add(solver, -a_is_black);
        ipasir_add(solver, -b_is_black);
        ipasir_add(solver, 0);

        // assume a and b are both white
        ipasir_assume(solver, a_is_white);
        ipasir_assume(solver, b_is_white);

        // the formula is unsatisfiable
        let unsat_status = ipasir_solve(solver);
        assert_eq!(unsat_status, 20);

        // this assumption caused its unsatisfiability
        let caused_unsat = ipasir_failed(solver, a_is_white);
        assert_eq!(caused_unsat, 1);

        // assume only a is white
        ipasir_assume(solver, a_is_white);

        // the formula is satisfiable
        let sat_status = ipasir_solve(solver);
        assert_eq!(sat_status, 10);

        // the solver has assigned a as white and b as black
        assert_eq!(ipasir_val(solver, a_is_white), a_is_white);
        assert_eq!(ipasir_val(solver, a_is_black), -a_is_black);

        assert_eq!(ipasir_val(solver, b_is_white), -b_is_white);
        assert_eq!(ipasir_val(solver, b_is_black), b_is_black);
    }
}
