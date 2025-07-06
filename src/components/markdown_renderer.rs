#![allow(non_snake_case)]
use dioxus::prelude::*;
use crate::components::TextWithIcons;

#[derive(Clone, Copy, PartialEq)]
pub enum MarkdownElement {
    Paragraph,
    Header1,
    Header2,
    Header3,
    BulletList,
    NumberedList,
    CodeBlock,
}

/// Parses a line of markdown and returns the element type and content
fn parse_line(line: &str) -> (MarkdownElement, String) {
    let trimmed = line.trim();
    
    if trimmed.starts_with("### ") {
        (MarkdownElement::Header3, trimmed[4..].to_string())
    } else if trimmed.starts_with("## ") {
        (MarkdownElement::Header2, trimmed[3..].to_string())
    } else if trimmed.starts_with("# ") {
        (MarkdownElement::Header1, trimmed[2..].to_string())
    } else if trimmed.starts_with("- ") || trimmed.starts_with("* ") {
        (MarkdownElement::BulletList, trimmed[2..].to_string())
    } else if trimmed.len() > 0 && trimmed.chars().nth(0).unwrap().is_numeric() && trimmed.contains(". ") {
        let dot_pos = trimmed.find(". ").unwrap();
        (MarkdownElement::NumberedList, trimmed[dot_pos + 2..].to_string())
    } else if trimmed.starts_with("```") {
        (MarkdownElement::CodeBlock, String::new())
    } else {
        (MarkdownElement::Paragraph, trimmed.to_string())
    }
}

/// Parses markdown text for inline formatting (bold, italic, code)
fn parse_inline_formatting(text: &str) -> Element {
    let mut result = vec![];
    let mut current = String::new();
    let mut chars = text.chars().peekable();
    let mut i = 0;
    
    while let Some(ch) = chars.next() {
        if ch == '*' && chars.peek() == Some(&'*') {
            // Bold text
            chars.next(); // consume second *
            
            // Add any accumulated text
            if !current.is_empty() {
                let text = current.clone();
                result.push(rsx! {
                    TextWithIcons { 
                        key: "{i}",
                        text: text 
                    }
                });
                i += 1;
                current.clear();
            }
            
            // Find closing **
            let mut bold_text = String::new();
            let mut found_closing = false;
            
            while let Some(ch) = chars.next() {
                if ch == '*' && chars.peek() == Some(&'*') {
                    chars.next(); // consume second *
                    found_closing = true;
                    break;
                }
                bold_text.push(ch);
            }
            
            if found_closing && !bold_text.is_empty() {
                let text = bold_text.clone();
                result.push(rsx! {
                    strong {
                        key: "{i}",
                        TextWithIcons { text: text }
                    }
                });
                i += 1;
            }
        } else if ch == '*' {
            // Italic text
            // Add any accumulated text
            if !current.is_empty() {
                let text = current.clone();
                result.push(rsx! {
                    TextWithIcons { 
                        key: "{i}",
                        text: text 
                    }
                });
                i += 1;
                current.clear();
            }
            
            // Find closing *
            let mut italic_text = String::new();
            let mut found_closing = false;
            
            while let Some(ch) = chars.next() {
                if ch == '*' {
                    found_closing = true;
                    break;
                }
                italic_text.push(ch);
            }
            
            if found_closing && !italic_text.is_empty() {
                let text = italic_text.clone();
                result.push(rsx! {
                    em {
                        key: "{i}",
                        TextWithIcons { text: text }
                    }
                });
                i += 1;
            }
        } else if ch == '`' {
            // Code text
            // Add any accumulated text
            if !current.is_empty() {
                let text = current.clone();
                result.push(rsx! {
                    TextWithIcons { 
                        key: "{i}",
                        text: text 
                    }
                });
                i += 1;
                current.clear();
            }
            
            // Find closing `
            let mut code_text = String::new();
            let mut found_closing = false;
            
            while let Some(ch) = chars.next() {
                if ch == '`' {
                    found_closing = true;
                    break;
                }
                code_text.push(ch);
            }
            
            if found_closing && !code_text.is_empty() {
                let text = code_text.clone();
                result.push(rsx! {
                    code {
                        key: "{i}",
                        style: "background: var(--color-bg-tertiary); padding: 2px 4px; border-radius: 3px; font-family: monospace; font-size: 0.9em;",
                        "{text}"
                    }
                });
                i += 1;
            }
        } else {
            current.push(ch);
        }
    }
    
    // Add any remaining text
    if !current.is_empty() {
        let text = current.clone();
        result.push(rsx! {
            TextWithIcons { 
                key: "{i}",
                text: text 
            }
        });
    }
    
    rsx! {
        span {
            style: "display: inline-flex; align-items: center; gap: 4px; flex-wrap: wrap;",
            {result.into_iter()}
        }
    }
}

