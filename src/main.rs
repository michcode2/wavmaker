use crate::functions::{constant::Constant, function, linear::Linear, onoff::OnOff};
use std::{f64, io::Write};

mod functions;

fn main() {
    let duration = 20;
    let per_sec = 44100;

    let data = WavFile {
        samples_per_sec: per_sec,
        bits_per_sample: 8,
        num_samples: per_sec * duration,
    };

    let on_length = 0.5;

    let a = Frequency {
        frequency: Constant::new(220.0),
        volume: OnOff::new(vec![0.0], vec![on_length], Box::new(Constant::new(0.75))),
    };

    let b = Frequency {
        frequency: Constant::new(249.93),
        volume: OnOff::new(
            vec![on_length],
            vec![on_length],
            Box::new(Constant::new(0.75)),
        ),
    };
    let c = Frequency {
        frequency: Constant::new(261.63),
        volume: OnOff::new(
            vec![2.0 * on_length],
            vec![on_length],
            Box::new(Constant::new(0.75)),
        ),
    };

    let d = Frequency {
        frequency: Constant::new(293.66),
        volume: OnOff::new(
            vec![3.0 * on_length],
            vec![on_length],
            Box::new(Constant::new(0.75)),
        ),
    };

    let e = Frequency {
        frequency: Constant::new(329.63),
        volume: OnOff::new(
            vec![4.0 * on_length],
            vec![on_length],
            Box::new(Constant::new(0.75)),
        ),
    };
    let f = Frequency {
        frequency: Constant::new(349.23),
        volume: OnOff::new(
            vec![5.0 * on_length],
            vec![on_length],
            Box::new(Constant::new(0.75)),
        ),
    };
    let g = Frequency {
        frequency: Constant::new(392.0),
        volume: OnOff::new(
            vec![6.0 * on_length],
            vec![on_length],
            Box::new(Constant::new(0.75)),
        ),
    };
    let a2 = Frequency {
        frequency: Constant::new(440.0),
        volume: OnOff::new(
            vec![7.0 * on_length],
            vec![on_length],
            Box::new(Constant::new(0.75)),
        ),
    };

    let constant = Frequency {
        frequency: Linear::new_all(0.0, 8.0 * on_length, (220.0) / (8.0 * on_length), 220.0),
        volume: OnOff::new(
            vec![0.0],
            vec![8.0 * on_length],
            Box::new(Constant::new(0.25)),
        ),
    };

    let frequencies = vec![a, b, c, d, e, f, g, a2, constant];

    let mut bytes = data.create_header();

    for sample_number in 0..data.num_samples {
        let t = sample_number as f64 / data.samples_per_sec as f64;
        let value = data.add_tones_u8(
            &frequencies
                .iter()
                .map(|f| f.sample_at(t))
                .collect::<Vec<f64>>(),
        );
        bytes.extend(value.to_le_bytes());
    }

    let mut file = std::fs::File::create("./output.wav").unwrap();
    file.write(&bytes).unwrap();
}

struct Frequency<F: function::Function, V: function::Function> {
    frequency: F,
    volume: V,
}

impl<F: function::Function, V: function::Function> Frequency<F, V> {
    fn sample_at(&self, time: f64) -> f64 {
        let unit_value = (time * self.frequency.at(time) * f64::consts::PI * 2.0).sin();
        let corrected_value = unit_value * self.volume.at(time);
        return corrected_value;
    }
}

struct WavFile {
    samples_per_sec: u32,
    bits_per_sample: u16,
    num_samples: u32,
}

impl WavFile {
    fn create_header(&self) -> Vec<u8> {
        let mut bytes = b"RIFF".to_vec();
        bytes.extend((36 + self.num_samples * (self.bits_per_sample as u32) / 8).to_le_bytes());
        bytes.extend(b"WAVEfmt ");
        bytes.extend(16_u32.to_le_bytes()); //ckSize
        bytes.extend(1_u16.to_le_bytes()); //wFormatTag (PCM)
        bytes.extend(1_u16.to_le_bytes()); //wChannels
        bytes.extend(self.samples_per_sec.to_le_bytes()); // dwSamplesPerSec
        bytes.extend((self.samples_per_sec * (self.bits_per_sample as u32 / 8)).to_le_bytes()); //dwAvgBytesPerSec
        bytes.extend(1_u16.to_le_bytes()); //wBlockAlign
        bytes.extend(self.bits_per_sample.to_le_bytes()); // wBitsPerSample
        bytes.extend(b"data");
        bytes.extend((self.num_samples * (self.bits_per_sample as u32 / 8)).to_le_bytes()); //chunk size

        return bytes;
    }

    fn add_tones_u8(&self, tones: &[f64]) -> u8 {
        let unit_added: f64 = tones.iter().sum();
        let fullscale = unit_added * 128.0;
        let actual: f64 = (fullscale + 127.0).min(255.0).max(0.0);
        return actual as u8;
    }
}
