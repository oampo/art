use portaudio;

use types::ArtResult;
use constants::Constants;

pub type Stream<'a> = portaudio::stream::Stream<'a, f32, f32>;
pub type Callback<'a> = portaudio::stream::StreamCallback<'a, f32, f32>;

#[derive(Copy)]
pub enum DeviceId {
    Id(u32),
    Default
}

impl DeviceId {
    pub fn from_option(id: Option<u32>) -> DeviceId {
        id.map_or(DeviceId::Default, |id| DeviceId::Id(id))
    }
}

pub struct Device<'a> {
    input_device: DeviceId,
    output_device: DeviceId,
    input_channels: u32,
    output_channels: u32,
    stream: Option<Stream<'a>>
}

impl<'a> Device <'a> {
    pub fn new(input_device: DeviceId, output_device: DeviceId,
               input_channels: u32, output_channels: u32) -> Device<'a> {
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
        try!(portaudio::initialize());
        Ok(())
    }

    pub fn uninit() -> ArtResult<()> {
        info!("Terminating PortAudio");
        try!(portaudio::terminate());
        Ok(())
    }


    pub fn list() -> ArtResult<()> {
        let count = try!(
            portaudio::device::get_count()
        );

        println!("{} devices available:", count);
        println!("");

        for i in range(0, count) {
            if let Some(device_info) = portaudio::device::get_info(i) {
                println!("{}: {} [I: {}, O: {}]", i, device_info.name,
                         device_info.max_input_channels,
                         device_info.max_output_channels);
            }
        }
        Ok(())
    }

    pub fn is_open(&self) -> bool {
        self.stream.is_some()
    }

    pub fn start(&mut self) -> ArtResult<()> {
        let stream = try!(
            self.stream.as_mut().ok_or(portaudio::pa::PaError::BadStreamPtr)
        );
        try!(stream.start());
        Ok(())
    }

    pub fn open(&mut self, callback: &'a mut Callback<'a>,
                constants: Constants) -> ArtResult<()> {
        // Currently pa-rs requires both input and output
        let input_device_id = match self.input_device {
            DeviceId::Id(id) =>  id,
            DeviceId::Default => {
                try!(portaudio::device::get_default_input_index())
            }
        };

        let input_device_info = try!(
            portaudio::device::get_info(input_device_id).ok_or(
                portaudio::pa::PaError::InvalidDevice
            )
        );

        let input_parameters = portaudio::stream::StreamParameters {
            device: input_device_id,
            channel_count: self.input_channels,
            suggested_latency: input_device_info.default_low_input_latency
        };

        let output_device_id = match self.output_device {
            DeviceId::Id(id) => id,
            DeviceId::Default => {
                try!(portaudio::device::get_default_output_index())
            }
        };

        let output_device_info = try!(
            portaudio::device::get_info(output_device_id).ok_or(
                portaudio::pa::PaError::InvalidDevice
            )
        );

        info!("Creating audio stream: input_device = {}, output_device = {}, \
               input_channels = {}, output_channels = {}",
              input_device_info.name, output_device_info.name,
              self.input_channels, self.output_channels);


        let output_parameters = portaudio::stream::StreamParameters {
            device: output_device_id,
            channel_count: self.output_channels,
            suggested_latency: output_device_info.default_low_input_latency
        };

        try!(
            portaudio::stream::is_format_supported(
                input_parameters, output_parameters,
                constants.audio_rate as f64
            )
        );

        self.stream = Some(
            try!(
                portaudio::stream::Stream::open(
                    input_parameters,
                    output_parameters,
                    constants.audio_rate as f64,
                    constants.block_size as u64,
                    portaudio::stream::StreamFlags::empty(),
                    Some(callback)
                )
            )
        );
        Ok(())
    }
}

