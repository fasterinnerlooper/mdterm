use crate::font::FontRenderer;
use crate::image::ImageProcessor;
use crate::terminal::TerminalRenderer;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum ElementType {
    Heading(u8),
    Paragraph,
    CodeBlock,
    Image,
    Text,
    Blockquote,
    List,
    ListItem,
}

#[derive(Debug)]
pub struct MarkdownElement {
    pub element_type: ElementType,
    pub content: String,
    pub children: Vec<MarkdownElement>,
    pub attributes: HashMap<String, String>,
}

pub fn process_markdown(content: &str, width: usize) -> String {
    // Parse markdown into elements
    let elements = parse_markdown(content);

    // Render elements to terminal
    let mut renderer = TerminalRenderer::new(width);
    let mut image_processor = ImageProcessor::new();

    // Load embedded font for pixel rendering
    let font_data = include_bytes!("../assets/DejaVuSans.ttf");
    let mut font_renderer = FontRenderer::new(font_data);

    let mut result = String::new();

    for element in elements {
        match element.element_type {
            ElementType::Heading(level) => {
                let heading_text = format!("{} {}", "#".repeat(level as usize), element.content);
                result.push_str(&renderer.render_text(&heading_text, &mut font_renderer));
            }
            ElementType::Paragraph => {
                result.push_str(&renderer.render_text(&element.content, &mut font_renderer));
            }
            ElementType::CodeBlock => {
                result.push_str(&renderer.render_code_block(&element.content, &mut font_renderer));
            }
            ElementType::Image => {
                if let Some(src) = element.attributes.get("src") {
                    // Process image with graphics protocols
                    let image_output = image_processor.process_image(src);
                    result.push_str(&image_output);
                }
            }
            ElementType::Text => {
                result.push_str(&renderer.render_text(&element.content, &mut font_renderer));
            }
            ElementType::Blockquote => {
                result.push_str(&renderer.render_blockquote(&element.content, &mut font_renderer));
            }
            ElementType::List => {
                result.push_str(&renderer.render_list(&element.children, &mut font_renderer));
            }
            ElementType::ListItem => {
                result.push_str(&renderer.render_list_item(&element.content, &mut font_renderer));
            }
        }
        result.push('\n');
    }

    result
}

fn parse_markdown(content: &str) -> Vec<MarkdownElement> {
    // Simple markdown parser for demonstration
    let mut elements = Vec::new();
    let mut current_content = String::new();
    let mut current_type = ElementType::Paragraph;

    let flush = |elements: &mut Vec<MarkdownElement>, content: &mut String, etype: &ElementType| {
        if !content.is_empty() {
            elements.push(MarkdownElement {
                element_type: etype.clone(),
                content: content.clone(),
                children: Vec::new(),
                attributes: HashMap::new(),
            });
            content.clear();
        }
    };

    for line in content.lines() {
        if line.starts_with("# ") {
            flush(&mut elements, &mut current_content, &current_type);
            elements.push(MarkdownElement {
                element_type: ElementType::Heading(1),
                content: line[2..].to_string(),
                children: Vec::new(),
                attributes: HashMap::new(),
            });
            current_type = ElementType::Paragraph;
        } else if line.starts_with("## ") {
            flush(&mut elements, &mut current_content, &current_type);
            elements.push(MarkdownElement {
                element_type: ElementType::Heading(2),
                content: line[3..].to_string(),
                children: Vec::new(),
                attributes: HashMap::new(),
            });
            current_type = ElementType::Paragraph;
        } else if line.starts_with("![](") && line.ends_with(')') {
            flush(&mut elements, &mut current_content, &current_type);
            let src = line[4..line.len() - 1].to_string();
            elements.push(MarkdownElement {
                element_type: ElementType::Image,
                content: String::new(),
                children: Vec::new(),
                attributes: vec![("src".to_string(), src)].into_iter().collect(),
            });
            current_type = ElementType::Paragraph;
        } else if line.trim().is_empty() {
            flush(&mut elements, &mut current_content, &current_type);
            current_type = ElementType::Paragraph;
        } else {
            if current_type == ElementType::Paragraph {
                if !current_content.is_empty() {
                    current_content.push('\n');
                }
                current_content.push_str(line);
            } else {
                flush(&mut elements, &mut current_content, &current_type);
                current_content.push_str(line);
                current_type = ElementType::Paragraph;
            }
        }
    }

    // Push the last element if it's not empty
    flush(&mut elements, &mut current_content, &current_type);

    elements
}
