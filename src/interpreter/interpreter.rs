// interpreter.rs
// As part of the TriCode project
// Created by Maxims Enterprise in 2024


use std::{env::var, process};

use lazy_static::lazy_static;
use std::sync::{Mutex, Arc};
use once_cell::sync::Lazy;

use crate::{compile::compile, error, formatter::formatter::INDENTLEVEL, lexer::tokens::TokenType, parser::nodes::{Literal, Node}, program::{add_variable, PROGRAM}};

use super::{components::{abbreviation::abbreviation, address::address, article::article, aside::aside, b::b, bdi::bdi, bdo::bdo, canvas::canvas, code::code, content::content_component, data::data, definition::dfn, description::{dd::dd, dl::dl, dt::dt}, div::div, embed::{embed::embed, iframe::iframe, object::object, picture::picture, source::source}, footer::footer, groups::{cite::cite, li::li, menu::menu, ol::ol}, header::header, inline::{br::br, del::del, emphasis::em, hr::hr, i::i, ins::ins, keyboard_input::kbd, mark::mark, q::q, s::s, samp::samp, small::small, span::span, strong::strong, sub::sub, sup::sup, time::time, u::u, wbr::wbr}, link::link, media::{area::area, audio::audio, figure::figure, figure_caption::figure_caption, image::image, map::map, track::track}, nav::nav, paragraph::paragraph, pre::pre, quote::quote, ruby::{rp::rp, rt::rt, ruby::ruby}, script::script, search::search, section::section, text::text, title::title}, interpreter_utils::{add_html, add_root_html, eat, except_token_type, Scope}};

