# Rust Language
- Type declarations are required when defining functions
- `let` declares **variable bindings** which are immutable by default
- `let a: i32 = 20;` - you can specify data types for the compiler
- `let a = 30i32;` - numeric literals can include types annotations
- `"` double quotes are used for string literals, `'` single quotes are for single characters which have their own `char` type
- String formatting uses `{}` for placeholders
- Function declarations *sometimes* look like `fn my_function() -> my_type {}`
- Conversion between types are always explicit
  - `b as i32` - casting is done with the `as` operand
  - Casting should usually only be done from a smaller data type to a larger data type to prevent unexpected behaviour

## Rust Numbers
- Rust numbers can have methods
- Rust has tolerances defined when comparing floating-point values
  - Defined as `f32::EPSILON` and `f64::EPSILON` for their respective datatypes

### Rust types for representing scalar (single) numbers
- i8, i16, i32, i64
  - Signed integers ranging from 8 bit to 64 bit
- u8, u16, u32, u64
  - Unsigned integers ranging from 8 bit to 64 bit
- f32, f64
  - Floating-point numbers in 32-bit and 64-bit variants
- isize, usize
  - Integers that assume the CPU’s “native” width (e.g., in 64-bit CPUs, usize and isize will be 64-bits wide)

### Limitations
- The Rust standard library is relatively small and doesn't include many numerical structures that can be found in other languages
  - Use the `num` crate for this