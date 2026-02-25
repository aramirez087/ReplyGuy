/// Reusable interactive prompts shared with the `upgrade` command.
use anyhow::Result;
use dialoguer::{Confirm, Input};

use super::helpers::parse_csv;

/// Collect persona fields interactively.
/// Returns (opinions, experiences, content_pillars).
pub(crate) fn prompt_persona() -> Result<(Vec<String>, Vec<String>, Vec<String>)> {
    let opinions_raw: String = Input::new()
        .with_prompt("Strong opinions, comma-separated (Enter to skip)")
        .allow_empty(true)
        .interact_text()?;

    let experiences_raw: String = Input::new()
        .with_prompt("Personal experiences, comma-separated (Enter to skip)")
        .allow_empty(true)
        .interact_text()?;

    let pillars_raw: String = Input::new()
        .with_prompt("Core content topics, comma-separated (Enter to skip)")
        .allow_empty(true)
        .interact_text()?;

    eprintln!();

    Ok((
        parse_csv(&opinions_raw),
        parse_csv(&experiences_raw),
        parse_csv(&pillars_raw),
    ))
}

/// Collect target account fields interactively.
pub(crate) fn prompt_target_accounts() -> Result<Vec<String>> {
    let accounts_raw: String = Input::new()
        .with_prompt("Accounts to monitor, comma-separated @usernames (Enter to skip)")
        .allow_empty(true)
        .interact_text()?;

    let accounts: Vec<String> = parse_csv(&accounts_raw)
        .into_iter()
        .map(|a| a.trim_start_matches('@').to_string())
        .collect();

    eprintln!();

    Ok(accounts)
}

/// Collect approval mode preference interactively.
pub(crate) fn prompt_approval_mode() -> Result<bool> {
    let approval_mode = Confirm::new()
        .with_prompt("Queue posts for review before posting?")
        .default(true)
        .interact()?;

    eprintln!();

    Ok(approval_mode)
}

/// Collect enhanced safety limit fields interactively.
/// Returns (max_replies_per_author_per_day, banned_phrases, product_mention_ratio).
pub(crate) fn prompt_enhanced_limits() -> Result<(u32, Vec<String>, f32)> {
    let max_replies: String = Input::new()
        .with_prompt("Max replies to same author per day")
        .default("1".to_string())
        .interact_text()?;
    let max_replies: u32 = max_replies.trim().parse().unwrap_or(1);

    let banned_raw: String = Input::new()
        .with_prompt("Banned phrases, comma-separated (Enter for defaults)")
        .default("check out, you should try, I recommend, link in bio".to_string())
        .interact_text()?;
    let banned_phrases = parse_csv(&banned_raw);

    let ratio_raw: String = Input::new()
        .with_prompt("Product mention ratio (0.0–1.0)")
        .default("0.2".to_string())
        .interact_text()?;
    let ratio: f32 = ratio_raw.trim().parse().unwrap_or(0.2);

    eprintln!();

    Ok((max_replies, banned_phrases, ratio))
}
