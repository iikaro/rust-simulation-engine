pub mod algebra;
pub mod friction;
pub mod hydrostatic;
pub mod robotics;
pub mod signal;

// fn main() {
//     let rho = 1000.;
//     let depth = 1.;

//     let p_water = hydrostatic::compute_hydrostatic_pressure(rho, depth);

//     let mu = 0.001;
//     let length = 1.;
//     let v = 1.;

//     let re = friction::utils::calc_reynolds(rho, mu, length, v);

//     println!("Hello, world! Water exerts {p_water} Pa per meter of depth and Re = {re}.");
// }

use plotters::prelude::*;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("plotters-doc-data/0.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("y=x^2", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(-1f32..1f32, -0.1f32..1f32)?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(
            (-50..=50).map(|x| x as f32 / 50.0).map(|x| (x, x * x)),
            &RED,
        ))?
        .label("y = x^2")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;

    Ok(())
}
