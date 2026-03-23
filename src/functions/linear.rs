use crate::functions::function::Function;

pub struct Linear {
    start_time: Option<f64>,
    end_time: Option<f64>,
    gradient: f64,
    offset: f64,
}

impl Function for Linear {
    fn at(&self, mut time: f64) -> f64 {
        if let Some(end) = self.end_time {
            if end < time {
                return 0.0;
            }
        }
        if let Some(start) = self.start_time {
            if time < start {
                return 0.0;
            }
            time = start - time;
        }

        return self.gradient * time + self.offset;
    }
}

impl Linear {
    pub fn new_all(start: f64, end: f64, gradient: f64, offset: f64) -> Linear {
        Linear {
            start_time: Some(start),
            end_time: Some(end),
            gradient,
            offset,
        }
    }
    pub fn new_always(gradient: f64, offset: f64) -> Linear {
        Linear {
            gradient,
            offset,
            start_time: None,
            end_time: None,
        }
    }
}
