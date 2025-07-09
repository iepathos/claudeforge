use claudeforge::{Cli, Commands};
use clap::Parser;

#[test]
fn test_cli_parsing_new_command() {
    let args = vec![
        "claudeforge",
        "new",
        "rust",
        "my-project",
        "-d",
        "/tmp/test",
        "-y",
    ];
    let cli = Cli::parse_from(args);

    match cli.command {
        Commands::New {
            language,
            name,
            directory,
            yes,
        } => {
            assert_eq!(language, claudeforge::cli::Language::Rust);
            assert_eq!(name, "my-project");
            assert_eq!(directory, Some(std::path::PathBuf::from("/tmp/test")));
            assert!(yes);
        }
        _ => panic!("Expected New command"),
    }
}

#[test]
fn test_cli_parsing_list_command() {
    let args = vec!["claudeforge", "list"];
    let cli = Cli::parse_from(args);

    match cli.command {
        Commands::List => {}
        _ => panic!("Expected List command"),
    }
}

#[test]
fn test_cli_parsing_update_command() {
    let args = vec!["claudeforge", "update"];
    let cli = Cli::parse_from(args);

    match cli.command {
        Commands::Update => {}
        _ => panic!("Expected Update command"),
    }
}

#[test]
fn test_cli_parsing_version_command() {
    let args = vec!["claudeforge", "version"];
    let cli = Cli::parse_from(args);

    match cli.command {
        Commands::Version => {}
        _ => panic!("Expected Version command"),
    }
}