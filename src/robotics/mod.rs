pub mod planar_robots;
pub mod cinematics;
use crate::{algebra::algebra::Coordinates, signal::timeseries::TimeSeries};

pub fn robotics_function(){
    println!("Robotics!")
}

trait Robot {
    fn forward_kinematics(&self, angle: f64) -> Coordinates;
    fn simulate_trajectory(&self, input: TimeSeries) -> Vec<Coordinates>;
}

