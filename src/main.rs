//alias kitty-light='kitty @ set_colors -a $HOME/.config/kitty/kitty-themes/themes/PencilLight.conf'
use anyhow::Result;
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;

use color_processing::Color;
use palette::{FromColor, Lch, Saturate, Shade, Srgb};
use palette::{Hue, LinSrgb};

struct ColorPair {
    name: String,
    value: Srgb,
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
                Some(ColorPair {
                    name,
                    value: Srgb::new(
                        color.red as f32 / 255.0,
                        color.green as f32 / 255.0,
                        color.blue as f32 / 255.0,
                    ),
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
                let new_color = Color::new_rgb(
                    (color.value.red * 255.0) as u8,
                    (color.value.green * 255.0) as u8,
                    (color.value.blue * 255.0) as u8,
                );
                format!("{} {}", color.name, new_color.to_hex_string())
            })
            .collect::<Vec<String>>()
            .join("\n")
    }
}

fn main() -> Result<()> {
    let mut theme = Theme::parse(
        "
background #f0f0f0
foreground #414141
cursor #20bafb
selection_background #b6d6fc
color0 #202020
color8 #414141
color1 #c30670
color9 #fb0079
color2 #10a778
color10 #5ed6ae
color3 #a79c14
color11 #f3e42f
color4 #008ec4
color12 #20bafb
color5 #523b78
color13 #6854de
color6 #20a4b9
color14 #4fb8cc
color7 #d9d9d9
color15 #f0f0f0
selection_foreground #f0f0f0
",
    );

    loop {
        theme.colors = theme
            .colors
            .iter()
            .map(|color_pair| {
                //
                let color = color_pair.value.clone();

                let lch_color: Lch = color.into();

                let new_color = Srgb::from(lch_color.shift_hue(2.1));
                ColorPair {
                    name: color_pair.name.clone(),
                    value: new_color,
                }
            })
            .collect();
        let mut file = File::create("/Users/brian/Desktop/kittytheme.conf")?;
        file.write_all(theme.dump().as_bytes())?;
        resync()?;

        let ten_millis = std::time::Duration::from_millis(15);
        std::thread::sleep(ten_millis);
    }
    println!("Hello, world!");
    Ok(())
}

fn resync() -> Result<()> {
    Command::new("kitty")
        .arg("@")
        .arg("set_colors")
        .arg("-a")
        .arg("/Users/brian/Desktop/kittytheme.conf")
        .output()?;
    Ok(())
}
