#![allow(unused)]
use sol_lang::introspec::project::Project;

fn main() {
    let p = Project::load_from_directory("./examples/project".into());
}
