# Copilot Instructions for rusty-sharutils

## Project Overview
This project is a self-contained, cross-platform Rust implementation of the GNU sharutils package, providing modern, safe implementations of `shar`, `unshar`, `uuencode`, and `uudecode` utilities. **The project is designed to be completely standalone with minimal external dependencies to maximize stability, security, and deployment simplicity.** The project maintains GPL-3.0 licensing to stay compatible with the original GNU sharutils.

### Core Design Principles
- **Zero Runtime Dependencies**: Implement all functionality using only the Rust standard library
- **Self-Contained**: No reliance on external libraries for core functionality
- **Security Through Simplicity**: Minimize attack surface by avoiding third-party dependencies
- **Stability**: Reduce maintenance burden and compatibility issues from external crates

## Code Quality Standards

### Rust Best Practices
- **Follow Rust conventions**: Use `cargo fmt` and `cargo clippy` standards
- **Idiomatic Rust**: Prefer iterators over loops, use `Result<T, E>` for error handling
- **Memory safety**: Leverage Rust's ownership system, avoid unnecessary `unsafe` blocks
- **Performance**: Use zero-cost abstractions, prefer `&str` over `String` when possible
- **Standard library first**: Implement functionality using `std` before considering external crates
- **Custom error types**: Define project-specific error types using standard library enums and structs
- **Testing**: Write comprehensive tests using only `std` testing framework and built-in assertions

### Code Structure
- **Modular design**: Keep core functionality in the `core` crate, CLI tools as separate binaries
- **Separation of concerns**: Separate parsing, encoding/decoding logic, and I/O operations
- **Trait-based design**: Use traits for common operations to enable testing and extensibility
- **Custom CLI parsing**: Implement argument parsing using standard library to avoid external dependencies
- **Standard library I/O**: Use `std::fs`, `std::io`, and related modules for all file operations

### Documentation Requirements
- **Public APIs**: All public functions, structs, and modules must have rustdoc comments
- **Examples**: Include code examples in documentation for public APIs
- **Error cases**: Document error conditions and expected behavior
- **Safety**: Document any `unsafe` code with safety invariants
- **README**: Keep README.md updated with build instructions and usage examples

## Implementation Guidelines

### Core Functionality
- **Encoding/Decoding**: Implement uuencoding/decoding using only standard library primitives
- **Archive creation**: Implement shar archive creation with file validation using `std::fs`
- **Archive extraction**: Implement safe unshar with path traversal protection using standard library path validation
- **Cross-platform**: Use `std::path::PathBuf` and platform-agnostic file operations exclusively
- **Binary compatibility**: Maintain compatibility with GNU sharutils where reasonable
- **Self-contained algorithms**: Implement all encoding, compression, and archive algorithms from scratch

### Security Considerations
- **Path traversal**: Validate file paths in unshar to prevent directory traversal attacks
- **Input validation**: Sanitize all user inputs and file contents
- **Resource limits**: Implement reasonable limits on file sizes and archive contents
- **Safe defaults**: Choose secure defaults for all operations

### Performance Requirements
- **Streaming**: Process large files using streaming I/O where possible
- **Memory efficiency**: Avoid loading entire files into memory when not necessary
- **Standard library threads**: Use `std::thread` for any parallel processing needs
- **Buffer management**: Use appropriate buffer sizes for I/O operations with standard library types
- **Efficient algorithms**: Optimize algorithms rather than relying on external performance libraries

## Testing Strategy

### Test Coverage
- **Unit tests**: Test individual functions and modules in isolation using `#[test]`
- **Integration tests**: Test complete workflows and CLI interactions using standard library
- **Round-trip tests**: Implement custom property-like testing for encoding/decoding validation
- **Compatibility tests**: Test against GNU sharutils output where applicable
- **Error cases**: Test error handling and edge cases thoroughly
- **Custom test harnesses**: Build specialized testing utilities using only standard library

