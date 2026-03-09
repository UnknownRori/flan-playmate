shader_type canvas_item;

uniform sampler2D atlas_texture : filter_nearest;
varying vec2 instance_uv;

void vertex() {
    vec2 uv_offset = INSTANCE_CUSTOM.xy;
    vec2 uv_scale  = INSTANCE_CUSTOM.zw;

    vec2 flipped_uv = vec2(UV.x, 1.0 - UV.y);
    instance_uv = flipped_uv * uv_scale + uv_offset;
}

void fragment() {
    COLOR = texture(atlas_texture, instance_uv);
}

//void light() {
//	// Called for every pixel for every light affecting the CanvasItem.
//	// Uncomment to replace the default light processing function with this one.
//}
