use std::{fs, path::Path};

use anyhow::{Context, Result, bail};
use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, ValueEnum,
)]
#[serde(rename_all = "lowercase")]
pub enum EngineChoice {
    #[default]
    Auto,
    Godot,
    Defold,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum Profile {
    Desktop,
    Android,
}

impl std::fmt::Display for Profile {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Desktop => write!(formatter, "desktop"),
            Self::Android => write!(formatter, "android"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum OutputFormat {
    Text,
    Json,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AllowRule {
    pub rule: String,
    pub resource: String,
    pub owners: Vec<String>,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct SaraConfig {
    pub schema_version: u32,
    pub engine: EngineChoice,
    pub profiles: Vec<Profile>,
    pub allow: Vec<AllowRule>,
}

impl Default for SaraConfig {
    fn default() -> Self {
        Self {
            schema_version: 1,
            engine: EngineChoice::Auto,
            profiles: vec![Profile::Desktop, Profile::Android],
            allow: Vec::new(),
        }
    }
}

impl SaraConfig {
    pub fn load(project: &Path) -> Result<Self> {
        let path = project.join("sara.toml");
        if !path.exists() {
            return Ok(Self::default());
        }
        let source = fs::read_to_string(&path)
            .with_context(|| format!("não consegui ler {}", path.display()))?;
        let config: Self = toml::from_str(&source)
            .with_context(|| format!("configuração inválida em {}", path.display()))?;
        config.validate()?;
        Ok(config)
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != 1 {
            bail!(
                "schema_version={} não é suportado; esperado 1",
                self.schema_version
            );
        }
        if self.profiles.is_empty() {
            bail!("profiles precisa conter desktop, android ou ambos");
        }
        for item in &self.allow {
            if item.rule.trim().is_empty()
                || item.resource.trim().is_empty()
                || item.reason.trim().is_empty()
                || item.owners.is_empty()
                || item.owners.iter().any(|owner| owner.trim().is_empty())
            {
                bail!("exceção inválida: regra, recurso, dono(s) e motivo são obrigatórios");
            }
        }
        Ok(())
    }
}
