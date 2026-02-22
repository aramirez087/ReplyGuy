use anyhow::{bail, Context, Result};
use console::Style;
use dialoguer::{Confirm, Input, MultiSelect, Select};

// ---------------------------------------------------------------------------
// Change tracking
// ---------------------------------------------------------------------------

pub(super) struct Change {
    pub section: String,
    pub field: String,
    pub old_value: String,
    pub new_value: String,
}

pub(super) struct ChangeTracker {
    pub changes: Vec<Change>,
}

impl ChangeTracker {
    pub fn new() -> Self {
        Self {
            changes: Vec::new(),
        }
    }

    pub fn record(&mut self, section: &str, field: &str, old_value: &str, new_value: &str) {
        if old_value != new_value {
            self.changes.push(Change {
                section: section.to_string(),
                field: field.to_string(),
                old_value: old_value.to_string(),
                new_value: new_value.to_string(),
            });
        }
    }
}

// ---------------------------------------------------------------------------
// Field editing helpers (interactive)
// ---------------------------------------------------------------------------

pub(super) fn edit_string(label: &str, current: &str) -> Result<String> {
    let val: String = Input::new()
        .with_prompt(label)
        .default(current.to_string())
        .interact_text()?;
    Ok(val.trim().to_string())
}

pub(super) fn edit_optional_string(
    label: &str,
    current: &Option<String>,
) -> Result<Option<String>> {
    let default = current.as_deref().unwrap_or("").to_string();
    let prompt = if current.is_some() {
        format!("{label} (type \"none\" to clear)")
    } else {
        format!("{label} (Enter to skip)")
    };
    let val: String = Input::new()
        .with_prompt(prompt)
        .default(default)
        .allow_empty(true)
        .interact_text()?;
    let trimmed = val.trim();
    if trimmed.is_empty() || trimmed.eq_ignore_ascii_case("none") {
        Ok(None)
    } else {
        Ok(Some(trimmed.to_string()))
    }
}

pub(super) fn edit_bool(label: &str, current: bool) -> Result<bool> {
    let val = Confirm::new()
        .with_prompt(label)
        .default(current)
        .interact()?;
    Ok(val)
}

pub(super) fn edit_u32(label: &str, current: u32, help: Option<&str>) -> Result<u32> {
    if let Some(h) = help {
        let dim = Style::new().dim();
        eprintln!("  {}", dim.apply_to(h));
    }
    let val: String = Input::new()
        .with_prompt(label)
        .default(current.to_string())
        .validate_with(|input: &String| -> std::result::Result<(), String> {
            input
                .trim()
                .parse::<u32>()
                .map(|_| ())
                .map_err(|_| "Must be a positive number".to_string())
        })
        .interact_text()?;
    Ok(val.trim().parse().unwrap())
}

pub(super) fn edit_u64(label: &str, current: u64, help: Option<&str>) -> Result<u64> {
    if let Some(h) = help {
        let dim = Style::new().dim();
        eprintln!("  {}", dim.apply_to(h));
    }
    let val: String = Input::new()
        .with_prompt(label)
        .default(current.to_string())
        .validate_with(|input: &String| -> std::result::Result<(), String> {
            input
                .trim()
                .parse::<u64>()
                .map(|_| ())
                .map_err(|_| "Must be a positive number".to_string())
        })
        .interact_text()?;
    Ok(val.trim().parse().unwrap())
}

pub(super) fn edit_u8(label: &str, current: u8) -> Result<u8> {
    let val: String = Input::new()
        .with_prompt(label)
        .default(current.to_string())
        .validate_with(|input: &String| -> std::result::Result<(), String> {
            input
                .trim()
                .parse::<u8>()
                .ok()
                .filter(|&v| v <= 23)
                .map(|_| ())
                .ok_or_else(|| "Must be 0-23".to_string())
        })
        .interact_text()?;
    Ok(val.trim().parse().unwrap())
}

pub(super) fn edit_f32(label: &str, current: f32, help: Option<&str>) -> Result<f32> {
    if let Some(h) = help {
        let dim = Style::new().dim();
        eprintln!("  {}", dim.apply_to(h));
    }
    let val: String = Input::new()
        .with_prompt(label)
        .default(format!("{current:.2}"))
        .validate_with(|input: &String| -> std::result::Result<(), String> {
            input
                .trim()
                .parse::<f32>()
                .map(|_| ())
                .map_err(|_| "Must be a number".to_string())
        })
        .interact_text()?;
    Ok(val.trim().parse().unwrap())
}

