use portaudio;

use types::ArtResult;
use constants::Constants;
use options::Options;

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

pub struct Device;

impl Device {
    pub fn init() -> ArtResult<()> {
        debug!("Initializing PortAudio");
        try!(portaudio::initialize());
        Ok(())
    }

    pub fn uninit() -> ArtResult<()> {
        debug!("Terminating PortAudio");
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

    pub fn open<'a>(options: &Options, callback: &'a mut Callback<'a>,
                    constants: Constants) -> ArtResult<Stream<'a>> {
        // Currently pa-rs requires both input and output
        let input_device_id = match options.input_device {
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
            channel_count: options.input_channels,
            suggested_latency: input_device_info.default_low_input_latency,
            data: 0f32
        };

        let output_device_id = match options.output_device {
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

        debug!("Creating audio stream: input_device = {}, output_device = {}, \
                input_channels = {}, output_channels = {}",
                input_device_info.name, output_device_info.name,
                options.input_channels, options.output_channels);


        let output_parameters = portaudio::stream::StreamParameters {
            device: output_device_id,
            channel_count: options.output_channels,
            suggested_latency: output_device_info.default_low_input_latency,
            data: 0f32
        };

        try!(
            portaudio::stream::is_format_supported(
                input_parameters, output_parameters,
                constants.audio_rate as f64
            )
        );

        Ok(
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
        )
    }
}