### Test Organization
- **Core tests**: Place in `core/src/lib.rs` and `core/tests/`
- **CLI tests**: Place in each binary crate's `tests/` directory
- **Test data**: Store test files in `tests/data/` with clear naming
- **Benchmarks**: Use `criterion` for performance benchmarks

## Collaboration Guidelines

### Code Review Standards
- **Incremental changes**: Submit focused PRs that address single concerns
- **Clear descriptions**: Explain the motivation and approach in PR descriptions
- **Test inclusion**: Include tests for new functionality and bug fixes
- **Documentation updates**: Update docs when changing public APIs
- **Backwards compatibility**: Maintain API compatibility or clearly document breaking changes

### Git Practices
- **Conventional commits**: Use conventional commit format for clear history
- **Feature branches**: Create feature branches from `main` for development
- **Clean history**: Squash or rebase commits before merging
- **Descriptive messages**: Write clear, descriptive commit messages

### Issue Management
- **Bug reports**: Include minimal reproduction steps and environment details
- **Feature requests**: Describe use cases and expected behavior
- **Security issues**: Follow responsible disclosure practices
- **Documentation**: Tag documentation-related issues appropriately

## Dependencies and Tooling

### Dependency Policy
- **Zero Runtime Dependencies**: The project must compile and run using only the Rust standard library
- **Exception Policy**: Dependencies are only permitted for development tools that don't affect the final binary
- **Justification Required**: Any proposed dependency must have compelling justification and community approval
- **Security First**: Avoiding dependencies reduces attack surface and supply chain risks
- **Stability**: No dependency updates can break functionality

### Approved Dependencies (Development Only)
- **No runtime dependencies**: All core functionality implemented with `std` only
- **Development tools only**: Tools that don't link into the final binary may be considered case-by-case

### Development Tools
- **Formatting**: `cargo fmt` with default settings
- **Linting**: `cargo clippy` with pedantic warnings enabled
- **Documentation**: `cargo doc` for API documentation
- **Testing**: `cargo test` using standard library testing framework
- **Custom implementations**: Build custom testing utilities rather than external test crates

## Specific Implementation Notes

### uuencode/uudecode
- Implement standard Base64-like encoding with proper line wrapping
- Handle both binary and text file modes correctly
- Preserve file permissions where supported by the platform
- Support both classic and MIME-style uuencoding

### shar/unshar
- Generate POSIX-compatible shell archives
- Include proper error checking in generated shell scripts
- Support directory structures and symbolic links
- Implement compression options where beneficial

### Cross-platform Considerations
- Use `std::env::consts` for platform detection
- Handle Windows vs. Unix path separators correctly
- Respect platform-specific file permissions
- Test on multiple platforms in CI/CD

## Continuous Integration

### Required Checks
- **Compilation**: Must compile on stable Rust
- **Tests**: All tests must pass
- **Formatting**: Code must be formatted with `cargo fmt`
- **Linting**: No clippy warnings allowed
- **Documentation**: Documentation must build without warnings
- **MSRV**: Maintain compatibility with declared minimum supported Rust version

### Optional Checks
- **Performance**: Benchmark critical paths for performance regressions using custom benchmarks
- **Coverage**: Maintain reasonable test coverage (aim for >80%)
- **Dependency audit**: Verify zero runtime dependencies are maintained
- **Binary size**: Monitor final binary size and bloat
- **Cross-platform**: Test on Windows, macOS, and Linux

## License and Legal

### GPL-3.0 Compliance
- All contributions must be compatible with GPL-3.0
- Include proper license headers in all source files
- Document any third-party code or algorithms used
- Maintain copyright notices and attribution

### Contributor Guidelines
- Contributors must agree to license their contributions under GPL-3.0
- Follow the project's code of conduct
- Respect intellectual property and attribution requirements
- Ensure no proprietary or incompatible code is included

---

*This document should be reviewed and updated as the project evolves. All contributors should familiarize themselves with these guidelines before contributing code.*