pub(super) fn edit_list(label: &str, current: &[String]) -> Result<Vec<String>> {
    let actions = if current.is_empty() {
        vec!["Add items", "Replace all"]
    } else {
        vec!["Add items", "Remove items", "Replace all"]
    };

    let selection = Select::new()
        .with_prompt(format!("{label} — what do you want to do?"))
        .items(&actions)
        .default(0)
        .interact()?;

    let action = actions[selection];

    match action {
        "Add items" => {
            let raw: String = Input::new()
                .with_prompt("Items to add (comma-separated)")
                .interact_text()?;
            let new_items = parse_csv(&raw);
            let mut result = current.to_vec();
            result.extend(new_items);
            Ok(result)
        }
        "Remove items" => {
            if current.is_empty() {
                eprintln!("Nothing to remove.");
                return Ok(current.to_vec());
            }
            let items: Vec<&str> = current.iter().map(|s| s.as_str()).collect();
            let selections = MultiSelect::new()
                .with_prompt("Select items to remove (Space to toggle, Enter to confirm)")
                .items(&items)
                .interact()?;
            let result: Vec<String> = current
                .iter()
                .enumerate()
                .filter(|(i, _)| !selections.contains(i))
                .map(|(_, s)| s.clone())
                .collect();
            Ok(result)
        }
        "Replace all" => {
            let raw: String = Input::new()
                .with_prompt("New items (comma-separated)")
                .allow_empty(true)
                .interact_text()?;
            Ok(parse_csv(&raw))
        }
        _ => Ok(current.to_vec()),
    }
}

pub(super) fn edit_duration_minutes(label: &str, current_seconds: u64) -> Result<u64> {
    let dim = Style::new().dim();
    eprintln!(
        "  {}",
        dim.apply_to(format!(
            "Currently: {}",
            super::show::format_duration(current_seconds)
        ))
    );
    eprintln!(
        "  {}",
        dim.apply_to("Enter value in minutes (e.g., 15) or hours (e.g., 3h)")
    );

    let default_display = if current_seconds >= 3600 && current_seconds % 3600 == 0 {
        format!("{}h", current_seconds / 3600)
    } else {
        format!("{}", current_seconds / 60)
    };

    let val: String = Input::new()
        .with_prompt(format!("{label} (minutes, or Nh for hours)"))
        .default(default_display)
        .validate_with(|input: &String| -> std::result::Result<(), String> {
            parse_duration_input(input.trim())
                .map(|_| ())
                .map_err(|e| e.to_string())
        })
        .interact_text()?;

    parse_duration_input(val.trim())
}

// ---------------------------------------------------------------------------
// Pure helpers
// ---------------------------------------------------------------------------

pub(super) fn parse_duration_input(input: &str) -> Result<u64> {
    let input = input.trim().to_lowercase();
    if let Some(hours) = input.strip_suffix('h') {
        let h: u64 = hours.trim().parse().context("Invalid number of hours")?;
        Ok(h * 3600)
    } else if let Some(days) = input.strip_suffix('d') {
        let d: u64 = days.trim().parse().context("Invalid number of days")?;
        Ok(d * 86400)
    } else {
        let mins: u64 = input
            .parse()
            .context("Enter a number (minutes), or Nh for hours, Nd for days")?;
        Ok(mins * 60)
    }
}

pub(super) fn parse_bool(value: &str) -> Result<bool> {
    match value.to_lowercase().as_str() {
        "true" | "yes" | "1" | "on" => Ok(true),
        "false" | "no" | "0" | "off" => Ok(false),
        _ => bail!("Invalid boolean value: {value} (use true/false, yes/no, 1/0)"),
    }
}

pub(super) fn parse_csv(s: &str) -> Vec<String> {
    s.split(',')
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty())
        .collect()
}

pub(super) fn escape_toml(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

pub(super) fn format_toml_array(items: &[String]) -> String {
    let inner: Vec<String> = items
        .iter()
        .map(|s| format!("\"{}\"", escape_toml(s)))
        .collect();
    format!("[{}]", inner.join(", "))
}
