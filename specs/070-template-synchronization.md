# Spec 070: Template Synchronization

## Feature Summary

This feature adds the ability to synchronize templates across multiple repositories after they've been created with claudeforge. It enables users to maintain a base template repository and propagate updates to derived projects while preserving project-specific customizations through variable substitution and selective file updates.

The primary use case is maintaining consistency across multiple projects that share common configurations (like `.claude` files and GitHub workflows) while allowing for minor differences between projects.

## Goals & Requirements

### Functional Requirements
- Support updating existing projects with changes from base templates
- Preserve project-specific customizations through templating variables
- Allow selective file synchronization (choose which files to update)
- Support dry-run mode to preview changes before applying
- Handle conflicts gracefully with clear reporting
- Support multiple template sources (local directories, git repos)
- Track template version/source for each synchronized project

### Non-Functional Requirements
- Maintain backward compatibility with existing claudeforge functionality
- Provide clear feedback on what changes will be/were made
- Ensure operations are idempotent (can be run multiple times safely)
- Support rollback or undo functionality
- Fast performance for large template sets

### Success Criteria
- Users can update multiple repos with template changes in a single command
- Project-specific variables are preserved during updates
- Clear diff output shows what will change before applying
- No data loss or unexpected file modifications
- Integration with existing claudeforge template system

## API/Interface Design

### CLI Commands

```go
// New command to sync templates to existing projects
type SyncCmd struct {
    Source      string   `short:"s" help:"Template source (path or git URL)" required:""`
    Target      string   `short:"t" help:"Target directory or repo to update" default:"."`
    Files       []string `short:"f" help:"Specific files/patterns to sync (default: all)"`
    DryRun      bool     `help:"Preview changes without applying them"`
    Interactive bool     `short:"i" help:"Interactively approve each change"`
    Variables   []string `short:"v" help:"Variables in key=value format"`
    VarFile     string   `help:"Path to variables file"`
    Force       bool     `help:"Force update even with conflicts"`
}

// Command to initialize tracking in a project
type InitSyncCmd struct {
    Template string `help:"Template source to track" required:""`
    Version  string `help:"Template version/commit to pin to"`
}

// Command to show sync status
type SyncStatusCmd struct {
    Verbose bool `help:"Show detailed diff of changes"`
}
```

### Core Interfaces

```go
// TemplateSynchronizer handles syncing templates to projects
type TemplateSynchronizer interface {
    // Sync updates target with changes from source
    Sync(source TemplateSource, target string, opts SyncOptions) (*SyncResult, error)
    
    // Preview shows what would change without applying
    Preview(source TemplateSource, target string, opts SyncOptions) (*SyncPreview, error)
    
    // Status checks current sync state of a project
    Status(projectPath string) (*SyncStatus, error)
}

// TemplateSource represents a template source
type TemplateSource interface {
    // GetFiles returns files available in the template
    GetFiles() ([]TemplateFile, error)
    
    // GetFile retrieves a specific file's content
    GetFile(path string) ([]byte, error)
    
    // GetMetadata returns template metadata
    GetMetadata() (*TemplateMetadata, error)
}

// SyncOptions configures synchronization behavior
type SyncOptions struct {
    Files       []string          // Specific files to sync
    Variables   map[string]string // Variable substitutions
    DryRun      bool              // Preview mode
    Interactive bool              // Prompt for each change
    Force       bool              // Overwrite conflicts
    Exclude     []string          // Patterns to exclude
}

// SyncResult contains results of a sync operation
type SyncResult struct {
    Updated    []FileUpdate   // Files that were updated
    Created    []string       // New files created
    Skipped    []SkipReason   // Files skipped and why
    Conflicts  []Conflict     // Conflicts encountered
    Success    bool           // Overall success status
}
```

## File and Package Structure

