// cSpell:words Sinc
use rubato::{
    Resampler, SincFixedIn, SincInterpolationParameters, SincInterpolationType, WindowFunction,
};
use symphonia::core::audio::SampleBuffer;
use symphonia::core::codecs::DecoderOptions;
use symphonia::core::errors::Error;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;
use symphonia::core::units::Duration;
use symphonia::default::{get_codecs, get_probe};

pub fn convert_to_mono_f32_16khz(input_path: &str) -> Result<Vec<f32>, Box<dyn std::error::Error>> {
    // Create a media source.
    let src = std::fs::File::open(input_path)?;
    let mss = MediaSourceStream::new(Box::new(src), Default::default());

    // Create a hint to help the format prober guess the format.
    let mut hint = Hint::new();
    if let Some(ext) = input_path.split('.').last() {
        hint.with_extension(ext);
    }

    let meta_opts: MetadataOptions = Default::default();
    let fmt_opts: FormatOptions = Default::default();

    // Probe the media source.
    let probed = get_probe().format(&hint, mss, &fmt_opts, &meta_opts)?;
    let mut format = probed.format;

    // Find the first audio track with a known codec format.
    let track = format
        .tracks()
        .iter()
        .find(|t| t.codec_params.codec != symphonia::core::codecs::CODEC_TYPE_NULL)
        .ok_or("No audio track found")?;

    let sample_rate = track
        .codec_params
        .sample_rate
        .ok_or("Sample rate unknown")?;
    let channels = track.codec_params.channels.ok_or("Channel info unknown")?;

    // Create a decoder for the track.
    let mut decoder = get_codecs().make(&track.codec_params, &DecoderOptions::default())?;

    // Store the decoded audio samples as f32.
    let mut samples_f32 = Vec::new();

    // The decode loop.
    loop {
        // Get the next packet from the media format.
        let packet = match format.next_packet() {
            Ok(packet) => packet,
            Err(Error::IoError(err)) => {
                if err.kind() == std::io::ErrorKind::UnexpectedEof {
                    break;
                } else {
                    return Err(Box::new(err));
                }
            }
            Err(err) => {
                return Err(Box::new(err));
            }
        };

        // Decode the packet into audio samples.
        let audio_buf = decoder.decode(&packet)?;

        // Copy the samples to a local buffer.
        let mut sample_buf =
            SampleBuffer::<f32>::new(audio_buf.capacity() as Duration, *audio_buf.spec());
        sample_buf.copy_interleaved_ref(audio_buf);

        // Get the interleaved samples.
        let buffer = sample_buf.samples();

        // Convert to mono and append to the main buffer.
        if channels.count() == 1 {
            samples_f32.extend_from_slice(buffer);
        } else {
            for frame in buffer.chunks(channels.count()) {
                let sum: f32 = frame.iter().sum();
                samples_f32.push(sum / channels.count() as f32);
            }
        }
    }

    // Resample if necessary.
    let samples_resampled = if sample_rate != 16000 {
        let params = SincInterpolationParameters {
            sinc_len: 256,
            f_cutoff: 0.95,
            interpolation: SincInterpolationType::Linear,
            oversampling_factor: 256,
            window: WindowFunction::BlackmanHarris2,
        };
        let mut resampler = SincFixedIn::<f32>::new(
            16000.0 / sample_rate as f64,
            2.0,
            params,
            samples_f32.len(),
            1,
        )?;
        let chunks = vec![samples_f32];
        let resampled_chunks = resampler.process(&chunks, None)?;
        resampled_chunks[0].clone()
    } else {
        samples_f32
    };

    Ok(samples_resampled)
}
