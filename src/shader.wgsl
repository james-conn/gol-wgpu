@group(0) @binding(0) var<storage, read> input_buf: array<u32>;
@group(0) @binding(1) var<storage, read_write> output_buf: array<u32>;

@compute
@workgroup_size(1)
fn gol(@builtin(global_invocation_id) global_id: vec3<u32>) {
	output_buf[global_id.x] = select(1u, 0u, input_buf[global_id.x] == 0u);
}
