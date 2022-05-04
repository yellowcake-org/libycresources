use crate::common::graphics::{AnimatedPixel, Pixel};
use crate::common::types::ScaledValue;

impl AnimatedPixel {
    #[inline]
    pub fn alarm() -> Self {
        let scale = 0..256;

        let mut values = Vec::new();
        let duration = std::time::Duration::from_millis(33);

        let mapper = |index: usize| Pixel {
            red: ScaledValue {
                value: index * 4,
                scale: scale.start..scale.end,
            },
            green: ScaledValue {
                value: 0,
                scale: scale.start..scale.end,
            },
            blue: ScaledValue {
                value: 0,
                scale: scale.start..scale.end,
            },
        };

        for i in 1..16 {
            values.push(mapper(i));
        }

        for i in (2..15).rev() {
            values.push(mapper(i));
        }

        Self { values, duration }
    }

    #[inline]
    pub fn slime() -> Self {
        let scale = 0..256;

        let values = vec![
            Pixel {
                red: ScaledValue {
                    value: 0,
                    scale: scale.start..scale.end,
                },
                green: ScaledValue {
                    value: 108,
                    scale: scale.start..scale.end,
                },
                blue: ScaledValue {
                    value: 0,
                    scale: scale.start..scale.end,
                },
            },
            Pixel {
                red: ScaledValue {
                    value: 11,
                    scale: scale.start..scale.end,
                },
                green: ScaledValue {
                    value: 115,
                    scale: scale.start..scale.end,
                },
                blue: ScaledValue {
                    value: 7,
                    scale: scale.start..scale.end,
                },
            },
            Pixel {
                red: ScaledValue {
                    value: 27,
                    scale: scale.start..scale.end,
                },
                green: ScaledValue {
                    value: 123,
                    scale: scale.start..scale.end,
                },
                blue: ScaledValue {
                    value: 15,
                    scale: scale.start..scale.end,
                },
            },
            Pixel {
                red: ScaledValue {
                    value: 43,
                    scale: scale.start..scale.end,
                },
                green: ScaledValue {
                    value: 131,
                    scale: scale.start..scale.end,
                },
                blue: ScaledValue {
                    value: 27,
                    scale: scale.start..scale.end,
                },
            },
        ];

        Self {
            values,
            duration: std::time::Duration::from_millis(200),
        }
    }

    #[inline]
    pub fn shore() -> Self {
        let scale = 0..256;

        let values = vec![
            Pixel {
                red: ScaledValue {
                    value: 83,
                    scale: scale.start..scale.end,
                },
                green: ScaledValue {
                    value: 63,
                    scale: scale.start..scale.end,
                },
                blue: ScaledValue {
                    value: 43,
                    scale: scale.start..scale.end,
                },
            },
            Pixel {
                red: ScaledValue {
                    value: 75,
                    scale: scale.start..scale.end,
                },
                green: ScaledValue {
                    value: 59,
                    scale: scale.start..scale.end,
                },
                blue: ScaledValue {
                    value: 43,
                    scale: scale.start..scale.end,
                },
            },
            Pixel {
                red: ScaledValue {
                    value: 67,
                    scale: scale.start..scale.end,
                },
                green: ScaledValue {
                    value: 55,
                    scale: scale.start..scale.end,
                },
                blue: ScaledValue {
                    value: 39,
                    scale: scale.start..scale.end,
                },
            },
            Pixel {
                red: ScaledValue {
                    value: 63,
                    scale: scale.start..scale.end,
                },
                green: ScaledValue {
                    value: 51,
                    scale: scale.start..scale.end,
                },
                blue: ScaledValue {
                    value: 39,
                    scale: scale.start..scale.end,
                },
            },
            Pixel {
                red: ScaledValue {
                    value: 55,
                    scale: scale.start..scale.end,
                },
                green: ScaledValue {
                    value: 37,
                    scale: scale.start..scale.end,
                },
                blue: ScaledValue {
                    value: 45,
                    scale: scale.start..scale.end,
                },
            },
            Pixel {
                red: ScaledValue {
                    value: 51,
                    scale: scale.start..scale.end,
                },
                green: ScaledValue {
                    value: 43,
                    scale: scale.start..scale.end,
                },
                blue: ScaledValue {
                    value: 35,
                    scale: scale.start..scale.end,
                },
            },
        ];

        Self {
            values,
            duration: std::time::Duration::from_millis(200),
        }
    }