lazy_static! {
    pub static ref FINALHTML: Mutex<String> = Mutex::new(String::from(r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta http-equiv="X-UA-Compatible" content="IE=edge">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
"#));
    pub static ref INNER_HTML: Mutex<String> = Mutex::new(String::new());
}

pub fn run(nodes: &mut Vec<Node>, add_to_dom: bool) -> Option<String> {
    let mut scope: Scope = Scope::Top;
    if add_to_dom == false {
        let _ = INDENTLEVEL.lock().saturating_add(1);
        loop {
            if nodes.len() == 0 {
                break;
            }
            else {
                add_html(content(nodes, add_to_dom).as_str(), add_to_dom);
            }
        }
        return None;
    }
    loop {
        if nodes.len() == 0 {
            break;
        }

        let node = nodes[0].clone();
        if scope == Scope::Top {
            match node {
                Node::Metadata(metadata) => {
                    eat(nodes);
                    if metadata.value[0].value == "name" {
                        let name = metadata.value[2].value.clone();
                        let html = format!("<title>{}</title>", name);
                        add_html(html.as_str(), add_to_dom);
                    }
                    else if metadata.value[0].value == "author" {
                        let author = metadata.value[2].value.clone();
                        let html = format!("<meta name=\"author\" content=\"{}\">", author);
                        add_html(html.as_str(), add_to_dom);
                    }
                    else if metadata.value[0].value == "icon" {
                        let icon = metadata.value[2].value.clone();
                        let html = format!("<link rel=\"icon\" href=\"{}\">", icon);
                        add_html(html.as_str(), add_to_dom);
                    }
                    else if metadata.value[0].value == "description" {
                        let description = metadata.value[2].value.clone();
                        let html = format!("<meta name=\"description\" content=\"{}\">", description);
                        add_html(html.as_str(), add_to_dom);
                    }
                    else if metadata.value[0].value == "base" {
                        if metadata.value[1].token_type == TokenType::Colons {
                            let value = metadata.value[2].value.clone();
                            let html = format!("<base href=\"{}\">", value);
                            add_html(html.as_str(), add_to_dom);
                        }
                        else if metadata.value[1].token_type == TokenType::OpenParentheses {
                            let value = metadata.value[2].value.clone();
                            let html = format!("<base target=\"{}\">", value);
                            add_html(html.as_str(), add_to_dom);
                        }
                        else {
                            error!("SPK-004: Invalid metadata found at parsing: {}", metadata.value[0].value);
                        }
                    }
                    else {
                        error!("SPK-004: Invalid metadata found at parsing: {}", metadata.value[0].value);
                    }
                }
                Node::ScopeDirective(directive) => {
                    eat(nodes);
                    if directive.value == "content" {
                        add_root_html("</head>");
                        add_root_html("<body>");
                        scope = Scope::Content;
                    }
                    else {
                        error!("SPK-003: Invalid scope directive found at parsing: {}", directive.value);
                    }
                }
                Node::Other(other) => {
                    eat(nodes);
                    match other.token.token_type {
                        TokenType::NewLine => {
                            continue;
                        }
                        TokenType::EndOfTheFile => {
                            break;
                        }
                        TokenType::ExclamationMark => {
                            match nodes[0].clone() {
                                Node::Literal(literal) => {
                                    eat(nodes);
                                    if literal.value == "html" {
                                        let content = except_token_type(nodes, TokenType::Script);
                                        add_html(&content.value, add_to_dom);
                                    }
                                    else if literal.value == "javascript" {
                                        let content = except_token_type(nodes, TokenType::Script);
                                        add_html(format!("<script>{}</script>", content.value).as_str(), add_to_dom);
                                    }
                                    else if literal.value == "css" {
                                        let content = except_token_type(nodes, TokenType::Script);
                                        add_html(format!("<style>{}</style>", content.value).as_str(), add_to_dom);
                                    }
                                    else {
                                        error!("SPK-005: Invalid instruction found at parsing: {}", literal.value);
                                    }
                                }
                                _ => {
                                    error!("SPK-007: Invalid instruction found at parsing");
                                }
                            }
                        }
                        TokenType::Use => {
                            if nodes.len() == 0 {
                                error!("SPK-005: Unexpected end of file");
                            }
                            match nodes[0].clone() {
                                Node::StringLiteral(literal) => {
                                    eat(nodes);
                                    let value = literal.value;
                                    if value.ends_with(".css") {
                                        add_html(&format!("<link rel=\"stylesheet\" href=\"{}\">", value), add_to_dom);
                                    }
                                    else {
                                        error!("SPK-006: Invalid use found at parsing: {}", value);
                                    }
                                }
                                _ => {
                                    error!("SPK-007: Invalid instruction found at parsing");
                                }
                            }
                        }
                        TokenType::Define => {
                            eat(nodes);
                            match nodes[0].clone() {
                                Node::Literal(literal) => {
                                    let symbol = literal.value;
                                    eat(nodes);
                                    except_token_type(nodes, TokenType::EqualSign);
                                    match nodes[0].clone() {
                                        Node::StringLiteral(literal) => {
                                            let value = literal.value;
                                            eat(nodes);
                                            add_variable(symbol, value);
                                        }
                                        _ => {
                                            error!("SPK-007: Invalid instruction found at parsing");
                                        }
                                    }
                                }
                                _ => {
                                    error!("SPK-007: Invalid instruction found at parsing");
                                }
                            }
                        }
                        _ => {
                            error!("SPK-001: Invalid instruction at top level: {:?}", other.token.token_type);
                        }
                    }
                }
                _ => {
                    error!("SPK-002: Invalid instruction at top level");
                }
            }
        }
        else if scope == Scope::Content {
            add_html(content(nodes, add_to_dom).as_str(), add_to_dom);
        }
    }

    add_root_html("</body>");
    add_root_html("</html>");

    let final_html: String = FINALHTML.lock().unwrap().to_string();
    reset_html();
    return Some(final_html);
}

pub fn content(nodes: &mut Vec<Node>, add_to_dom: bool) -> String {
    match nodes[0].clone() {
        Node::Literal(literal) => {
            eat(nodes);
            match literal.value.as_str() {
                "title" => {
                    let result = title(nodes);
                    result
                }
                "paragraph" => {
                    let result = paragraph(nodes);
                    result
                }
                "p" => {
                    let result = paragraph(nodes);
                    result
                }
                "section" => {
                    section(nodes, add_to_dom)
                }
                "search" => {
                    search(nodes, add_to_dom)
                }
                "script" => {
                    let result = script(nodes);
                    result
                }
                "footer" => {
                    footer(nodes, add_to_dom)
                }
                "header" => {
                    header(nodes, add_to_dom)
                }
                "image" => {
                    image(nodes)
                }
                "map" => {
                    map(nodes, add_to_dom)
                }
                "track" => {
                    track(nodes)
                }
                "embed" => {
                    embed(nodes)
                } 
                "iframe" => {
                    iframe(nodes)
                }
                "frame" => {
                    iframe(nodes)
                }
                "canvas" => {
                    canvas(nodes, add_to_dom)
                }
                "object" => {
                    object(nodes)
                }
                "picture" => {
                    picture(nodes, add_to_dom)
                }
                "source" => {
                    source(nodes)
                }
                "video" => {
                    track(nodes)
                }
                "img" => {
                    image(nodes)
                }
                "content" => {
                    content(nodes, add_to_dom)
                }
                "main" => {
                    content_component(nodes, add_to_dom)
                }
                "navigation" => {
                    nav(nodes, add_to_dom)
                }
                "area" => {
                    area(nodes)
                }
                "audio" => {
                    audio(nodes)
                }
                "nav" => {
                    nav(nodes, add_to_dom)
                }
                "abbreviation" => {
                    let result = abbreviation(nodes);
                    result
                }
                "dlist" => {
                    dl(nodes, add_to_dom)
                }
                "dl" => {
                    dl(nodes, add_to_dom)
                }
                "dtext" => {
                    dt(nodes)
                }
                "dt" => {
                    dt(nodes)
                }
                "ddescription" => {
                    dd(nodes)
                }
                "dd" => {
                    dd(nodes)
                }
                "abbr" => {
                    let result = abbreviation(nodes);
                    result
                }
                "blockquote" => {
                    quote(nodes, add_to_dom)
                }
                "div" => {
                    let result = div(nodes, add_to_dom);
                    result
                }
                "figure_caption" => {
                    figure_caption(nodes)
                }
                "figure" => {
                    figure(nodes, add_to_dom)
                }
                "figcaption" => {
                    figure_caption(nodes)
                }
                "text" => {
                    let result = text(nodes);
                    result
                }
                "tline" => {
                    hr(nodes)
                }
                "item" => {
                    li(nodes)
                } 
                "li" => {
                    li(nodes)
                }
                "list_item" => {
                    li(nodes)
                }
                "nlist" => {
                    ol(nodes, add_to_dom)
                }
                "ol" => {
                    ol(nodes, add_to_dom)
                }
                "menu" => {
                    menu(nodes, add_to_dom)
                }
                "address" => {
                    let result = address(nodes, add_to_dom);
                    result
                }
                "article" => {
                    article(nodes, add_to_dom)
                }
                "rp" => {
                    rp(nodes)
                }
                "rby_parenthesis" => {
                    rp(nodes)
                }
                "rt" => {
                    rt(nodes)
                }
                "rby_text" => {
                    rt(nodes)
                }
                "c_text" => {
                    rt(nodes)
                }
                "c_parenthesis" => {
                    rp(nodes)
                }
                "aside" => {
                    aside(nodes, add_to_dom)
                }
                "ruby" => {
                    ruby(nodes, add_to_dom)
                }
                "cannotation" => {
                    ruby(nodes, add_to_dom)
                }
                "character_annotation" => {
                    ruby(nodes, add_to_dom)
                }
                "data" => {
                    data(nodes)
                }
                "cite" => {
                    cite(nodes, add_to_dom)
                }
                _ => {
                    process_inline_elements(nodes, literal)
                }
            }
        }
        Node::Other(other) => {
            eat(nodes);
            match other.token.token_type {
                TokenType::NewLine => {
                    return "".to_string();
                }
                TokenType::EndOfTheFile => {
                    process::exit(0);
                }
                TokenType::InLineComments => {
                    format!("<!--{}-->", other.token.value)
                }
                TokenType::ExclamationMark => {
                    match nodes[0].clone() {
                        Node::Literal(literal) => {
                            eat(nodes);
                            if literal.value == "html" {
                                let content = except_token_type(nodes, TokenType::Script);
                                content.value
                            }
                            else if literal.value == "javascript" {
                                let content = except_token_type(nodes, TokenType::Script);
                                format!("<script>{}</script>", content.value)
                            }
                            else if literal.value == "no_script" {
                                let content = except_token_type(nodes, TokenType::Script);
                                format!("<noscript>{}</noscript>", content.value)
                            }
                            else if literal.value == "css" {
                                let content = except_token_type(nodes, TokenType::Script);
                                format!("<style>{}</style>", content.value)
                            }
                            else {
                                error!("SPK-005: Invalid instruction found at parsing: {}", literal.value);
                            }
                        }
                        _ => {
                            error!("SPK-007: Invalid instruction found at parsing");
                        }
                    }
                }
                _ => {
                    error!("SPK-009: Invalid instruction found at parsing: {:?}", other.token.token_type);
                }
            }
        }
        _ => {
            error!("SPK-009: Invalid instruction found at parsing: {:?}", nodes[0]);
        }
    }
}

pub fn process_inline_elements(nodes: &mut Vec<Node>, literal: Literal) -> String {
    match literal.value.as_str() {
        "pre" => {
                    pre(nodes)
                }
                "preformatted" => {
                    pre(nodes)
                }
                "definition" => {
                    dfn(nodes)
                }
                "dfn" => {
                    dfn(nodes)
                }
                "em" => {
                    em(nodes)
                }
                "insert" => {
                    ins(nodes)
                } 
                "ins" => {
                    ins(nodes)
                }
                "del" => {
                    del(nodes)
                }
                "deleted" => {
                    del(nodes)
                }
                "emphasis" => {
                    em(nodes)
                }
                "i" => {
                    i(nodes)
                }
                "s" => {
                    s(nodes)
                }
                "sampler" => {
                    samp(nodes)
                }
                "samp" => {
                    samp(nodes)
                }
                "striketrhough" => {
                    s(nodes)
                }
                "offset" => {
                    i(nodes)
                }
                "keyboard_text" => {
                    kbd(nodes)
                }
                "small" => {
                    small(nodes)
                }
                "kbd" => {
                    kbd(nodes)
                }
                "link" => {
                    let result = link(nodes);
                    result
                }
                "a" => {
                    let result = link(nodes);
                    result
                }
                "mark" => {
                    mark(nodes)
                }
                "highlighted" => {
                    mark(nodes)
                }
                "sub" => {
                    sub(nodes)
                }
                "u" => {
                    u(nodes)
                }
                "underline" => {
                    u(nodes)
                }
                "var" => {
                    super::components::inline::var::var(nodes)
                }
                "wbr" => {
                    wbr(nodes)
                }
                "line_opportunity" => {
                    wbr(nodes)
                }
                "variable" => {
                    super::components::inline::var::var(nodes)
                }
                "subscript" => {
                    sub(nodes)
                }
                "sup" => {
                    sup(nodes)
                }
                "time" => {
                    time(nodes)
                }
                "superscript" => {
                    sup(nodes)
                }
                "strong" => {
                    strong(nodes)
                }
                "span" => {
                    span(nodes)
                }
                "container" => {
                    span(nodes)
                }
                "q" => {
                    q(nodes)
                }
                "quote" => {
                    q(nodes)
                }
                "code" => {
                    code(nodes)
                }
                "code_block" => {
                    code(nodes)
                }
                "pformatted" => {
                    pre(nodes)
                } 
                "tbreak" => {
                    hr(nodes)
                }
                "hr" => {
                    hr(nodes)
                }
                "bold" => {
                    b(nodes)
                }
                "b" => {
                    b(nodes)
                }
                "bdo" => {
                    bdo(nodes)
                }
                "bidiroverride" => {
                    bdo(nodes)
                }
                "line" => {
                    br(nodes)
                }
                "br" => {
                    br(nodes)
                }
                "bdi" => {
                    bdi(nodes)
                }
                
                "bidirectional" => {
                    bdi(nodes)
                }
                _ => {
                    error!("SPK-008: Invalid instruction found at parsing: {}", literal.value);
                }
    }
}

fn reset_html() {
    let mut final_html = FINALHTML.lock().unwrap();
    let mut inner_html = INNER_HTML.lock().unwrap();
    inner_html.clear();
    final_html.clear();

    final_html.push_str(r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta http-equiv="X-UA-Compatible" content="IE=edge">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
"#)
} 