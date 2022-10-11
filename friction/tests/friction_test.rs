use friction::friction::utils::calc_reynolds;

#[test]
fn calc_reynolds_laminar() {
    let rho = 1000.;
    let mu = 0.001;
    let length = 1.;
    let v = 1.;
    let result = calc_reynolds(rho, mu, length, v);
    assert_eq!(result, 1000000.);
}
