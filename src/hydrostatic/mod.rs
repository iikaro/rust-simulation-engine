/// Compute the relative pressure.
pub fn compute_hydrostatic_pressure(rho: f64, z: f64) -> f64 {
    const GRAVITY: f64 = 9.80665;
    rho * GRAVITY * z
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn water_column_pressure() {
        let rho = 1000.;
        let depth = 1.;
        let pressure = compute_hydrostatic_pressure(rho, depth);

        assert_eq!(9806.65, pressure);
    }
}
