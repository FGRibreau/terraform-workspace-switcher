extern crate dialoguer;

use std::process::Command;

use dialoguer::Select;

fn run(cmd: &str, args: &[&str]) -> String {
    let output = Command::new(cmd)
        .args(args)
        .output()
        .expect(&format!("failed to execute {}", cmd));

    assert!(
        output.status.success(),
        format!("error running {} {:?}", cmd, args)
    );

    String::from_utf8(output.stdout)
        .map(|x| x.trim().to_string())
        .expect("could not convert stdout to utf8")
}

fn remove_and_trim(x: &str) -> String {
    x.replace("*", "").trim().to_string()
}

fn main() {
    println!("? Select Terraform workspace (Use arrow keys)");
    let workspace_list = run("terraform", &["workspace", "list"]);

    let mut workspaces = workspace_list
        .split("\n")
        .map(remove_and_trim)
        .collect::<Vec<String>>();

    if workspaces.len() == 0 {
        panic!("Expected defined workspaces. Are you inside a terraform directory?")
    }

    let current_workspace: String = workspace_list
        .split("\n")
        .find(|&x| x.trim().starts_with("*"))
        .map(remove_and_trim)
        .expect("Expected selected workspace");

    workspaces.sort();

    let selected_workspace_index = Select::new()
        .default(
            workspaces
                .iter()
                .position(|x| *x == current_workspace)
                .expect("current selected workspace should be in selection"),
        )
        .items(&workspaces.iter().map(|x| x.trim()).collect::<Vec<&str>>()[..])
        .interact()
        .unwrap();

    println!(
        "{}",
        run(
            "terraform",
            &["workspace", "select", &workspaces[selected_workspace_index]],
        )
    );
}
