#![feature(const_fn_floating_point_arithmetic)]
use bevy::prelude::*;
use bevy::winit::WinitWindows;
use futures::executor::block_on;

use wgpu::*;

pub mod types;
use types::*;
mod create_buffers;
mod create_pipeline;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(render.system().chain(render_error.system()))
        .run();
}

const TYPE_COLORS: &[RgbaColor] = &[
    // Dark blue
    RgbaColor::new_rgb_u8(55, 80, 120),
];

fn setup(commands: &mut Commands, windows: Res<WinitWindows>) {
    let windows = &windows.windows;
    assert_eq!(windows.len(), 1);
    for window in windows.values() {
        setup_window(commands, window, TYPE_COLORS);
    }
}

fn setup_window(commands: &mut Commands, window: &winit::window::Window, type_colors: &[RgbaColor]) {
    let size = window.inner_size();
    let instance = Instance::new(BackendBit::PRIMARY);
    let surface = unsafe { instance.create_surface(window) };
    let adapter = block_on(instance.request_adapter(&RequestAdapterOptions {
        power_preference: PowerPreference::HighPerformance,
        compatible_surface: Some(&surface),
    }))
    .unwrap();

    let (device, queue) = block_on(adapter.request_device(
        &DeviceDescriptor {
            features: Features::empty(),
            limits: Limits::default(),
            shader_validation: true,
        },
        None,
    ))
    .unwrap();

    let sc_desc = SwapChainDescriptor {
        usage: TextureUsage::OUTPUT_ATTACHMENT,
        format: TextureFormat::Bgra8UnormSrgb,
        width: size.width,
        height: size.height,
        present_mode: PresentMode::Fifo,
    };
    let swap_chain = device.create_swap_chain(&surface, &sc_desc);

    let bind_group_layout = create_buffers::create_buffers(commands, &device, &queue, type_colors);

    create_pipeline::create_pipeline(commands, &device, &sc_desc, &bind_group_layout);

    commands
        .insert_resource(surface)
        .insert_resource(device)
        .insert_resource(queue)
        .insert_resource(sc_desc)
        .insert_resource(swap_chain)
        .insert_resource(size);
}

fn render(
    mut swap_chain: ResMut<SwapChain>,
    device: Res<Device>,
    render_pipeline: Res<RenderPipeline>,
    bind_group: Res<TextureBindGroup>,
    vertex_buffer: Res<VertexBuffer>,
    vertex_buffer_length: Res<VertexBufferLength>,
    queue: Res<Queue>,
) -> Result<(), SwapChainError> {
    let frame = swap_chain.get_current_frame()?.output;
    let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor {
        label: Some("Render Encoder"),
    });

    {
        let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
            color_attachments: &[RenderPassColorAttachmentDescriptor {
                attachment: &frame.view,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Clear(Color {
                        r: 0.1,
                        g: 0.2,
                        b: 0.3,
                        a: 1.0,
                    }),
                    store: true,
                },
            }],
            depth_stencil_attachment: None,
        });
        render_pass.set_pipeline(&render_pipeline);
        render_pass.set_bind_group(0, &bind_group.0, &[]);
        render_pass.set_vertex_buffer(0, vertex_buffer.0.slice(..));
        render_pass.draw(0..(vertex_buffer_length.0 as u32), 0..1);
    }

    queue.submit(std::iter::once(encoder.finish()));

    Ok(())
}

fn render_error(In(result): In<Result<(), SwapChainError>>) {
    if let Err(err) = result {
        println!("{}", err);
    }
}
