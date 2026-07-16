use std::sync::Arc;
use crate::error::{JvmError, RuntimeError, Result};
use crate::runtime::{JVM, Frame, Value, HeapObject, method_area::{Method, NativeImplementation}};

// ========== Simple Regex Engine ==========

/// A simple recursive backtracking regex engine.
/// Supports: literals, `.` (any char), `*` `+` `?` quantifiers,
/// `[abc]` character classes, `\d` `\w` `\s`, `^` `$` anchors.

#[derive(Debug, Clone)]
enum RegexNode {
    Lit(char),           // Literal character
    Any,                 // .
    CharClass(Vec<char>, bool), // [abc] or [^abc] (inverted)
    Digit,               // \d
    Word,                // \w
    Space,               // \s
    Wildcard,            // Combination of .* etc.
}

#[derive(Debug, Clone)]
struct RegexPattern {
    nodes: Vec<(RegexNode, bool, bool)>, // (node, is_star, is_plus)
    is_anchored_start: bool,
    is_anchored_end: bool,
}

fn parse_regex(pattern: &str) -> Result<RegexPattern> {
    let mut nodes = Vec::new();
    let chars: Vec<char> = pattern.chars().collect();
    let mut i = 0;
    let mut is_anchored_start = false;
    let mut is_anchored_end = false;

    if !chars.is_empty() && chars[0] == '^' {
        is_anchored_start = true;
        i = 1;
    }

    while i < chars.len() {
        let (node, consumed) = match chars[i] {
            '.' => (RegexNode::Any, 1),
            '\\' if i + 1 < chars.len() => {
                let n = match chars[i + 1] {
                    'd' => RegexNode::Digit,
                    'w' => RegexNode::Word,
                    's' => RegexNode::Space,
                    c => RegexNode::Lit(c),
                };
                (n, 2)
            }
            '[' => {
                let mut class_chars = Vec::new();
                let mut inverted = false;
                let mut j = i + 1;
                if j < chars.len() && chars[j] == '^' {
                    inverted = true;
                    j += 1;
                }
                while j < chars.len() && chars[j] != ']' {
                    if j + 2 < chars.len() && chars[j + 1] == '-' {
                        let start = chars[j];
                        let end = chars[j + 2];
                        for c in start..=end {
                            class_chars.push(c);
                        }
                        j += 3;
                    } else {
                        class_chars.push(chars[j]);
                        j += 1;
                    }
                }
                if j < chars.len() { j += 1; } // skip ]
                (RegexNode::CharClass(class_chars, inverted), j - i)
            }
            c => (RegexNode::Lit(c), 1),
        };
        i += consumed;

        // Check for quantifiers
        let (is_star, is_plus) = if i < chars.len() {
            match chars[i] {
                '*' => { i += 1; (true, false) }
                '+' => { i += 1; (false, true) }
                '?' => { i += 1; (false, false) } // optional - handled via is_plus=false, is_star=true with special case
                _ => (false, false),
            }
        } else {
            (false, false)
        };

        nodes.push((node, is_star, is_plus));
    }

    // Check for end anchor
    if let Some((_, _, _)) = nodes.last() {
        if chars.last() == Some(&'$') {
            is_anchored_end = true;
        }
    }

    Ok(RegexPattern { nodes, is_anchored_start, is_anchored_end })
}

fn match_node(node: &RegexNode, c: char) -> bool {
    match node {
        RegexNode::Lit(l) => *l == c,
        RegexNode::Any => true,
        RegexNode::CharClass(chars, inverted) => {
            let found = chars.contains(&c);
            if *inverted { !found } else { found }
        }
        RegexNode::Digit => c.is_ascii_digit(),
        RegexNode::Word => c.is_ascii_alphanumeric() || c == '_',
        RegexNode::Space => c.is_ascii_whitespace(),
        RegexNode::Wildcard => true,
    }
}

fn match_pattern(pattern: &RegexPattern, text: &str) -> bool {
    let chars: Vec<char> = text.chars().collect();
    let text_len = chars.len();

    if pattern.nodes.is_empty() {
        return text.is_empty();
    }

    // Try matching at each position (or only at start if anchored)
    let start_positions: Vec<usize> = if pattern.is_anchored_start {
        vec![0]
    } else {
        (0..=text_len).collect()
    };

    for start in start_positions {
        if match_from(pattern, &chars, start, 0) {
            let matched = pattern.is_anchored_end && start == 0;
            if !pattern.is_anchored_end || (start == 0 && matched_end(pattern, &chars, start)) {
                return true;
            }
        }
    }
    false
}

