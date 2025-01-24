use makepad_widgets::*;

live_design! {
    use link::widgets::*;
     
    use makepad_draw::shader::std::*;
    use link::gen_components::*;

    GShaderExample = <ScrollYView>{
        height: 200.0,
        width: Fill,
        spacing: 10.0,
        flow: Down,
        <GLabel>{
            text: "GShader"
        }
        <GShader>{
            height: 200.0,
            width: 200.0,
            draw_shader:{
                fn pixel(self) -> vec4 {
                                
                    let uv = self.pos - 0.5;
                    uv.x *= self.rect_size.x / self.rect_size.y;

                    let radius = length(uv);
                    let wave = sin(radius * 10.0 - self.time * 2.0);
                    let intensity = wave * 0.5 + 0.5;
                    let col = vec3(intensity);

                    return vec4(col, 1.0);
                }
            }
        }
        <GShader>{
            height: 200.0,
            width: 200.0,
            draw_shader:{
                fn pixel(self) -> vec4 {
                                
                    let uv = self.pos - 0.5;
                    
                    let time = self.time * 0.5;

                    let col = vec3(0.0);
                    let noise = fract(sin(dot(uv, vec2(12.9898, 78.233))) * 43758.5453);
                    
                    col += 0.1 * vec3(noise, noise* 0.5, noise * 0.2);

                    let r = length(uv);
                    let a = atan(uv.y, uv.x);
                    let f = 0.5 + 0.5 * sin(6.0 * (a + time) + r * 10.0);
                    col += vec3(f* 0.3, f* 0.2, f* 0.5);
                    let i = 0;
                    for _i in 0..10 {
                        // let x = sin(float(i)) * 0.1 + time;
                        // let star_posi = vec2(fract( * 0.1 + time), fract(sin(float(i) * 23421.6313) * 0.1 + time));
                        let star_pos = vec2(fract(sin(float(i) * 43758.5453) * 0.1 + time), fract(sin(float(i) * 23421.6313) * 0.1 + time));
                        star_pos = star_pos * 2.0 - 1.0;
                        star_pos.x *= uv.x / uv.y;
                        let d = uv - star_pos;
                        let star = 1.0 / length(d) * 0.05;
                        col += vec3(star);
                        i = i + 1;
                    }
                    let flicker = fract(sin(dot(uv.xy + time * 20.0, vec2(12.9898,78.233))) * 43758.5453);
                    col += 0.1 * vec3(flicker, flicker * 0.5, flicker * 0.2);

                    let plasma = sin(uv.x * 10.0 + time * 2.0) * cos(uv.y * 10.0 + time * 2.0); 
                    col += vec3(0.2, 0.1, 0.3) * plasma;
                    
                    let morph = sin(time + r * 10.0) * 0.5 + 0.5;
                    col *= vec3(morph, morph * 0.8, morph * 1.2);

                    return vec4(col, 1.0);
                }
            }
        }
    }
}