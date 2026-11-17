use cognix_workflows_types::Workflow;
use std::collections::HashSet;
use std::fs;
use std::path::Path;

/// Extract `{{argument_name}}` placeholders from a workflow command.
///
/// Handles the `${{{name}}}` bash-array idiom (literal `${` + placeholder + literal `}`) so a
/// leading `{` is not mistakenly swallowed into the placeholder name.
fn command_placeholders(command: &str) -> Vec<String> {
    let mut out = Vec::new();
    let chars: Vec<char> = command.chars().collect();
    let mut i = 0;
    while i + 1 < chars.len() {
        if chars[i] == '{' && chars[i + 1] == '{' {
            // Triple brace `{{{`: first `{` is literal, real placeholder starts at i + 1.
            if i + 2 < chars.len() && chars[i + 2] == '{' {
                i += 1;
                continue;
            }
            // Locate the closing `}}`.
            let mut j = i + 2;
            while j + 1 < chars.len() && !(chars[j] == '}' && chars[j + 1] == '}') {
                j += 1;
            }
            if j + 1 < chars.len() {
                let name: String = chars[i + 2..j].iter().collect();
                let name = name.trim();
                if !name.is_empty() {
                    out.push(name.to_string());
                }
                i = j + 2;
            } else {
                break;
            }
        } else {
            i += 1;
        }
    }
    out
}

#[test]
fn all_specs_deserialize_and_are_consistent() {
    let specs_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../specs");
    assert!(
        specs_dir.is_dir(),
        "specs directory not found at {:?}",
        specs_dir
    );

    let mut names = HashSet::new();
    let mut count = 0;

    for entry in walkdir::WalkDir::new(&specs_dir)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
        if ext != "yaml" && ext != "yml" {
            continue;
        }

        let content = fs::read_to_string(path).unwrap_or_else(|e| {
            panic!("failed to read {:?}: {e}", path);
        });
        let workflow: Workflow = serde_yaml::from_str(&content).unwrap_or_else(|e| {
            panic!("failed to parse {:?}: {e}", path);
        });

        assert!(
            workflow.name.is_empty() == false,
            "workflow in {:?} has an empty name",
            path
        );
        assert!(
            workflow.command.is_empty() == false,
            "workflow {:?} in {:?} has an empty command",
            workflow.name,
            path
        );

        assert!(
            names.insert(workflow.name.clone()),
            "duplicate workflow name {:?} in {:?} (already defined elsewhere)",
            workflow.name,
            path
        );

        let declared: HashSet<String> = workflow.arguments.iter().map(|a| a.name.clone()).collect();
        let placeholders: HashSet<String> = command_placeholders(&workflow.command)
            .into_iter()
            .collect();

        // Every {{placeholder}} must have a declared argument.
        for ph in &placeholders {
            assert!(
                declared.contains(ph),
                "workflow {:?} ({:?}) uses undeclared argument {{{{{}}}}} in its command",
                workflow.name,
                path,
                ph
            );
        }

        count += 1;
    }

    assert!(count > 0, "no workflow specs were found");
    println!("validated {count} workflow specs");
}
