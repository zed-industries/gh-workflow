use gh_workflow::generate::Generate;
use gh_workflow::*;

fn main() {
    Generate::new(Workflow::setup_rust())
        .name("ci.yml")
        .generate()
        .unwrap();
}
