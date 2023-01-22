extern crate core;

mod cli_args;
mod data;
mod file_utils;
mod raylib_utils;
mod string_utils;

use std::borrow::BorrowMut;
use std::fs::read_to_string;
use std::path::PathBuf;

use raylib::color::Color;
use raylib::drawing::RaylibDrawHandle;
use raylib::misc::AsF32;
use raylib::prelude::{Font, RaylibDraw, Vector2};
use raylib::text::{FontLoadEx, measure_text_ex};
use regex::Regex;
use lazy_static::lazy_static;
use crate::data::{AppData, TextLine, TextLineVector};

const SCREEN_WIDTH: i32 = 1024;
const SCREEN_HEIGHT: i32 = 768;
const MOVE_SPEED: f32 = 1.0;



fn get_text_measure(text: &str, font: &Font, font_size: f32) -> Vector2 {
    measure_text_ex(font, text, font_size, 0.0)
}

fn generate_font_mipmaps(font: &mut Font) {
    unsafe {
        let texture_ptr = font.texture.borrow_mut();
        raylib::ffi::GenTextureMipmaps(texture_ptr);
    }
}

fn render(draw_handle: &mut RaylibDrawHandle, text_lines: &mut TextLineVector, font: &Font) {
    // Get real width and height of screen
    let screen_width = draw_handle.get_screen_width();
    let screen_height = draw_handle.get_screen_height();

    let mut text_position = Vector2::new(0.0, 0.0);

    // Render scrolling text and update position
    let mut prev_line_offset = 0.0;
    let mut prev_line_height = 0.0;
    for tl in text_lines.iter_mut() {
        let mut render_line = true;

        // Update offset for the text lines
        if tl.line_offset.is_none() {
            let offset = if prev_line_offset > 0.0 {
                prev_line_offset + prev_line_height
            } else {
                screen_height.as_f32()
            };
            tl.line_offset = Some(offset.as_f32());
        } else if tl.line_offset.unwrap() <= -(tl.line_height) {
            render_line = false;
        } else {
            let offset = tl.line_offset.unwrap();
            tl.line_offset = Some(offset - MOVE_SPEED);
        }

        if render_line {
            // Calculate horizontal position for line
            text_position.x = screen_width.as_f32() / 2.0 - tl.line_width / 2.0;
            text_position.y = tl.line_offset.unwrap();
            draw_handle.draw_text_ex(&font, tl.line.as_str(), text_position,
                                     font.baseSize.as_f32(), 0.0, Color::WHITE);
        }

        prev_line_offset = tl.line_offset.unwrap();
        prev_line_height = tl.line_height;
    }

    // Reset scrolling after it has been completely shown
    let last_line = &text_lines[text_lines.len() - 1];
    if last_line.line_offset.is_some() {
        let offset_value = last_line.line_offset.unwrap();
        if offset_value <= -(last_line.line_height) {
            for tl in text_lines.iter_mut() {
                tl.line_offset = None
            }
        }
    }
}

/// Creates a raylib color from string.
fn get_color_from_str(str: &str) -> Option<&str> {
    None
}

fn main() {
    // Process command line args
    let mut app_data = AppData::new();

    // Initialize window
    let (mut raylib_handle, raylib_thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("raylib
        } aliased font scroll example rust")
        .build();
    raylib_handle.set_target_fps(60);

    // Load font
    let mut font = raylib_handle.load_font_ex(
        &raylib_thread, "assets/Trueno-wml2.otf".into(), app_data.font_size, FontLoadEx::Default(0)
    ).expect("Failed to load font");
    generate_font_mipmaps(&mut font);

    // get text dimension of each line
    let font_size = app_data.font_size.as_f32();
    for line in &mut app_data.text_lines {
        let measure = get_text_measure(&line.line, &font, font_size);
        line.line_width = measure.x;
        line.line_height = measure.y;
    }

    // Tada.... the mainloop
    while !raylib_handle.window_should_close() {
        let mut draw_handle = raylib_handle.begin_drawing(&raylib_thread);
        draw_handle.clear_background(app_data.bgcolor);
        render(&mut draw_handle, &mut app_data.text_lines, &font);
    }
}
