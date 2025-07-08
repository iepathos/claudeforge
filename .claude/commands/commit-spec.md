# /commit-spec

Create a git commit after implementing a specification. This command handles the git workflow for committing spec implementations with proper commit messages and verification.

## Variables

SPEC_NAME: $ARGUMENTS (e.g., "010-project-foundation", "090-gesture-recognition")
COMMIT_MESSAGE: $ARGUMENTS (optional, will auto-generate if not provided)

## Execute

### Phase 1: Pre-commit Checks

1. Check if there are any uncommitted changes in the working directory
   - If no changes, exit with message "No changes to commit"
   - If changes exist, proceed to commit

2. Verify the spec file exists
   - Check that `specs/{SPEC_NAME}.md` exists
   - If not found, exit with error "Spec file not found: specs/{SPEC_NAME}.md"

3. Run enhanced pre-commit validation using justfile:
   - Check if justfile exists in project root
   - If justfile exists:
     - Execute `just fmt` to format code
     - Execute `just lint` to run linter
     - Execute `just test` to run unit tests
     - Execute `just test-race` to run race detection tests
   - If justfile doesn't exist, fall back to individual commands:
     - Execute `go mod tidy` to ensure dependencies are clean
     - Execute `go build ./...` to ensure code compiles
     - Execute `go test ./...` to ensure tests pass
   - If any validation fails, exit with error and details

4. Optional CI skip flag:
   - Allow `--skip-ci` flag to bypass CI validation for emergency fixes
   - Log warning when CI is skipped

### Phase 2: Commit Preparation

1. Generate commit message if not provided:
   - Format: `feat: implement {SPEC_NAME} specification`
   - For example: `feat: implement 010-project-foundation specification`

2. Stage all changes:
   - Execute `git add .` to stage all modified files

3. Create the commit:
   - Execute `git commit -m "{COMMIT_MESSAGE}"`

### Phase 3: Post-commit Verification

1. Verify commit was created successfully:
   - Execute `git log --oneline -1` to show the latest commit
   - Display the commit hash and message

2. Show commit summary:
   - Execute `git show --stat` to show files changed
   - Display a summary of what was committed

3. Update spec status:
   - If `specs/000-specs-status.md` exists and the committed spec is listed, update its status to `✅ Complete` in the table.
   - If not listed, optionally add a new entry to the completed specs table.
   - Stage and commit the status update with a message like `chore: update spec status for {SPEC_NAME}`.

4. Provide next steps:
   - Suggest running `git push` if ready to push to remote
   - Mention any follow-up tasks or next specs to implement

## Example Usage

```
/commit-spec 010-project-foundation
/commit-spec 090-gesture-recognition "feat: add TensorFlow Lite hand tracking"
/commit-spec --skip-ci 240-justfile-ci-integration
```

## Error Handling

- If no changes are staged: "No changes to commit"
- If spec file not found: "Spec file not found: specs/{SPEC_NAME}.md"
- If justfile commands fail: Enhanced error reporting with actionable suggestions
  - `just fmt` failure: "💡 Code formatting failed. Try running: just fmt"
  - `just lint` failure: "💡 Linting failed. Try: just lint to see specific issues"
  - `just test` failure: "💡 Unit tests failed. Try: just test to see which tests failed"
  - `just test-race` failure: "💡 Race condition detected. Try: just test-race to see race conditions"
- If justfile not found: "⚠️ Justfile not found, using fallback Go commands"
- If build fails: "Build failed: {error details}"
- If tests fail: "Tests failed: {error details}"
- If git commit fails: "Commit failed: {error details}"
- If unable to update spec status: "Warning: Could not update specs/000-specs-status.md for {SPEC_NAME}"