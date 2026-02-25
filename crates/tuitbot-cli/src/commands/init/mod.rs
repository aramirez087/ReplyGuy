/// `tuitbot init` — interactive setup wizard or template copy.
///
/// Walks new users through X API credentials, business profile, and LLM
/// provider configuration in eight guided steps. Falls back to copying
/// `config.example.toml` with `--non-interactive`.
///
/// After writing the config the wizard offers to continue seamlessly
/// through `auth → test → run` so the user doesn't have to remember
/// three separate commands.
mod display;
mod helpers;
mod prompts;
mod render;
mod steps;
mod wizard;

#[cfg(test)]
mod tests;

use std::fs;
use std::io::IsTerminal;
use std::path::PathBuf;

use anyhow::{bail, Context, Result};
use dialoguer::Confirm;
use tuitbot_core::config::Config;
use tuitbot_core::startup::data_dir;

use display::{print_remaining_steps, print_summary, print_welcome_banner};
use render::render_config_toml;
use steps::{
    step_approval_mode, step_brand_voice, step_business_profile, step_llm_provider, step_persona,
    step_schedule, step_target_accounts, step_x_api,
};

use super::{auth, run, test};

// Re-export prompt functions used by the upgrade command.
pub(crate) use prompts::{
    prompt_approval_mode, prompt_enhanced_limits, prompt_persona, prompt_target_accounts,
};

/// Embedded copy of the example config shipped with the repo.
const EXAMPLE_CONFIG: &str = include_str!("../../../config.example.toml");

/// Run the init command.
pub async fn execute(force: bool, non_interactive: bool) -> Result<()> {
    let dir = data_dir();
    let config_path: PathBuf = dir.join("config.toml");

    if config_path.exists() && !force {
        eprintln!(
            "Configuration already exists at {}\n\
             Use --force to overwrite.",
            config_path.display()
        );
        return Ok(());
    }

    if non_interactive {
        return write_template(&dir, &config_path);
    }

    // Guard: must be a real terminal for interactive mode.
    if !std::io::stdin().is_terminal() {
        bail!(
            "Interactive wizard requires a terminal.\n\
             Use --non-interactive to copy the template config instead."
        );
    }

    run_wizard(&dir, &config_path).await
}

/// Non-interactive path: copy the embedded template.
fn write_template(dir: &PathBuf, config_path: &PathBuf) -> Result<()> {
    fs::create_dir_all(dir)?;
    fs::write(config_path, EXAMPLE_CONFIG)?;

    eprintln!("Created {}\n", config_path.display());
    eprintln!("Next steps:");
    eprintln!(
        "  1. Edit {} with your X API and LLM credentials",
        config_path.display()
    );
    eprintln!("  2. tuitbot auth    — authenticate with X");
    eprintln!("  3. tuitbot test    — validate configuration");
    eprintln!("  4. tuitbot run     — start the agent");

    Ok(())
}

/// Interactive wizard: collect credentials, write config, then offer to
/// continue through auth → test → run inline.
async fn run_wizard(dir: &PathBuf, config_path: &PathBuf) -> Result<()> {
    print_welcome_banner();

    let result = step_x_api()?;
    let result = step_business_profile(result)?;
    let result = step_brand_voice(result)?;
    let result = step_llm_provider(result)?;
    let result = step_persona(result)?;
    let result = step_target_accounts(result)?;
    let result = step_approval_mode(result)?;
    let result = step_schedule(result)?;

    print_summary(&result);

    let confirm = Confirm::new()
        .with_prompt("Write this configuration?")
        .default(true)
        .interact()?;

    if !confirm {
        eprintln!("Aborted. No files were written.");
        return Ok(());
    }

    fs::create_dir_all(dir)?;
    let toml = render_config_toml(&result);
    fs::write(config_path, &toml)
        .with_context(|| format!("Failed to write {}", config_path.display()))?;

    eprintln!("\nWrote {}", config_path.display());

    // --- Seamless chaining: auth → test → run ---

    let config_path_str = config_path.display().to_string();
    let config = Config::load(Some(&config_path_str))
        .context("Failed to reload the config we just wrote")?;

    // Step A: Authenticate
    let do_auth = Confirm::new()
        .with_prompt("Authenticate with X now?")
        .default(true)
        .interact()?;

    if !do_auth {
        print_remaining_steps(&[
            "tuitbot auth    — authenticate with X",
            "tuitbot test    — validate configuration",
            "tuitbot run     — start the agent",
        ]);
        return Ok(());
    }

    if let Err(e) = auth::execute(&config, None).await {
        eprintln!("\nAuth failed: {e:#}");
        print_remaining_steps(&[
            "tuitbot auth    — retry authentication",
            "tuitbot test    — validate configuration",
            "tuitbot run     — start the agent",
        ]);
        return Ok(());
    }

    // Step B: Validate
    let do_test = Confirm::new()
        .with_prompt("Validate configuration now?")
        .default(true)
        .interact()?;

    if !do_test {
        print_remaining_steps(&[
            "tuitbot test    — validate configuration",
            "tuitbot run     — start the agent",
        ]);
        return Ok(());
    }

    let all_passed = test::run_checks(&config, &config_path_str).await;
    if !all_passed {
        eprintln!("Fix the issues above, then:");
        print_remaining_steps(&[
            "tuitbot test    — re-validate configuration",
            "tuitbot run     — start the agent",
        ]);
        return Ok(());
    }

    // Step C: Run (defaults No — bigger commitment)
    let do_run = Confirm::new()
        .with_prompt("Start the agent now?")
        .default(false)
        .interact()?;

    if !do_run {
        print_remaining_steps(&["tuitbot run     — start the agent"]);
        return Ok(());
    }

    run::execute(&config, 0).await?;

    Ok(())
}
