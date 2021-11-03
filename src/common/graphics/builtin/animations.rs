use crate::common::graphics::{AnimatedColorPixel, ColorPixel, Pixel};

impl AnimatedColorPixel {
    #[inline]
    pub fn alarm() -> Self {
        let scale = 0..256;

        let mut frames = Vec::new();
        let duration = std::time::Duration::from_millis(33);

        let mapper = |index: usize| ColorPixel {
            red: Pixel {
                value: index * 4,
                scale: scale.start..scale.end,
            },
            green: Pixel {
                value: 0,
                scale: scale.start..scale.end,
            },
            blue: Pixel {
                value: 0,
                scale: scale.start..scale.end,
            },
        };

        for i in 1..16 {
            frames.push(mapper(i));
        }

        for i in (2..15).rev() {
            frames.push(mapper(i));
        }

        Self { frames, duration }
    }

    #[inline]
    pub fn slime() -> Self {
        let scale = 0..256;

        let frames = vec![
            ColorPixel {
                red: Pixel {
                    value: 0,
                    scale: scale.start..scale.end,
                },
                green: Pixel {
                    value: 108,
                    scale: scale.start..scale.end,
                },
                blue: Pixel {
                    value: 0,
                    scale: scale.start..scale.end,
                },
            },
            ColorPixel {
                red: Pixel {
                    value: 11,
                    scale: scale.start..scale.end,
                },
                green: Pixel {
                    value: 115,
                    scale: scale.start..scale.end,
                },
                blue: Pixel {
                    value: 7,
                    scale: scale.start..scale.end,
                },
            },
            ColorPixel {
                red: Pixel {
                    value: 27,
                    scale: scale.start..scale.end,
                },
                green: Pixel {
                    value: 123,
                    scale: scale.start..scale.end,
                },
                blue: Pixel {
                    value: 15,
                    scale: scale.start..scale.end,
                },
            },
            ColorPixel {
                red: Pixel {
                    value: 43,
                    scale: scale.start..scale.end,
                },
                green: Pixel {
                    value: 131,
                    scale: scale.start..scale.end,
                },
                blue: Pixel {
                    value: 27,
                    scale: scale.start..scale.end,
                },
            },
        ];

        Self {
            frames,
            duration: std::time::Duration::from_millis(200),
        }
    }

    #[inline]
    pub fn shore() -> Self {
        let scale = 0..256;

        let frames = vec![
            ColorPixel {
                red: Pixel {
                    value: 83,
                    scale: scale.start..scale.end,
                },
                green: Pixel {
                    value: 63,
                    scale: scale.start..scale.end,
                },
                blue: Pixel {
                    value: 43,
                    scale: scale.start..scale.end,
                },
            },
            ColorPixel {
                red: Pixel {
                    value: 75,
                    scale: scale.start..scale.end,
                },
                green: Pixel {
                    value: 59,
                    scale: scale.start..scale.end,
                },
                blue: Pixel {
                    value: 43,
                    scale: scale.start..scale.end,
                },
            },
            ColorPixel {
                red: Pixel {
                    value: 67,
                    scale: scale.start..scale.end,
                },
                green: Pixel {
                    value: 55,
                    scale: scale.start..scale.end,
                },
                blue: Pixel {
                    value: 39,
                    scale: scale.start..scale.end,
                },
            },
            ColorPixel {
                red: Pixel {
                    value: 63,
                    scale: scale.start..scale.end,
                },
                green: Pixel {
                    value: 51,
                    scale: scale.start..scale.end,
                },
                blue: Pixel {
                    value: 39,
                    scale: scale.start..scale.end,
                },
            },
            ColorPixel {
                red: Pixel {
                    value: 55,
                    scale: scale.start..scale.end,
                },
                green: Pixel {
                    value: 37,
                    scale: scale.start..scale.end,
                },
                blue: Pixel {
                    value: 45,
                    scale: scale.start..scale.end,
                },
            },
            ColorPixel {
                red: Pixel {
                    value: 51,
                    scale: scale.start..scale.end,
                },
                green: Pixel {
                    value: 43,
                    scale: scale.start..scale.end,
                },
                blue: Pixel {
                    value: 35,
                    scale: scale.start..scale.end,
                },
            },
        ];

        Self {
            frames,
            duration: std::time::Duration::from_millis(200),
        }
    }

