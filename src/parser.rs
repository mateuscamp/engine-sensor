use anyhow::{Context, Result, bail};
use regex::Regex;
use tree_sitter::{Node, Parser};

use crate::model::{Engine, Span};

#[derive(Debug, Clone)]
pub struct CallSite {
    pub callee: String,
    pub args: Vec<String>,
    pub text: String,
    pub owner: String,
    pub span: Span,
    pub end_line: u32,
    pub control_path: String,
    pub inside_loop: bool,
}

#[derive(Debug, Clone)]
pub struct FunctionSite {
    pub name: String,
    pub start_line: u32,
    pub end_line: u32,
    pub text: String,
}

#[derive(Debug, Clone)]
pub struct ParsedSource {
    pub path: String,
    pub source: String,
    pub calls: Vec<CallSite>,
    pub functions: Vec<FunctionSite>,
}

pub fn parse_source(engine: Engine, path: String, source: String) -> Result<ParsedSource> {
    let mut parser = Parser::new();
    let language = match engine {
        Engine::Godot => tree_sitter_gdscript::LANGUAGE.into(),
        Engine::Defold => tree_sitter_lua::LANGUAGE.into(),
    };
    parser
        .set_language(&language)
        .with_context(|| format!("não consegui carregar o parser de {engine}"))?;
    let parseable = if engine == Engine::Godot {
        ascii_shape(&source)
    } else {
        source.clone()
    };
    let tree = parser
        .parse(&parseable, None)
        .with_context(|| format!("parser não devolveu árvore para {path}"))?;
    if tree.root_node().has_error() {
        let error = first_error(tree.root_node()).unwrap_or(tree.root_node());
        bail!(
            "SAR-PARSE-001 {}:{}:{} contém sintaxe não compreendida pelo parser de {}",
            path,
            error.start_position().row + 1,
            error.start_position().column + 1,
            engine
        );
    }

    let mut calls = Vec::new();
    let mut functions = Vec::new();
    visit(
        tree.root_node(),
        source.as_bytes(),
        &path,
        "<arquivo>",
        &mut calls,
        &mut functions,
    );
    calls.sort_by_key(|item| (item.span.line, item.span.column, item.callee.clone()));
    functions.sort_by_key(|item| (item.start_line, item.end_line, item.name.clone()));
    Ok(ParsedSource {
        path,
        source,
        calls,
        functions,
    })
}

fn first_error(node: Node<'_>) -> Option<Node<'_>> {
    if node.is_error() || node.is_missing() {
        return Some(node);
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.has_error()
            && let Some(error) = first_error(child)
        {
            return Some(error);
        }
    }
    None
}

fn visit(
    node: Node<'_>,
    source: &[u8],
    path: &str,
    current_owner: &str,
    calls: &mut Vec<CallSite>,
    functions: &mut Vec<FunctionSite>,
) {
    let kind = node.kind();
    let text = node.utf8_text(source).unwrap_or_default();
    let is_function = kind.contains("function") && !kind.contains("call");
    let owner = if is_function {
        function_name(text, node.start_position().row as u32 + 1)
    } else {
        current_owner.to_owned()
    };

    if is_function {
        functions.push(FunctionSite {
            name: owner.clone(),
            start_line: node.start_position().row as u32 + 1,
            end_line: node.end_position().row as u32 + 1,
            text: text.to_owned(),
        });
    }

    if matches!(
        kind,
        "call" | "call_expression" | "function_call" | "attribute_call"
    ) && let Some((call_text, call_node)) = expanded_call(node, source)
        && let Some((callee, args)) = call_parts(call_text)
    {
        calls.push(CallSite {
            callee,
            args,
            text: call_text.to_owned(),
            owner: owner.clone(),
            span: Span {
                path: path.to_owned(),
                line: call_node.start_position().row as u32 + 1,
                column: call_node.start_position().column as u32 + 1,
            },
            end_line: call_node.end_position().row as u32 + 1,
            control_path: control_context(call_node).0,
            inside_loop: control_context(call_node).1,
        });
    }

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        visit(child, source, path, &owner, calls, functions);
    }
}

fn ascii_shape(source: &str) -> String {
    source
        .as_bytes()
        .iter()
        .map(|byte| if byte.is_ascii() { *byte } else { b'x' })
        .map(char::from)
        .collect()
}

fn control_context(node: Node<'_>) -> (String, bool) {
    let mut parts = Vec::new();
    let mut inside_loop = false;
    let mut current = node.parent();
    while let Some(ancestor) = current {
        let kind = ancestor.kind();
        if kind.contains("function") && !kind.contains("call") {
            break;
        }
        let is_loop = matches!(
            kind,
            "for_statement" | "while_statement" | "repeat_statement"
        );
        let is_control = is_loop
            || matches!(
                kind,
                "block"
                    | "body"
                    | "if_statement"
                    | "elseif_statement"
                    | "else_statement"
                    | "else_clause"
                    | "match_statement"
                    | "pattern_section"
            );
        if is_loop {
            inside_loop = true;
        }
        if is_control {
            parts.push(format!(
                "{}@{}:{}",
                kind,
                ancestor.start_position().row + 1,
                ancestor.start_position().column + 1
            ));
        }
        current = ancestor.parent();
    }
    parts.reverse();
    (parts.join("/"), inside_loop)
}

