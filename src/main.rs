use anyhow::Result;
use std::time::{Duration, Instant};
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;

use color_processing::Color;
use palette::encoding::pixel::Pixel;
use palette::{FromColor, Lch, Saturate, Shade, Srgb};
use palette::{Hue, LinSrgb};

struct ColorPair {
    name: String,
    value: Srgb,
}

impl ColorPair {

    fn to_hex(&self) -> String {

                let conv: Srgb<u8> = self.value.into_format();
                let new_color = Color::new_rgb(conv.red, conv.green, conv.blue);
                new_color.to_hex_string()
    }
}
struct Theme {
    colors: Vec<ColorPair>,
}

impl Theme {
    fn parse(theme: &str) -> Theme {
        let colors = theme
            .lines()
            .filter_map(|line| {
                let mut parts = line.split_whitespace();
                let name = parts.next()?.trim().to_string();
                let color = Color::new_string(parts.next()?.trim())?;
                let buffer = [color.red, color.green, color.blue];
                let raw = Srgb::from_raw(&buffer);
                Some(ColorPair {
                    name,
                    value: raw.into_format(),
                })
            })
            .collect();

        Theme { colors }
    }

    fn dump(&self) -> String {
        self.colors
            .iter()
            .map(|color| {
                //
                let conv: Srgb<u8> = color.value.into_format();
                let new_color = Color::new_rgb(conv.red, conv.green, conv.blue);
                format!("{} {}", color.name, new_color.to_hex_string())
            })
            .collect::<Vec<String>>()
            .join("\n")
    }

}

fn main() -> Result<()> {
  
    let current_colors = read_colors()?;
    //println!("{}", current_colors);
    let mut theme = Theme::parse(
        &current_colors
    );

    loop {
        // let fg = theme.colors[0].value.clone();
        // let lch_color: Lch = fg.into();

        // let new_color = Srgb::from(lch_color.shift_hue(1.1));
        // theme.colors[0].value = new_color;

        theme.colors = theme
            .colors
            .iter()
            .map(|color_pair| {
                //
                let color = color_pair.value.clone();

                let lch_color: Lch = color.into();

                let new_color = Srgb::from(lch_color.shift_hue(1.1));
                ColorPair {
                    name: color_pair.name.clone(),
                    value: new_color,
                }
            })
            .collect();
        
        let start = Instant::now();

        set_color(&theme.colors)?;
        let duration = start.elapsed();

        //println!("Time elapsed in expensive_function() is: {:?}", duration);

        // Provide a consistent framerate. Increase the high end to reduce load on kitty.
        let target_sleep_time = std::time::Duration::from_millis(500);
        let sleep_time = if target_sleep_time > duration {
            target_sleep_time - start.elapsed()
        } else {
            std::time::Duration::from_millis(0)
        };
        std::thread::sleep(target_sleep_time);
    }
}

fn set_color(color_pairs: &[ColorPair]) -> Result<()> {
    
    let mut command = Command::new("kitty");


        command.arg("@");
        command.arg("set_colors");
        command.arg("-a");

    for color_pair in color_pairs {
        command.arg(format!("{}={}", color_pair.name, color_pair.to_hex()));
    }
    
    command.output()?;

    Ok(())
}


fn read_colors() -> Result<String> {

    let out = Command::new("kitty")
        .arg("@")
        .arg("get_colors")
        .output()?;

    Ok(String::from_utf8(out.stdout)?)

}
