#[proc_macro]
pub fn check_specific_dependency_version_usage(
    crate_name_token_stream: proc_macro::TokenStream,
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
    let toml_table_map = cargo_toml_workspace_content
        .parse::<toml::Table>()
        .unwrap_or_else(|e| {
            panic!("cannot parse::<toml::Table>() cargo_toml_workspace_content, error:\"{e}\"")
        });
    let toml_table_workspace_members_map_vec = if let Some(toml_table_map_value) =
        toml_table_map.get("workspace")
    {
        if let toml::Value::Table(toml_table_workspace_map) = toml_table_map_value {
            if let Some(toml_table_workspace_map_value) = toml_table_workspace_map.get("members") {
                if let toml::Value::Array(toml_table_workspace_members_map_vec) =
                    toml_table_workspace_map_value
                {
                    toml_table_workspace_members_map_vec
                        .iter()
                        .map(|path_value| {
                            if let toml::Value::String(path) = path_value {
                                path
                            } else {
                                panic!("path is not a toml::Value::String");
                            }
                        })
                        .collect::<Vec<&String>>()
                } else {
                    panic!("toml_table_workspace_map is not a toml::Value::Array");
                }
            } else {
                panic!("no members in toml_table_workspace_map");
            }
        } else {
            panic!("toml_table_map_value is not a toml::Value::Table");
        }
    } else {
        panic!("no workspace in toml_table_map");
    };
    let forbidden_dependency_logic_symbols = ['>', '<', '*', '~', '^'];
    let crate_name_token_stream_stringified = crate_name_token_stream.to_string();
    let mut is_logic_executed = true;
    toml_table_workspace_members_map_vec
        .iter()
        .filter(|member|member.contains(&crate_name_token_stream_stringified))
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
            let cargo_toml_member_map = cargo_toml_member_content.parse::<toml::Table>().unwrap();
            check_version_on_specific_usage(
                member,
                "dependencies",
                &cargo_toml_member_map,
                forbidden_dependency_logic_symbols,
            );
            check_version_on_specific_usage(
                member,
                "dev-dependencies",
                &cargo_toml_member_map,
                forbidden_dependency_logic_symbols,
            );
            is_logic_executed = true;
        });
    if let false = is_logic_executed {
        panic!("logic is not executed, please check tokenized crate name(input parameter for check_specific_dependency_version_usage!(HERE)");
    }
    quote::quote! {}.into()
}

fn check_version_on_specific_usage(
    member: &String,
    key: &str,
    cargo_toml_member_map: &toml::map::Map<String, toml::Value>,
    forbidden_dependency_logic_symbols: [char; 5],
) {
    if let Some(toml_member_table_map_value) = cargo_toml_member_map.get(key) {
        if let toml::Value::Table(toml_member_table_dependencies_map) = toml_member_table_map_value
        {
            toml_member_table_dependencies_map
            .iter()
            .for_each(|(crate_name, crate_value)| {
                if let toml::Value::Table(crate_value_map) = crate_value {
                    if let Some(version_value) = crate_value_map.get("version") {
                        if let toml::Value::String(version) = version_value {
                            forbidden_dependency_logic_symbols.iter().for_each(|symbol|{
                                if let true = version.contains(*symbol) {
                                    panic!("{crate_name} version of {member} contains forbidden symbol {symbol}");
                                }
                            });
                        }
                        else {
                            panic!("{crate_name} version_value is not a toml::Value::String {member}");
                        }
                    }
                }
                else {
                    panic!("{crate_name} crate_value is not a toml::Value::Table {member}");
                }
            });
        } else {
            panic!("no {key} in cargo_toml_member_map of {member}");
        }
    }
}
