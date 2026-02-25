/// Collected wizard answers from all 8 steps.
pub(super) struct WizardResult {
    // X API
    pub(super) client_id: String,
    pub(super) client_secret: Option<String>,
    // Business
    pub(super) product_name: String,
    pub(super) product_description: String,
    pub(super) product_url: Option<String>,
    pub(super) target_audience: String,
    pub(super) product_keywords: Vec<String>,
    pub(super) industry_topics: Vec<String>,
    // Brand voice
    pub(super) brand_voice: Option<String>,
    pub(super) reply_style: Option<String>,
    pub(super) content_style: Option<String>,
    // Persona
    pub(super) persona_opinions: Vec<String>,
    pub(super) persona_experiences: Vec<String>,
    pub(super) content_pillars: Vec<String>,
    // Target accounts
    pub(super) target_accounts: Vec<String>,
    // Approval mode
    pub(super) approval_mode: bool,
    // Schedule
    pub(super) timezone: String,
    pub(super) active_hours_start: u8,
    pub(super) active_hours_end: u8,
    pub(super) active_days: Vec<String>,
    // LLM
    pub(super) llm_provider: String,
    pub(super) llm_api_key: Option<String>,
    pub(super) llm_model: String,
    pub(super) llm_base_url: Option<String>,
}
