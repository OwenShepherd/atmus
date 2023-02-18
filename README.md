# Atmus
The 1976 US Standard Atmosphere model, written in Rust.

## Usage
The API exposes a single public function. See the following example:  
```rust
fn test_atmus_return_values() {
	let some_result = atmus(1.0);
	assert_eq!(f32::floor(some_result.0), 89874.0);
}
```
Atmus expects the altitude in kilometers.

## Development References
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Versioning SemVer Compatibility](https://doc.rust-lang.org/cargo/reference/semver.html)

