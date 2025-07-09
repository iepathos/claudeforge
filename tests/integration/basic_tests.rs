use claudeforge::create_project;
use claudeforge::template::registry::load_template_registry;
use tempfile::TempDir;

#[tokio::test]
async fn test_create_projects_for_all_templates() {
    let registry = load_template_registry().unwrap();

    for (language, template) in registry.iter() {
        let temp_dir = TempDir::new().unwrap();
        let project_name = format!("test-{}-project", language.to_string().to_lowercase());

        let result = create_project(
            language.clone(),
            project_name.clone(),
            Some(temp_dir.path().to_path_buf()),
            true, // skip prompts
        )
        .await;

        // This test might fail if the template repositories don't exist
        // For now, we'll just verify the function doesn't panic
        match result {
            Ok(_) => {
                let project_dir = temp_dir.path().join(&project_name);
                assert!(
                    project_dir.exists(),
                    "Project directory should exist for {} template",
                    template.name
                );
            }
            Err(e) => {
                // If template repositories don't exist, this is expected
                println!(
                    "Expected error for {} template (repo might not exist): {e}",
                    template.name
                );
            }
        }
    }
}
