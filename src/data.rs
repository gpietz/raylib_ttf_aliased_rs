use crate::cli_args::CliArgs;

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
}

impl AppData {
    pub(crate) fn new() -> Self {
        Self {
            cli_args: CliArgs::new(),
            text_lines: vec![],
            font_size: 64
        }
    }
}