fn expanded_call<'a>(node: Node<'a>, source: &'a [u8]) -> Option<(&'a str, Node<'a>)> {
    if node.kind() != "attribute_call" {
        return Some((node.utf8_text(source).ok()?, node));
    }
    let parent = node.parent()?;
    let parent_text = parent.utf8_text(source).ok()?;
    if parent_text.contains('.') && parent.end_byte() == node.end_byte() {
        Some((parent_text, parent))
    } else {
        Some((node.utf8_text(source).ok()?, node))
    }
}

fn function_name(text: &str, line: u32) -> String {
    let gdscript = Regex::new(r"(?m)^\s*func\s+([A-Za-z_][A-Za-z0-9_]*)\s*\(").unwrap();
    if let Some(captures) = gdscript.captures(text) {
        return captures[1].to_owned();
    }
    let lua =
        Regex::new(r"(?m)^\s*(?:local\s+)?function\s+([A-Za-z_][A-Za-z0-9_:.]*)\s*\(").unwrap();
    if let Some(captures) = lua.captures(text) {
        return captures[1].to_owned();
    }
    format!("<callback@{line}>")
}

fn call_parts(text: &str) -> Option<(String, Vec<String>)> {
    let mut depth = 0_u32;
    let mut quote = None;
    let mut escaped = false;
    let mut open = None;
    for (index, character) in text.char_indices() {
        if let Some(active) = quote {
            if escaped {
                escaped = false;
            } else if character == '\\' {
                escaped = true;
            } else if character == active {
                quote = None;
            }
            continue;
        }
        match character {
            '\'' | '"' => quote = Some(character),
            '(' => {
                if depth == 0 {
                    open = Some(index);
                }
                depth += 1;
            }
            ')' => depth = depth.saturating_sub(1),
            _ => {}
        }
    }
    let open = open?;
    let close = matching_close(text, open)?;
    let callee = text[..open].trim().to_owned();
    if callee.is_empty() {
        return None;
    }
    Some((callee, split_arguments(&text[open + 1..close])))
}

fn matching_close(text: &str, open: usize) -> Option<usize> {
    let mut depth = 0_u32;
    let mut quote = None;
    let mut escaped = false;
    for (offset, character) in text[open..].char_indices() {
        if let Some(active) = quote {
            if escaped {
                escaped = false;
            } else if character == '\\' {
                escaped = true;
            } else if character == active {
                quote = None;
            }
            continue;
        }
        match character {
            '\'' | '"' => quote = Some(character),
            '(' => depth += 1,
            ')' => {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    return Some(open + offset);
                }
            }
            _ => {}
        }
    }
    None
}

fn split_arguments(source: &str) -> Vec<String> {
    let mut args = Vec::new();
    let mut start = 0;
    let mut depth = 0_i32;
    let mut quote = None;
    let mut escaped = false;
    for (index, character) in source.char_indices() {
        if let Some(active) = quote {
            if escaped {
                escaped = false;
            } else if character == '\\' {
                escaped = true;
            } else if character == active {
                quote = None;
            }
            continue;
        }
        match character {
            '\'' | '"' => quote = Some(character),
            '(' | '[' | '{' => depth += 1,
            ')' | ']' | '}' => depth -= 1,
            ',' if depth == 0 => {
                args.push(source[start..index].trim().to_owned());
                start = index + 1;
            }
            _ => {}
        }
    }
    let final_arg = source[start..].trim();
    if !final_arg.is_empty() {
        args.push(final_arg.to_owned());
    }
    args
}

#[cfg(test)]
mod tests {
    use crate::model::Engine;

    use super::{call_parts, parse_source, split_arguments};

    #[test]
    fn separates_nested_arguments() {
        assert_eq!(
            split_arguments("node, vmath.vector3(1, 2, 3), \"scale,x\""),
            ["node", "vmath.vector3(1, 2, 3)", "\"scale,x\""]
        );
    }

    #[test]
    fn reads_last_call_in_chain() {
        let (callee, args) =
            call_parts("create_tween().tween_property(node, \"scale\", 1, 2)").expect("call");
        assert_eq!(callee, "create_tween().tween_property");
        assert_eq!(args[0], "node");
    }

    #[test]
    fn finds_gdscript_method_calls() {
        let parsed = parse_source(
            Engine::Godot,
            "main.gd".to_owned(),
            "extends Node\nfunc run():\n\tvar t = create_tween()\n\tt.tween_property(self, \"scale\", 1, 2)\n".to_owned(),
        )
        .expect("parse");
        assert!(
            parsed
                .calls
                .iter()
                .any(|call| call.callee.ends_with(".tween_property")),
            "calls={:?}",
            parsed.calls
        );
    }

    #[test]
    fn finds_tween_property_followed_by_a_fluent_chain() {
        let parsed = parse_source(
            Engine::Godot,
            "bomba_na_tela.gd".to_owned(),
            "extends Node2D\nfunc pulsar(no: Node2D) -> void:\n\tvar pulso: Tween = no.create_tween().set_loops()\n\tpulso.tween_property(no, \"scale\", Vector2.ONE, 0.2) \\\n+\t\t.set_trans(Tween.TRANS_SINE).set_ease(Tween.EASE_IN_OUT)\n"
                .to_owned(),
        )
        .expect("parse");
        assert!(
            parsed
                .calls
                .iter()
                .any(|call| call.callee.ends_with(".tween_property")),
            "calls={:?}",
            parsed.calls
        );
    }
}
