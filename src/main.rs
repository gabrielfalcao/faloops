use std::time::{Duration, Instant};

use clap::Parser;
use cpal::platform::Device;
use cpal::traits::{DeviceTrait, HostTrait};
use faloops::{counter, Error, Result};
use rand::distr::{Distribution, Uniform};
use rand_chacha::ChaCha12Rng;
use rand_core::SeedableRng;

fn valid_frequency_range(val: &str) -> ::std::result::Result<f64, String> {
    let freq = val.parse::<f64>().map_err(|error| {
        format!("{val:#?} is not a valid frequency number: {error}")
    })?;
    if freq < 22.0 {
        Err(format!("minimum frequency is 22hz"))
    } else if freq > 22000.0 {
        Err(format!("maximum frequency is 22000hz"))
    } else {
        Ok(freq)
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[arg(
        short,
        long = "pulse duration (milliseconds)",
        default_value = "60"
    )]
    duration_millis: u64,

    #[arg(
        short,
        long = "execution timeout (seconds)"
    )]
    timeout_secs: Option<u64>,

    #[arg(
        short = 'm',
        long,
        help = "minimum of pulse duration",
        default_value = "240"
    )]
    min_duration: u64,

    #[arg(
        short = 'M',
        long,
        help = "maximum of pulse duration",
        default_value = "720"
    )]
    max_duration: u64,

    #[arg(short, long, help="beginning of frequency range", default_value = "9000.0", value_parser = valid_frequency_range)]
    beg_frequency: f64,

    #[arg(short, long, help="end of frequency range", default_value = "22000.0", value_parser = valid_frequency_range)]
    end_frequency: f64,

    #[arg(
        short,
        long,
        help = "name of output device (for playback)"
    )]
    output_device: Option<String>,

    #[arg(
        short,
        long,
        help = "random seed to initialize the pseudo-random number generator"
    )]
    random_seed: Option<u64>,

    #[arg(
        short,
        long,
        help = "lists names of available output devices and exits"
    )]
    list: bool,
}
impl Cli {
    pub fn device(&self) -> Result<Device> {
        let host = cpal::default_host();
        if self.output_device.is_none() {
            let default = host.default_output_device().ok_or_else(|| {
                Error::DevicesError(format!("no output device available"))
            })?;
            return Ok(default);
        }

        let devices = host.output_devices()?.collect::<Vec<Device>>();
        let total = devices.len();
        let selected_name = self.output_device.clone().unwrap();

        for (index, device) in devices.into_iter().enumerate() {
            let index = index + 1;
            match device.name() {
                Ok(name) =>
                    if name.to_lowercase() == selected_name.to_lowercase() {
                        return Ok(device);
                    },
                Err(error) => {
                    eprintln!("could not obtain name of output device {index}/{total}: {error}")
                },
            }
        }
        let mut error_messages = Vec::<String>::new();
        error_messages
            .push(format!("could not set {selected_name:#?} as output device"));
        if let Some(device) = host.default_output_device() {
            error_messages
                .push(format!("no named output devices found. Using default."));
            for message in error_messages {
                eprintln!("{message}");
            }
            return Ok(device);
        } else {
            error_messages.push(format!("no output devices found."));
        }
        Err(Error::DevicesError(error_messages.join("\n")))
    }

    pub fn rng(&self) -> ChaCha12Rng {
        if let Some(seed) = &self.random_seed {
            ChaCha12Rng::seed_from_u64(*seed)
        } else {
            ChaCha12Rng::from_rng(&mut rand::rng())
        }
    }

    pub fn duration_range(&self) -> Result<Uniform<u64>> {
        Ok(Uniform::new(self.min_duration, self.max_duration)?)
    }

    pub fn frequency_range(&self) -> Result<Uniform<f64>> {
        Ok(Uniform::new(self.beg_frequency, self.end_frequency)?)
    }

    pub fn dispatch() -> Result<()> {
        ctrlc::set_handler(|| {
            eprintln!("\rExitting due to Ctrl-C");
            std::process::exit(201);
        })?;

        let args = Self::parse();

        if args.list {
            list_devices(&args)?;
        } else {
            play(&args)?;
        }

        Ok(())
    }
}
fn main() {
    match Cli::dispatch() {
        Ok(_) => {},
        Err(error) => {
            eprintln!("ERROR: {error}");
            std::process::exit(101);
        },
    }
}

pub fn list_devices(_: &Cli) -> Result<()> {
    let host = cpal::default_host();
    let devices = host.output_devices()?.collect::<Vec<Device>>();
    let total = devices.len();
    for (index, device) in devices.into_iter().enumerate() {
        println!(
            "{} ({index}/{total})",
            device.name().unwrap_or_else(|error| format!(
                "failed to obtain device name ({error})"
            ))
        );
    }
    Ok(())
}
pub fn play(op: &Cli) -> Result<()> {
    let mut rng = op.rng();
    let device = op.device()?;
    let started = Instant::now();
    let config = device.default_output_config()?;
    let duration_range = op.duration_range()?;
    let frequency_range = op.frequency_range()?;
    eprintln!(
        "setting system volume"
    );
    set_volume();

    loop {
        let config = config.clone();
        let frequency = frequency_range.sample(&mut rng);
        let millis = duration_range.sample(&mut rng);
        let duration = Duration::from_millis(millis);
        let result = match config.sample_format() {
            cpal::SampleFormat::F32 =>
                counter::<f32>(&device, &config.into(), duration, frequency),
            cpal::SampleFormat::F64 =>
                counter::<f64>(&device, &config.into(), duration, frequency),
            sample_format => panic!("Unsupported sample format '{sample_format}'"),
        };

        match result {
            Ok(_) => {},
            Err(error) => {
                eprintln!(
                    "WARNING: playing frequency {frequency}hz for {millis}ms: {error}"
                );
                eprintln!(
                    "WARNING: resetting system volume"
                );
                set_volume();
            },
        }
        if let Some(duration) = &op.timeout_secs {
            if (Instant::now() - started).as_secs() > *duration {
                break;
            }
        }
    }

    Ok(())
}

pub fn set_volume() {
    cpvc::set_system_volume(100);
}
