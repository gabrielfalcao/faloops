mod errors;
use cpal::traits::{DeviceTrait, StreamTrait};
use cpal::{FromSample, Sample, SizedSample};
pub use errors::{Error, Result};

pub fn counter<T>(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
    duration: std::time::Duration,
    frequency: f64,
) -> Result<()>
where
    T: SizedSample + FromSample<f64>,
{
    let sample_rate = config.sample_rate.0 as f64;
    let channels = config.channels as usize;

    // Produce a sinusoid of maximum amplitude.
    let mut sample_clock = 0f64;
    let mut next_value = move || {
        sample_clock = (sample_clock + 1.0) % sample_rate;
        (sample_clock * frequency * 2.0 * std::f64::consts::PI / sample_rate).sin()
    };

    let err_fn = |err| eprintln!("an error occurred on output_stream: {}", err);

    let output_stream = device.build_output_stream(
        config,
        move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
            write_data(data, channels, &mut next_value)
        },
        err_fn,
        None,
    )?;
    output_stream.play()?;

    std::thread::sleep(duration);

    Ok(())
}

fn write_data<T>(
    output: &mut [T],
    channels: usize,
    next_sample: &mut dyn FnMut() -> f64,
) where
    T: Sample + FromSample<f64>,
{
    for frame in output.chunks_mut(channels) {
        let value: T = T::from_sample(next_sample());
        for sample in frame.iter_mut() {
            *sample = value;
        }
    }
}
