# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v0.1.0 (2024-11-08)

<csr-id-7f8b45f3f2065fc953da5fd4447183d0bcb94e38/>
<csr-id-d73d9faf093e15fc9d91a9318ca84f6113a310b0/>
<csr-id-1a004fc27cb6c44a6b76d5ca20b50dbfc90b4efe/>
<csr-id-55f52de53065d626aaea8007651251a9a86acc98/>
<csr-id-90545329d44378175e1bbabe5595868720961dad/>
<csr-id-dccdb3612602559e3c39e83ae3894086e06c5a5d/>
<csr-id-5e43aa120b9f37227bc0d1e9d2c3c840a652319e/>
<csr-id-cbf51abc82da429539cff463aebb83f941b62922/>

### Chore

 - <csr-id-7f8b45f3f2065fc953da5fd4447183d0bcb94e38/> update license
 - <csr-id-d73d9faf093e15fc9d91a9318ca84f6113a310b0/> add license
 - <csr-id-1a004fc27cb6c44a6b76d5ca20b50dbfc90b4efe/> update build
 - <csr-id-55f52de53065d626aaea8007651251a9a86acc98/> accept path `AsRef<Path>`
 - <csr-id-90545329d44378175e1bbabe5595868720961dad/> add warning in generated config
 - <csr-id-dccdb3612602559e3c39e83ae3894086e06c5a5d/> autogen CI
 - <csr-id-5e43aa120b9f37227bc0d1e9d2c3c840a652319e/> update folder structure

### Chore

 - <csr-id-cbf51abc82da429539cff463aebb83f941b62922/> add changelog

### New Features

 - <csr-id-8f3615d7d3e6b71946b09c68a4ec8dc7cb3418a7/> validate on CI
 - <csr-id-d770fc8bec19a8dd6e7b680b8d61819383b50498/> add events

### Bug Fixes

 - <csr-id-e91cd8944cfa9cf758889c4157f12bc01758fee1/> tests
   * refactor: move tests from `workflow.rs` to tests dir
* remove unused deps
* move fixtures dir
* chore: rename files
* chore: import updates
* chore: update workflows
* chore: update workflow
* fix err on workflow-bench.yml
* partial
* partially fix tests
* fix tests
* drop unused struct
* revert unwanted change in `needs`
* revert unwanted change in `needs`
* drop `build_matrix.yml`
* revert

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 15 commits contributed to the release.
 - 11 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 4 unique issues were worked on: [#13](https://github.com/tailcallhq/rust-gh-workflows/issues/13), [#14](https://github.com/tailcallhq/rust-gh-workflows/issues/14), [#19](https://github.com/tailcallhq/rust-gh-workflows/issues/19), [#21](https://github.com/tailcallhq/rust-gh-workflows/issues/21)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#13](https://github.com/tailcallhq/rust-gh-workflows/issues/13)**
    - Tests ([`e91cd89`](https://github.com/tailcallhq/rust-gh-workflows/commit/e91cd8944cfa9cf758889c4157f12bc01758fee1))
 * **[#14](https://github.com/tailcallhq/rust-gh-workflows/issues/14)**
    - Autogen CI ([`dccdb36`](https://github.com/tailcallhq/rust-gh-workflows/commit/dccdb3612602559e3c39e83ae3894086e06c5a5d))
 * **[#19](https://github.com/tailcallhq/rust-gh-workflows/issues/19)**
    - Accept path `AsRef<Path>` ([`55f52de`](https://github.com/tailcallhq/rust-gh-workflows/commit/55f52de53065d626aaea8007651251a9a86acc98))
 * **[#21](https://github.com/tailcallhq/rust-gh-workflows/issues/21)**
    - Validate on CI ([`8f3615d`](https://github.com/tailcallhq/rust-gh-workflows/commit/8f3615d7d3e6b71946b09c68a4ec8dc7cb3418a7))
 * **Uncategorized**
    - Release gh-workflow v0.1.0 ([`4602f22`](https://github.com/tailcallhq/rust-gh-workflows/commit/4602f22c2437cad2467ee083402b0eb7f29ab045))
    - Add changelog ([`cbf51ab`](https://github.com/tailcallhq/rust-gh-workflows/commit/cbf51abc82da429539cff463aebb83f941b62922))
    - Update license ([`7f8b45f`](https://github.com/tailcallhq/rust-gh-workflows/commit/7f8b45f3f2065fc953da5fd4447183d0bcb94e38))
    - Update manifest ([`770b3e3`](https://github.com/tailcallhq/rust-gh-workflows/commit/770b3e33773db936b91a416fb3ac26c809b2ad14))
    - Add license ([`d73d9fa`](https://github.com/tailcallhq/rust-gh-workflows/commit/d73d9faf093e15fc9d91a9318ca84f6113a310b0))
    - Update build ([`1a004fc`](https://github.com/tailcallhq/rust-gh-workflows/commit/1a004fc27cb6c44a6b76d5ca20b50dbfc90b4efe))
    - Update event ([`a0bf957`](https://github.com/tailcallhq/rust-gh-workflows/commit/a0bf95769e66eed96cc9b5b81c51bcf38e1b49fc))
    - Drop commented code ([`fe53956`](https://github.com/tailcallhq/rust-gh-workflows/commit/fe539566c65313f19f8773fea6cf78aa49cb7e65))
    - Add events ([`d770fc8`](https://github.com/tailcallhq/rust-gh-workflows/commit/d770fc8bec19a8dd6e7b680b8d61819383b50498))
    - Add warning in generated config ([`9054532`](https://github.com/tailcallhq/rust-gh-workflows/commit/90545329d44378175e1bbabe5595868720961dad))
    - Update folder structure ([`5e43aa1`](https://github.com/tailcallhq/rust-gh-workflows/commit/5e43aa120b9f37227bc0d1e9d2c3c840a652319e))
</details>

