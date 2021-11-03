use super::super::*;
use super::*;

pub fn regular(source: &RawColorValues) -> Regular {
    let scale = 0..64;
    let color_mapper = |color: RawColorValue| {
        if scale.contains(&(color.red as usize))
            && scale.contains(&(color.green as usize))
            && scale.contains(&(color.blue as usize))
        {
            Some(ColorPixel {
                red: Pixel {
                    value: color.red as usize,
                    scale: scale.start..scale.end,
                },
                green: Pixel {
                    value: color.green as usize,
                    scale: scale.start..scale.end,
                },
                blue: Pixel {
                    value: color.blue as usize,
                    scale: scale.start..scale.end,
                },
            })
        } else {
            None
        }
    };

    Regular {
        colors: source.values.map(color_mapper),
    }
}

pub fn animated(source: &RawColorValues) -> Animated {
    let scale = 0..256;
    let animated_color_mapper = |color: &RawColorValue| -> ColorPixel {
        ColorPixel {
            red: Pixel {
                value: (color.red as usize),
                scale: scale.start..scale.end,
            },
            green: Pixel {
                value: (color.green as usize),
                scale: scale.start..scale.end,
            },
            blue: Pixel {
                value: (color.blue as usize),
                scale: scale.start..scale.end,
            },
        }
    };

    let mut slime_values = Vec::new();
    for color in &(source.values[229..233]) {
        slime_values.push(animated_color_mapper(color));
    }

    let mut screen_values = Vec::new();
    for color in &(source.values[233..238]) {
        screen_values.push(animated_color_mapper(color));
    }

    let mut fire_slow_values = Vec::new();
    for color in &(source.values[238..243]) {
        fire_slow_values.push(animated_color_mapper(color));
    }

    let mut fire_fast_values = Vec::new();
    for color in &(source.values[243..248]) {
        fire_fast_values.push(animated_color_mapper(color));
    }

    let mut shore_values = Vec::new();
    for color in &(source.values[248..254]) {
        shore_values.push(animated_color_mapper(color));
    }

    // original Fallout™ engine calculates these values
    // using color at index 254 for char overflow arithmetics, hardcoded
    // so we hardcode it too, just another way
    let mut alarm_values = Vec::new();
    let alarm_value_mapper = |index: u8| {
        let color = RawColorValue {
            red: index * 4,
            green: 0,
            blue: 0,
        };
        animated_color_mapper(&color)
    };

    for i in 1..16 {
        alarm_values.push(alarm_value_mapper(i));
    }

    for i in (2..15).rev() {
        alarm_values.push(alarm_value_mapper(i));
    }

    Animated {
        alarm: FrameColors {
            values: alarm_values,
            duration: std::time::Duration::from_millis(33),
        },
        slime: FrameColors {
            values: slime_values,
            duration: std::time::Duration::from_millis(200),
        },
        shore: FrameColors {
            values: shore_values,
            duration: std::time::Duration::from_millis(200),
        },
        screen: FrameColors {
            values: screen_values,
            duration: std::time::Duration::from_millis(100),
        },

        fire_slow: FrameColors {
            values: fire_slow_values,
            duration: std::time::Duration::from_millis(200),
        },
        fire_fast: FrameColors {
            values: fire_fast_values,
            duration: std::time::Duration::from_millis(142),
        },
    }
}
