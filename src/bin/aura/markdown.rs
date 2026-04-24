use pulldown_cmark::{Event, Parser, Tag, TagEnd};
use std::fmt::Write;
use crate::constants::{heading_size, BULLET, MONOSPACE_FAMILY};

pub fn markdown_to_pango(markdown: &str) -> String {
    let parser = Parser::new(markdown);
    let mut output = String::new();

    for event in parser {
        match event {
            Event::Start(tag) => match tag {
                Tag::Paragraph => {}
                Tag::Heading { level, .. } => {
                    let size = heading_size(level);
                    let _ = write!(output, "\n<span weight='bold' size='{}'>", size);
                }
                Tag::Strong => output.push_str("<b>"),
                Tag::Emphasis => output.push_str("<i>"),
                Tag::CodeBlock(_) => {
                    let _ = write!(output, "\n<span font_family='{}'>", MONOSPACE_FAMILY);
                }
                Tag::List(_) => {
                    output.push_str("\n");
                }
                Tag::Item => output.push_str(BULLET),
                Tag::Link { dest_url, .. } => {
                    let _ = write!(output, "<a href='{}'>", escape_pango(&dest_url));
                }
                _ => {}
            },
            Event::End(tag_end) => match tag_end {
                TagEnd::Paragraph => output.push_str("\n\n"),
                TagEnd::Heading(_) => output.push_str("</span>\n"),
                TagEnd::Strong => output.push_str("</b>"),
                TagEnd::Emphasis => output.push_str("</i>"),
                TagEnd::CodeBlock => output.push_str("</span>\n\n"),
                TagEnd::List(_) => output.push_str("\n"),
                TagEnd::Item => output.push_str("\n"),
                TagEnd::Link => output.push_str("</a>"),
                _ => {}
            },
            Event::Text(text) => {
                output.push_str(&escape_pango(&text));
            }
            Event::Code(code) => {
                output.push_str("<tt>");
                output.push_str(&escape_pango(&code));
                output.push_str("</tt>");
            }
            Event::SoftBreak => output.push(' '),
            Event::HardBreak => output.push('\n'),
            Event::Html(html) | Event::InlineHtml(html) => {
                output.push_str(&escape_pango(&html));
            }
            _ => {}
        }
    }

    output.trim().to_string()
}

fn escape_pango(input: &str) -> String {
    input.replace("&", "&amp;")
         .replace("<", "&lt;")
         .replace(">", "&gt;")
         .replace("\'", "&apos;")
         .replace("\"", "&quot;")
}
