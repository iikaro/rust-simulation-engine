pub mod friction {
    pub mod utils {
        pub fn calc_reynolds(rho: f64, mu: f64, length: f64, v: f64) -> f64 {
            rho * v * length / mu
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::friction::utils::calc_reynolds;

    #[test]
    fn calc_reynolds_laminar() {
        let rho = 1000.;
        let mu = 0.001;
        let length = 1.;
        let v = 1.;
        let result = calc_reynolds(rho, mu, length, v);
        assert_eq!(result, 1000000.);
    }
}
