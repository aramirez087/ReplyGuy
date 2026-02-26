use anyhow::Result;
use console::Style;
use dialoguer::Confirm;
use tuitbot_core::config::{Config, EnrichmentStage, ProfileCompleteness};

use super::helpers::ChangeTracker;
use super::interactive;

/// Run the guided enrichment flow, walking through each incomplete stage.
pub(super) fn run_enrichment(config: &mut Config, tracker: &mut ChangeTracker) -> Result<()> {
    let bold = Style::new().bold();
    let dim = Style::new().dim();

    eprintln!();
    eprintln!("{}", bold.apply_to("Profile Enrichment"));
    eprintln!("{}", dim.apply_to("──────────────────"));
    eprintln!();

    let completeness = config.profile_completeness();
    print_stage_status(&completeness);

    if completeness.is_fully_enriched() {
        eprintln!();
        eprintln!("All enrichment stages are complete. Nice work!");
        eprintln!("Use `tuitbot settings voice`, `persona`, or `targets` to fine-tune.");
        return Ok(());
    }

    eprintln!();
    eprintln!("Each stage improves content quality. Press Enter to skip any stage.");
    eprintln!();

    let mut changed = false;

    for &(stage, done) in &completeness.stages {
        if done {
            continue;
        }

        let prompt = format!("Configure {}? ({})", stage.label(), stage.description());
        let yes = Confirm::new()
            .with_prompt(&prompt)
            .default(true)
            .interact()?;

        if !yes {
            continue;
        }

        match stage {
            EnrichmentStage::Voice => interactive::edit_category_voice(config, tracker)?,
            EnrichmentStage::Persona => interactive::edit_category_persona(config, tracker)?,
            EnrichmentStage::Targeting => interactive::edit_category_targets(config, tracker)?,
        }

        changed = true;
    }

    if changed {
        eprintln!();
        eprintln!("Updated status:");
        let updated = config.profile_completeness();
        print_stage_status(&updated);
    }

    Ok(())
}

fn print_stage_status(completeness: &ProfileCompleteness) {
    for &(stage, done) in &completeness.stages {
        let status = if done { "OK" } else { "--" };
        eprintln!(
            "  {:<12} {:<4} {}",
            stage.label(),
            status,
            stage.description()
        );
    }
}
