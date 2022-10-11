use crate::hydrostatic::hydrostatic::compute_hydrostatic_pressure;
use friction::friction::utils::calc_reynolds;
pub mod hydrostatic;

fn main() {
    let rho = 1000.;
    let depth = 1.;

    let p_water = compute_hydrostatic_pressure(rho, depth);

    let mu = 0.001;
    let length = 1.;
    let v = 1.;

    let re = calc_reynolds(rho, mu, length, v);

    println!("Hello, world! Water exerts {p_water} Pa per meter of depth and Re = {re}.");
}
