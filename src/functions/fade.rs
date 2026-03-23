use std::f64;

use crate::functions::function::Function;

pub struct Fade {
    tau: f64,
    hit_times: Vec<f64>,
    start_value: f64,
}

impl Function for Fade {
    fn at(&self, time: f64) -> f64 {
        let mut i = self.hit_times.iter().len() - 1;
        while self.hit_times[i] > time {
            if i == 0 {
                return 0.0;
            }
            i -= 1;
        }

        let time_since_hit = time - self.hit_times[i];

        return self.start_value * f64::consts::E.powf(-self.tau * time_since_hit);
    }
}

impl Fade {
    pub fn new(tau: f64, hit_times: Vec<f64>, start_value: f64) -> Fade {
        Fade {
            tau,
            hit_times,
            start_value,
        }
    }
}
