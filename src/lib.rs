pub mod ivp_solvers;
use ndarray;

type NDvector = ndarray::Array1<f64>;
type NDMatrix = ndarray::Array2<f64>;

#[cfg(test)]
mod tests {
    use std::path::Path;
    use crate::{IVPproblem, NDvector, ivp_solvers};
    use crate::ivp_solvers::runge_kutta::TableauLoader;

    fn test_rhs(t: f64, u: &NDvector)  -> NDvector {
        5.0*u*(1.0-u)
    }


    #[test]
    fn test_initial_cond() {
        let p = IVPproblem {
            t0: 0.0,
            tmax: 13.0,
            step_size: 0.05,
            initial_cond: ndarray::array![0.1],
            rhs: test_rhs
        };
        let res = ivp_solvers::euler_methods::explicit_euler(&p);

        //println!("{}", res.sol_values);
        assert_eq!(p.initial_cond, res.sol_values.row(0))
    }

    #[test]
    fn read_expl_rk_tableau() {
        // NOT WORKING, FIX!!!!
        let p = "C:\\Users\\Vasil\\CLionProjects\\rust_ode_solvers\\src\\ivp_solvers\\explicit_rk_tableau.json";
        let path = Path::new(p);
        let bt = ivp_solvers::runge_kutta::ExplicitButcherTableau::load_from_file(&path);

        println!("{}", bt.time_weights)
    }
}

pub struct IVPproblem {
    t0: f64,
    tmax: f64,
    step_size: f64,
    initial_cond: NDvector,
    rhs: fn(f64, &NDvector) -> NDvector,
}

pub struct IVPSolutionSparse {
    time_nodes: NDvector,
    sol_values: NDMatrix,
}
