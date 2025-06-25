# Contributing to dbtective

Welcome! 🎩  
Thank you for your interest in contributing to **dbt-tective**. We value all contributions—whether it’s bug reports, features, documentation, or feedback.

## How to Contribute

### 1. Open Issues First

- **Bugs:** If you find a bug, open an [issue](../../issues/new) describing the problem and steps to reproduce.
- **Feature Requests:** Please propose new features or changes by opening an issue first. This allows for discussion before you start work.
- **Questions:** Feel free to open issues for usage questions or clarifications.

### 2. Fork and Branch

- Fork the repository to your own GitHub account.
- Create a new branch for your fix or feature:  
  ```
  git checkout -b feat/short-description
  ```
- Work on your branch and make commits.

### 3. Commit Message Guidelines

- **We use [gitmoji](https://gitmoji.dev) for commit messages!**
- Every commit message should start with the appropriate emoji and follow the gitmoji convention.
- It should contain a subject type (e.g. feat/fix/docs/test/refactor) between []
- Examples:  
  ```
  ✨ [feat] Add detective mascot to CLI output
  🐛 [fix] Resolve parsing error in SQL files
  📝 [docs] Update installation instructions
  ♻️ [refactor] Simplify error handling logic
  ✅ [test] Add unit tests for parser
  ```

- Please use clear, descriptive messages.

### 4. Sync and Rebase

- **Always rebase on the latest `main` before submitting a pull request.**  
  This helps keep the history clean and avoids unnecessary merge commits.
  ```
  git fetch upstream
  git rebase upstream/main
  ```

### 5. Code Style

- Follow Rust best practices and conventions.
- Run `cargo fmt` and `cargo clippy` before submitting your PR.
- Add or update tests as appropriate.

### 6. Commit and PR Guidelines

- **One logical change per PR.** Separate unrelated changes into different pull requests.
- Reference any relevant open issues in your PR description.
- Keep your pull requests focused and small for easier review.

### 7. Pull Request Process

- Open your pull request against the `main` branch.
- **All PRs must be up-to-date with `main` and have no merge conflicts.**
- **Rebase-only merge strategy:**  
  We do not allow merge commits or squashing.  
  _All PRs will be merged using **rebase and merge** only._
- Add a clear description of what your PR changes and why.

### 8. Reviews and Merging

- A maintainer will review your PR.
- Address all comments and requested changes.
- Once approved, a maintainer will rebase and merge your PR, or ask you to do a final rebase.

### 9. Code of Conduct

Be respectful and considerate. See our [Code of Conduct](./CODE_OF_CONDUCT.md).

---

## Need Help?

- Check [open issues](../../issues) to see if your question is already answered.
- For anything else, feel free to open a new issue!

---

Thank you for helping make **dbt-tective** better!  
( •_•)>⌐■-■  
(⌐■_■)   Case solved!