```
internal/
├── sync/                    # Template synchronization package
│   ├── sync.go             # Main synchronizer implementation
│   ├── source.go           # Template source interfaces
│   ├── source_local.go     # Local directory source
│   ├── source_git.go       # Git repository source
│   ├── diff.go             # Diff generation and display
│   ├── conflict.go         # Conflict detection and resolution
│   ├── variables.go        # Variable substitution logic
│   └── tracking.go         # Template tracking metadata
│
├── tracking/               # Project tracking package
│   ├── metadata.go         # .claudeforge metadata handling
│   ├── version.go          # Version tracking
│   └── store.go           # Metadata storage
│
cmd/
├── sync.go                # Sync command implementation
├── init_sync.go           # Init-sync command
└── sync_status.go         # Sync-status command

tests/
├── sync_test.go           # Sync functionality tests
├── fixtures/              # Test fixtures
│   ├── templates/         # Test templates
│   └── projects/          # Test projects
```

## Implementation Details

### Step 1: Template Tracking

Add `.claudeforge` metadata file to track template source:

```yaml
# .claudeforge
template:
  source: "github.com/user/base-templates"
  version: "v1.2.0"
  commit: "abc123..."
  files:
    - path: ".claude"
      checksum: "sha256:..."
      variables:
        - "PROJECT_NAME"
        - "GITHUB_REPO"
    - path: ".github/workflows/ci.yml"
      checksum: "sha256:..."
      variables:
        - "NODE_VERSION"
```

### Step 2: Diff Generation

```go
func (s *Synchronizer) generateDiff(source, target string, file TemplateFile) (*FileDiff, error) {
    // Read source template
    sourceContent, err := s.source.GetFile(file.Path)
    if err != nil {
        return nil, err
    }
    
    // Apply variable substitution
    processed := s.processTemplate(sourceContent, s.opts.Variables)
    
    // Read target file if exists
    targetPath := filepath.Join(target, file.Path)
    targetContent, err := os.ReadFile(targetPath)
    if os.IsNotExist(err) {
        return &FileDiff{
            Path:   file.Path,
            Action: ActionCreate,
            New:    processed,
        }, nil
    }
    
    // Generate unified diff
    diff := difflib.UnifiedDiff{
        A:        difflib.SplitLines(string(targetContent)),
        B:        difflib.SplitLines(string(processed)),
        FromFile: targetPath,
        ToFile:   targetPath,
        Context:  3,
    }
    
    return &FileDiff{
        Path:   file.Path,
        Action: ActionUpdate,
        Diff:   diff,
    }, nil
}
```

### Step 3: Variable Processing

```go
func (s *Synchronizer) processTemplate(content []byte, vars map[string]string) []byte {
    // Use existing template engine from claudeforge
    tmpl := template.New("sync")
    
    // Add custom functions for sync-specific needs
    tmpl.Funcs(template.FuncMap{
        "default": func(def, val interface{}) interface{} {
            if val == nil || val == "" {
                return def
            }
            return val
        },
        "env": os.Getenv,
    })
    
    // Parse and execute template
    parsed, _ := tmpl.Parse(string(content))
    var buf bytes.Buffer
    parsed.Execute(&buf, vars)
    
    return buf.Bytes()
}
```

### Step 4: Conflict Detection

```go
func (s *Synchronizer) detectConflict(target string, file TemplateFile) (*Conflict, error) {
    targetPath := filepath.Join(target, file.Path)
    
    // Check if file was modified locally
    tracking := s.getTracking(target)
    if tracking == nil {
        return nil, nil // No tracking, can't detect conflicts
    }
    
    fileInfo := tracking.GetFile(file.Path)
    if fileInfo == nil {
        return nil, nil // File not tracked
    }
    
    // Compare checksums
    currentChecksum, _ := s.calculateChecksum(targetPath)
    if currentChecksum != fileInfo.Checksum {
        return &Conflict{
            Path:   file.Path,
            Type:   ConflictLocalModification,
            Detail: "File was modified locally since last sync",
        }, nil
    }
    
    return nil, nil
}
```

## Testing Strategy

### Unit Tests

```go
// Test variable substitution
func TestProcessTemplate(t *testing.T) {
    sync := NewSynchronizer()
    vars := map[string]string{
        "PROJECT_NAME": "my-app",
        "NODE_VERSION": "18",
    }
    
    template := []byte(`
    name: {{.PROJECT_NAME}}
    node: {{.NODE_VERSION}}
    `)
    
    result := sync.processTemplate(template, vars)
    assert.Contains(t, string(result), "name: my-app")
    assert.Contains(t, string(result), "node: 18")
}

// Test conflict detection
func TestConflictDetection(t *testing.T) {
    // Setup test project with tracking
    // Modify tracked file
    // Run conflict detection
    // Assert conflict detected
}
```

