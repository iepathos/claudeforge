# Debug Command

Runs comprehensive debugging analysis for the Gazelle eye tracking project, including tests, builds, and intelligent error analysis optimized for Claude Code workflows.

## Usage
This command performs automated debugging and provides structured error analysis for rapid issue resolution.

## Core Analysis Steps
1. **Test Execution**: Comprehensive test suite with race detection and coverage
2. **Build Verification**: Main prototype and all test applications
3. **Static Analysis**: Code formatting, linting, and quality checks
4. **Error Intelligence**: Advanced error categorization and solution suggestions
5. **Dependency Validation**: Go module integrity and OpenCV prerequisites

## Test Execution Strategy
```bash
# Progressive test execution with detailed output
just test          # Basic unit tests with verbose output
just test-race     # Race condition detection for UI safety
just test-cover    # Coverage analysis with threshold reporting
```

## Build Verification Pipeline
```bash
# Core prototype and test applications
just build         # Main gazelle-prototype executable
just test-apps     # All cmd/* test applications for component validation
just deps          # Go module verification and cleanup
go mod verify      # Module integrity check
```

## Code Quality Analysis
```bash
# Automated formatting and linting
just fmt           # goimports + go fmt with project-wide application
just lint          # golangci-lint with Gazelle-specific rules
```

## Complete CI Pipeline
```bash
# Full continuous integration workflow
just ci            # Sequential: fmt → lint → test → test-race
```

## Advanced Error Analysis & Claude Code Integration

### Intelligent Error Detection
The debug command provides structured error analysis optimized for Claude Code workflows:

1. **Context-Aware Pattern Recognition**:
   - **Race Conditions**: Detects unsafe UI operations, missing `fyne.Do()` calls
   - **OpenCV Integration**: gocv import issues, camera device access failures  
   - **Go Module Issues**: Missing dependencies, version conflicts, replace directives
   - **Fyne UI Errors**: Threading violations, resource lifecycle problems
   - **Test Failures**: Assertion analysis with suggested fixes

2. **Project-Specific Error Categories**:
   - **Camera Interface**: Device access, OpenCV cascade loading, frame processing
   - **Eye Tracking**: Algorithm convergence, calibration data validation
   - **UI Thread Safety**: Concurrent access to Fyne widgets, goroutine management
   - **System Integration**: robotgo cursor control, platform-specific permissions
   - **Performance**: Memory leaks in video processing, frame rate optimization

3. **Claude Code Optimized Diagnostics**:
   - **File Location**: Precise `file_path:line_number` references for easy navigation
   - **Search Patterns**: Provides `rg` commands for investigating related code
   - **Fix Suggestions**: Actionable code changes with context
   - **Test Recommendations**: Specific test cases to verify fixes

### Automated Recovery Actions
```bash
# Safe automatic fixes applied in sequence
just deps          # Go module cleanup: download + tidy
just fmt           # Code formatting: goimports + go fmt  
go mod verify      # Module integrity validation
just check-prereqs # OpenCV and cascade file verification
```

## Example Debug Session Output

