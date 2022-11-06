pub mod algebra {
    use std::fmt;

    pub struct Coordinates {
        pub x: f64,
        pub y: f64,
    }

    impl fmt::Display for Coordinates {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::algebra::algebra::Coordinates;

    #[test]
    fn print_origin_coordinates() {
        let origin = Coordinates { x: 0., y: 0. };
        assert_eq!(format!("The origin is: {origin}"), "The origin is: (0, 0)");
    }
}
