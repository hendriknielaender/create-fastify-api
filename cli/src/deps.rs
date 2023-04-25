use template::Template;

use crate::colors::*;
use crate::internal::template;
use crate::package_manager::PackageManager;
use std::process::Command;


fn is_node_installed() -> bool {
    Command::new("node")
        .arg("-v")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}



pub fn print_missing_deps(pkg_manager: PackageManager, template: Template) {
    let deps: &[(&str, String, &dyn Fn() -> bool, bool)] = &[
        (
            "Node.js",
            format!("Visit {BLUE}{BOLD}https://nodejs.org/en/{RESET}"),
            &is_node_installed,
            !pkg_manager.is_node(),
        ),
    ];

    let missing_deps: Vec<(String, String)> = deps
        .iter()
        .filter(|(_, _, exists, skip)| !skip && !exists())
        .map(|(s, d, _, _)| (s.to_string(), d.clone()))
        .collect();

    let (largest_first_cell, largest_second_cell) =
        missing_deps
            .iter()
            .fold((0, 0), |(mut prev_f, mut prev_s), (f, s)| {
                let f_len = f.len();
                if f_len > prev_f {
                    prev_f = f_len;
                }

                let s_len = remove_colors(s).len();
                if s_len > prev_s {
                    prev_s = s_len;
                }

                (prev_f, prev_s)
            });

    if !missing_deps.is_empty() {
        println!("\n\nYour system is {YELLOW}missing dependencies{RESET} (or they do not exist in {YELLOW}$PATH{RESET}):");
        for (index, (name, instruction)) in missing_deps.iter().enumerate() {
            if index == 0 {
                println!(
                    "╭{}┬{}╮",
                    "─".repeat(largest_first_cell + 2),
                    "─".repeat(largest_second_cell + 2)
                );
            } else {
                println!(
                    "├{}┼{}┤",
                    "─".repeat(largest_first_cell + 2),
                    "─".repeat(largest_second_cell + 2)
                );
            }
            println!(
                "│ {YELLOW}{name}{RESET}{} │ {instruction}{} │",
                " ".repeat(largest_first_cell - name.len()),
                " ".repeat(largest_second_cell - remove_colors(instruction).len()),
            );
        }
        println!(
            "╰{}┴{}╯",
            "─".repeat(largest_first_cell + 2),
            "─".repeat(largest_second_cell + 2),
        );
        println!();
        println!("Make sure you have installed the prerequisites for your OS: {BLUE}{BOLD}, then run:");
    } else {
        println!(" To get started run:")
    }
}
