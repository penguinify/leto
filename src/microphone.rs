use anyhow::{Context, Result};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

pub struct InputAudio {
    host: cpal::Host,
    input_device: cpal::Device,
    input_stream: Option<cpal::Stream>,
}

impl InputAudio {
    pub fn new() -> Result<Self> {
        let host = cpal::default_host();
        let input_device = host
            .default_input_device()
            .context("Failed to get default input device")?;

        Ok(Self {
            host,
            input_device,
            input_stream: None,
        })
    }

    pub fn start_stream(&mut self) -> Result<()> {
        let config = self.input_device.default_input_config()?;
        let sample_format = config.sample_format();
        let config = config.into();

        let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

        let stream = match sample_format {
            cpal::SampleFormat::F32 => self.input_device.build_input_stream(
                &config,
                move |data: &[f32], _: &cpal::InputCallbackInfo| {
                    // Handle input data here
                },
                err_fn,
                None,
            )?,
            cpal::SampleFormat::I16 => self.input_device.build_input_stream(
                &config,
                move |data: &[i16], _: &cpal::InputCallbackInfo| {
                    // Handle input data here
                },
                err_fn,
                None,
            )?,
            cpal::SampleFormat::U16 => self.input_device.build_input_stream(
                &config,
                move |data: &[u16], _: &cpal::InputCallbackInfo| {
                    // Handle input data here
                },
                err_fn,
                None,
            )?,
            _ => return Err(anyhow::anyhow!("Unsupported sample format")),
        };

        stream.play()?;
        self.input_stream = Some(stream);

        Ok(())
    }
}