### Integration Tests

```go
// Test full sync workflow
func TestSyncWorkflow(t *testing.T) {
    // Create test template
    // Create test project from template
    // Modify template
    // Run sync
    // Verify updates applied correctly
}

// Test with Git source
func TestGitTemplateSource(t *testing.T) {
    // Clone test repo
    // Use as template source
    // Verify sync works
}
```

## Edge Cases & Error Handling

### Edge Cases
- Syncing to a non-claudeforge project
- Template files deleted between syncs
- Circular template dependencies
- Binary files in templates
- Symlinks and special files
- Case-sensitive filesystem issues
- Very large template files
- Network failures during Git operations

### Error Handling
```go
// Custom error types
type SyncError struct {
    Path    string
    Op      string
    Err     error
}

type ConflictError struct {
    Conflicts []Conflict
}

// Graceful degradation
func (s *Synchronizer) Sync() (*SyncResult, error) {
    result := &SyncResult{}
    
    for _, file := range files {
        err := s.syncFile(file)
        if err != nil {
            if conflict, ok := err.(*ConflictError); ok {
                result.Conflicts = append(result.Conflicts, conflict.Conflicts...)
                continue // Skip this file but continue
            }
            // Hard error, stop sync
            return result, err
        }
        result.Updated = append(result.Updated, file)
    }
    
    result.Success = len(result.Conflicts) == 0
    return result, nil
}
```

## Dependencies

### External Dependencies
- `github.com/spf13/cobra` - CLI command handling (existing)
- `github.com/spf13/viper` - Configuration (existing)
- `github.com/go-git/go-git/v5` - Git operations
- `github.com/sergi/go-diff` - Diff generation
- `github.com/charmbracelet/lipgloss` - Terminal UI (existing)
- `gopkg.in/yaml.v3` - YAML parsing for metadata

### Internal Dependencies
- `internal/template` - Existing template engine
- `internal/config` - Configuration handling
- `internal/forge` - Core claudeforge functionality

## Configuration

### Config Options
```yaml
# ~/.claudeforge/config.yaml
sync:
  # Default variable values
  defaults:
    NODE_VERSION: "18"
    GO_VERSION: "1.21"
  
  # Auto-sync on forge new
  auto_track: true
  
  # Exclude patterns
  exclude:
    - "*.log"
    - ".git"
    - "node_modules"
  
  # Conflict resolution
  conflicts:
    strategy: "prompt" # prompt, skip, overwrite
    backup: true      # Create .bak files
```

### Environment Variables
```bash
CLAUDEFORGE_SYNC_SOURCE    # Default template source
CLAUDEFORGE_SYNC_VARS      # Default variables (JSON)
CLAUDEFORGE_SYNC_AUTO      # Enable auto-sync
```

## Documentation

### Command Help
```
claudeforge sync - Synchronize templates to existing projects

Usage:
  claudeforge sync [flags]

Examples:
  # Sync from local template directory
  claudeforge sync -s ../base-templates

  # Sync specific files with variables
  claudeforge sync -s github.com/org/templates -f .claude -v PROJECT=myapp

  # Preview changes without applying
  claudeforge sync -s ../templates --dry-run

  # Interactive mode
  claudeforge sync -s ../templates -i

Flags:
  -s, --source string     Template source (required)
  -t, --target string     Target directory (default ".")
  -f, --files strings     Specific files to sync
  -v, --variables strings Variables (key=value)
      --var-file string   Variables file path
      --dry-run          Preview changes only
  -i, --interactive      Approve each change
      --force            Force overwrite conflicts
```

### README Updates
- Add "Template Synchronization" section
- Document `.claudeforge` metadata format
- Provide sync workflow examples
- Explain conflict resolution strategies
- Add troubleshooting guide

### GoDoc
```go
// Package sync provides template synchronization functionality for claudeforge.
// It enables updating existing projects with changes from base templates while
// preserving project-specific customizations.
//
// Basic usage:
//
//     syncer := sync.NewSynchronizer()
//     result, err := syncer.Sync(source, target, sync.Options{
//         Variables: map[string]string{"PROJECT": "myapp"},
//     })
//
package sync
```