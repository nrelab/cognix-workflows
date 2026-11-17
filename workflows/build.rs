use anyhow::Result;
use cognix_workflows_types::Workflow;
use convert_case::{Case, Casing};
use std::collections::HashSet;
use std::ffi::OsStr;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::process::Command;
use walkdir::WalkDir;

/// Generates Workflows as rust files from the yaml stored in /specs. Each Workflow is stored within
/// its own mod within the `generated_workflows` module. Additionally, a function called `workflows`
/// is generated that returns a vector of all the `Workflow`s that were created.
fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=../specs");

    let gen_dir = "src/generated_workflows";
    fs::create_dir_all(gen_dir)?;

    let parent_module_path = format!("{gen_dir}/mod.rs");
    let mut parent_module = File::create(&parent_module_path)?;

    let mut workflows_added = Vec::new();
    let mut expected_modules = HashSet::new();

    for entry in WalkDir::new("../specs") {
        let entry = entry?;
        let path = entry.path();
        let valid_yaml_file = path
            .extension()
            .and_then(OsStr::to_str)
            .is_some_and(|extension| extension == "yaml" || extension == "yml");

        if valid_yaml_file {
            let mut file = File::open(path)?;
            let mut yaml_content = String::new();
            file.read_to_string(&mut yaml_content)?;

            println!("attempting to generate workflow at {:?}", path);
            let workflow: Workflow = serde_yaml::from_str(&yaml_content)?;
            println!("generated workflow is {workflow:?}");

            let file_name = path
                .file_name()
                .expect("spec file should have a name")
                .to_str()
                .expect("spec file name should be valid UTF-8")
                .replace(".yaml", "")
                .replace(".yml", "")
                .to_case(Case::Snake);
            println!("file name is {file_name:?}");

            // Create a module for each Workflow within the parent module.
            expected_modules.insert(file_name.clone());
            workflows_added.push(file_name.clone());

            write_workflow(workflow, file_name.as_str())?;

            writeln!(parent_module, "pub mod {file_name};")?;
        }
    }

    // Remove stale generated modules that no longer have a matching spec file.
    if let Ok(entries) = fs::read_dir(gen_dir) {
        for entry in entries.flatten() {
            let file_name = entry.file_name().to_string_lossy().to_string();
            if file_name == "mod.rs" || !file_name.ends_with(".rs") {
                continue;
            }
            let module_name = &file_name[..file_name.len() - 3];
            if !expected_modules.contains(module_name) {
                println!("removing stale generated workflow {file_name}");
                let _ = fs::remove_file(entry.path());
            }
        }
    }

    write_workflows_function(parent_module, &mut workflows_added)?;

    // Keep generated code rustfmt-clean so `cargo fmt --check` stays stable in CI.
    Command::new("cargo")
        .args(["fmt", "-p", "cognix-workflows"])
        .output()?;

    Ok(())
}

/// Writes a `workflows` function into the module at path `parent_modules`. The generated function
/// will look approximately like:
/// ```ignore
/// use cognix_workflows_metadata::*;
/// pub fn workflows() -> Vec<Workflow> {
///     vec![
///         foo::workflow(),
///         bar::workflow(),
///         bazz:workflow(),
///     ]
/// }
/// ```
fn write_workflows_function(
    mut parent_module: File,
    workflows_added: &mut Vec<String>,
) -> Result<()> {
    writeln!(parent_module, "\nuse cognix_workflows_types::*;")?;
    writeln!(parent_module, "pub fn workflows() -> Vec<Workflow> {{")?;
    writeln!(parent_module, "vec![")?;
    for workflows in workflows_added {
        writeln!(parent_module, "{workflows}::workflow(),")?;
    }
    writeln!(parent_module, "]")?;
    writeln!(parent_module, "}}")?;

    Ok(())
}

/// Writes a Workflow into the path at `src/generated_workflows/<file_name>.rs`. The generated Rust
/// will look like:
/// ```ignore
/// use cognix_workflows_metadata::*;
/// pub fn workflow -> Workflow {
///    Workflow {
///      ...
///    }
/// }
/// ```
fn write_workflow(workflow: Workflow, file_name: &str) -> Result<()> {
    let module = std::fs::File::create(format!("src/generated_workflows/{file_name}.rs"))?;

    writeln!(&module, "use cognix_workflows_types::*;")?;
    writeln!(&module, r#"pub fn workflow() -> Workflow {{"#)?;
    writeln!(&module, "{}", uneval::to_string(workflow)?)?;
    writeln!(&module, "}}")?;

    Ok(())
}
