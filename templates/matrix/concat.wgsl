
{%- include "structs.wgsl" -%}

[[group(0), binding(0)]]
var<storage, read> var_{{ inputs[0] }}: Array;

[[group(0), binding(1)]]
var<storage, read> var_{{ inputs[1] }}: Array;

[[group(0), binding(2)]]
var<storage, write> var_{{ outputs[0] }}: Array;


[[stage(compute), workgroup_size(256, 1, 1)]]
fn main([[builtin(global_invocation_id)]] global_id: vec3<u32>) {
    let gidx = global_id.x;
    if (gidx < {{ i_lens[0] }}u) {
	    var_{{ outputs[0] }}.data[gidx] = var_{{ inputs[0] }}.data[gidx];
    } else { 
        if (gidx < {{ i_lens[0] + i_lens[1] }}u) {
    	var_{{ outputs[0] }}.data[gidx] = var_{{ inputs[1] }}.data[gidx - {{ i_lens[0] }}u];
        }
    }
}
