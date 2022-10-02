use error::Error;
use fizzerb_impulse::{Compressor, ImpulseRenderer};
use fizzerb_model::{
    glam::vec2, walls, Material, Microphone, MicrophoneIndex, Space, Speaker, SpeakerIndex,
};
use fizzerb_tracer::{Recording, Tracer, TracerConfig, SPEED_OF_SOUND_IN_AIR};
use glam::Vec2;
use hound::{SampleFormat, WavSpec, WavWriter};
use pixels::{wgpu::TextureFormat, Pixels, PixelsBuilder, SurfaceTexture};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use renderer::{
    context::RenderContext,
    recording::{RecordingRenderer, RecordingStyle},
    shuffler::ShufflingRenderer,
    space::{SpaceRenderer, SpaceStyle},
    transform::Transform,
    Color,
};
use winit::{
    dpi::{LogicalSize, PhysicalSize},
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

mod error;
mod renderer;

const WIDTH: u32 = 600;
const HEIGHT: u32 = 600;

const SAMPLE_RATE: u32 = 48000;

struct State {
    space: Space,
    trace_recording: Option<Recording>,
    speaker: SpeakerIndex,
    microphone: MicrophoneIndex,
    impulse_renderer: ImpulseRenderer,
    rendered_impulse: Vec<f32>,

    window: Window,
    pixels: Pixels,
    shuffling_renderer: ShufflingRenderer,
    renderer: RenderContext,
    space_style: SpaceStyle,
    recording_style: RecordingStyle,
}

fn main() -> Result<(), Error> {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("fizzerb")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        PixelsBuilder::new(WIDTH, HEIGHT, surface_texture)
            .surface_texture_format(TextureFormat::Bgra8UnormSrgb)
            .build()?
    };
    let shuffling_renderer = ShufflingRenderer::new(&pixels, WIDTH, HEIGHT);

    let renderer =
        unsafe { RenderContext::new(&mut pixels.get_frame_mut()[0] as *mut u8, WIDTH, HEIGHT)? };

    let mut space = Space::new();
    let material = space.add_material(Material {
        diffuse: 1.0,
        roughness: 0.0,
    });
    let room_size = vec2(10.0, 10.0);
    space.add_walls(walls::make_box(-room_size / 2.0, room_size, material));
    let speaker = space.add_speaker(Speaker {
        position: room_size / 3.0,
        power: 10.0,
    });
    let microphone = space.add_microphone(Microphone {
        position: -room_size / 3.0,
    });

    let impulse_renderer = ImpulseRenderer::new(SAMPLE_RATE as f32);

    let space_style = SpaceStyle::default();
    let recording_style = RecordingStyle::default();

    let mut state = State {
        space,
        impulse_renderer,
        trace_recording: None,
        speaker,
        microphone,
        rendered_impulse: vec![],

        window,
        pixels,
        shuffling_renderer,
        renderer,
        space_style,
        recording_style,
    };

    event_loop.run(move |event, _, control_flow| {
        if let Err(error) = event_loop_inner(event, control_flow, &mut state) {
            log::error!("{error}");
            *control_flow = ControlFlow::Exit;
        }
    })
}

fn event_loop_inner(
    event: Event<()>,
    control_flow: &mut ControlFlow,
    state: &mut State,
) -> Result<(), Error> {
    match event {
        Event::RedrawRequested(_) => {
            draw(state)?;

            state
                .pixels
                .render_with(|encoder, render_target, context| {
                    let texture = state.shuffling_renderer.get_texture_view();
                    context.scaling_renderer.render(encoder, texture);
                    state.shuffling_renderer.render(
                        encoder,
                        render_target,
                        context.scaling_renderer.clip_rect(),
                    );
                    Ok(())
                })?;
        }
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::Resized(PhysicalSize { width, height }) => {
                state
                    .shuffling_renderer
                    .resize(&state.pixels, width, height);
                state.pixels.resize_surface(width, height);
                state.pixels.resize_buffer(width, height);
                unsafe {
                    state.renderer.resize(
                        &mut state.pixels.get_frame_mut()[0] as *mut u8,
                        width,
                        height,
                    )?
                };
            }
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(keycode),
                        ..
                    },
                ..
            } => match keycode {
                VirtualKeyCode::Q => *control_flow = ControlFlow::Exit,
                VirtualKeyCode::Space => state.retrace(),
                _ => (),
            },
            WindowEvent::CloseRequested => {
                *control_flow = ControlFlow::Exit;
            }
            _ => (),
        },
        _ => (),
    }
    state.window.request_redraw();

    Ok(())
}

fn draw(
    State {
        renderer,
        trace_recording,
        space,
        space_style,
        recording_style,
        ..
    }: &mut State,
) -> Result<(), Error> {
    renderer.set_source_color(&Color::from_hex_rgb(0xF7F7F8));
    renderer.paint()?;

    // let waveform_height = renderer.height * 0.75;
    // renderer.new_path();
    // for (i, &sample) in rendered_impulse.iter().step_by(10).enumerate() {
    //     let x = i as f64 / rendered_impulse.len() as f64 * renderer.width;
    //     let y = renderer.height / 2.0 - sample as f64 * waveform_height;
    //     if i == 0 {
    //         renderer.move_to(x, y);
    //     } else {
    //         renderer.line_to(x, y);
    //     }
    // }
    // renderer.set_line_width(1.0);
    // renderer.set_source_color(&Color::from_hex_rgb(0xCDD3DA));
    // renderer.stroke()?;

    let transform = Transform {
        pan: vec2(0.0, 0.0),
        zoom: 50.0,
    };

    if let Some(recording) = trace_recording {
        RecordingRenderer {
            recording,
            transform: &transform,
            style: recording_style,
        }
        .draw(renderer)?;
    }

    SpaceRenderer {
        space,
        style: space_style,
        transform: &transform,
    }
    .draw(renderer)?;

    Ok(())
}

impl State {
    fn retrace(&mut self) {
        let recordings: Vec<_> = (0..1024)
            .into_par_iter()
            .map(|_| {
                let mut tracer = Tracer::new(
                    &self.space,
                    &TracerConfig {
                        speed_of_sound: SPEED_OF_SOUND_IN_AIR,
                        max_bounces: 1024,
                        record_rays: true,
                    },
                );
                let angle = fastrand::f32() * 2.0 * std::f32::consts::PI;
                let start_ray = Vec2::from_angle(angle);
                tracer.perform_trace(self.microphone, self.speaker, start_ray)
            })
            .collect();
        for recording in recordings {
            self.impulse_renderer.add_responses(&recording.responses);
            self.trace_recording = Some(recording);
        }

        self.rendered_impulse = self.impulse_renderer.render(
            1.0,
            Compressor {
                sample_rate: SAMPLE_RATE as f32,
                threshold: 0.8,
                release: 2.0,
            },
        );
        Self::save_wav(&self.rendered_impulse);
    }

    fn save_wav(impulse_response: &[f32]) {
        if let Err(error) = Self::save_wav_inner(impulse_response) {
            log::error!("error saving WAV: {error}");
        }
    }

    fn save_wav_inner(impulse_response: &[f32]) -> Result<(), Error> {
        let spec = WavSpec {
            channels: 1,
            sample_rate: SAMPLE_RATE,
            bits_per_sample: 32,
            sample_format: SampleFormat::Float,
        };
        let mut writer = WavWriter::create("impulse_response.wav", spec)?;
        for &sample in impulse_response {
            // let sample = (sample * i16::MAX as f32) as i16;
            writer.write_sample(sample)?;
        }
        writer.finalize()?;
        Ok(())
    }
}
