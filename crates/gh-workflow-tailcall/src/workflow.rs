//! Workflow is designed to be used for most Rust projects that are built at
//! Tailcall. Though gh-workflow makes it much easier to write workflows you
//! still need to constantly keep referring to the Github documentation to write
//! your own workflows. This module saves all that time by using feature flags
//! to enable or disable features that you want in your workflow. Based on the
//! features enabled or disabled a workflow is generated.

use ctx::Context;
use derive_setters::Setters;
use gh_workflow::error::Result;
use gh_workflow::{Workflow as GHWorkflow, *};
use release_plz::{Command, Release};
use toolchain::Toolchain;

#[derive(Debug, Clone, Setters)]
pub struct Workflow {
    /// When enabled, a release job is added to the workflow.
    /// *IMPORTANT:* Ensure `secrets.CARGO_REGISTRY_TOKEN` is set for your
    /// github action.
    pub auto_release: bool,

    /// Name of the workflow.
    pub name: String,

    /// When enabled, a benchmark job is added to the workflow.
    pub benchmarks: bool,

    /// Auto-fixes the code after
    pub auto_fix: bool,
}

impl Default for Workflow {
    fn default() -> Self {
        Self {
            auto_release: false,
            name: "CI".into(),
            benchmarks: false,
            auto_fix: false,
        }
    }
}

impl Workflow {
    /// Generates and tests the workflow file.
    pub fn generate(self) -> Result<()> {
        let workflow: GHWorkflow = self.into();
        workflow.generate()
    }

    /// Converts the workflow into a Github workflow.
    pub fn to_github_workflow(&self) -> GHWorkflow {
        GHWorkflow::new(self.name.clone())
            .add_env(self.workflow_flags())
            .on(self.workflow_event())
            .add_job("build", self.test_job())
            .add_job("lint", self.lint_job())
            .add_job_when(self.auto_release, "release", self.release_job())
            .add_job_when(self.auto_release, "release-pr", self.release_pr_job())
    }

    fn release_pr_job(&self) -> Job {
        Job::new("Release PR")
            .cond(self.workflow_cond())
            .concurrency(
                Concurrency::new(Expression::new("release-${{github.ref}}"))
                    .cancel_in_progress(false),
            )
            .add_needs(self.test_job())
            .add_needs(self.lint_job())
            .add_env(Env::github())
            .add_env(Env::new(
                "CARGO_REGISTRY_TOKEN",
                "${{ secrets.CARGO_REGISTRY_TOKEN }}",
            ))
            .permissions(self.write_permissions())
            .add_step(Step::checkout())
            .add_step(Release::default().command(Command::ReleasePR))
    }

    fn release_job(&self) -> Job {
        Job::new("Release")
            .cond(self.workflow_cond())
            .add_needs(self.test_job())
            .add_needs(self.lint_job())
            .add_env(Env::github())
            .add_env(Env::new(
                "CARGO_REGISTRY_TOKEN",
                "${{ secrets.CARGO_REGISTRY_TOKEN }}",
            ))
            .permissions(self.write_permissions())
            .add_step(Step::checkout())
            .add_step(Release::default().command(Command::Release))
    }

    fn lint_job(&self) -> Job {
        Job::new("Lint")
            .permissions(Permissions::default().contents(Level::Read))
            .add_step(Step::checkout())
            .add_step(Toolchain::default().add_nightly().add_clippy().add_fmt())
            .add_step(
                Cargo::new("fmt")
                    .name("Cargo Fmt")
                    .nightly()
                    .add_args("--all")
                    .add_args_when(!self.auto_fix, "--check"),
            )
            .add_step(
                Cargo::new("clippy")
                    .name("Cargo Clippy")
                    .nightly()
                    .add_args_when(self.auto_fix, "--fix")
                    .add_args("--all-features --workspace -- -D warnings"),
            )
    }

    /// Creates the "Build and Test" job for the workflow.
    fn test_job(&self) -> Job {
        Job::new("Build and Test")
            .permissions(Permissions::default().contents(Level::Read))
            .add_step(Step::checkout())
            .add_step(Toolchain::default().add_stable())
            .add_step(
                Cargo::new("test")
                    .args("--all-features --workspace")
                    .name("Cargo Test"),
            )
            .add_step_when(
                self.benchmarks,
                Cargo::new("bench").args("--workspace").name("Cargo Bench"),
            )
    }

    fn write_permissions(&self) -> Permissions {
        Permissions::default()
            .pull_requests(Level::Write)
            .packages(Level::Write)
            .contents(Level::Write)
    }

    fn workflow_cond(&self) -> Context<bool> {
        let is_main = Context::github().ref_().eq("refs/heads/main".into());
        let is_push = Context::github().event_name().eq("push".into());
        let cond = is_main.and(is_push);
        cond
    }

    fn workflow_event(&self) -> Event {
        Event::default()
            .push(Push::default().add_branch("main"))
            .pull_request(
                PullRequest::default()
                    .add_type(PullRequestType::Opened)
                    .add_type(PullRequestType::Synchronize)
                    .add_type(PullRequestType::Reopened)
                    .add_branch("main"),
            )
    }

    fn workflow_flags(&self) -> RustFlags {
        RustFlags::deny("warnings")
    }
}

impl From<Workflow> for GHWorkflow {
    fn from(value: Workflow) -> Self {
        value.to_github_workflow()
    }
}
