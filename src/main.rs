use error::Error;
use fizzerb_model::{glam::vec2, walls, Material, Microphone, Space, Speaker};
use pixels::{wgpu::TextureFormat, Pixels, PixelsBuilder, SurfaceTexture};
use renderer::{
    context::RenderContext,
    shuffler::ShufflingRenderer,
    space::{draw_space, SpaceStyle},
    Color,
};
use winit::{
    dpi::{LogicalSize, PhysicalSize},
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

mod error;
mod renderer;

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;

struct State {
    space: Space,

    window: Window,
    pixels: Pixels,
    shuffling_renderer: ShufflingRenderer,
    renderer: RenderContext,
    space_style: SpaceStyle,
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
    space.add_walls(walls::make_box(vec2(-1.0, -1.0), vec2(2.0, 2.0), material));
    space.add_speaker(Speaker {
        position: vec2(-0.5, 0.5),
    });
    space.add_microphone(Microphone {
        position: vec2(0.5, -0.5),
    });

    let space_style = SpaceStyle::default();

    let mut state = State {
        space,
        window,
        pixels,
        shuffling_renderer,
        renderer,
        space_style,
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
        space,
        space_style,
        ..
    }: &mut State,
) -> Result<(), Error> {
    renderer.set_source_color(&Color::from_hex_rgb(0xF7F7F8));
    renderer.paint()?;

    renderer.save()?;
    renderer.translate(renderer.width / 2.0, renderer.height / 2.0);
    renderer.scale(200.0, 200.0);
    draw_space(renderer, space, space_style)?;
    renderer.restore()?;

    Ok(())
}