#[component]
pub fn MarkdownRenderer(content: String) -> Element {
    let lines: Vec<&str> = content.lines().collect();
    let mut elements = vec![];
    let mut i = 0;
    let mut in_code_block = false;
    let mut code_block_content = String::new();
    
    // Group consecutive list items
    let mut current_list_items: Vec<String> = vec![];
    let mut current_list_type: Option<MarkdownElement> = None;
    
    for line in lines {
        let (element_type, content) = parse_line(line);
        
        if in_code_block {
            if line.trim() == "```" {
                // End code block
                in_code_block = false;
                let code = code_block_content.clone();
                elements.push(rsx! {
                    pre {
                        key: "{i}",
                        style: "background: var(--color-bg-tertiary); padding: 12px; border-radius: 6px; overflow-x: auto; margin: 12px 0;",
                        code {
                            style: "font-family: monospace; font-size: 0.9em; color: var(--color-text-primary);",
                            "{code}"
                        }
                    }
                });
                i += 1;
                code_block_content.clear();
            } else {
                code_block_content.push_str(line);
                code_block_content.push('\n');
            }
            continue;
        }
        
        if element_type == MarkdownElement::CodeBlock {
            in_code_block = true;
            continue;
        }
        
        // Handle list items
        if matches!(element_type, MarkdownElement::BulletList | MarkdownElement::NumberedList) {
            if current_list_type.is_none() || current_list_type == Some(element_type) {
                current_list_type = Some(element_type);
                current_list_items.push(content);
            } else {
                // Different list type, flush current list
                if !current_list_items.is_empty() {
                    let items = current_list_items.clone();
                    let list_type = current_list_type.unwrap();
                    elements.push(render_list(items, list_type, i));
                    i += 1;
                    current_list_items.clear();
                }
                current_list_type = Some(element_type);
                current_list_items.push(content);
            }
        } else {
            // Not a list item, flush any pending list
            if !current_list_items.is_empty() {
                let items = current_list_items.clone();
                let list_type = current_list_type.unwrap();
                elements.push(render_list(items, list_type, i));
                i += 1;
                current_list_items.clear();
                current_list_type = None;
            }
            
            // Render non-list element
            if !content.is_empty() || element_type == MarkdownElement::Paragraph {
                match element_type {
                    MarkdownElement::Header1 => {
                        elements.push(rsx! {
                            h1 {
                                key: "{i}",
                                style: "margin: 16px 0 8px 0; font-size: 24px; font-weight: 600; color: var(--color-text-primary);",
                                {parse_inline_formatting(&content)}
                            }
                        });
                    }
                    MarkdownElement::Header2 => {
                        elements.push(rsx! {
                            h2 {
                                key: "{i}",
                                style: "margin: 16px 0 8px 0; font-size: 20px; font-weight: 600; color: var(--color-text-primary);",
                                {parse_inline_formatting(&content)}
                            }
                        });
                    }
                    MarkdownElement::Header3 => {
                        elements.push(rsx! {
                            h3 {
                                key: "{i}",
                                style: "margin: 12px 0 6px 0; font-size: 16px; font-weight: 600; color: var(--color-text-primary);",
                                {parse_inline_formatting(&content)}
                            }
                        });
                    }
                    MarkdownElement::Paragraph => {
                        if !content.is_empty() {
                            elements.push(rsx! {
                                p {
                                    key: "{i}",
                                    style: "margin: 8px 0; line-height: 1.6; color: var(--color-text-secondary);",
                                    {parse_inline_formatting(&content)}
                                }
                            });
                        }
                    }
                    _ => {}
                }
                i += 1;
            }
        }
    }
    
    // Flush any remaining list
    if !current_list_items.is_empty() {
        let items = current_list_items.clone();
        let list_type = current_list_type.unwrap();
        elements.push(render_list(items, list_type, i));
    }
    
    rsx! {
        div {
            style: "display: flex; flex-direction: column;",
            {elements.into_iter()}
        }
    }
}

fn render_list(items: Vec<String>, list_type: MarkdownElement, key: usize) -> Element {
    if list_type == MarkdownElement::BulletList {
        rsx! {
            ul {
                key: "{key}",
                style: "margin: 8px 0; padding-left: 24px; list-style-type: disc;",
                for (idx, item) in items.iter().enumerate() {
                    li {
                        key: "{idx}",
                        style: "margin: 4px 0; line-height: 1.6; color: var(--color-text-secondary);",
                        {parse_inline_formatting(item)}
                    }
                }
            }
        }
    } else {
        rsx! {
            ol {
                key: "{key}",
                style: "margin: 8px 0; padding-left: 24px;",
                for (idx, item) in items.iter().enumerate() {
                    li {
                        key: "{idx}",
                        style: "margin: 4px 0; line-height: 1.6; color: var(--color-text-secondary);",
                        {parse_inline_formatting(item)}
                    }
                }
            }
        }
    }
}