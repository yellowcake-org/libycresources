use crate::common::graphics::{AnimatedPixel, Pixel};
use crate::common::types::geometry::Scaled;

impl AnimatedPixel {
    #[inline]
    pub fn alarm() -> Self {
        let scale = 0..256;

        let mut values = Vec::new();
        let duration = std::time::Duration::from_millis(33);

        let mapper = |index: usize| Pixel {
            red: Scaled {
                value: index * 4,
                scale: scale.clone(),
            },
            green: Scaled {
                value: 0,
                scale: scale.clone(),
            },
            blue: Scaled {
                value: 0,
                scale: scale.clone(),
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
                red: Scaled {
                    value: 0,
                    scale: scale.clone(),
                },
                green: Scaled {
                    value: 108,
                    scale: scale.clone(),
                },
                blue: Scaled {
                    value: 0,
                    scale: scale.clone(),
                },
            },
            Pixel {
                red: Scaled {
                    value: 11,
                    scale: scale.clone(),
                },
                green: Scaled {
                    value: 115,
                    scale: scale.clone(),
                },
                blue: Scaled {
                    value: 7,
                    scale: scale.clone(),
                },
            },
            Pixel {
                red: Scaled {
                    value: 27,
                    scale: scale.clone(),
                },
                green: Scaled {
                    value: 123,
                    scale: scale.clone(),
                },
                blue: Scaled {
                    value: 15,
                    scale: scale.clone(),
                },
            },
            Pixel {
                red: Scaled {
                    value: 43,
                    scale: scale.clone(),
                },
                green: Scaled {
                    value: 131,
                    scale: scale.clone(),
                },
                blue: Scaled {
                    value: 27,
                    scale: scale.clone(),
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
                red: Scaled {
                    value: 83,
                    scale: scale.clone(),
                },
                green: Scaled {
                    value: 63,
                    scale: scale.clone(),
                },
                blue: Scaled {
                    value: 43,
                    scale: scale.clone(),
                },
            },
            Pixel {
                red: Scaled {
                    value: 75,
                    scale: scale.clone(),
                },
                green: Scaled {
                    value: 59,
                    scale: scale.clone(),
                },
                blue: Scaled {
                    value: 43,
                    scale: scale.clone(),
                },
            },
            Pixel {
                red: Scaled {
                    value: 67,
                    scale: scale.clone(),
                },
                green: Scaled {
                    value: 55,
                    scale: scale.clone(),
                },
                blue: Scaled {
                    value: 39,
                    scale: scale.clone(),
                },
            },
            Pixel {
                red: Scaled {
                    value: 63,
                    scale: scale.clone(),
                },
                green: Scaled {
                    value: 51,
                    scale: scale.clone(),
                },
                blue: Scaled {
                    value: 39,
                    scale: scale.clone(),
                },
            },
            Pixel {
                red: Scaled {
                    value: 55,
                    scale: scale.clone(),
                },
                green: Scaled {
                    value: 37,
                    scale: scale.clone(),
                },
                blue: Scaled {
                    value: 45,
                    scale: scale.clone(),
                },
            },
            Pixel {
                red: Scaled {
                    value: 51,
                    scale: scale.clone(),
                },
                green: Scaled {
                    value: 43,
                    scale: scale.clone(),
                },
                blue: Scaled {
                    value: 35,
                    scale: scale.clone(),
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
                red: Scaled {
                    value: 107,
                    scale: scale.clone(),
                },
                green: Scaled {
                    value: 107,
                    scale: scale.clone(),
                },
                blue: Scaled {
                    value: 111,
                    scale: scale.clone(),
                },
            },
            Pixel {
                red: Scaled {
                    value: 99,
                    scale: scale.clone(),
                },
                green: Scaled {
                    value: 103,
                    scale: scale.clone(),
                },
                blue: Scaled {
                    value: 127,
                    scale: scale.clone(),
                },
            },
            Pixel {
                red: Scaled {
                    value: 87,
                    scale: scale.clone(),
                },
                green: Scaled {
                    value: 107,
                    scale: scale.clone(),
                },
                blue: Scaled {
                    value: 143,
                    scale: scale.clone(),
                },
            },
            Pixel {
                red: Scaled {
                    value: 0,
                    scale: scale.clone(),
                },
                green: Scaled {
                    value: 147,
                    scale: scale.clone(),
                },
                blue: Scaled {
                    value: 163,
                    scale: scale.clone(),
                },
            },
            Pixel {
                red: Scaled {
                    value: 107,
                    scale: scale.clone(),
                },
                green: Scaled {
                    value: 187,
                    scale: scale.clone(),
                },
                blue: Scaled {
                    value: 255,
                    scale: scale.clone(),
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
                red: Scaled {
                    value: 255,
                    scale: scale.clone(),
                },
                green: Scaled {
                    value: 0,
                    scale: scale.clone(),
                },
                blue: Scaled {
                    value: 0,
                    scale: scale.clone(),
                },
            },
            Pixel {
                red: Scaled {
                    value: 215,
                    scale: scale.clone(),
                },
                green: Scaled {
                    value: 0,
                    scale: scale.clone(),
                },
                blue: Scaled {
                    value: 0,
                    scale: scale.clone(),
                },
            },
            Pixel {
                red: Scaled {
                    value: 147,
                    scale: scale.clone(),
                },
                green: Scaled {
                    value: 43,
                    scale: scale.clone(),
                },
                blue: Scaled {
                    value: 11,
                    scale: scale.clone(),
                },
            },
            Pixel {
                red: Scaled {
                    value: 255,
                    scale: scale.clone(),
                },
                green: Scaled {
                    value: 119,
                    scale: scale.clone(),
                },
                blue: Scaled {
                    value: 0,
                    scale: scale.clone(),
                },
            },
            Pixel {
                red: Scaled {
                    value: 255,
                    scale: scale.clone(),
                },
                green: Scaled {
                    value: 59,
                    scale: scale.clone(),
                },
                blue: Scaled {
                    value: 0,
                    scale: scale.clone(),
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
                red: Scaled {
                    value: 71,
                    scale: scale.clone(),
                },
                green: Scaled {
                    value: 0,
                    scale: scale.clone(),
                },
                blue: Scaled {
                    value: 0,
                    scale: scale.clone(),
                },
            },
            Pixel {
                red: Scaled {
                    value: 123,
                    scale: scale.clone(),
                },
                green: Scaled {
                    value: 0,
                    scale: scale.clone(),
                },
                blue: Scaled {
                    value: 0,
                    scale: scale.clone(),
                },
            },
            Pixel {
                red: Scaled {
                    value: 179,
                    scale: scale.clone(),
                },
                green: Scaled {
                    value: 0,
                    scale: scale.clone(),
                },
                blue: Scaled {
                    value: 0,
                    scale: scale.clone(),
                },
            },
            Pixel {
                red: Scaled {
                    value: 123,
                    scale: scale.clone(),
                },
                green: Scaled {
                    value: 0,
                    scale: scale.clone(),
                },
                blue: Scaled {
                    value: 0,
                    scale: scale.clone(),
                },
            },
            Pixel {
                red: Scaled {
                    value: 71,
                    scale: scale.clone(),
                },
                green: Scaled {
                    value: 0,
                    scale: scale.clone(),
                },
                blue: Scaled {
                    value: 0,
                    scale: scale.clone(),
                },
            },
        ];

        Self {
            values,
            duration: std::time::Duration::from_millis(142),
        }
    }
}
