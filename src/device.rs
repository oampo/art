use portaudio;
use portaudio::pa;
use portaudio::device;
use portaudio::stream;

use rates::AUDIO_RATE;
use types::ArtResult;
use errors::PortAudioError;

pub type Stream<'a> = portaudio::stream::Stream<'a, f32, f32>;
pub type Callback<'a> = portaudio::stream::StreamCallback<'a, f32, f32>;

pub struct Device<'a> {
    input_device: int,
    output_device: int,
    input_channels: uint,
    output_channels: uint,
    stream: Option<Stream<'a>>
}

impl<'a> Device <'a> {
    pub fn new(input_device: int, output_device: int, input_channels: uint,
               output_channels: uint) -> Device<'a> {
        Device {
            input_device: input_device,
            output_device: output_device,
            input_channels: input_channels,
            output_channels: output_channels,
            stream: None
        }
    }

    pub fn init() -> ArtResult<()> {
        info!("Initializing PortAudio");
        portaudio::initialize().map_err(|err| PortAudioError::new(err))
    }

    pub fn uninit() -> ArtResult<()> {
        info!("Terminating PortAudio");
        portaudio::terminate().map_err(|err| PortAudioError::new(err))
    }


    pub fn list() -> ArtResult<()> {
        let count = try!(
            portaudio::device::get_count().map_err(
                |err| PortAudioError::new(err)
            )
        );

        info!("{} devices available:", count);

        for i in range(0, count) {
            let device_info = try!(
                portaudio::device::get_info(i).ok_or(
                    PortAudioError::new(
                        portaudio::pa::PaError::InvalidDevice
                    )
                )
            );

            info!("{}: {}", i, device_info.name);
        }
        Ok(())
    }

    pub fn open(&mut self, callback: Callback<'a>) -> ArtResult<()> {
        self.stream = Some(
            try!(
                self._open(callback).map_err(|err| {
                    PortAudioError::new(err)
                })
            )
        );
        Ok(())
    }

    pub fn start(&mut self) -> ArtResult<()> {
        let stream = try!(
            self.stream.as_mut().ok_or(
                PortAudioError::new(portaudio::pa::PaError::BadStreamPtr)
            )
        );
        try!(
            stream.start().map_err(|err| {
                PortAudioError::new(err)
            })
        );
        Ok(())
    }

    fn _open(&self, callback: Callback<'a>)
            -> Result<Stream<'a>, portaudio::pa::PaError> {
        // Currently pa-rs requires both input and output
        let input_device_id = match self.input_device {
            id if id >= 0 =>  id as uint,
            _ => try!(portaudio::device::get_default_input_index())
        };

        let input_device_info = try!(
            portaudio::device::get_info(input_device_id as uint).ok_or(
                portaudio::pa::PaError::InvalidDevice
            )
        );

        let input_parameters = portaudio::stream::StreamParameters {
            device: input_device_id as uint,
            channel_count: self.input_channels,
            suggested_latency: input_device_info.default_low_input_latency
        };

        let mut output_device_id = self.output_device;
        let output_device_id = match self.output_device {
            id if id >= 0 => id as uint,
            _ => try!(portaudio::device::get_default_output_index())
        };

        let output_device_info = try!(
            portaudio::device::get_info(output_device_id as uint).ok_or(
                portaudio::pa::PaError::InvalidDevice
            )
        );

        info!("Creating audio stream: input_device = {}, output_device = {}, \
               input_channels = {}, output_channels = {}",
              input_device_info.name, output_device_info.name,
              self.input_channels, self.output_channels);


        let output_parameters = portaudio::stream::StreamParameters {
            device: output_device_id as uint,
            channel_count: self.output_channels,
            suggested_latency: output_device_info.default_low_input_latency
        };

        try!(portaudio::stream::is_format_supported(input_parameters,
                                                    output_parameters,
                                                    AUDIO_RATE as f64));

        let stream = try!(
            portaudio::stream::Stream::open(
                input_parameters,
                output_parameters,
                AUDIO_RATE as f64,
                portaudio::stream::FRAMES_PER_BUFFER_UNSPECIFIED,
                portaudio::stream::StreamFlags::empty(),
                Some(callback)
            )
        );
        Ok(stream)
    }
}

