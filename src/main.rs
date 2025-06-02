use pollster::FutureExt;

#[derive(Debug)]
struct GameState<const W: usize, const H: usize> {
	cells: [[bool; W]; H]
}

impl<const W: usize, const H: usize> Default for GameState<W, H> {
	fn default() -> Self {
		let cells = [[false; W]; H];
		Self { cells }
	}
}

// yes, this is somewhat inefficient
impl<const W: usize, const H: usize> std::fmt::Display for GameState<W, H> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
		for yi in 0..H {
			for xi in 0..W {
				if self.cells[yi][xi] {
					write!(f, "#")?;
				} else {
					write!(f, ".")?;
				}
			}

			writeln!(f)?;
		}

		Ok(())
	}
}

const GAME_WIDTH: usize = 50;
const GAME_HEIGHT: usize = 20;

fn main() {
	let mut game = GameState::<GAME_WIDTH, GAME_HEIGHT>::default();
	let mut gpu = WgpuStuff::<GAME_WIDTH, GAME_HEIGHT>::new().block_on();

	// initial state
	for y in 6..=13 {
		game.cells[y][24] = true;
	}

	loop {
		println!("{game}");
		game = game.next_state(&mut gpu).block_on();
		std::thread::sleep(std::time::Duration::from_millis(50));
	}
}

struct WgpuStuff<const W: usize, const H: usize> {
	device: wgpu::Device,
	queue: wgpu::Queue,
	input_buf: wgpu::Buffer,
	output_buf: wgpu::Buffer,
	pipeline: wgpu::ComputePipeline,
	bind_group: wgpu::BindGroup,
	width_uniform: wgpu::Buffer,
	height_uniform: wgpu::Buffer
}

impl<const W: usize, const H: usize> WgpuStuff<W, H> {
	async fn new() -> Self {
		let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
			backends: wgpu::Backends::all(),
			flags: wgpu::InstanceFlags::VALIDATION,
			backend_options: wgpu::BackendOptions::default()
		});

		let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions::default()).await.unwrap();

		let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor {
			required_features: wgpu::Features::empty(),
			required_limits: wgpu::Limits::default(),
			memory_hints: wgpu::MemoryHints::Performance,
			label: Some("device"),
			trace: wgpu::Trace::Off
		}).await.unwrap();

		let input_buf = device.create_buffer(&wgpu::BufferDescriptor {
			label: Some("input_buf"),
			size: (H * W * 4) as u64,
			usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::STORAGE,
			mapped_at_creation: false
		});

		let output_buf = device.create_buffer(&wgpu::BufferDescriptor {
			label: Some("output_buf"),
			size: (H * W * 4) as u64,
			usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
			mapped_at_creation: false
		});

		let width_uniform = device.create_buffer(&wgpu::BufferDescriptor {
			label: Some("width_uniform"),
			size: 4,
			usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
			mapped_at_creation: false
		});

		let height_uniform = device.create_buffer(&wgpu::BufferDescriptor {
			label: Some("height_uniform"),
			size: 4,
			usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
			mapped_at_creation: false
		});

		let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));

		let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
			label: Some("bind_group_layout"),
			entries: &[
				// input_buf
				wgpu::BindGroupLayoutEntry {
					binding: 0,
					visibility: wgpu::ShaderStages::COMPUTE,
					ty: wgpu::BindingType::Buffer {
						ty: wgpu::BufferBindingType::Storage {
							read_only: true
						},
						has_dynamic_offset: false,
						min_binding_size: None
					},
					count: None
				},
				// output_buf
				wgpu::BindGroupLayoutEntry {
					binding: 1,
					visibility: wgpu::ShaderStages::COMPUTE,
					ty: wgpu::BindingType::Buffer {
						ty: wgpu::BufferBindingType::Storage {
							read_only: false
						},
						has_dynamic_offset: false,
						min_binding_size: None
					},
					count: None
				},
				// width
				wgpu::BindGroupLayoutEntry {
					binding: 2,
					visibility: wgpu::ShaderStages::COMPUTE,
					ty: wgpu::BindingType::Buffer {
						ty: wgpu::BufferBindingType::Uniform,
						has_dynamic_offset: false,
						min_binding_size: None
					},
					count: None
				},
				// height
				wgpu::BindGroupLayoutEntry {
					binding: 3,
					visibility: wgpu::ShaderStages::COMPUTE,
					ty: wgpu::BindingType::Buffer {
						ty: wgpu::BufferBindingType::Uniform,
						has_dynamic_offset: false,
						min_binding_size: None
					},
					count: None
				}
			]
		});

		let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
			label: Some("gol_pipeline_layout"),
			bind_group_layouts: &[&bind_group_layout],
			push_constant_ranges: &[]
		});

		let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
			label: Some("gol_pipeline"),
			layout: Some(&pipeline_layout),
			module: &shader,
			entry_point: Some("gol"),
			compilation_options: wgpu::PipelineCompilationOptions::default(),
			cache: None
		});

		let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
			label: Some("bind_group"),
			layout: &bind_group_layout,
			entries: &[
				wgpu::BindGroupEntry {
					binding: 0,
					resource: input_buf.as_entire_binding()
				},
				wgpu::BindGroupEntry {
					binding: 1,
					resource: output_buf.as_entire_binding()
				},
				wgpu::BindGroupEntry {
					binding: 2,
					resource: width_uniform.as_entire_binding()
				},
				wgpu::BindGroupEntry {
					binding: 3,
					resource: height_uniform.as_entire_binding()
				}
			]
		});

		Self {
			device,
			queue,
			input_buf,
			output_buf,
			pipeline,
			bind_group,
			width_uniform,
			height_uniform
		}
	}
}

