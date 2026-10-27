// src/main.rs
use clap::{Arg, Command};
use std::fs;
use std::path::Path;
use std::process;

const USR_BIN_DIR: &str = "/usr/local/bin";

fn main() {
    let matches = Command::new("elfinstaller")
        .version("0.1.0")
        .about("A simple terminal application to install and manage ELF packages")
        .about("Note: Requires sudo privileges to install/uninstall ELF packages to /usr/local/bin")
        .arg(
            Arg::new("install")
                .long("install")
                .value_name("ELF_NAME")
                .help("Installs an ELF package into /usr/local/bin (requires sudo)")
                .required(false)
                .value_parser(clap::value_parser!(String)),
        )
        .arg(
            Arg::new("uninstall")
                .long("uninstall")
                .value_name("ELF_NAME")
                .help("Uninstalls an ELF package from /usr/local/bin")
                .required(false)
                .value_parser(clap::value_parser!(String)),
        )
        .arg(
            Arg::new("rename")
                .long("rename")
                .value_name("NEW_NAME")
                .help("Renames the installed ELF package in /usr/local/bin (requires sudo)")
                .required(false)
                .value_parser(clap::value_parser!(String)),
        )
        .get_matches();

    if let Some(elf_name) = matches.get_one::<String>("install") {
        let rename_to = matches.get_one::<String>("rename").unwrap_or(elf_name);
        install_elf(elf_name, rename_to);
    } else if let Some(elf_name) = matches.get_one::<String>("uninstall") {
        uninstall_elf(elf_name);
    } else {
        println!("No command provided. Use --help for more information.");
        process::exit(1);
    }
}

fn install_elf(elf_name: &str, new_name: &str) {
    // Create a simple shell script (simulating an ELF binary)
    let elf_content = format!(
        "#!/bin/bash\n\necho 'Running {} (simulated ELF binary)'\nexit 0",
        new_name
    );

    let elf_path = format!("{}/{}_v0.1.0", USR_BIN_DIR, new_name);

    // Write the file with sudo privileges
    if let Err(e) = fs::write(&elf_path, elf_content) {
        eprintln!(
            "Failed to create ELF file: {}. Ensure you have sudo privileges.",
            e
        );
        process::exit(1);
    }

    // Set executable permissions using chmod
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if let Err(e) = fs::set_permissions(&elf_path, fs::Permissions::from_mode(0o755)) {
            eprintln!(
                "Failed to set executable permissions: {}. Ensure you have sudo privileges.",
                e
            );
            process::exit(1);
        }
    }

    println!("Successfully installed {} as {}", elf_name, new_name);
    println!("ELF binary is located at: {}", elf_path);
    println!("Note: Ensure you have sudo privileges to run this command.");
}

fn uninstall_elf(elf_name: &str) {
    let elf_path = format!("{}/{}_v0.1.0", USR_BIN_DIR, elf_name);

    if Path::new(&elf_path).exists() {
        if let Err(e) = fs::remove_file(&elf_path) {
            eprintln!(
                "Failed to remove ELF file: {}. Ensure you have sudo privileges.",
                e
            );
        } else {
            println!("Successfully uninstalled {}", elf_name);
        }
    } else {
        println!("ELF package {} not found", elf_name);
    }
}
