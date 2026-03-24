use crate::functions::{
    constant::Constant,
    fade::Fade,
    function::{self, Function},
    linear::Linear,
    onoff::OnOff,
};
use std::{f64, io::Write};

mod functions;

fn main() {
    let duration = 6;
    let per_sec = 44100;

    let data = WavFile {
        samples_per_sec: per_sec,
        bits_per_sample: 8,
        num_samples: per_sec * duration,
    };

    let mut bytes = data.create_header();

    let harmonic_frequency =
        |fundimental: f64, harmonic_number: u8| fundimental * (harmonic_number as f64);

    let harmonic_volume = |harmonic_number: u8| {
        let h = harmonic_number as f64;
        (-0.5 * h).exp() * h.powi(2) / 2.5
    };

    let note = Harmonic::new(
        Constant::new(110.0),
        Fade::new(10.0, vec![0.5], 1.0),
        harmonic_frequency,
        harmonic_volume,
        5,
    );

    for sample_number in 0..data.num_samples {
        let t = sample_number as f64 / data.samples_per_sec as f64;

        let harmonics_tones = note.sample_at(t);

        let value = data.add_tones_u8(&harmonics_tones);
        bytes.extend(value.to_le_bytes());
    }

    let mut file = std::fs::File::create("./output.wav").unwrap();
    file.write(&bytes).unwrap();
}

struct Harmonic {
    fundimental: Box<dyn Function>,
    harmonic_frequency: Box<dyn Fn(f64, u8) -> f64>,
    harmonic_volume: Box<dyn Fn(u8) -> f64>,
    num_harmonics: u8,
    fundimental_volume: Box<dyn Function>,
}

impl Harmonic {
    fn new<
        F: function::Function + 'static,
        V: function::Function + 'static,
        HF: Fn(f64, u8) -> f64 + 'static,
        HV: Fn(u8) -> f64 + 'static,
    >(
        fundimental: F,
        fundimental_volume: V,
        harmonic_frequency: HF,
        harmonic_volume: HV,
        num_harmonics: u8,
    ) -> Harmonic {
        Harmonic {
            fundimental: Box::new(fundimental),
            harmonic_frequency: Box::new(harmonic_frequency),
            harmonic_volume: Box::new(harmonic_volume),
            num_harmonics,
            fundimental_volume: Box::new(fundimental_volume),
        }
    }

    fn sample_at(&self, time: f64) -> Vec<f64> {
        let fundimental_frequency = self.fundimental.at(time);
        let fundimental_volume = self.fundimental_volume.at(time);

        let mut all_tones = vec![Tone::new(
            Constant::new(fundimental_frequency),
            Constant::new(fundimental_volume),
        )];

        for harmonic_number in 1..=self.num_harmonics {
            let frequency = (self.harmonic_frequency)(fundimental_frequency, harmonic_number);
            let volume = (self.harmonic_volume)(harmonic_number) * fundimental_volume;
            let this_tone = Tone::new(Constant::new(frequency), Constant::new(volume));
            all_tones.push(this_tone);
        }

        return all_tones
            .iter()
            .map(|t| t.sample_at(time))
            .collect::<Vec<f64>>();
    }
}

struct Tone {
    frequency: Box<dyn function::Function>,
    volume: Box<dyn function::Function>,
}

impl Tone {
    fn sample_at(&self, time: f64) -> f64 {
        let unit_value = (time * self.frequency.at(time) * f64::consts::PI * 2.0).sin();
        let corrected_value = unit_value * self.volume.at(time);
        return corrected_value;
    }

    fn new<F: function::Function + 'static, V: function::Function + 'static>(
        frequency: F,
        volume: V,
    ) -> Tone {
        Tone {
            frequency: Box::new(frequency),
            volume: Box::new(volume),
        }
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
