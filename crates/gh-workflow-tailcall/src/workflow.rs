//! Workflow for all tailcall projects free and open source for everyone.

use ctx::Context;
use derive_setters::Setters;
use gh_workflow::*;
use release_plz::{Command, Release};
use toolchain::Toolchain;

#[derive(Default, Debug, Clone, Setters)]
pub struct TailcallWorkflow {
    pub release: bool,
}

impl TailcallWorkflow {}

impl From<TailcallWorkflow> for Workflow {
    fn from(value: TailcallWorkflow) -> Self {
        let flags = RustFlags::deny("warnings");

        let event = Event::default()
            .push(Push::default().add_branch("main"))
            .pull_request(
                PullRequest::default()
                    .add_type(PullRequestType::Opened)
                    .add_type(PullRequestType::Synchronize)
                    .add_type(PullRequestType::Reopened)
                    .add_branch("main"),
            );

        let is_main = Context::github().ref_().eq("refs/heads/main".into());
        let is_push = Context::github().event_name().eq("push".into());
        let cond = is_main.and(is_push);

        // Jobs
        let build = build_and_test();
        let mut workflow = Workflow::new("CI")
            .add_env(flags)
            .on(event)
            .add_job("build", build.clone());

        if value.release {
            let permissions = Permissions::default()
                .pull_requests(Level::Write)
                .packages(Level::Write)
                .contents(Level::Write);

            let release = release_job(&cond, &build, &permissions);
            let release_pr = release_pr_job(cond, &build, permissions);
            workflow = workflow
                .add_job("release", release)
                .add_job("release-pr", release_pr);
        }

        workflow
    }
}

fn release_pr_job(cond: Context<bool>, build: &Job, permissions: Permissions) -> Job {
    Job::new("Release PR")
        .cond(cond.clone())
        .concurrency(
            Concurrency::new(Expression::new("release-${{github.ref}}")).cancel_in_progress(false),
        )
        .add_needs(build.clone())
        .add_env(Env::github())
        .add_env(Env::new(
            "CARGO_REGISTRY_TOKEN",
            "${{ secrets.CARGO_REGISTRY_TOKEN }}",
        ))
        .permissions(permissions)
        .add_step(Step::checkout())
        .add_step(Release::default().command(Command::ReleasePR))
}

fn release_job(cond: &Context<bool>, build: &Job, permissions: &Permissions) -> Job {
    Job::new("Release")
        .cond(cond.clone())
        .add_needs(build.clone())
        .add_env(Env::github())
        .add_env(Env::new(
            "CARGO_REGISTRY_TOKEN",
            "${{ secrets.CARGO_REGISTRY_TOKEN }}",
        ))
        .permissions(permissions.clone())
        .add_step(Step::checkout())
        .add_step(Release::default().command(Command::Release))
}

fn build_and_test() -> Job {
    Job::new("Build and Test")
        .permissions(Permissions::default().contents(Level::Read))
        .add_step(Step::checkout())
        .add_step(
            Toolchain::default()
                .add_stable()
                .add_nightly()
                .add_clippy()
                .add_fmt(),
        )
        .add_step(
            Cargo::new("test")
                .args("--all-features --workspace")
                .name("Cargo Test"),
        )
        .add_step(
            Cargo::new("fmt")
                .nightly()
                .args("--check")
                .name("Cargo Fmt"),
        )
        .add_step(
            Cargo::new("clippy")
                .nightly()
                .args("--all-features --workspace -- -D warnings")
                .name("Cargo Clippy"),
        )
}
