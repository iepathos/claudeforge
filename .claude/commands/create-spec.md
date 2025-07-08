# /create-spec

Create a detailed specification file for a new feature or improvement. This command generates comprehensive specs in the /specs directory following the project's conventions and guidelines to ensure Claude Code can implement the feature effectively.

## Variables

FEATURE_DESCRIPTION: $ARGUMENTS (required - describe the feature to implement)

## Execute

### Phase 1: Analysis and Planning

1. Analyze the feature description to understand:
   - What functionality is being requested
   - How it fits into the existing codebase
   - What components it will interact with
   - What dependencies it might need

2. Determine the next spec number:
   - Find the highest numbered spec file in /specs directory
   - Increment by 10 to get the next spec number (e.g., 150 → 160)
   - Generate a feature name from the description for the filename

3. Review existing codebase structure:
   - Check relevant packages and interfaces
   - Identify similar patterns or implementations
   - Consider integration points with existing features

4. Generate feature title for status tracking:
   - Create a concise, descriptive title from the feature description
   - Use title case formatting (e.g., "Mouse Wheel Scrolling" not "mouse wheel scrolling")
   - Keep it under 40 characters for table formatting

### Phase 2: Spec Creation

Create a comprehensive specification file at `/specs/{number}-{feature-name}.md` with the following sections:

1. **Feature Summary**
   - Clear description of what this feature does
   - Why it's needed and what problem it solves
   - High-level overview of the approach

2. **Goals & Requirements**
   - Specific, measurable objectives
   - Functional requirements
   - Non-functional requirements (performance, reliability, etc.)
   - Success criteria

3. **API/Interface Design**
   - Function signatures and types
   - Public interfaces and structs
   - Method specifications with parameters and return values
   - Error handling patterns

4. **File and Package Structure**
   - Where code should be located (internal/, cmd/, etc.)
   - Package organization and naming
   - File naming conventions
   - Import structure

5. **Implementation Details**
   - Step-by-step implementation approach
   - Code examples and patterns
   - Key algorithms or logic
   - Integration with existing systems

6. **Testing Strategy**
   - Unit test requirements
   - Integration test scenarios
   - Test file structure and naming
   - Mock requirements and test data

7. **Edge Cases & Error Handling**
   - Potential failure modes
   - Error handling patterns
   - Recovery strategies
   - Validation requirements

8. **Dependencies**
   - External libraries needed
   - Internal package dependencies
   - Version requirements
   - Platform-specific considerations

9. **Configuration**
   - Config options to expose
   - Default values
   - Environment variables
   - Viper integration

10. **Documentation**
    - GoDoc requirements
    - README updates needed
    - Example usage
    - API documentation

### Phase 3: Validation and Finalization

1. Ensure the spec follows project conventions:
   - Uses Go idioms and patterns
   - Follows naming conventions from CLAUDE.md
   - Integrates with existing architecture
   - Includes proper error handling

2. Verify completeness:
   - All sections are filled out
   - Code examples are realistic
   - Testing approach is comprehensive
   - Dependencies are identified

3. Generate the spec file with complete content ready for implementation

### Phase 4: Status File Update

1. Update the specs status file (`specs/000-specs-status.md`):
   - Add the new spec to the "In Progress / Planned Specs" table
   - Use the format: `| {spec_number} | {feature_title} | ⬜ Planned |`
   - Maintain alphabetical/numerical ordering in the table
   - Update the "Notes" section if needed

2. Ensure the status file remains properly formatted:
   - Keep table headers consistent
   - Maintain proper markdown table syntax
   - Preserve existing completed specs
   - Keep the document structure intact

### Phase 5: Git Commit

1. Add both files to git:
   - `git add specs/{number}-{feature-name}.md`
   - `git add specs/000-specs-status.md`

2. Create a commit with a descriptive message:
   - Format: `add spec {number}: {feature-title}`
   - Example: `add spec 260: Mouse Wheel Scrolling`
   - Include the 🤖 signature as per project conventions

3. Commit the changes:
   - Use proper commit message format
   - Include Claude Code attribution
   - Make sure both files are included in the commit

## Example Usage

```
/create-spec "add mouse wheel scrolling support for cursor control"
/create-spec "implement voice command recognition for accessibility"
/create-spec "add multi-monitor support for eye tracking"
/create-spec "create plugin system for custom gesture recognition"
/create-spec "implement data export functionality for usage analytics"
```

## Output Format

The command will create:
1. A spec file following this naming pattern:
   - `{next-number}-{feature-name}.md`
   - Example: `160-mouse-wheel-scrolling.md`
   - Example: `170-voice-command-recognition.md`

2. Update the specs status file (`specs/000-specs-status.md`):
   - Add new spec to "In Progress / Planned Specs" table
   - Maintain proper table formatting and ordering

3. Create a git commit with both files:
   - Commit message: `add spec {number}: {feature-title}`
   - Include Claude Code attribution
   - Both spec file and status file in the commit

## Integration with Other Commands

- Once spec is created, use `/implement-spec` to implement the feature
- Use `/commit-spec` to commit the completed implementation
- Specs integrate with the overall project development workflow

## Quality Standards

The generated spec must be:
- Detailed enough for Claude Code to implement without ambiguity
- Following Go idioms and project conventions
- Compatible with existing codebase architecture
- Testable and maintainable
- Complete with all necessary sections filled out
- Ready for immediate implementation

The status file update must:
- Maintain proper markdown table formatting
- Use consistent status indicators (✅ Complete, ⬜ Planned, 🔄 In Progress)
- Keep feature titles concise and descriptive
- Preserve existing completed specs and their status