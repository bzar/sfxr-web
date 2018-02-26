extern crate sfxr;
extern crate hound;

use sfxr::{Sample, Generator, WaveType};

const BUFFER_SIZE: usize = 44_100;

struct State {
    generator: Generator,
    buffer: [f32; BUFFER_SIZE],
    wav_buffer: Vec<u8>
}

static mut STATE: Option<State> = None;

#[no_mangle]
pub fn init() {
    let generator = Generator::new(Sample::new());
    unsafe {
        STATE = Some(State {
            generator,
            buffer: [0.0; BUFFER_SIZE],
            wav_buffer: Vec::new()

        });
    }
}

#[no_mangle]
pub fn render() {
    unsafe {
        let state = STATE.as_mut().unwrap();
        state.generator.reset();
        state.generator.generate(&mut state.buffer);
        state.generator.reset();
    }
}

#[no_mangle]
pub fn buffer_ptr() -> *const f32 {
    unsafe {
        let state = STATE.as_ref().unwrap();
        state.buffer.as_ptr()
    }
}

#[no_mangle]
pub fn buffer_len() -> usize {
    unsafe {
        let state = STATE.as_ref().unwrap();
        state.buffer.len()
    }
}

#[no_mangle]
pub fn render_wav() {
    use hound;
    use std::io::Cursor;
    let state = unsafe {
        STATE.as_mut().unwrap()
    };

    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float,
    };

    state.generator.generate(&mut state.buffer);
    state.generator.reset();
    state.wav_buffer.clear();
    let writer = Cursor::new(&mut state.wav_buffer);
    let mut wav_writer = hound::WavWriter::new(writer, spec).unwrap();
    state.buffer.iter().for_each(|&s| wav_writer.write_sample(s).unwrap());
}

#[no_mangle]
pub fn wav_buffer_ptr() -> *const u8 {
    unsafe {
        let state = STATE.as_ref().unwrap();
        state.wav_buffer.as_ptr()
    }
}

#[no_mangle]
pub fn wav_buffer_len() -> usize {
    unsafe {
        let state = STATE.as_ref().unwrap();
        state.wav_buffer.len()
    }
}

#[no_mangle]
pub fn wave_type() -> usize {
    unsafe {
        let state = STATE.as_ref().unwrap();
        state.generator.sample.wave_type as usize
    }
}
#[no_mangle]
pub fn set_wave_type(value: usize) {
    unsafe {
        let state = STATE.as_mut().unwrap();
        let wave_types = [WaveType::Square, WaveType::Sine, WaveType::Triangle, WaveType::Noise];
        let value = wave_types[value];
        state.generator.sample.wave_type = value;
    }
}

macro_rules! getter_setter {
    ($value_type:ty, $field:ident, $setter:ident) => {
        #[no_mangle]
        pub fn $field() -> $value_type {
            unsafe {
                let state = STATE.as_ref().unwrap();
                state.generator.sample.$field
            }
        }
        #[no_mangle]
        pub fn $setter(value: $value_type) {
            unsafe {
                let state = STATE.as_mut().unwrap();
                state.generator.sample.$field = value;
            }
        }
    };
}
            
getter_setter!(f64, base_freq, set_base_freq);
getter_setter!(f64, freq_limit, set_freq_limit);
getter_setter!(f64, freq_ramp, set_freq_ramp);
getter_setter!(f64, freq_dramp, set_freq_dramp);
getter_setter!(f32, duty, set_duty);
getter_setter!(f32, duty_ramp, set_duty_ramp);
getter_setter!(f64, vib_strength, set_vib_strength);
getter_setter!(f64, vib_speed, set_vib_speed);
getter_setter!(f32, vib_delay, set_vib_delay);
getter_setter!(f32, env_attack, set_env_attack);
getter_setter!(f32, env_sustain, set_env_sustain);
getter_setter!(f32, env_decay, set_env_decay);
getter_setter!(f32, env_punch, set_env_punch);
getter_setter!(f32, lpf_resonance, set_lpf_resonance);
getter_setter!(f32, lpf_freq, set_lpf_freq);
getter_setter!(f32, lpf_ramp, set_lpf_ramp);
getter_setter!(f32, hpf_freq, set_hpf_freq);
getter_setter!(f32, hpf_ramp, set_hpf_ramp);
getter_setter!(f32, pha_offset, set_pha_offset);
getter_setter!(f32, pha_ramp, set_pha_ramp);
getter_setter!(f32, repeat_speed, set_repeat_speed);
getter_setter!(f32, arp_speed, set_arp_speed);
getter_setter!(f64, arp_mod, set_arp_mod);
