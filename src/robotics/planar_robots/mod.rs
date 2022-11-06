pub mod planar_robots {
    use crate::algebra::algebra::Coordinates;
    use crate::robotics;
    use crate::signal::timeseries::TimeSeries;

    pub struct PlanarRobot {
        pub name: String,
        pub dof: u8,
        pub length: f64,
    }

    impl robotics::Robot for PlanarRobot {
        fn forward_kinematics(&self, angle: f64) -> Coordinates {
            Coordinates {
                x: self.length * angle.sin(),
                y: self.length * angle.cos(),
            }
        }

        fn simulate_trajectory(&self, input: TimeSeries) -> Vec<Coordinates> {
            let mut coordinates: Vec<Coordinates> = vec![];
            for angle in input.values {
                coordinates.push(self.forward_kinematics(angle))
            }
            return coordinates;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{robotics::Robot, signal::timeseries::TimeSeries};

    use super::planar_robots::PlanarRobot;

    #[test]
    fn forward_kinematics_1_dof() {
        let robot = PlanarRobot {
            name: "1-dof planar robot".to_owned(),
            dof: 1,
            length: 1.,
        };
        println!("{}", robot.name);
        println!(
            "The robot EE coordinates are {}",
            robot.forward_kinematics(0.)
        );
        let values = vec![1.0, 2.5, 3.2, 4.0, 3.0];
        let angles = TimeSeries::new(values);
        let coordinates = robot.simulate_trajectory(angles);

        for coordinate in coordinates {
            println!("{}", coordinate)
        }
    }
}
