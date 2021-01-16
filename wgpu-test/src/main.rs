use bevy::prelude::*;
use bevy::winit::WinitWindows;
use futures::executor::block_on;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::*;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

impl Vertex {
    fn desc() -> VertexBufferDescriptor<'static> {
        VertexBufferDescriptor {
            stride: std::mem::size_of::<Vertex>() as BufferAddress,
            step_mode: InputStepMode::Vertex,
            attributes: &[
                VertexAttributeDescriptor {
                    offset: 0,
                    shader_location: 0,
                    format: VertexFormat::Float3,
                },
                VertexAttributeDescriptor {
                    offset: std::mem::size_of::<[f32; 3]>() as BufferAddress,
                    shader_location: 1,
                    format: VertexFormat::Float3,
                },
            ],
        }
    }
}

const VERTICES: &[Vertex] = &[
    Vertex {
        position: [0.0, 0.5, 0.0],
        color: [1.0, 0.0, 0.0],
    },
    Vertex {
        position: [-0.5, -0.5, 0.0],
        color: [0.0, 1.0, 0.0],
    },
    Vertex {
        position: [0.5, -0.5, 0.0],
        color: [0.0, 0.0, 1.0],
    },
];

struct VertexBuffer(Buffer);

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(render.system().chain(render_error.system()))
        .run();
}

fn setup(commands: &mut Commands, windows: Res<WinitWindows>) {
    let windows = &windows.windows;
    assert_eq!(windows.len(), 1);
    for window in windows.values() {
        setup_window(commands, window);
    }
}

fn setup_window(commands: &mut Commands, window: &winit::window::Window) {
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

    let vs_module = device.create_shader_module(include_spirv!("shader.vert.spv"));
    let fs_module = device.create_shader_module(include_spirv!("shader.frag.spv"));

    let render_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
        label: Some("Render Pipeline Layout"),
        bind_group_layouts: &[],
        push_constant_ranges: &[],
    });

    let render_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
        label: Some("Render Pipeline"),
        layout: Some(&render_pipeline_layout),
        vertex_stage: ProgrammableStageDescriptor {
            module: &vs_module,
            entry_point: "main",
        },
        fragment_stage: Some(ProgrammableStageDescriptor {
            module: &fs_module,
            entry_point: "main",
        }),
        rasterization_state: Some(RasterizationStateDescriptor {
            front_face: FrontFace::Ccw,
            cull_mode: CullMode::Back,
            depth_bias: 0,
            depth_bias_slope_scale: 0.0,
            depth_bias_clamp: 0.0,
            clamp_depth: false,
        }),
        color_states: &[ColorStateDescriptor {
            format: sc_desc.format,
            color_blend: BlendDescriptor::REPLACE,
            alpha_blend: BlendDescriptor::REPLACE,
            write_mask: ColorWrite::ALL,
        }],
        primitive_topology: PrimitiveTopology::TriangleList,
        depth_stencil_state: None,
        vertex_state: VertexStateDescriptor {
            index_format: IndexFormat::Uint16,
            vertex_buffers: &[Vertex::desc()],
        },
        sample_count: 1,
        sample_mask: !0,
        alpha_to_coverage_enabled: false,
    });

    let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
        label: Some("Vertex Buffer"),
        contents: bytemuck::cast_slice(VERTICES),
        usage: BufferUsage::VERTEX,
    });

    commands
        .insert_resource(surface)
        .insert_resource(device)
        .insert_resource(queue)
        .insert_resource(sc_desc)
        .insert_resource(swap_chain)
        .insert_resource(size)
        .insert_resource(render_pipeline)
        .insert_resource(VertexBuffer(vertex_buffer));
}

fn render(
    mut swap_chain: ResMut<SwapChain>,
    device: Res<Device>,
    render_pipeline: Res<RenderPipeline>,
    vertex_buffer: Res<VertexBuffer>,
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
        render_pass.set_vertex_buffer(0, vertex_buffer.0.slice(..));
        render_pass.draw(0..3, 0..1);
    }

    queue.submit(std::iter::once(encoder.finish()));

    Ok(())
}

fn render_error(In(result): In<Result<(), SwapChainError>>) {
    if let Err(err) = result {
        println!("{}", err);
    }
}
