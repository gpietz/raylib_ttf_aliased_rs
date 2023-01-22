use std::fs::read_to_string;
use raylib::color::Color;
use regex::internal::Input;
use structopt::StructOpt;
use crate::cli_args::CliArgs;
use crate::file_utils::FileUtils;
use crate::raylib_utils::string_to_color;
use crate::string_utils::remove_first_last_char;

const MSG: &str = "Lorem ipsum dolor sit amet,
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

pub struct TextLine {
    pub line: String,
    pub line_height: f32,
    pub line_width: f32,
    pub line_offset: Option<f32>,
}

impl TextLine {
    pub(crate) fn new(line: &str) -> Self {
        Self {
            line: line.into(),
            line_height: 0.0,
            line_width: 0.0,
            line_offset: None,
        }
    }
}

pub type TextLineVector = Vec<TextLine>;

pub struct AppData {
    pub cli_args: CliArgs,
    pub text_lines: TextLineVector,
    pub font_size: i32,
    pub bgcolor: Color,
    pub fgcolor: Color,
}

impl AppData {
    pub(crate) fn new() -> Self {
        let cli_args = CliArgs::from_args();

        //-- textfile
        let mut text_lines = vec![];
        if let Some(filename) = &cli_args.text_file {
            if !filename.exists() {
                eprintln!("File not found: {:?}", filename.get_path_str());
            } else {
                println!("Loading text file: {:?}", filename.get_path_str());
                let file_read_result = read_to_string(filename.clone());
                if let Ok(content) = file_read_result {
                    for line in AppData::split_text_by_newlines(content.as_str()) {
                        text_lines.push(line);
                    }
                } else {
                    eprintln!("Error loading text file: {:?}", filename.get_path_str())
                }
            }
        }
        if text_lines.len() == 0 {
            for line in AppData::split_text_by_newlines(MSG) {
                text_lines.push(line);
            }
        }

        //-- font size
        let mut font_size = 64;
        if let Some(size) = cli_args.font_size {
            if size > 0 && size < 180 {
                font_size = size;
            }
        }

        //-- background color
        let mut background_color = Color::GREEN;
        if let Some(str_color) = &cli_args.bgcolor {
            if let Some(color) = string_to_color(str_color.as_str()) {
                background_color = color;
                println!("Using color \"{}\" as background color.", str_color);
            } else {
                eprintln!("Invalid color format \"{}\" in background color!", str_color);
            }
        }

        //-- foreground color
        let mut foreground_color = Color::WHITE;
        if let Some(str_color) = &cli_args.fgcolor {
            if let Some(color) = string_to_color(str_color.as_str()) {
                foreground_color = color;
                println!("Using color \"{}\" as foreground color.", str_color);
            } else {
                eprintln!("Invalid color format \"{}\" in foreground color!", str_color);
            }
        }

        Self {
            cli_args,
            text_lines,
            font_size,
            bgcolor: background_color,
            fgcolor: foreground_color,
        }
    }

    fn split_text_by_newlines(text: &str) -> TextLineVector {
        let mut lines = Vec::new();
        for l in text.split("\n") {
            lines.push(TextLine::new(l.trim()));
        }
        lines
    }

}
