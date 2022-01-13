//! QOI image decoder for embedded applications.

#![no_std]
#![deny(missing_docs)]

use embedded_graphics::{pixelcolor::Rgb888, prelude::*};
use nom::{
    bytes::complete::tag,
    number::complete::{be_u32, be_u8},
    IResult,
};

/// QOI image.
pub struct Qoi<'a> {
    data: &'a [u8],
    size: Size,
}

impl<'a> Qoi<'a> {
    /// Creates a new OOI image.
    pub fn new(data: &'a [u8]) -> Self {
        let (data, size) = parse_header(data).unwrap();
        Self { data, size }
    }

    /// Returns an iterator over this pixels in this image.
    pub fn pixels(&'a self) -> PixelsIter<'a> {
        PixelsIter::new(self)
    }
}

fn parse_header(input: &[u8]) -> IResult<&[u8], Size> {
    let (input, _) = tag("qoif")(input)?;
    let (input, width) = be_u32(input)?;
    let (input, height) = be_u32(input)?;
    let (input, _channels) = be_u8(input)?;
    let (input, _colorspace) = be_u8(input)?;

    Ok((input, Size::new(width, height)))
}

impl ImageDrawable for Qoi<'_> {
    type Color = Rgb888;

    fn draw<D>(&self, target: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        target.fill_contiguous(&self.bounding_box(), self.pixels())
    }

    fn draw_sub_image<D>(
        &self,
        target: &mut D,
        area: &embedded_graphics::primitives::Rectangle,
    ) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        self.draw(&mut target.translated(-area.top_left).clipped(area))
    }
}

impl OriginDimensions for Qoi<'_> {
    fn size(&self) -> Size {
        self.size
    }
}

fn hash_pixel(pixel: Rgb888, alpha: u8) -> u8 {
    pixel
        .r()
        .wrapping_mul(3)
        .wrapping_add(pixel.g().wrapping_mul(5))
        .wrapping_add(pixel.b().wrapping_mul(7))
        .wrapping_add(alpha.wrapping_mul(11))
        % 64
}

/// Iterator over the pixels of a QOI image.
pub struct PixelsIter<'a> {
    previous_color: Rgb888,
    previous_alpha: u8,
    previous_colors: [Rgb888; 64],
    previous_alphas: [u8; 64],
    data: &'a [u8],
    run_length: u8,
}

impl<'a> PixelsIter<'a> {
    fn new(qoi: &'a Qoi<'a>) -> Self {
        Self {
            previous_color: Rgb888::BLACK,
            previous_alpha: 255,
            previous_colors: [Rgb888::BLACK; 64],
            previous_alphas: [0; 64],
            data: qoi.data,
            run_length: 0,
        }
    }
}

impl Iterator for PixelsIter<'_> {
    type Item = Rgb888;

    fn next(&mut self) -> Option<Self::Item> {
        if self.run_length > 0 {
            self.run_length -= 1;
            return Some(self.previous_color);
        }

        let (byte, rest) = self.data.split_first()?;
        self.data = rest;

        match byte {
            0b11111110 => {
                // QOI_OP_RGB
                if self.data.len() >= 3 {
                    self.previous_color = Rgb888::new(self.data[0], self.data[1], self.data[2]);
                    self.data = &self.data[3..];
                } else {
                    return None;
                }
            }
            0b11111111 => {
                // QOI_OP_RGBA
                if self.data.len() >= 4 {
                    self.previous_color = Rgb888::new(self.data[0], self.data[1], self.data[2]);
                    self.previous_alpha = self.data[3];
                    self.data = &self.data[4..];
                } else {
                    return None;
                }
            }
            _ => match byte & 0b11000000 {
                0b00000000 => {
                    // QOI_OP_INDEX
                    let index = usize::from(byte & 0x3F);
                    self.previous_color = self.previous_colors[index];
                    self.previous_alpha = self.previous_alphas[index];
                    return Some(self.previous_color);
                }
                0b01000000 => {
                    // QOI_OP_DIFF
                    let dr = (byte >> 4) & 0x3;
                    let dg = (byte >> 2) & 0x3;
                    let db = byte & 0x3;

                    let r = self.previous_color.r().wrapping_add(dr).wrapping_sub(2);
                    let g = self.previous_color.g().wrapping_add(dg).wrapping_sub(2);
                    let b = self.previous_color.b().wrapping_add(db).wrapping_sub(2);

                    self.previous_color = Rgb888::new(r, g, b);
                }
                0b10000000 => {
                    // QOI_OP_LUMA
                    if self.data.len() >= 1 {
                        let byte2 = self.data[0];
                        self.data = &self.data[1..];

                        let dg = (byte & 0x3F).wrapping_sub(32);
                        let dr = (byte2 >> 4).wrapping_sub(8).wrapping_add(dg);
                        let db = (byte2 & 0x0F).wrapping_sub(8).wrapping_add(dg);

                        let r = self.previous_color.r().wrapping_add(dr);
                        let g = self.previous_color.g().wrapping_add(dg);
                        let b = self.previous_color.b().wrapping_add(db);

                        self.previous_color = Rgb888::new(r, g, b);
                    } else {
                        return None;
                    }
                }
                0b11000000 | _ => {
                    // QOI_OP_RUN
                    self.run_length = byte & 0x3F;
                    return Some(self.previous_color);
                }
            },
        }

        let index = usize::from(hash_pixel(self.previous_color, self.previous_alpha));
        self.previous_colors[index] = self.previous_color;
        self.previous_alphas[index] = self.previous_alpha;
        Some(self.previous_color)
    }
}