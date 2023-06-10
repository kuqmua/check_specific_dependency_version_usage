#[derive(Debug, serde::Deserialize)]
struct CargoTomlWorkspaceConfig {
    workspace: Workspace,
}

#[derive(Debug, serde::Deserialize)]
struct Workspace {
    members: Vec<String>,
}

#[derive(Debug, serde::Deserialize)]
struct CargoTomlMemberConfig {
    dependencies: std::collections::HashMap<std::string::String, std::string::String>,
    #[serde(rename(deserialize = "dev-dependencies"))]
    dev_dependencies: std::collections::HashMap<std::string::String, std::string::String>,
}

#[proc_macro]
pub fn check_specific_dependency_version_usage(
    _: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    proc_macro_helpers::panic_location::panic_location();
    let cargo_toml = "Cargo.toml";
    let cannot_open_file = "cannot open ";
    let file_error = " file, error: ";
    let mut buf_reader = std::io::BufReader::new(
        std::fs::File::open(std::path::Path::new(&cargo_toml))
            .unwrap_or_else(|e| panic!("{cannot_open_file}{cargo_toml}{file_error}\"{e}\"")),
    );
    let mut cargo_toml_workspace_content = String::new();
    {
        use std::io::Read;
        buf_reader
            .read_to_string(&mut cargo_toml_workspace_content)
            .unwrap_or_else(|e| {
                panic!("cannot read_to_string from {cargo_toml}{file_error}\"{e}\"")
            });
    }
    toml::from_str::<CargoTomlWorkspaceConfig>(&cargo_toml_workspace_content)
        .unwrap_or_else(|e| panic!("toml::from_str::<CargoTomlWorkspaceConfig> error:\"{e}\""))
        .workspace
        .members
        .iter()
        .for_each(|member| {
            let path_to_cargo_toml_member = format!("{member}/{cargo_toml}");
            let mut buf_reader_member = std::io::BufReader::new(
                std::fs::File::open(std::path::Path::new(&path_to_cargo_toml_member))
                    .unwrap_or_else(|e| {
                        panic!("{cannot_open_file}{path_to_cargo_toml_member}{file_error}\"{e}\"")
                    }),
            );
            let mut cargo_toml_member_content = String::new();
            {
                use std::io::Read;
                buf_reader_member
                .read_to_string(&mut cargo_toml_member_content)
                .unwrap_or_else(|e| {
                    panic!("cannot read_to_string from {path_to_cargo_toml_member}{file_error}\"{e}\"")
                });
            }
            println!("cargo_toml_member_content {cargo_toml_member_content}");
            println!("-----------------------------");
            // let f = toml::from_str::<CargoTomlMemberConfig>(&cargo_toml_member_content)
            // .unwrap_or_else(|e| panic!("toml::from_str::<CargoTomlMemberConfig> error:\"{e}\""));
        });
    quote::quote! {}.into()
}
