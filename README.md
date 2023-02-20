# Atmus
The 1976 US Standard Atmosphere model, written in Rust.

PLEASE NOTE:  
- Pressure and temperature are tested, but assumes altitude above 0km and below 80km.  
- Expanded altitude regimes are being worked on.  
- Improved documentation and structure are being worked on.  
## Usage
The API exposes a single public function. See the following example:  
```rust
use crate::atmus::atmus;
fn test_atmus_return_values() {
	let some_result = atmus(1.0);
	assert_eq!(f32::floor(some_result.0), 89874.0);
	assert_eq!(f32::floor(some_result.1), 0.0);
}
```
The return is simply a tuple: (Pressure [Pa], Temperature [K]).  
Atmus expects the altitude in kilometers.  

## Development References
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Versioning SemVer Compatibility](https://doc.rust-lang.org/cargo/reference/semver.html)
