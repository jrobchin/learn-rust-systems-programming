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

## & (Ampersand)
- Unary operator that returns a reference to a variable
- For any type `T`, `&T` returns a read-only reference to `T`

## For-loops
| Shorthand                     | Equivalent to                                     | Access     |
| ----------------------------- | ------------------------------------------------- | ---------- |
| `for item in collection`      | `for item in IntoIterator::into_iter(collection)` | Ownership  |
| `for item in &collection`     | `for item in collection.iter()`                   | Read-only  |
| `for item in &mut collection` | `for item in collection.iter_mut()`               | Read-write |

### Anonymous Loops
- Use `_` by convention when the local variable is not used
- Using the *exclusive range syntax* (`n..m`) and the *inclusive range syntax* (`n..=m) makes it clear that the intent is to perform the loop a fixed number of times

```rust
for _ in 0..10 {
    // ...
}
```

### Iterating Over the Indexes of a Collection
- Avoid manually managing an index variable that's incremented each loop
```rust
// This is better
let collection = [1, 2, 3, 4, 5];
for i in 0..collection.len() {
    let item = collection[i];
    // ...
}
```

## Loop
- The `loop` keyword is recommended for long-running loops
```rust
// This loop will execute until it hits a `break;`
loop {
    // ...
}
```

## Break from Nested Loops
- *Loop labels* can be added to identify loops by prefixing the identifier with an apostrophe `'`
```rust
'outer: for x in 0..10 {
    for y in 0..20 {
        for z in 0..30 {
            if x + y + z > 40 {
                break 'outer;
            }
        }
    }
}
```

## True and False
- There are no truthy or falsey types in Rust such as 1, 0, empty list, etc.
- Only `true` and `false` are true and false

## Rust Expressions
- `return` keywords can be omitted from functions
- Below is valid Rust
```rust
fn is_even(n: i32) -> bool {
    n % 2 == 0;
}
```
- The above snippet can then be used to assign a variable
```rust
fn main() {
    let n = 12345;
    let description = if is_even(n) {
        "even"
    } else {
        "odd"
    }
    println!("{} is {}", n, description);
}
```
Or using `match`
```rust
fn main() {
    let n = 12345;
    let description = if is_even(n) {
        "even"
    } else {
        "odd"
    }
    println!("{} is {}", n, description);
}
```
`break`s can also return values
```rust
fn main() {
    let n = loop {
        break 123;
    };

    println!("{}", n);
}
```

## Match
- Safer alternative to `if/else` blocks that warns about values that have not been considered
- Uses the `match` keyword

```rust
match item {
    0 => {}, // Single value match
    0..=20 => {}, // Inclusive range match
    40 | 80 => {}, // Or match
    _ => {}, // Every other value
}
```

### Some
`Some(T)` is the positive case of an option, `None` is a negative case.

```rust
use regex::Regex;

fn main() {
  let re = Regex::new("picture").unwrap();

  let quote = "Every face, every shop, bedroom window, public-house, and
dark square is a picture feverishly turned--in search of what?
It is the same with books. What do we seek through millions of pages?";

  for line in quote.lines() {
    let contains_substring = re.find(line);
    match contains_substring {
        Some(_) => println!("{}", line),
        None => (),
    }
  }
}
```