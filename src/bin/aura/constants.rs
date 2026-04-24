use pulldown_cmark::HeadingLevel;

pub const STYLE: &str = r#"
    window {
        background-color: transparent;
    }
    .main-window-box {
        background-color: @theme_bg_color;
        border-radius: 12px;
        box-shadow: 0 5px 15px rgba(0, 0, 0, 0.3);
        margin: 10px;
    }
    entry {
        background: transparent;
        border: none;
        box-shadow: none;
        outline: none;
        font-size: 20px;
        padding: 10px;
        color: @theme_fg_color;
    }
    entry:focus {
        background: transparent;
        box-shadow: none;
        outline: none;
    }
    label {
        font-size: 16px;
        color: @theme_fg_color;
        margin-bottom: 5px;
    }
    scrolledwindow {
        min-height: 0px;
    }
"#;

pub const THINKING_MARKUP: &str = "<span alpha='50%'><i>Thinking...</i></span>";
pub const BULLET: &str = "• ";
pub const MONOSPACE_FAMILY: &str = "monospace";

pub fn heading_size(level: HeadingLevel) -> &'static str {
    match level {
        HeadingLevel::H1 => "160%",
        HeadingLevel::H2 => "140%",
        HeadingLevel::H3 => "120%",
        _ => "110%",
    }
}
