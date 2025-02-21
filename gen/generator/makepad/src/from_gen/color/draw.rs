use gen_analyzer::value::{Hex, LinearGradient, RadialGradient};
use gen_utils::error::Error;
use proc_macro2::TokenStream;
use quote::quote;
use crate::str_to_tk;

/// convert hex to pixel
#[allow(dead_code)]
pub fn hex_to_pixel(value: &Hex) -> Result<TokenStream, Error> {
    // convert hex str to vec4
    let color = str_to_tk!(&value.to_vec4())?;
    Ok(quote! {
        fn pixel(self) -> vec4{
            return #color;
        }
    })
}

pub fn draw_radial_gradient(value: &RadialGradient) -> Result<TokenStream, Error> {
    let RadialGradient { colors } = value;

    let mut draw_color_tk = TokenStream::new();

    for (index, (hex, percentage)) in colors.iter().enumerate() {
        let color_ident = str_to_tk!(&format!("color{}", index))?;
        let percentage_ident = str_to_tk!(&format!("stop{}", index))?;
        let hex = str_to_tk!(&hex.to_vec4())?;
        let percentage = str_to_tk!(&percentage.to_token_str())?;
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

    let mix_colors_tk = mix_color_to_token(mix_colors)?;

    Ok(quote! {
        let center = vec2(0.5, 0.5);
        let distance = distance(self.pos, center);
        let factor = clamp(distance, 0.0, 1.0);

        #draw_color_tk

        return #mix_colors_tk;
    })
}

/// draw linear gradient use glsl code
/// - value: &LinearGradient
/// - fn_name: &str (function name)
pub fn draw_linear_gradient(value: &LinearGradient) -> Result<TokenStream, Error> {
    let LinearGradient { angle, colors } = value;
    let angle = str_to_tk!(angle.to_string().as_str())?;
    let mut draw_color_tk = TokenStream::new();

    for (index, (hex, percentage)) in colors.iter().enumerate() {
        let color_ident = str_to_tk!(&format!("color{}", index))?;
        let percentage_ident = str_to_tk!(&format!("stop{}", index))?;
        let hex = str_to_tk!(&hex.to_vec4())?;
        let percentage = str_to_tk!(&percentage.to_token_str())?;
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

    let mix_colors_tk = mix_color_to_token(mix_colors)?;

    Ok(quote! {
        let gradient_angle = #angle;
        let direction = vec2(cos(radians(gradient_angle)), sin(radians(gradient_angle)));
        let factor = dot(self.pos, direction);

        #draw_color_tk

        return #mix_colors_tk;
    })
}

pub fn mix_color_to_token(
    mix_colors: Vec<((String, String), (String, String))>,
) -> Result<TokenStream, Error> {
    fn nested_mix(
        codes: &Vec<((String, String), (String, String))>,
        index: usize,
    ) -> Result<TokenStream, Error> {
        if index >= codes.len() - 1 {
            // 最后一个颜色段，不需要再嵌套
            let ((color, next_color), (stop, next_stop)) = &codes[index];

            let color = str_to_tk!(color)?;
            let next_color = str_to_tk!(next_color)?;
            let stop = str_to_tk!(stop)?;
            let next_stop = str_to_tk!(next_stop)?;

            Ok(quote! {
                mix(#color, #next_color, smoothstep(#stop, #next_stop, factor))
            })
        } else {
            // 递归生成嵌套的mix调用
            let ((color, next_color), (stop, next_stop)) = &codes[index];
            let color = str_to_tk!(color)?;
            let _next_color = str_to_tk!(next_color)?;
            let stop = str_to_tk!(stop)?;
            let next_stop = str_to_tk!(next_stop)?;
            let next_mix = nested_mix(codes, index + 1)?;

            Ok(quote! {
                mix(
                    #color,
                    #next_mix,
                    smoothstep(#stop, #next_stop, factor)
                )
            })
        }
    }

    nested_mix(&mix_colors, 0)
}
