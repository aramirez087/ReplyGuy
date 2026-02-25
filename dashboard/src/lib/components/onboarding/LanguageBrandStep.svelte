<script lang="ts">
	import { onboardingData } from '$lib/stores/onboarding';
	import { Globe, Palette } from 'lucide-svelte';

	let language = $state($onboardingData.language);
	let brandVoice = $state($onboardingData.brand_voice);

	const languages = [
		{ value: 'en', label: 'English', description: 'All content generated in English' },
		{ value: 'es', label: 'Spanish', description: 'All content generated in Spanish' },
		{ value: 'bilingual', label: 'Bilingual', description: 'Mix of English and Spanish content' }
	];

	const voices = [
		{ value: 'conservative', label: 'Conservative', description: 'Professional, cautious tone. Lower risk of controversy.' },
		{ value: 'balanced', label: 'Balanced', description: 'Friendly and knowledgeable. Good default for most brands.' },
		{ value: 'bold', label: 'Bold', description: 'Opinionated, direct voice. Higher engagement but more polarizing.' }
	];

	$effect(() => {
		onboardingData.updateField('language', language);
	});

	$effect(() => {
		onboardingData.updateField('brand_voice', brandVoice);
	});
</script>

<div class="step">
	<h2>Language & Brand Voice</h2>
	<p class="step-description">
		Choose your preferred language and brand personality for generated content.
	</p>

	<div class="field-group">
		<label class="field-label">
			<Globe size={14} />
			Language
		</label>
		<div class="option-cards">
			{#each languages as lang}
				<button
					class="option-card"
					class:selected={language === lang.value}
					onclick={() => (language = lang.value)}
				>
					<span class="option-label">{lang.label}</span>
					<span class="option-desc">{lang.description}</span>
				</button>
			{/each}
		</div>
	</div>

	<div class="field-group">
		<label class="field-label">
			<Palette size={14} />
			Brand Voice
		</label>
		<div class="option-cards">
			{#each voices as voice}
				<button
					class="option-card"
					class:selected={brandVoice === voice.value}
					onclick={() => (brandVoice = voice.value)}
				>
					<span class="option-label">{voice.label}</span>
					<span class="option-desc">{voice.description}</span>
				</button>
			{/each}
		</div>
	</div>
</div>

<style>
	.step {
		display: flex;
		flex-direction: column;
		gap: 24px;
	}

	h2 {
		font-size: 20px;
		font-weight: 700;
		color: var(--color-text);
		margin: 0;
	}

	.step-description {
		font-size: 14px;
		color: var(--color-text-muted);
		margin: -16px 0 0;
		line-height: 1.5;
	}

	.field-group {
		display: flex;
		flex-direction: column;
		gap: 10px;
	}

	.field-label {
		display: flex;
		align-items: center;
		gap: 6px;
		font-size: 13px;
		font-weight: 600;
		color: var(--color-text);
	}

	.option-cards {
		display: flex;
		flex-direction: column;
		gap: 8px;
	}

	.option-card {
		display: flex;
		flex-direction: column;
		gap: 4px;
		padding: 14px 16px;
		border: 2px solid var(--color-border);
		border-radius: 8px;
		background: var(--color-surface);
		text-align: left;
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.option-card:hover {
		border-color: var(--color-accent);
		background: var(--color-surface-hover);
	}

	.option-card.selected {
		border-color: var(--color-accent);
		background: color-mix(in srgb, var(--color-accent) 8%, var(--color-surface));
	}

	.option-label {
		font-size: 14px;
		font-weight: 600;
		color: var(--color-text);
	}

	.option-desc {
		font-size: 12px;
		color: var(--color-text-muted);
		line-height: 1.4;
	}
</style>
