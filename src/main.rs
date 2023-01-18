use std::borrow::BorrowMut;

use raylib::color::Color;
use raylib::drawing::RaylibDrawHandle;
use raylib::ffi::Texture;
use raylib::misc::AsF32;
use raylib::prelude::{Font, RaylibDraw, Vector2};
use raylib::text::{FontLoadEx, measure_text_ex};

const SCREEN_WIDTH: i32 = 1600;
const SCREEN_HEIGHT: i32 = 1200;
const FONT_SIZE: i32 = 84;
const MOVE_SPEED: f32 = 1.0;

struct TextLine {
    pub line: String,
    pub line_height: f32,
    pub line_width: f32,
    pub line_offset: Option<f32>,
}

impl TextLine {
    fn new(line: &str) -> Self {
        Self {
            line: line.into(),
            line_height: 0.0,
            line_width: 0.0,
            line_offset: None,
        }
    }
}

type TextLineVector = Vec<TextLine>;

fn split_text_by_newlines(text: &str) -> TextLineVector {
    let mut lines = Vec::new();
    for l in text.split("\n") {
        lines.push(TextLine::new(l.trim()));
    }
    lines
}

fn get_text_measure(text: &str, font: &Font) -> Vector2 {
    measure_text_ex(font, text, FONT_SIZE.as_f32(), 0.0)
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

fn main() {
    // Initialize window
    let (mut raylib_handle, raylib_thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("raylib aliased font scroll example rust")
        .build();
    raylib_handle.set_target_fps(60);

    // Load font
    let mut font = raylib_handle.load_font_ex(
        &raylib_thread, "assets/Trueno-wml2.otf".into(), FONT_SIZE, FontLoadEx::Default(0),
    ).expect("Failed to load font");
    generate_font_mipmaps(&mut font);

    // Convert text to vector of strings/textlines
    let msg: &str = "Lorem ipsum dolor sit amet,
                     consetetur sadipscing elitr,
                     sed diam nonumy eirmod tempor
                     invidunt ut labore et dolore
                     magna aliquyam erat, sed diam
                     voluptua. At vero eos et accusam
                     et justo duo dolores et ea rebum.
                     Stet clita kasd gubergren, no sea
                     takimata sanctus est Lorem ipsum
                     dolor sit amet. Lorem ipsum dolor
                     sit amet, consetetur sadipscing elitr,
                     sed diam nonumy eirmod tempor invidunt
                     ut labore et dolore magna aliquyam erat,
                     sed diam voluptua. At vero eos et
                     accusam et justo duo dolores et ea
                     rebum. Stet clita kasd gubergren, no
                     sea takimata sanctus est Lorem ipsum
                     dolor sit amet.
                     ****";
    let mut text_lines = split_text_by_newlines(msg);

    // get text dimension of each line
    for line in &mut text_lines {
        let measure = get_text_measure(&line.line, &font);
        line.line_width = measure.x;
        line.line_height = measure.y;
    }

    // Tada.... the mainloop
    while !raylib_handle.window_should_close() {
        let mut draw_handle = raylib_handle.begin_drawing(&raylib_thread);
        draw_handle.clear_background(Color::BLACK);
        render(&mut draw_handle, &mut text_lines, &font);
    }
}
