use rug::Float;
use rug::ops::Pow;

use crate::component::FloatComponent;
use crate::rgb::{Rgb, RgbChannel};

#[derive(Debug, PartialEq, Clone)]
pub struct XyzChannel {
    value: Float,
}

impl FloatComponent for XyzChannel {
    // TODO maybe make this try
    fn from_value(component_value: Float) -> Self {
        XyzChannel {
            value: component_value,
        }
    }

    fn value(&self) -> &Float {
        &self.value
    }
}

impl From<Float> for XyzChannel {
    fn from(val: Float) -> Self {
        XyzChannel::from_value(val)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Xyz {
    x: XyzChannel,
    y: XyzChannel,
    z: XyzChannel,
}

impl Xyz {
    pub fn x(&self) -> &XyzChannel {
        &self.x
    }

    pub fn y(&self) -> &XyzChannel {
        &self.y
    }

    pub fn z(&self) -> &XyzChannel {
        &self.z
    }

    /// Creates an opaque color based on the given color channels.
    pub fn from_channels(x: XyzChannel, y: XyzChannel, z: XyzChannel) -> Xyz {
        Xyz { x, y, z }
    }
}

impl From<Rgb> for Xyz {
    fn from(rgb: Rgb) -> Self {
        let r: Float = transform_color_value(rgb.red()) * 100;
        let g: Float = transform_color_value(rgb.green()) * 100;
        let b: Float = transform_color_value(rgb.blue()) * 100;

        let x: Float = r.clone() * 0.4124 + g.clone() * 0.3576 + b.clone() * 0.1805;
        let y: Float = r.clone() * 0.2126 + g.clone() * 0.7152 + b.clone() * 0.0722;
        let z: Float = r * 0.0193 + g * 0.1192 + b * 0.9505;

        Xyz::from_channels(x.into(), y.into(), z.into())
    }
}

// TODO merge with contrast fn
fn transform_color_value(channel: &RgbChannel) -> Float {
    let srgb_val = channel.value().clone();
    if srgb_val <= 0.04045 {
        srgb_val / 12.92
    } else {
        let tmp: Float = (srgb_val + 0.055) / 1.055;
        tmp.pow(2.4)
    }
}

#[cfg(test)]
mod tests {
    use rug::Float;

    use crate::rgb::DEFAULT_RGB_PRECISION;

    use super::*;

    #[test]
    fn from_rgb_to_xyz() {
        let rgb = Rgb::from_channels((0).into(), (255).into(), (64).into());

        let xyz = Xyz::from(rgb);

        let prec = 32;

        let expected_x = Float::with_val(prec, 36.68541372365148);
        let expected_y = Float::with_val(prec, 71.89016548946059);
        let expected_z = Float::with_val(prec, 16.79316201845281);

        let mut actual_x = xyz.x().value().to_owned();
        actual_x.set_prec(prec);
        let mut actual_y = xyz.y().value().to_owned();
        actual_y.set_prec(prec);
        let mut actual_z = xyz.z().value().to_owned();
        actual_z.set_prec(prec);

        assert_eq!(actual_x, expected_x);
        assert_eq!(actual_y, expected_y);
        assert_eq!(actual_z, expected_z);
    }
}