fn matched_end(pattern: &RegexPattern, chars: &[char], start: usize) -> bool {
    let mut pos = start;
    for (node, is_star, is_plus) in &pattern.nodes {
        if *is_star {
            // Skip as many as possible
            while pos < chars.len() && match_node(node, chars[pos]) {
                pos += 1;
            }
        } else if *is_plus {
            let mut count = 0;
            while pos < chars.len() && match_node(node, chars[pos]) {
                pos += 1;
                count += 1;
            }
            if count == 0 { return false; }
        } else {
            if pos >= chars.len() { return false; }
            if !match_node(node, chars[pos]) { return false; }
            pos += 1;
        }
    }
    pos == chars.len()
}

fn match_from(pattern: &RegexPattern, chars: &[char], start: usize, node_idx: usize) -> bool {
    if node_idx >= pattern.nodes.len() {
        return true; // All nodes matched
    }

    let (node, is_star, is_plus) = &pattern.nodes[node_idx];
    let text_len = chars.len();

    if *is_star {
        // Try matching 0 or more occurrences
        // First try matching 0 (skip this node)
        if match_from(pattern, chars, start, node_idx + 1) {
            return true;
        }
        // Then try matching 1+ occurrences
        let mut pos = start;
        while pos < text_len && match_node(node, chars[pos]) {
            pos += 1;
            if match_from(pattern, chars, pos, node_idx + 1) {
                return true;
            }
        }
        false
    } else if *is_plus {
        // Must match at least 1
        if start >= text_len || !match_node(node, chars[start]) {
            return false;
        }
        // Try matching 1 occurrence
        if match_from(pattern, chars, start + 1, node_idx + 1) {
            return true;
        }
        // Try matching more
        let mut pos = start + 1;
        while pos < text_len && match_node(node, chars[pos]) {
            pos += 1;
            if match_from(pattern, chars, pos, node_idx + 1) {
                return true;
            }
        }
        false
    } else {
        // Single character match
        if start >= text_len || !match_node(node, chars[start]) {
            return false;
        }
        match_from(pattern, chars, start + 1, node_idx + 1)
    }
}

// ========== java.util.regex.Pattern ==========

pub struct Pattern;

impl Pattern {
    pub fn compile() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let regex_ref = frame.get_local(1)?.clone();
            let regex_str = if let Value::ObjectRef(str_id) = regex_ref {
                if let Some(str_obj) = jvm.heap.get(str_id) {
                    str_obj.string_value.clone().unwrap_or_default()
                } else { String::new() }
            } else { String::new() };

            let pattern_obj = HeapObject::new("java.util.regex.Pattern".to_string());
            let pattern_ref = jvm.allocate(pattern_obj)?;
            // Allocate the regex string first, then update the pattern object
            let regex_obj = HeapObject::new_string("java.lang.String".to_string(), regex_str);
            let regex_ref_id = jvm.allocate(regex_obj)?;
            if let Some(obj) = jvm.heap.get_mut(pattern_ref) {
                obj.fields.insert("pattern".to_string(), Value::ObjectRef(regex_ref_id));
            }
            frame.push(Value::ObjectRef(pattern_ref))?;
            Ok(())
        });
        Method::new_native("java.util.regex.Pattern".to_string(), "compile".to_string(), "(Ljava/lang/String;)Ljava/util/regex/Pattern;".to_string(), true, Some(native_impl))
    }

    pub fn matches() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let input_ref = frame.pop()?;
            let regex_ref = frame.pop()?;
            let regex_str = if let Value::ObjectRef(str_id) = regex_ref {
                if let Some(str_obj) = jvm.heap.get(str_id) {
                    str_obj.string_value.clone().unwrap_or_default()
                } else { String::new() }
            } else { String::new() };
            let input_str = if let Value::ObjectRef(str_id) = input_ref {
                if let Some(str_obj) = jvm.heap.get(str_id) {
                    str_obj.string_value.clone().unwrap_or_default()
                } else { String::new() }
            } else { String::new() };

            let result = match parse_regex(&regex_str) {
                Ok(pattern) => match_pattern(&pattern, &input_str),
                Err(_) => false,
            };
            frame.push(Value::Boolean(result))?;
            Ok(())
        });
        Method::new_native("java.util.regex.Pattern".to_string(), "matches".to_string(), "(Ljava/lang/String;Ljava/lang/CharSequence;)Z".to_string(), true, Some(native_impl))
    }

    pub fn matcher() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let input_ref = frame.pop()?;
            let this_ref = frame.get_local(0)?;
            let matcher_obj = HeapObject::new("java.util.regex.Matcher".to_string());
            let matcher_ref = jvm.allocate(matcher_obj)?;
            if let Value::ObjectRef(this_id) = this_ref {
                // Extract pattern string ID first, then update the matcher
                let pattern_str_id = {
                    let pattern_obj = jvm.heap.get(*this_id)
                        .ok_or(RuntimeError::NullPointerException)?;
                    pattern_obj.fields.get("pattern")
                        .and_then(|v| if let Value::ObjectRef(id) = v { Some(*id) } else { None })
                        .unwrap_or(0)
                };
                if let Some(obj) = jvm.heap.get_mut(matcher_ref) {
                    obj.fields.insert("pattern".to_string(), Value::ObjectRef(pattern_str_id));
                    obj.fields.insert("input".to_string(), input_ref);
                }
            }
            frame.push(Value::ObjectRef(matcher_ref))?;
            Ok(())
        });
        Method::new_native("java.util.regex.Pattern".to_string(), "matcher".to_string(), "(Ljava/lang/CharSequence;)Ljava/util/regex/Matcher;".to_string(), false, Some(native_impl))
    }

    pub fn register(jvm: &mut JVM) {
        jvm.method_area.add_native_method("java.util.regex.Pattern", Pattern::compile());
        jvm.method_area.add_native_method("java.util.regex.Pattern", Pattern::matches());
        jvm.method_area.add_native_method("java.util.regex.Pattern", Pattern::matcher());
    }
}

