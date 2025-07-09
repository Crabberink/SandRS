extern crate js_sys;

mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, sand-rs!");
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PixelBehaviour {
    Dead,
    Powder,
    Liquid,
    Gas,
    Solid,
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Pixel {
    r: u8,
    g: u8,
    b: u8,
    behaviour: PixelBehaviour,
}

#[wasm_bindgen]
impl Pixel {
    pub fn empty() -> Pixel {
        Pixel {
            r: 0,
            g: 0,
            b: 0,
            behaviour: PixelBehaviour::Dead
        }
    }

    pub fn sand() -> Pixel {
        let col_y = (js_sys::Math::random() * 45.0) as u8 + 210;
        Pixel {
            r: col_y,
            g: col_y,
            b: 180,
            behaviour: PixelBehaviour::Powder
        }
    }

    pub fn water() -> Pixel {
        let col_c = (js_sys::Math::random() * 15.0) as u8 - 10;
        Pixel {
            r: 0,
            g: 100+col_c,
            b: 200+col_c,
            behaviour: PixelBehaviour::Liquid
        }
    }
}

#[wasm_bindgen]
pub struct World {
    width: usize,
    height: usize,
    pixels: Vec<Pixel>,
    texture_buffer: Vec<u8>,
    updated_pixels: Vec<bool>,
    to_update: Vec<usize>
}

impl World {
    fn get_xy(&self, index: usize) -> (usize, usize) {
        let x = index % self.width;
        let y = index / self.width;
        (x,y)
    }

    fn get_index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    fn index_get_pixel_offset(&self, index: usize, off_x: isize, off_y: isize) -> Option<Pixel> {
        let (x, y ) = self.get_xy(index);
        let (i_x, i_y) = (x as isize + off_x, y as isize + off_y);
        if i_x < 0 || i_x  >= self.width as isize {
            return Option::None;
        }
        if i_y < 0 || i_y >= self.height as isize {
            return Option::None;
        }

        let new_index = self.get_index(i_x as usize, i_y as usize);
        Some(self.pixels[new_index])
    }

    fn index_set_pixel_offset(&mut self, index: usize, off_x: isize, off_y: isize, pixel: Pixel) {
        let (x, y ) = self.get_xy(index);
        let (i_x, i_y) = (x as isize + off_x, y as isize + off_y);
        if i_x < 0 || i_x  >= self.width as isize {
            return;
        }
        if i_y < 0 || i_y >= self.height as isize {
            return;
        }

        let new_index = self.get_index(i_x as usize, i_y as usize);
        self.updated_pixels[new_index] = true;
        self.pixels[new_index] = pixel;
    }

}

#[wasm_bindgen]
impl World {
    pub fn new(width: usize, height: usize) -> World {
        let mut pixels = vec![Pixel::empty(); width * height];

        for x in 0..width {
            for y in 0..height {
                pixels[y * width + x] = Pixel::sand();
                if x > 100 && x < 200 && y > 200 && y < 300 {
                }
            }
        }

        let texture_buffer: Vec<u8> = vec![0; width * height * 4];

        World {
            width: width,
            height: height,
            pixels: pixels,
            texture_buffer: texture_buffer,
            updated_pixels: vec![false; width * height],
            to_update: (0..width*height).collect()
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, pixel: Pixel) {
        // if x >= self.width || y >= self.width {
        //     return;
        // }

        let index = self.get_index(x, y);
        self.pixels[index] = pixel;
    }

    pub fn generate_texture(&mut self) {
        for i in 0..(self.width * self.height) {
            self.texture_buffer[i*4 + 0] = self.pixels[i].r;
            self.texture_buffer[i*4 + 1] = self.pixels[i].g;
            self.texture_buffer[i*4 + 2] = self.pixels[i].b;
            self.texture_buffer[i*4 + 3] = 255;
        }
    }

    pub fn texture_buffer(&self) -> *const u8 {
        self.texture_buffer.as_ptr()
    }

    pub fn update(&mut self) {
        fastrand::shuffle(&mut self.to_update);

        for i in 1..(self.width * self.height) {
            let index = self.to_update[i];
            if self.updated_pixels[index] { self.pixels[index].b = 255; self.pixels[index].r = 0; continue; }
            self.updated_pixels[index] = true;
            match self.pixels[index].behaviour {
                PixelBehaviour::Powder => {
                    let maybe_bottom = self.index_get_pixel_offset(index, 0, 1);
                    if let Some(bottom) = maybe_bottom {
                        if bottom.behaviour == PixelBehaviour::Dead {
                            self.index_set_pixel_offset(index, 0, 1, self.pixels[index]);
                            self.pixels[index] = Pixel::empty();
                            continue;
                        }
                    } else { continue; }

                    match fastrand::bool() {
                        false => {
                            let maybe_bottom_right = self.index_get_pixel_offset(index, 1, 1);
                            if let Some(bottom_right) = maybe_bottom_right {
                                if bottom_right.behaviour == PixelBehaviour::Dead {
                                    self.index_set_pixel_offset(index, 1, 1, self.pixels[index]);
                                    self.pixels[index] = Pixel::empty();
                                    continue;
                                }
                            }

                            let maybe_bottom_left = self.index_get_pixel_offset(index, -1, 1);
                            if let Some(bottom_left) = maybe_bottom_left {
                                if bottom_left.behaviour == PixelBehaviour::Dead {
                                    self.index_set_pixel_offset(index, -1, 1, self.pixels[index]);
                                    self.pixels[index] = Pixel::empty();
                                    continue;
                                }
                            }
                        },
                        true => {
                            let maybe_bottom_left = self.index_get_pixel_offset(index, -1, 1);
                            if let Some(bottom_left) = maybe_bottom_left {
                                if bottom_left.behaviour == PixelBehaviour::Dead {
                                    self.index_set_pixel_offset(index, -1, 1, self.pixels[index]);
                                    self.pixels[index] = Pixel::empty();
                                    continue;
                                }
                            }

                            let maybe_bottom_right = self.index_get_pixel_offset(index, 1, 1);
                            if let Some(bottom_right) = maybe_bottom_right {
                                if bottom_right.behaviour == PixelBehaviour::Dead {
                                    self.index_set_pixel_offset(index, 1, 1, self.pixels[index]);
                                    self.pixels[index] = Pixel::empty();
                                    continue;
                                }
                            }
                        }
                    }
                },
                PixelBehaviour::Liquid => {
                    let maybe_bottom = self.index_get_pixel_offset(index, 0, 1);
                    if let Some(bottom) = maybe_bottom {
                        if bottom.behaviour == PixelBehaviour::Dead {
                            self.index_set_pixel_offset(index, 0, 1, self.pixels[index]);
                            self.pixels[index] = Pixel::empty();
                            continue;
                        }
                    }

                    match fastrand::bool() {
                        false => {
                            let maybe_bottom_right = self.index_get_pixel_offset(index, 1, 1);
                            if let Some(bottom_right) = maybe_bottom_right {
                                if bottom_right.behaviour == PixelBehaviour::Dead {
                                    self.index_set_pixel_offset(index, 1, 1, self.pixels[index]);
                                    self.pixels[index] = Pixel::empty();
                                    continue;
                                }
                            }

                            let maybe_bottom_left = self.index_get_pixel_offset(index, -1, 1);
                            if let Some(bottom_left) = maybe_bottom_left {
                                if bottom_left.behaviour == PixelBehaviour::Dead {
                                    self.index_set_pixel_offset(index, -1, 1, self.pixels[index]);
                                    self.pixels[index] = Pixel::empty();
                                    continue;
                                }
                            }

                            let maybe_right = self.index_get_pixel_offset(index, 1, 0);
                            if let Some(right) = maybe_right {
                                if right.behaviour == PixelBehaviour::Dead {
                                    self.index_set_pixel_offset(index, 1, 0, self.pixels[index]);
                                    self.pixels[index] = Pixel::empty();
                                    continue;
                                }
                            }

                            let maybe_left = self.index_get_pixel_offset(index, -1, 0);
                            if let Some(left) = maybe_left {
                                if left.behaviour == PixelBehaviour::Dead {
                                    self.index_set_pixel_offset(index, -1, 0, self.pixels[index]);
                                    self.pixels[index] = Pixel::empty();
                                    continue;
                                }
                            }
                        },
                        true => {
                            let maybe_bottom_left = self.index_get_pixel_offset(index, -1, 1);
                            if let Some(bottom_left) = maybe_bottom_left {
                                if bottom_left.behaviour == PixelBehaviour::Dead {
                                    self.index_set_pixel_offset(index, -1, 1, self.pixels[index]);
                                    self.pixels[index] = Pixel::empty();
                                    continue;
                                }
                            }

                            let maybe_bottom_right = self.index_get_pixel_offset(index, 1, 1);
                            if let Some(bottom_right) = maybe_bottom_right {
                                if bottom_right.behaviour == PixelBehaviour::Dead {
                                    self.index_set_pixel_offset(index, 1, 1, self.pixels[index]);
                                    self.pixels[index] = Pixel::empty();
                                    continue;
                                }
                            }
                        }
                    }
                }
                _ => { }
            }
        }

        for xo in -15..15 {
            for yo in -15..15 {
                self.index_set_pixel_offset(400 * 100 + 200, xo, yo, Pixel::sand());
            }
        }

        self.updated_pixels.fill(false);
    }
}

#[wasm_bindgen]
pub fn wasm_memory() -> wasm_bindgen::JsValue {
    wasm_bindgen::memory()
}