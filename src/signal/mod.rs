pub mod timeseries {

    pub struct TimeSeries {
        pub timestamps: Vec<f64>,
        pub values: Vec<f64>,
    }

    impl TimeSeries {
        pub fn new(values: Vec<f64>) -> TimeSeries {
            let timestamps = vec![0.; values.len()];
            TimeSeries {
                timestamps: timestamps,
                values: values,
            }
        }
        pub fn length(&self) -> usize {
            self.timestamps.len()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::signal::timeseries::TimeSeries;

    #[test]
    fn test_new() {
        let values = vec![1.0, 2.5, 3.2, 4.0, 3.0];

        let timeseries = TimeSeries::new(values);
        assert_eq!(timeseries.length(), 5);
    }
}
