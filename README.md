# Atmus
The US 1976 Standard Atmosphere, written in Rust.

## Usage
There is some basic usage in the unit tests of `./src/lib.rs`. Essentially:  
```
mod basic_usage {
	use crate::atmosphere::get_pressure;
	fn test_pressure_at_1km() {
		let some_result: f32 = get_pressure(1.0);
		assert_eq!(f32::floor(some_result), 89874.0);
	}
}
```
Currently, only `get_pressure` is tested. It takes one parameter as input: the 
altitude in kilometers.

## Development References
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)

