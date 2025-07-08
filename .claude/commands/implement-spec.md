# Implement Specification

## Purpose
Implement a specification document from the `specs/` directory into working code.

## Usage
```
/implement-spec <spec-id>
```

## Workflow

1. **Read the specification document** from `specs/<spec-id>-*.md`
2. **Analyze the current codebase** to understand existing implementation
3. **Plan the implementation** by identifying:
   - New files to create
   - Existing files to modify
   - Dependencies and imports needed
   - Integration points with existing code
4. **Implement the specification** following the detailed requirements
5. **Test the implementation** to ensure it works correctly
6. **Update any related documentation** if needed

## Guidelines

- **Follow the spec exactly** - implement all requirements as specified
- **Maintain code quality** - use proper Go conventions, error handling, and documentation
- **Integrate seamlessly** - work with existing architecture and patterns
- **Test thoroughly** - ensure the implementation works end-to-end
- **Handle errors gracefully** - implement proper error handling as specified
- **Use existing patterns** - follow the same coding style and structure as the rest of the codebase

## Example
```
/implement-spec 110
```
This would implement the calibration improvements specification (specs/110-calibration-improvements.md). 