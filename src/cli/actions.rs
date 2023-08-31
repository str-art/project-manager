use crate::{
    cli::constants::{NEW_NAME, PATH},
    config::structs::Config,
};

use super::constants::NAME;

pub fn add(mut config: Config, matches: &clap::ArgMatches) {
    let project = matches.get_one::<String>(NAME).unwrap();
    let mut path_string = matches.get_one::<String>(PATH).unwrap().to_owned();
    if path_string == "." {
        path_string = String::from("./");
    }

    config.add_project(project, &path_string).unwrap();
    config.save().unwrap();
}
pub fn rename(mut config: Config, matches: &clap::ArgMatches) {
    let current_name = matches.get_one::<String>(NAME).unwrap();
    let new_name = matches.get_one::<String>(NEW_NAME).unwrap();
    config.rename_project(current_name, new_name).unwrap();
    config.save().unwrap();
}
pub fn remove(mut config: Config, matches: &clap::ArgMatches) {
    let project = matches.get_one::<String>(NAME).unwrap();
    config.remove_project(project);
    config.save().unwrap();
}
pub fn goto(mut config: Config, matches: &clap::ArgMatches) {
    let project_name = matches.get_one::<String>(NAME).unwrap();
    match config.get_project(project_name) {
        Some(project) => {}
        None => {
            println!("Project {project_name} not found")
        }
    }
}
pub fn list(config: Config) {
    config.projects.into_iter().for_each(|p| {
        println!("Project {0} at {1}", p.name, p.path.display());
    })
}
