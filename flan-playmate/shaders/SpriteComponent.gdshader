shader_type canvas_item;

uniform float interpolation : hint_range(0, 1);
uniform bool  mode = false; // Maybe there is better way

void vertex() {
	// Called for every vertex the material is visible on.
}

void fragment() {
    if (mode) {
        if (COLOR.a > 0.0) {
            COLOR.a = 1. - interpolation;
        }
    } else {
        if (COLOR.a > 0.0) {
            COLOR = mix(COLOR, vec4(1), interpolation);
        }
    }
}

//void light() {
//	// Called for every pixel for every light affecting the CanvasItem.
//	// Uncomment to replace the default light processing function with this one.
//}