impl<const W: usize, const H: usize> GameState<W, H> {
	fn serialize(&self) -> Vec<u8> {
		self.cells.iter().flat_map(|row| {
			row.iter().map(|cell| match cell {
				false => 0u32,
				true => 1u32
			}).flat_map(u32::to_ne_bytes)
		}).collect()
	}

	fn deserialize(serialized: &[u8]) -> Self {
		let cells = <[[bool; W]; H]>::try_from(serialized.chunks(W * 4).map(|row| {
			<[bool; W]>::try_from(row.chunks(4).map(|cell| {
				let cell: [u8; 4] = cell.try_into().unwrap();
				let n = u32::from_ne_bytes(cell);
				match n {
					0 => false,
					1 => true,
					n => panic!("gpu returned invalid integer {n}")
				}
			}).collect::<Vec<bool>>()).unwrap()
		}).collect::<Vec<[bool; W]>>()).unwrap();

		Self { cells }
	}

	async fn next_state(&self, gpu: &mut WgpuStuff<W, H>) -> Self {
		let mut encoder = gpu.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
			label: Some("encoder")
		});

		gpu.queue.write_buffer(&gpu.input_buf, 0, &self.serialize());
		gpu.queue.write_buffer(&gpu.width_uniform, 0, &(W as u32).to_ne_bytes());
		gpu.queue.write_buffer(&gpu.height_uniform, 0, &(H as u32).to_ne_bytes());

		let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
			label: Some("gol_compute"),
			timestamp_writes: None
		});
		compute_pass.set_pipeline(&gpu.pipeline);
		compute_pass.set_bind_group(0, &gpu.bind_group, &[]);
		compute_pass.dispatch_workgroups((W * H) as u32, 1, 1);
		drop(compute_pass);

		let map_buf = gpu.device.create_buffer(&wgpu::BufferDescriptor {
			label: Some("map_buf"),
			size: (H * W * 4) as u64,
			usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
			mapped_at_creation: false
		});

		encoder.copy_buffer_to_buffer(
			&gpu.output_buf, 0,
			&map_buf, 0,
			(H * W * 4) as u64
		);

		gpu.queue.submit(std::iter::once(encoder.finish()));

		map_buf.map_async(wgpu::MapMode::Read, .., |r| r.unwrap());
		gpu.device.poll(wgpu::PollType::Wait).unwrap();

		let serialized_data = map_buf.get_mapped_range(..);
		Self::deserialize(&serialized_data)
	}
}
