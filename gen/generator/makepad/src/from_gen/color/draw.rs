use gen_parser::common::{hex_to_vec4, Hex, LinearGradient, RadialGradient};
use proc_macro2::TokenStream;
use quote::quote;
use syn::parse_str;

/// convert hex to pixel
#[allow(dead_code)]
pub fn hex_to_pixel(value: &Hex) -> TokenStream {
    // convert hex str to vec4
    let color = hex_to_vec4(value);
    // let color = parse_str::<TokenStream>(value).unwrap();
    quote! {
        fn pixel(self) -> vec4{
            return #color;
        }
    }
}

pub fn draw_radial_gradient(value: &RadialGradient) -> TokenStream {
    let RadialGradient { colors } = value;

    let mut draw_color_tk = TokenStream::new();

    for (index, (hex, percentage)) in colors.iter().enumerate() {
        let color_ident = parse_str::<TokenStream>(&format!("color{}", index)).unwrap();
        let percentage_ident = parse_str::<TokenStream>(&format!("stop{}", index)).unwrap();
        draw_color_tk.extend(quote! {
            let #color_ident = #hex;
            let #percentage_ident = #percentage;
        });
    }

    let mut mix_colors = Vec::new();

    for i in 0..colors.len() - 1 {
        let ident1 = format!("color{}", i);
        let ident2 = format!("color{}", i + 1);

        let stop1 = format!("stop{}", i);
        let stop2 = format!("stop{}", i + 1);

        mix_colors.push(((ident1, ident2), (stop1, stop2)));
    }

    let mix_colors_tk = mix_color_to_token(mix_colors);

    quote! {
        let center = vec2(0.5, 0.5);
        let distance = distance(self.pos, center);
        let factor = clamp(distance, 0.0, 1.0);

        #draw_color_tk

        return #mix_colors_tk;
    }
}

/// draw linear gradient use glsl code
/// - value: &LinearGradient
/// - fn_name: &str (function name)
pub fn draw_linear_gradient(value: &LinearGradient) -> TokenStream {
    let LinearGradient { angle, colors } = value;
    let angle = parse_str::<TokenStream>(angle.to_string().as_str()).unwrap();
    let mut draw_color_tk = TokenStream::new();

    for (index, (hex, percentage)) in colors.iter().enumerate() {
        let color_ident = parse_str::<TokenStream>(&format!("color{}", index)).unwrap();
        let percentage_ident = parse_str::<TokenStream>(&format!("stop{}", index)).unwrap();
        draw_color_tk.extend(quote! {
            let #color_ident = #hex;
            let #percentage_ident = #percentage;
        });
    }

    let mut mix_colors = Vec::new();

    for i in 0..colors.len() - 1 {
        let ident1 = format!("color{}", i);
        let ident2 = format!("color{}", i + 1);

        let stop1 = format!("stop{}", i);
        let stop2 = format!("stop{}", i + 1);

        mix_colors.push(((ident1, ident2), (stop1, stop2)));
    }

    let mix_colors_tk = mix_color_to_token(mix_colors);

    quote! {
        let gradient_angle = #angle;
        let direction = vec2(cos(radians(gradient_angle)), sin(radians(gradient_angle)));
        let factor = dot(self.pos, direction);

        #draw_color_tk

        return #mix_colors_tk;
    }
}

pub fn mix_color_to_token(mix_colors: Vec<((String, String), (String, String))>) -> TokenStream {
    fn nested_mix(codes: &Vec<((String, String), (String, String))>, index: usize) -> TokenStream {
        if index >= codes.len() - 1 {
            // 最后一个颜色段，不需要再嵌套
            let ((color, next_color), (stop, next_stop)) = &codes[index];

            let color = parse_str::<TokenStream>(color).unwrap();
            let next_color = parse_str::<TokenStream>(next_color).unwrap();
            let stop = parse_str::<TokenStream>(stop).unwrap();
            let next_stop = parse_str::<TokenStream>(next_stop).unwrap();

            quote! {
                mix(#color, #next_color, smoothstep(#stop, #next_stop, factor))
            }
        } else {
            // 递归生成嵌套的mix调用
            let ((color, next_color), (stop, next_stop)) = &codes[index];
            let color = parse_str::<TokenStream>(color).unwrap();
            let _next_color = parse_str::<TokenStream>(next_color).unwrap();
            let stop = parse_str::<TokenStream>(stop).unwrap();
            let next_stop = parse_str::<TokenStream>(next_stop).unwrap();
            let next_mix = nested_mix(codes, index + 1);

            quote! {
                mix(
                    #color,
                    #next_mix,
                    smoothstep(#stop, #next_stop, factor)
                )
            }
        }
    }

    nested_mix(&mix_colors, 0)
}
