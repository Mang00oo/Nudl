use std::process::Command;
use std::env;
use std::fs;
mod translator;
mod defaults;

fn main() {
    let action = std::env::args().nth(1).expect("No action arguement given.");

    if action == "run" {
        println!("running some program");
    } else if action == "create" {
        let name = std::env::args().nth(2).expect("No project name given.");
        println!("Creating new project with name: {:?}...", &name);

        let mut current_dir = env::current_dir().expect("Failed to get current directory");
        
        fs::create_dir(&name).expect("Could not create project: failed to init directory");
        current_dir.push(&name);
        fs::create_dir(format!("{}/{}", &name, "src")).expect("Could not create project: failed to init directory");
        fs::create_dir(format!("{}/{}", &name, "maps")).expect("Could not create project: failed to init directory");
        let flavorFile = fs::write(format!("{}/flavor.json", &name), defaults::flavorFileDefault());
        let mapFile = fs::write(format!("{}/maps/std.json", &name), defaults::stdMapDefault());
        let mainFile = fs::write(format!("{}/src/main.nudl", &name), defaults::mainScriptDefault());
        let rustOutput = Command::new("cargo")
            .current_dir(&current_dir)
            .arg("new")
            .arg(&name)
            .output()
            .expect("Couldn't create Rust project.");
        std::fs::rename(format!("{}/{}", &name, &name), format!("{}/translated", &name));
        println!("Created {:?} successfully!", &name);
    } else if action == "translate" {
        println!("Translating project to Rust...");
        translator::translate();
        println!("Translated successfully!");
    } else if action == "build" {
        println!("building some program");
    } else if action == "version" {
        println!("Nudl version: 0.1.0-alpha");
    }
}
