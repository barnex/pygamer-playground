use super::types::*;

use embedded_graphics as eg;

pub struct FrameBuffer {
    pub inner: [Rgb565; SCREEN_W * SCREEN_H],
}

impl FrameBuffer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set(&mut self, x: i32, y: i32, col: Rgb565) {
        if x >= 0 && x < ISCREEN_W && y >= 0 && y < ISCREEN_H {
            let x = x as usize;
            let y = y as usize;
            self.inner[y * SCREEN_W + x] = col;
        }
    }

    //pub fn iter_pixels(&self) -> impl Iterator<Item = Pixel<Rgb565>> + '_ {
    //    self.inner.iter().enumerate().flat_map(|(y, row)| {
    //        row.iter().enumerate().map(move |(x, v)| {
    //            Pixel(
    //                Point {
    //                    x: x as i32,
    //                    y: y as i32,
    //                },
    //                *v,
    //            )
    //        })
    //    })
    //}
}

impl Default for FrameBuffer {
    fn default() -> Self {
        Self {
            inner: [Default::default(); SCREEN_W * SCREEN_H],
        }
    }
}

impl eg::geometry::Dimensions for FrameBuffer {
    fn bounding_box(&self) -> Rectangle {
        Rectangle {
            top_left: Point { x: 0, y: 0 },
            size: Size {
                width: SCREEN_W as u32,
                height: SCREEN_H as u32,
            },
        }
    }
}

impl eg::draw_target::DrawTarget for FrameBuffer {
    type Color = Rgb565;

    type Error = ();

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(point, color) in pixels {
            self.set(point.x, point.y, color)
        }

        Ok(())
    }

    fn clear(&mut self, color: Self::Color) -> Result<(), Self::Error> {
        for c in self.inner.iter_mut() {
            *c = color
        }
        Ok(())
    }
}