    #[inline]
    pub fn screen() -> Self {
        let scale = 0..256;

        let frames = vec![
            ColorPixel {
                red: Pixel {
                    value: 107,
                    scale: scale.start..scale.end,
                },
                green: Pixel {
                    value: 107,
                    scale: scale.start..scale.end,
                },
                blue: Pixel {
                    value: 111,
                    scale: scale.start..scale.end,
                },
            },
            ColorPixel {
                red: Pixel {
                    value: 99,
                    scale: scale.start..scale.end,
                },
                green: Pixel {
                    value: 103,
                    scale: scale.start..scale.end,
                },
                blue: Pixel {
                    value: 127,
                    scale: scale.start..scale.end,
                },
            },
            ColorPixel {
                red: Pixel {
                    value: 87,
                    scale: scale.start..scale.end,
                },
                green: Pixel {
                    value: 107,
                    scale: scale.start..scale.end,
                },
                blue: Pixel {
                    value: 143,
                    scale: scale.start..scale.end,
                },
            },
            ColorPixel {
                red: Pixel {
                    value: 0,
                    scale: scale.start..scale.end,
                },
                green: Pixel {
                    value: 147,
                    scale: scale.start..scale.end,
                },
                blue: Pixel {
                    value: 163,
                    scale: scale.start..scale.end,
                },
            },
            ColorPixel {
                red: Pixel {
                    value: 107,
                    scale: scale.start..scale.end,
                },
                green: Pixel {
                    value: 187,
                    scale: scale.start..scale.end,
                },
                blue: Pixel {
                    value: 255,
                    scale: scale.start..scale.end,
                },
            },
        ];

        Self {
            frames,
            duration: std::time::Duration::from_millis(100),
        }
    }

    #[inline]
    pub fn fire_slow() -> Self {
        let scale = 0..256;

        let frames = vec![
            ColorPixel {
                red: Pixel {
                    value: 255,
                    scale: scale.start..scale.end,
                },
                green: Pixel {
                    value: 0,
                    scale: scale.start..scale.end,
                },
                blue: Pixel {
                    value: 0,
                    scale: scale.start..scale.end,
                },
            },
            ColorPixel {
                red: Pixel {
                    value: 215,
                    scale: scale.start..scale.end,
                },
                green: Pixel {
                    value: 0,
                    scale: scale.start..scale.end,
                },
                blue: Pixel {
                    value: 0,
                    scale: scale.start..scale.end,
                },
            },
            ColorPixel {
                red: Pixel {
                    value: 147,
                    scale: scale.start..scale.end,
                },
                green: Pixel {
                    value: 43,
                    scale: scale.start..scale.end,
                },
                blue: Pixel {
                    value: 11,
                    scale: scale.start..scale.end,
                },
            },
            ColorPixel {
                red: Pixel {
                    value: 255,
                    scale: scale.start..scale.end,
                },
                green: Pixel {
                    value: 119,
                    scale: scale.start..scale.end,
                },
                blue: Pixel {
                    value: 0,
                    scale: scale.start..scale.end,
                },
            },
            ColorPixel {
                red: Pixel {
                    value: 255,
                    scale: scale.start..scale.end,
                },
                green: Pixel {
                    value: 59,
                    scale: scale.start..scale.end,
                },
                blue: Pixel {
                    value: 0,
                    scale: scale.start..scale.end,
                },
            },
        ];

        Self {
            frames,
            duration: std::time::Duration::from_millis(200),
        }
    }

    #[inline]
    pub fn fire_fast() -> Self {
        let scale = 0..256;

        let frames = vec![
            ColorPixel {
                red: Pixel {
                    value: 71,
                    scale: scale.start..scale.end,
                },
                green: Pixel {
                    value: 0,
                    scale: scale.start..scale.end,
                },
                blue: Pixel {
                    value: 0,
                    scale: scale.start..scale.end,
                },
            },
            ColorPixel {
                red: Pixel {
                    value: 123,
                    scale: scale.start..scale.end,
                },
                green: Pixel {
                    value: 0,
                    scale: scale.start..scale.end,
                },
                blue: Pixel {
                    value: 0,
                    scale: scale.start..scale.end,
                },
            },
            ColorPixel {
                red: Pixel {
                    value: 179,
                    scale: scale.start..scale.end,
                },
                green: Pixel {
                    value: 0,
                    scale: scale.start..scale.end,
                },
                blue: Pixel {
                    value: 0,
                    scale: scale.start..scale.end,
                },
            },
            ColorPixel {
                red: Pixel {
                    value: 123,
                    scale: scale.start..scale.end,
                },
                green: Pixel {
                    value: 0,
                    scale: scale.start..scale.end,
                },
                blue: Pixel {
                    value: 0,
                    scale: scale.start..scale.end,
                },
            },
            ColorPixel {
                red: Pixel {
                    value: 71,
                    scale: scale.start..scale.end,
                },
                green: Pixel {
                    value: 0,
                    scale: scale.start..scale.end,
                },
                blue: Pixel {
                    value: 0,
                    scale: scale.start..scale.end,
                },
            },
        ];

        Self {
            frames,
            duration: std::time::Duration::from_millis(142),
        }
    }
}
