use gh_workflow::*;

fn main() {
    let rust_flags = RustFlags::deny("warnings");

    Workflow::new("CI")
        .env(rust_flags)
        .permissions(Permissions::read())
        .on(Event::push().branch("main"))
        .on(Event::pull_request()
            .open()
            .synchronize()
            .reopen()
            .branch("main"))
        .add_job(
            "build",
            Job::new("Build and Test")
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
                )),
        )
        .unwrap()
        .generate()
        .unwrap();
}
