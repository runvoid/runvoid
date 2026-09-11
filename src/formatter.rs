pub fn format_source(source: &str) -> String {
    let mut formatted = Vec::new();
    let mut indent_level: usize = 0;
    let mut prev_was_empty = false;

    for raw_line in source.lines() {
        let trimmed = raw_line.trim();

        if trimmed.is_empty() {
            if !prev_was_empty && !formatted.is_empty() {
                formatted.push(String::new());
                prev_was_empty = true;
            }
            continue;
        }
        prev_was_empty = false;

        let mut leading_closes = 0;
        let mut chars = trimmed.chars().peekable();
        while let Some(&c) = chars.peek() {
            if c == '}' {
                leading_closes += 1;
                chars.next();
            } else if c.is_whitespace() {
                chars.next();
            } else {
                break;
            }
        }

        let effective_indent = indent_level.saturating_sub(leading_closes);
        let indent_str = "    ".repeat(effective_indent);

        let open_count = trimmed.chars().filter(|&c| c == '{').count();
        let close_count = trimmed.chars().filter(|&c| c == '}').count();

        indent_level = (indent_level + open_count).saturating_sub(close_count);

        formatted.push(format!("{}{}", indent_str, trimmed));
    }

    let mut result = formatted.join("\n");
    if !result.is_empty() {
        result.push('\n');
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_formatter_indentation() {
        let input = r#"
remember x = 10
if x > 5 {
say "big"
} otherwise {
say "small"
}
"#;
        let expected = r#"remember x = 10
if x > 5 {
    say "big"
} otherwise {
    say "small"
}
"#;
        assert_eq!(format_source(input), expected);
    }
}
