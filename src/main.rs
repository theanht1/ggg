extern crate ggg;

use log;
use std::path::Path;
use clap::{App, Arg, crate_authors, crate_version};
use ggg::{list_templates, find_template, create_file};



fn list_command() {
    let templates = list_templates();

    for template in templates {
        println!("{}", template.name);
    }
}

fn generate_command(template_name: &str, outfile_path: &str, is_force: bool) {
    match find_template(template_name) {
        Some(template) => {
            if Path::new(outfile_path).exists() {
                if !is_force {
                    log::error!("'{}' already existed, please use '-f' option to override it",
                                outfile_path);
                    return;
                }
            }
            create_file(template, outfile_path);
            println!("Done!");
        },
        None => log::error!("Template '{}' not found", template_name),
    }
}

fn main() {
    env_logger::init();

    let matches = App::new("ggg")
        .version(crate_version!())
        .about("Generate gitignore file your project")
        .author(crate_authors!())
        .arg(Arg::with_name("template")
             .about("template name for gitignore file")
             .index(1)
             .conflicts_with("list")
             .required(false))
        .arg(Arg::with_name("list")
             .short('l')
             .long("list")
             .about("list all templates"))
        .arg(Arg::with_name("outfile")
             .short('o')
             .long("outfile")
             .default_value(".gitignore")
             .about("destination for generated ignore file"))
        .arg(Arg::with_name("force")
             .short('f')
             .long("force")
             .about("force to override if 'outfile' exists"))
        .get_matches();

    if matches.is_present("list") {
         list_command();
    } else if matches.is_present("template") {
        generate_command(
            matches.value_of("template").unwrap(),
            matches.value_of("outfile").unwrap(),
            matches.is_present("force"),
        );
    }
}
