use crate::*; // use top-level deps

pub fn explicit_euler(problem: &IVPproblem) -> IVPSolutionSparse {
    let time_mesh = ndarray::Array::range(problem.t0, problem.tmax, problem.step_size);
    let n = time_mesh.shape()[0]; // number of nodes
    let m = problem.initial_cond.shape()[0]; // number of values (vec dim) per node)

    let mut sol_vals = ndarray::Array::zeros([n, m]);
    sol_vals.row_mut(0).assign(&problem.initial_cond);

    for i in 0..n - 1 {
        let old_row = sol_vals.row(i);
        let new_row =
            &old_row + problem.step_size * (problem.rhs)(time_mesh[i], &old_row.to_owned());

        sol_vals.row_mut(i + 1).assign(&new_row);
    }

    IVPSolutionSparse{
        time_nodes: time_mesh,
        sol_values: sol_vals
    }
}