    #[inline]
    pub fn screen() -> Self {
        let scale = 0..256;

        let values = vec![
            Pixel {
                red: ScaledValue {
                    value: 107,
                    scale: scale.start..scale.end,
                },
                green: ScaledValue {
                    value: 107,
                    scale: scale.start..scale.end,
                },
                blue: ScaledValue {
                    value: 111,
                    scale: scale.start..scale.end,
                },
            },
            Pixel {
                red: ScaledValue {
                    value: 99,
                    scale: scale.start..scale.end,
                },
                green: ScaledValue {
                    value: 103,
                    scale: scale.start..scale.end,
                },
                blue: ScaledValue {
                    value: 127,
                    scale: scale.start..scale.end,
                },
            },
            Pixel {
                red: ScaledValue {
                    value: 87,
                    scale: scale.start..scale.end,
                },
                green: ScaledValue {
                    value: 107,
                    scale: scale.start..scale.end,
                },
                blue: ScaledValue {
                    value: 143,
                    scale: scale.start..scale.end,
                },
            },
            Pixel {
                red: ScaledValue {
                    value: 0,
                    scale: scale.start..scale.end,
                },
                green: ScaledValue {
                    value: 147,
                    scale: scale.start..scale.end,
                },
                blue: ScaledValue {
                    value: 163,
                    scale: scale.start..scale.end,
                },
            },
            Pixel {
                red: ScaledValue {
                    value: 107,
                    scale: scale.start..scale.end,
                },
                green: ScaledValue {
                    value: 187,
                    scale: scale.start..scale.end,
                },
                blue: ScaledValue {
                    value: 255,
                    scale: scale.start..scale.end,
                },
            },
        ];

        Self {
            values,
            duration: std::time::Duration::from_millis(100),
        }
    }

    #[inline]
    pub fn fire_slow() -> Self {
        let scale = 0..256;

        let values = vec![
            Pixel {
                red: ScaledValue {
                    value: 255,
                    scale: scale.start..scale.end,
                },
                green: ScaledValue {
                    value: 0,
                    scale: scale.start..scale.end,
                },
                blue: ScaledValue {
                    value: 0,
                    scale: scale.start..scale.end,
                },
            },
            Pixel {
                red: ScaledValue {
                    value: 215,
                    scale: scale.start..scale.end,
                },
                green: ScaledValue {
                    value: 0,
                    scale: scale.start..scale.end,
                },
                blue: ScaledValue {
                    value: 0,
                    scale: scale.start..scale.end,
                },
            },
            Pixel {
                red: ScaledValue {
                    value: 147,
                    scale: scale.start..scale.end,
                },
                green: ScaledValue {
                    value: 43,
                    scale: scale.start..scale.end,
                },
                blue: ScaledValue {
                    value: 11,
                    scale: scale.start..scale.end,
                },
            },
            Pixel {
                red: ScaledValue {
                    value: 255,
                    scale: scale.start..scale.end,
                },
                green: ScaledValue {
                    value: 119,
                    scale: scale.start..scale.end,
                },
                blue: ScaledValue {
                    value: 0,
                    scale: scale.start..scale.end,
                },
            },
            Pixel {
                red: ScaledValue {
                    value: 255,
                    scale: scale.start..scale.end,
                },
                green: ScaledValue {
                    value: 59,
                    scale: scale.start..scale.end,
                },
                blue: ScaledValue {
                    value: 0,
                    scale: scale.start..scale.end,
                },
            },
        ];

        Self {
            values,
            duration: std::time::Duration::from_millis(200),
        }
    }

    #[inline]
    pub fn fire_fast() -> Self {
        let scale = 0..256;

        let values = vec![
            Pixel {
                red: ScaledValue {
                    value: 71,
                    scale: scale.start..scale.end,
                },
                green: ScaledValue {
                    value: 0,
                    scale: scale.start..scale.end,
                },
                blue: ScaledValue {
                    value: 0,
                    scale: scale.start..scale.end,
                },
            },
            Pixel {
                red: ScaledValue {
                    value: 123,
                    scale: scale.start..scale.end,
                },
                green: ScaledValue {
                    value: 0,
                    scale: scale.start..scale.end,
                },
                blue: ScaledValue {
                    value: 0,
                    scale: scale.start..scale.end,
                },
            },
            Pixel {
                red: ScaledValue {
                    value: 179,
                    scale: scale.start..scale.end,
                },
                green: ScaledValue {
                    value: 0,
                    scale: scale.start..scale.end,
                },
                blue: ScaledValue {
                    value: 0,
                    scale: scale.start..scale.end,
                },
            },
            Pixel {
                red: ScaledValue {
                    value: 123,
                    scale: scale.start..scale.end,
                },
                green: ScaledValue {
                    value: 0,
                    scale: scale.start..scale.end,
                },
                blue: ScaledValue {
                    value: 0,
                    scale: scale.start..scale.end,
                },
            },
            Pixel {
                red: ScaledValue {
                    value: 71,
                    scale: scale.start..scale.end,
                },
                green: ScaledValue {
                    value: 0,
                    scale: scale.start..scale.end,
                },
                blue: ScaledValue {
                    value: 0,
                    scale: scale.start..scale.end,
                },
            },
        ];

        Self {
            values,
            duration: std::time::Duration::from_millis(142),
        }
    }
}
