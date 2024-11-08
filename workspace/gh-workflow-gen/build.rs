use gh_workflow::*;

fn main() {
    let job_build = Job::new("Build and Test")
        .add_step(Step::checkout())
        .add_step(
            Step::setup_rust()
                .toolchain_stable()
                .toolchain_nightly()
                .component_clippy()
                .component_rustfmt(),
        )
        // TODO: Improve type-safety and intellisense
        .add_step(Step::cargo("test", vec!["--all-features", "--workspace"]))
        .add_step(Step::cargo_nightly("fmt", vec!["--check"]))
        .add_step(Step::cargo_nightly(
            "clippy",
            vec!["--all-features", "--workspace", "--", "-D warnings"],
        ));

    let on_push = Event::push().branch("main");

    let on_pull_request = Event::pull_request_target()
        .open()
        .synchronize()
        .reopen()
        .branch("main");

    let rust_flags = RustFlags::deny("warnings");

    Workflow::new("CI")
        .env(rust_flags)
        .permissions(Permissions::read())
        .on(on_push)
        .on(on_pull_request)
        .add_job("build", job_build)
        .generate()
        .unwrap();
}
