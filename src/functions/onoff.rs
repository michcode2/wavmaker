use crate::functions::function::Function;

pub struct OnOff {
    switch_on: Vec<f64>,
    durations: Vec<f64>,
    on_function: Box<dyn Function>,
}

impl Function for OnOff {
    fn at(&self, time: f64) -> f64 {
        if self.switch_on[self.switch_on.len() - 1] + self.durations[self.durations.len() - 1]
            < time
        {
            return 0.0;
        }

        let mut i = 0;
        while self.switch_on[i] + self.durations[i] < time {
            i += 1;
        }

        if self.switch_on[i] + self.durations[i] > time && time > self.switch_on[i] {
            return self.on_function.at(time);
        }

        return 0.0;
    }
}

impl OnOff {
    pub fn new(switch_on: Vec<f64>, durations: Vec<f64>, on_function: Box<dyn Function>) -> OnOff {
        OnOff {
            switch_on,
            durations,
            on_function,
        }
    }
}