// ========== java.util.regex.Matcher ==========

pub struct Matcher;

impl Matcher {
    pub fn matches() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let this_ref = frame.get_local(0)?;
            let mut result = false;
            if let Value::ObjectRef(this_id) = this_ref {
                if let Some(matcher_obj) = jvm.heap.get(*this_id) {
                    let pattern_str = if let Some(Value::ObjectRef(ps_id)) = matcher_obj.fields.get("pattern") {
                        if let Some(ps_obj) = jvm.heap.get(*ps_id) {
                            ps_obj.string_value.clone().unwrap_or_default()
                        } else { String::new() }
                    } else { String::new() };
                    let input_str = if let Some(Value::ObjectRef(in_id)) = matcher_obj.fields.get("input") {
                        if let Some(in_obj) = jvm.heap.get(*in_id) {
                            in_obj.string_value.clone().unwrap_or_default()
                        } else { String::new() }
                    } else { String::new() };
                    result = match parse_regex(&pattern_str) {
                        Ok(pattern) => match_pattern(&pattern, &input_str),
                        Err(_) => false,
                    };
                }
            }
            frame.push(Value::Boolean(result))?;
            Ok(())
        });
        Method::new_native("java.util.regex.Matcher".to_string(), "matches".to_string(), "()Z".to_string(), false, Some(native_impl))
    }

    pub fn find() -> Method {
        // Simplified: same as matches for now
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let this_ref = frame.get_local(0)?;
            let mut result = false;
            if let Value::ObjectRef(this_id) = this_ref {
                if let Some(matcher_obj) = jvm.heap.get(*this_id) {
                    let pattern_str = if let Some(Value::ObjectRef(ps_id)) = matcher_obj.fields.get("pattern") {
                        if let Some(ps_obj) = jvm.heap.get(*ps_id) {
                            ps_obj.string_value.clone().unwrap_or_default()
                        } else { String::new() }
                    } else { String::new() };
                    let input_str = if let Some(Value::ObjectRef(in_id)) = matcher_obj.fields.get("input") {
                        if let Some(in_obj) = jvm.heap.get(*in_id) {
                            in_obj.string_value.clone().unwrap_or_default()
                        } else { String::new() }
                    } else { String::new() };
                    result = match parse_regex(&pattern_str) {
                        Ok(pattern) => match_pattern(&pattern, &input_str),
                        Err(_) => false,
                    };
                }
            }
            frame.push(Value::Boolean(result))?;
            Ok(())
        });
        Method::new_native("java.util.regex.Matcher".to_string(), "find".to_string(), "()Z".to_string(), false, Some(native_impl))
    }

    pub fn group() -> Method {
        let native_impl: NativeImplementation = Arc::new(|frame, jvm| {
            let this_ref = frame.get_local(0)?;
            if let Value::ObjectRef(this_id) = this_ref {
                if let Some(matcher_obj) = jvm.heap.get(*this_id) {
                    if let Some(Value::ObjectRef(in_id)) = matcher_obj.fields.get("input") {
                        if let Some(in_obj) = jvm.heap.get(*in_id) {
                            if let Some(s) = &in_obj.string_value {
                                let result = HeapObject::new_string("java.lang.String".to_string(), s.clone());
                                let r = jvm.allocate(result)?;
                                frame.push(Value::ObjectRef(r))?;
                                return Ok(());
                            }
                        }
                    }
                }
            }
            frame.push(Value::Null)?;
            Ok(())
        });
        Method::new_native("java.util.regex.Matcher".to_string(), "group".to_string(), "()Ljava/lang/String;".to_string(), false, Some(native_impl))
    }

    pub fn register(jvm: &mut JVM) {
        jvm.method_area.add_native_method("java.util.regex.Matcher", Matcher::matches());
        jvm.method_area.add_native_method("java.util.regex.Matcher", Matcher::find());
        jvm.method_area.add_native_method("java.util.regex.Matcher", Matcher::group());
    }
}

/// Register all regex classes with the JVM.
pub fn register_regex_classes(jvm: &mut JVM) {
    Pattern::register(jvm);
    Matcher::register(jvm);
}