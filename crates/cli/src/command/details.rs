use std::io::Write;

use termcolor::{ColorChoice, StandardStream};

use color_utils::rgb::Rgb;

use crate::color_format::ColorFormat;
use crate::color_printing::print_color;
use crate::options::Options;

pub fn print_details(color: &Rgb, options: &Options) -> std::io::Result<()> {
    let mut out = StandardStream::stdout(ColorChoice::Auto);

    write!(&mut out, "Details for color ")?;
    print_color(&mut out, color, &options.format)?;
    writeln!(&mut out, ":")?;
    writeln!(&mut out, "-------")?;

    print_general_details(&mut out, color)?;

    print_format_details(&mut out, color)
}

fn print_general_details(out: &mut StandardStream, color: &Rgb) -> std::io::Result<()> {
    writeln!(out, "General: ")?;
    writeln!(out, "\tIs opaque: {}.", color.is_opaque())?;
    writeln!(
        out,
        "\tEvery channel can be represented by a single byte: {}.",
        color.channels_fit_in_u8()
    )
}

fn print_format_details(out: &mut StandardStream, color: &Rgb) -> std::io::Result<()> {
    writeln!(out, "Formats: ")?;

    write!(out, "\tIn RGB hexadecimal notation: ")?;
    print_color(out, color, &ColorFormat::RgbHex)?;
    if !color.channels_fit_in_u8() {
        write!(out, " (Warning: Channel values were rounded)")?;
    }
    writeln!(out, ".")?;

    write!(out, "\tIn RGB function notation: ")?;
    print_color(out, color, &ColorFormat::RgbFunction)?;
    writeln!(out, ".")
}