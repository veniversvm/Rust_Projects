use image::ColorType;
use image::png::PngEncoder;
use num::Complex;
use std::fs::File;
use std::str::FromStr;

fn main() {
    println!("Hello, world!");
}

/// Try to determine if `c` is in the Mandelbrot set,
/// using at most `limit` iterations to decide.
///
/// If `c` is not a member, return `Some(i)`, where `i`
/// is the number of iterations it took for `c` to leave
/// the circle of radius 2 centered on the origin.
///
/// If `c` seems to be a member (more precisely, if we
/// reached the iteration limit without being able to prove
/// ta `c` is not a member), return `None`.
fn complex_square_add_loop(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex { re: 0.0, im: 0.0 };

    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }
    None
}

/// Parse the string `s` as a coordinate pair, like "400x600"
/// or `"1.0, 0.5"`.
///
/// Specifically, `s` should have the form <left><sep><rigth>,
/// where ser is the character given by the `separator` argument,
/// and left <left> and <right> are both string that can be parse by
/// `T::from_str`. `separator` must be an ASCII character.
///
/// If `s` has the proper form, return `Some<(x, y)>`.
/// If itdoesn't parse correctly, return `None`.
fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
            (Ok(l), Ok(r)) => Some((l, r)),
            _ => None,
        },
    }
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("", ','), None);
    assert_eq!(parse_pair::<i32>("10,", ','), None);
    assert_eq!(parse_pair::<i32>(",10", ','), None);
    assert_eq!(parse_pair::<i32>("10,20", ','), Some((10, 20)));
    assert_eq!(parse_pair::<i32>("10,20xy", ','), None);
    assert_eq!(parse_pair::<f64>("0.5x", 'x'), None);
    assert_eq!(parse_pair::<f64>("0.5x1.5", 'x'), Some((0.5, 1.5)));
}

/// Parse a pair of floating-point numbers separated
/// by a comma as complex number.
fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        Some((re, im)) => Some(Complex { re, im }),
        None => None,
    }
}

#[test]
fn test_parse_copmplex() {
    assert_eq!(
        parse_complex("1.25,-0.0625"),
        Some(Complex {
            re: 1.25,
            im: -0.0625
        })
    );
    assert_eq!(parse_complex(",-0.0625"), None);
}

/// Given the row and column of a pixel in the output image
/// return the corresponding point on the complex plane.
///
/// `bounds` is a pair giving the width and the height of the
/// image pixels.
/// `pixel` is a (column, row) pair indicating a particular pixel
/// in that image.
/// Te `upper_left` and `lower_right` paramaters are points
/// on the complex plane designating the area our image covers.
fn pixel_to_point(
    bounds: (usize, usize),
    pixel: (usize, usize),
    upper_left: Complex<f64>,
    lower_rigth: Complex<f64>,
) -> Complex<f64> {
    let (width, height) = (
        lower_rigth.re - upper_left.re,
        upper_left.im - lower_rigth.im,
    );

    Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        im: upper_left.im - pixel.1 as f64 * width / bounds.1 as f64,
    }
}

#[test]
fn test_pixel_to_point() {
    assert_eq!(
        pixel_to_point(
            (100, 200),
            (25, 175),
            Complex { re: -1.0, im: 1.0 },
            Complex { re: 1.0, im: -1.0 }
        ),
        Complex {
            re: -0.5,
            im: -0.75
        }
    );
}

/// Render a rectangle of the Mandelbrot set into a buffer of pixels.
///
/// the `bounds` argyment gives the width and height of the
/// buffer `pixels`, wich holds one grayscale per byte.
/// The `upper_left` and `lower_right` arguments specify points
/// on the complex plane corresponding to the upper_left
/// and lower_rigth corners of the pixel buffer.
fn render(
    pixels: &mut [u8],
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complez<f64>,
) {
    assert!(pixels.len() == bounds.0 * bounds.1);

    for row in 0..bounds.1 {
        for column in 0..bounds.0 {
            let point = pixel_to_point(bounds, (column, row), upper_lef, lower_right);
            pixels[row * bounds.0 + columm] = match escape_time(point, 255) {
                None => 0,
                Some(count) => 255 - count as u8,
            };
        }
    }
}

/// Write the buffer `pixels`, whose dimensions are given by
/// `bounds`, to the file name `filename`.
fn write_image(
    filename: &str,
    pixels: &[u8],
    bounds: (usize, usize),
) -> Result<(), std::io::Error> {
    let output = File::create(filename)?;
    let encoder = PNGEncoder::new(output);
    encoder.encode(
        &pixels,
        bounds.0 as u32,
        bounds.1 as u32,
        ColorType::Gray(8),
    )?;
    Ok(())
}
