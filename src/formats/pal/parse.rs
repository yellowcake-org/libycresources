use super::{AnimatedColors, Palette};
use crate::common::graphics::{ColorPixel, Pixel};

use std::convert::TryInto;
use std::io::{Read, Seek, SeekFrom};
use std::mem::size_of;

#[derive(Debug)]
pub enum Error {
    Read(std::io::Error),
    Source,
}

pub fn palette<S: Read + Seek>(source: &mut S) -> Result<Palette, Error> {
    if let Err(error) = source.seek(SeekFrom::Start(0)) {
        return Err(Error::Read(error));
    }

    let scale = 0..64;
    let scale_animated = 0..256;

    let mut colors: [(usize, usize, usize, bool); 256] = [(0, 0, 0, false); 256];

    for color in &mut colors {
        let mut red_bytes = vec![0u8; size_of::<u8>()];
        match source.read_exact(&mut red_bytes) {
            Err(error) => return Err(Error::Read(error)),
            Ok(value) => value,
        };

        let red = u8::from_le_bytes(match red_bytes.try_into() {
            Err(_) => return Err(Error::Source),
            Ok(value) => value,
        }) as usize;

        let mut green_bytes = vec![0u8; size_of::<u8>()];
        match source.read_exact(&mut green_bytes) {
            Err(error) => return Err(Error::Read(error)),
            Ok(value) => value,
        };

        let green = u8::from_le_bytes(match green_bytes.try_into() {
            Err(_) => return Err(Error::Source),
            Ok(value) => value,
        }) as usize;

        let mut blue_bytes = vec![0u8; size_of::<u8>()];
        match source.read_exact(&mut blue_bytes) {
            Err(error) => return Err(Error::Read(error)),
            Ok(value) => value,
        };

        let blue = u8::from_le_bytes(match blue_bytes.try_into() {
            Err(_) => return Err(Error::Source),
            Ok(value) => value,
        }) as usize;

        if scale.contains(&red) && scale.contains(&green) && scale.contains(&blue) {
            *color = (red, green, blue, true)
        }
    }

    let color_mapper = |(red, green, blue, is_mapped)| {
        if is_mapped {
            Some(ColorPixel {
                red: Pixel {
                    value: red,
                    scale: scale.start..scale.end,
                },
                green: Pixel {
                    value: green,
                    scale: scale.start..scale.end,
                },
                blue: Pixel {
                    value: blue,
                    scale: scale.start..scale.end,
                },
            })
        } else {
            None
        }
    };

    let animated_color_mapper =
        |(red, green, blue, _): &(usize, usize, usize, bool)| -> ColorPixel {
            ColorPixel {
                red: Pixel {
                    value: *red,
                    scale: scale_animated.start..scale_animated.end,
                },
                green: Pixel {
                    value: *green,
                    scale: scale_animated.start..scale_animated.end,
                },
                blue: Pixel {
                    value: *blue,
                    scale: scale_animated.start..scale_animated.end,
                },
            }
        };

    let mut slime_values = Vec::new();
    for color in &colors[229..233] {
        slime_values.push(animated_color_mapper(color));
    }

    let mut screen_values = Vec::new();
    for color in &colors[233..238] {
        screen_values.push(animated_color_mapper(color));
    }

    let mut fire_slow_values = Vec::new();
    for color in &colors[238..243] {
        fire_slow_values.push(animated_color_mapper(color));
    }

    let mut fire_fast_values = Vec::new();
    for color in &colors[243..248] {
        fire_fast_values.push(animated_color_mapper(color));
    }

    let mut shore_values = Vec::new();
    for color in &colors[248..254] {
        shore_values.push(animated_color_mapper(color));
    }

    // original Fallout™ engine calculates these values
    // using color at index 254 for char overflow arithmetics, hardcoded
    // so we hardcode it too, just another way
    let mut alarm_values = Vec::new();
    let alarm_value_mapper = |index: usize| {
        let color = (index * 4, 0, 0, false);
        animated_color_mapper(&color)
    };

    for i in 1..16 {
        alarm_values.push(alarm_value_mapper(i));
    }

    for i in (0..15).rev() {
        alarm_values.push(alarm_value_mapper(i));
    }

    Ok(Palette {
        colors: colors.map(color_mapper),
        alarm: AnimatedColors {
            values: alarm_values,
            frametime: std::time::Duration::from_millis(33),
        },
        slime: AnimatedColors {
            values: slime_values,
            frametime: std::time::Duration::from_millis(200),
        },
        shore: AnimatedColors {
            values: shore_values,
            frametime: std::time::Duration::from_millis(200),
        },
        screen: AnimatedColors {
            values: screen_values,
            frametime: std::time::Duration::from_millis(100),
        },

        fire_slow: AnimatedColors {
            values: fire_slow_values,
            frametime: std::time::Duration::from_millis(200),
        },
        fire_fast: AnimatedColors {
            values: fire_fast_values,
            frametime: std::time::Duration::from_millis(142),
        },
    })
}