```
🔍 Gazelle Debug Analysis - Claude Code Optimized

=== BUILD VERIFICATION ===
✅ just build: gazelle-prototype built successfully
✅ just test-apps: All 6 test applications compiled
❌ just deps: go mod tidy detected inconsistencies

=== TEST EXECUTION ===  
✅ just test: 24/24 tests passed
❌ just test-race: Race condition detected
   📍 internal/ui/calibration_window.go:45 - concurrent widget access
✅ just test-cover: 87.3% coverage (target: 80%+)

=== STATIC ANALYSIS ===
❌ just lint: 2 issues found
   📍 internal/eyetracking/detector.go:123 - unused variable 'threshold'
   📍 cmd/integrated-ui-test/main.go:67 - potential nil pointer dereference

🧠 INTELLIGENT ERROR ANALYSIS

1. **Race Condition** - internal/ui/calibration_window.go:45
   🔍 Pattern: Unsafe UI operation from background goroutine
   💡 Fix: Wrap in fyne.Do() for thread safety
   📝 Code: `fyne.Do(func() { window.Close() })`
   🔎 Search: `rg "window\.Close\(\)" internal/ui/`

2. **Unused Variable** - internal/eyetracking/detector.go:123  
   🔍 Pattern: Variable declared but not used in scope
   💡 Fix: Remove unused variable or implement usage
   🔎 Search: `rg "threshold.*:=" internal/eyetracking/`

3. **Nil Pointer Risk** - cmd/integrated-ui-test/main.go:67
   🔍 Pattern: Potential access to nil pointer in camera initialization
   💡 Fix: Add nil check before camera.Start()
   🔎 Search: `rg "camera\.Start" cmd/`

🔧 AUTOMATED RECOVERY
✅ just deps: Module dependencies synchronized  
✅ just fmt: Code formatting applied
✅ go mod verify: Module integrity confirmed
✅ just check-prereqs: OpenCV cascades verified

🎯 CLAUDE CODE ACTIONS
1. Edit internal/ui/calibration_window.go:45 - Add fyne.Do() wrapper
2. Edit internal/eyetracking/detector.go:123 - Remove unused variable  
3. Edit cmd/integrated-ui-test/main.go:67 - Add nil check
4. Run: `just ci` to verify all fixes
5. Test: `just prototype` to validate eye tracking functionality

📊 PROJECT HEALTH
- Build Status: ⚠️  Requires fixes (3 issues)
- Test Coverage: ✅ 87.3% (exceeds 80% target)
- Dependencies: ✅ All modules verified
- Prerequisites: ✅ OpenCV + cascades ready
```

## Gazelle-Specific Intelligence

### Eye Tracking Domain Expertise
- **Camera Integration**: Validates OpenCV camera device access and cascade loading
- **UI Thread Safety**: Detects Fyne widget access violations and missing `fyne.Do()` calls  
- **Calibration Logic**: Verifies eye tracking calibration data and algorithm convergence
- **System Permissions**: Checks macOS camera/accessibility permissions for robotgo
- **Performance Patterns**: Identifies memory leaks in video frame processing loops

### Claude Code Workflow Optimization
- **Precise Navigation**: `file_path:line_number` format for instant IDE navigation
- **Search Integration**: Ready-to-use `rg` commands for code investigation
- **Batch Operations**: Parallelizable fix suggestions for efficient execution
- **Context Preservation**: Maintains error context across multiple debugging iterations

### Justfile Command Alignment
All commands verified against current `justfile` structure:
- ✅ `just build` → `bin/gazelle-prototype` 
- ✅ `just test-apps` → All 6 cmd/* applications
- ✅ `just ci` → Complete fmt/lint/test pipeline
- ✅ `just check-prereqs` → OpenCV + cascade validation
- ✅ `just prototype` → Main development workflow

## Prerequisites & Dependencies

### Required Tools
```bash
# Core Go toolchain
go version          # Go 1.19+ required
just --version      # Just command runner

# Optional but recommended
golangci-lint --version    # Static analysis
goimports --version        # Import formatting
```

### Project Dependencies  
```bash
# Computer vision and UI
github.com/hybridgroup/gocv  # OpenCV bindings
fyne.io/fyne/v2              # Cross-platform GUI
github.com/go-vgo/robotgo    # System cursor control

# Development and testing
github.com/sirupsen/logrus   # Structured logging
github.com/stretchr/testify  # Testing framework
```

### System Requirements
```bash
# macOS specific
brew install opencv pkg-config  # OpenCV + pkg-config
# Camera and accessibility permissions required

# Cascade files (auto-installed via just dev-setup)
assets/cascades/haarcascade_frontalface_default.xml
assets/cascades/haarcascade_eye.xml
```