use std::{path::PathBuf, process::ExitCode};

use anyhow::Result;
use clap::{Parser, Subcommand};
use sara_ai_first::{
    CheckRequest, check_project,
    config::{EngineChoice, OutputFormat, Profile, SaraConfig},
    initialize_project, report,
};

#[derive(Debug, Parser)]
#[command(name = "sara", version, about = "Verificador AI-first interno")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Cria configuração e fragmentos para AGENTS.md e CLAUDE.md sem sobrescrevê-los.
    Init {
        #[arg(default_value = ".")]
        path: PathBuf,
        #[arg(long, value_enum, default_value_t = EngineChoice::Auto)]
        engine: EngineChoice,
    },
    /// Analisa posse de animação e entrada.
    Check {
        #[arg(default_value = ".")]
        path: PathBuf,
        #[arg(long, value_enum, default_value_t = EngineChoice::Auto)]
        engine: EngineChoice,
        #[arg(long, value_enum)]
        profile: Option<Profile>,
        #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
        format: OutputFormat,
    },
}

fn main() -> ExitCode {
    match run(Cli::parse()) {
        Ok(code) => code,
        Err(error) => {
            eprintln!("ERRO SARA: {error:#}");
            ExitCode::from(2)
        }
    }
}

fn run(cli: Cli) -> Result<ExitCode> {
    match cli.command {
        Command::Init { path, engine } => {
            let created = initialize_project(&path, engine)?;
            if created.is_empty() {
                println!("Sara já inicializado; nenhum arquivo foi sobrescrito.");
            } else {
                println!("Sara inicializado. Arquivos criados:");
                for path in created {
                    println!("- {path}");
                }
            }
            println!(
                "Copie o fragmento de .sara/AGENTS.fragment.md para AGENTS.md e o de .sara/CLAUDE.fragment.md para CLAUDE.md."
            );
            Ok(ExitCode::SUCCESS)
        }
        Command::Check {
            path,
            engine,
            profile,
            format,
        } => {
            let config = SaraConfig::load(&path)?;
            let engine = if engine == EngineChoice::Auto {
                config.engine
            } else {
                engine
            };
            let profiles = profile.map_or(config.profiles, |value| vec![value]);
            let report = check_project(&CheckRequest {
                project: path,
                engine,
                profiles,
                allow: config.allow,
            })?;
            match format {
                OutputFormat::Text => println!("{}", report::text(&report)),
                OutputFormat::Json => println!("{}", report::json(&report)?),
            }
            Ok(if report.has_errors() {
                ExitCode::from(1)
            } else {
                ExitCode::SUCCESS
            })
        }
    }
}
