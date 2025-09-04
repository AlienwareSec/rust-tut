//! Rust Basics: A single-file, comprehensive set of notes with runnable examples.
//!
//! How to use this file:
//! - Read through the comments to learn core Rust concepts.
//! - Many examples are small functions you can explore; most are not called.
//! - The binary compiles and runs a minimal `main` so `cargo run --bin notes` works.
//! - Search for "SECTION" to navigate quickly.
//!
//! Notation quick reference:
//! - `{}`: Display formatting placeholder (uses `fmt::Display`).
//! - `{:?}`: Debug formatting (uses `fmt::Debug`), `{:#?}` is pretty-printed Debug.
//! - `::`: Path separator for modules, crates, and associated items (e.g., `String::from`, `std::cmp::Ordering::Less`).
//! - `.`: Method call or field access on a value (e.g., `s.len()`, `point.x`).
//! - `&` / `&mut`: Shared/Mutable borrow references. `*` dereferences a pointer/reference.
//! - `?`: Propagate `Result` errors up to the caller.
//! - `!` after a name (e.g., `println!`) denotes a macro invocation.

#![allow(dead_code, unused_imports, unused_variables, unused_mut, unused_assignments, unused_must_use)]

use std::collections::{HashMap, BTreeMap, BTreeSet, HashSet, VecDeque};
use std::fmt;
use std::sync::{Arc, Mutex};
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;
use std::thread;

fn main() {
    // Minimal runtime output to verify the binary works.
    println!("Rust Basics Notes — open `src/bin/notes.rs` to read comments and examples");
}

// ============================================================================
// SECTION 1: Variables, Mutability, Shadowing, Constants
// ============================================================================

fn variables_and_mutability() {
    // `let` binds an immutable variable by default.
    let x = 10; // cannot assign to `x` again (immutable)

    // Use `mut` for a mutable binding.
    let mut y = 5;
    y = 6; // ok

    // Shadowing: re-declare a new `x`, hiding the previous binding.
    let x = x + 1; // new `x` shadows the old one

    // Types can change with shadowing (unlike mutation which preserves type).
    let spaces = "   ";
    let spaces = spaces.len(); // shadowing with a new type (usize)

    // Constants: must have explicit type and be set to a constant expression.
    const MAX_POINTS: u32 = 100_000;
    let _ = (x, y, spaces, MAX_POINTS);
}

// ============================================================================
// SECTION 2: Statements vs Expressions; Semicolons
// ============================================================================

fn statements_vs_expressions() {
    // In Rust, most things are expressions that evaluate to a value.
    // A block `{ ... }` is an expression; the last expression (without `;`) is the value.
    let value = {
        let a = 2;
        let b = 3;
        a + b // no semicolon => this is the block's value (5)
    };
    assert_eq!(value, 5);

    // Adding a `;` turns an expression into a statement (value becomes `()` unit).
    let unit: () = { let a = 2; a + 3; };
    let _ = unit;
}

// ============================================================================
// SECTION 3: Primitive Types and Compound Types
// ============================================================================

fn primitive_and_compound_types() {
    // Scalars
    let _i: i32 = -42;
    let _u: u64 = 42;
    let _f: f64 = 3.14;
    let _b: bool = true;
    let _c: char = 'λ';

    // Tuples (fixed-size, heterogeneous)
    let tuple: (i32, f32, &str) = (1, 2.5, "hi");
    let (a, b, c) = tuple; // destructuring
    let _first = tuple.0;

    // Arrays (fixed-size, homogeneous)
    let arr: [i32; 3] = [1, 2, 3];
    let _repeated = [0u8; 5];

    // Slices (views into arrays, strings, or Vec)
    let slice: &[i32] = &arr[0..2];
    let _ = (a, b, c, _first, slice);
}

// ============================================================================
// SECTION 4: Strings — `String` vs `&str`
// ============================================================================

fn strings_basics() {
    // `&str` is a string slice (borrowed view), typically string literals are `&'static str`.
    let s_slice: &str = "hello";

    // `String` is an owned, heap-allocated, growable UTF-8 string.
    let mut s_string = String::from("hello"); // `::from` is an associated function using `::`
    s_string.push(' ');
    s_string.push_str("world");

    // Concatenate and format
    let s1 = "hi ".to_string();
    let s2 = "there";
    let s3 = s1 + s2; // `+` takes ownership of left side, borrows right side
    let s4 = format!("{} {}!", s_string, s2); // non-consuming formatting

    // Slicing strings: be careful — strings are UTF-8, indices are bytes
    let hello = "Здравствуйте"; // 12 Cyrillic letters, but 24 bytes
    let _slice_ok = &hello[0..4]; // OK: cuts at valid UTF-8 boundary
    // let _bad = &hello[0..1]; // BAD: would panic at runtime due to invalid boundary

    let _ = (s_slice, s3, s4, _slice_ok);
}

// ============================================================================
// SECTION 5: Ownership, Moves, Borrowing, Lifetimes (Basics)
// ============================================================================

fn ownership_and_borrowing() {
    // Move semantics for non-Copy types (e.g., String)
    let s = String::from("abc");
    let t = s; // move: `s` is no longer valid
    // println!("{}", s); // ERROR: `s` moved
    println!("{}", t);

    // Copy semantics for simple types (implement `Copy`): integers, bool, char, etc.
    let a = 5;
    let b = a; // copy: `a` still usable
    println!("a={}, b={}", a, b);

    // Borrowing with `&` (shared) and `&mut` (exclusive mutable)
    let mut s2 = String::from("hello");
    let len = calculate_length(&s2); // shared borrow
    change(&mut s2); // mutable borrow
    println!("len={}, s2={}", len, s2);

    // Lifetime basics (inferred here). References must not outlive their owners.
    // fn longest<'a>(x: &'a str, y: &'a str) -> &'a str { if x.len() > y.len() { x } else { y } }
}

fn calculate_length(s: &String) -> usize { s.len() }
fn change(s: &mut String) { s.push_str("!"); }

// ============================================================================
// SECTION 6: Functions, Return Types, Unit `()` and Never `!`
// ============================================================================

fn add(x: i32, y: i32) -> i32 { x + y }

fn unit_function() { // returns `()` implicitly
}

fn never_returns() -> ! {
    panic!("This function never returns");
}

// ============================================================================
// SECTION 7: Control Flow (if, loop, while, for, match)
// ============================================================================

fn control_flow() {
    let n = 3;
    let _sign = if n >= 0 { 1 } else { -1 }; // `if` is an expression

    let mut counter = 0;
    let result = loop { // infinite until `break`
        counter += 1;
        if counter == 3 { break counter * 2; } // break with a value
    };
    assert_eq!(result, 6);

    let mut m = 0;
    while m < 3 { m += 1; }

    for i in 0..3 { println!("i={}", i); }

    use std::cmp::Ordering;
    let cmp = 2.cmp(&3);
    match cmp {
        Ordering::Less => println!("less"),
        Ordering::Equal => println!("equal"),
        Ordering::Greater => println!("greater"),
    }
}

// ============================================================================
// SECTION 8: Structs, Methods (`impl`), Associated Functions (`::new`)
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
struct Point { x: i32, y: i32 }

impl Point {
    // Associated function (no `self`); called with `Point::new(...)`. Uses `::`.
    fn new(x: i32, y: i32) -> Self { Self { x, y } }

    // Method (has `&self`)
    fn manhattan_len(&self) -> i32 { self.x.abs() + self.y.abs() }

    // Mutable method
    fn translate(&mut self, dx: i32, dy: i32) { self.x += dx; self.y += dy; }
}

fn struct_examples() {
    let mut p = Point::new(1, -2);
    println!("p={:?}, len={}", p, p.manhattan_len());
    p.translate(3, 4);
    println!("p={:#?}", p);
}

// ============================================================================
// SECTION 9: Enums, Pattern Matching, `Option` and `Result`
// ============================================================================

#[derive(Debug)]
enum Shape {
    Circle { radius: f64 },
    Rectangle { w: f64, h: f64 },
    Unit,
}

fn area(s: &Shape) -> f64 {
    match s {
        Shape::Circle { radius } => std::f64::consts::PI * radius * radius,
        Shape::Rectangle { w, h } => w * h,
        Shape::Unit => 0.0,
    }
}

fn option_and_result() -> Result<(), std::io::Error> {
    let maybe_num: Option<i32> = Some(10);
    if let Some(v) = maybe_num { println!("v={}", v); }

    use std::fs::File;
    // `?` propagates error if `File::open` returns `Err`.
    let _file = File::open("/this/path/might/not/exist").map_err(|e| {
        // Map or log the error as needed
        e
    })?;
    Ok(())
}

// ============================================================================
// SECTION 10: Traits, Trait Bounds, `impl Trait`, Generics
// ============================================================================

trait Summarize { fn summary(&self) -> String; }

impl Summarize for Point {
    fn summary(&self) -> String { format!("Point({}, {})", self.x, self.y) }
}

fn print_summary<T: Summarize>(item: &T) { println!("{}", item.summary()); }

fn returns_impl_trait() -> impl Summarize { Point::new(0, 0) }

fn generics_example<T: PartialOrd + Copy>(a: T, b: T) -> T { if a < b { a } else { b } }

// ============================================================================
// SECTION 11: Formatting: `{}`, `{:?}`, `{:#?}`, implementing `Display`
// ============================================================================

#[derive(Debug)]
struct User { name: String, age: u8 }

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({} years)", self.name, self.age)
    }
}

fn formatting_examples() {
    let u = User { name: "Ada".into(), age: 28 };
    println!("Display: {}", u); // uses `{}`
    println!("Debug: {:?}", u); // uses `{:?}`
    println!("Pretty Debug: {:#?}", u); // `{:#?}` pretty-prints

    // Positional and named arguments
    println!("{0} + {0} = {1}", "hi", "hihi");
    println!("name={name}, age={age}", name=u.name, age=u.age);
}

// ============================================================================
// SECTION 12: Collections (`Vec`, `HashMap`, etc.)
// ============================================================================

fn collections_examples() {
    let mut v = vec![1, 2, 3];
    v.push(4);
    if let Some(last) = v.pop() { println!("popped {}", last); }

    let mut map = HashMap::new();
    map.insert("a", 1);
    map.entry("b").or_insert(2);
    if let Some(val) = map.get("a") { println!("a -> {}", val); }

    let mut set = HashSet::new();
    set.insert(1);
    set.insert(2);
    println!("contains 2? {}", set.contains(&2));
}

// ============================================================================
// SECTION 13: Iterators and Closures
// ============================================================================

fn iterators_and_closures() {
    let nums = vec![1, 2, 3, 4, 5];
    let evens: Vec<_> = nums.iter().filter(|n| *n % 2 == 0).collect();
    let squares: Vec<_> = nums.iter().map(|n| n * n).collect();
    let sum: i32 = nums.iter().copied().sum();
    println!("evens={:?}, squares={:?}, sum={}", evens, squares, sum);

    // Closures capture environment by reference or move as needed.
    let factor = 10;
    let mul = |x: i32| x * factor; // borrows factor by reference
    println!("mul(3)={}", mul(3));
}

// ============================================================================
// SECTION 14: Module System, Paths, `::`, `use`, `self`, `super`, `crate`
// ============================================================================

mod animals {
    pub mod cat {
        pub fn sound() -> &'static str { "meow" }
    }
    pub mod dog {
        pub fn sound() -> &'static str { "woof" }
    }
}

fn modules_and_paths() {
    // `::` joins path segments: crate -> module -> item
    let c = crate::animals::cat::sound();
    let d = crate::animals::dog::sound();
    println!("cat={}, dog={}", c, d);

    // `use` brings names into scope
    use crate::animals::cat::sound as cat_sound;
    println!("cat={}", cat_sound());
}

// ============================================================================
// SECTION 15: Macros vs Functions; Common Standard Macros
// ============================================================================

fn macros_overview() {
    // Macros (with `!`) are expanded at compile time and can generate code.
    println!("println! is a macro using formatting like {{}} and {{:?}}.");
    eprintln!("stderr message");
    let _vec = vec![1, 2, 3];
    let _s = format!("formatted: {}", 42);
    dbg!(&_vec); // prints file:line with Debug formatting
}

// ============================================================================
// SECTION 16: Slices, Borrowing, and `as` Casting
// ============================================================================

fn slices_and_casting() {
    let bytes: [u8; 4] = [1, 2, 3, 4];
    let slice: &[u8] = &bytes[1..3];
    println!("slice={:?}", slice);

    let big: u64 = 500;
    let small = big as u8; // narrowing cast (may truncate)
    println!("small={}", small);
}

// ============================================================================
// SECTION 17: Visibility `pub`, `pub(crate)`, `pub(super)`, `pub(in path)`
// ============================================================================

mod visibility_demo {
    pub struct PublicType(pub i32); // tuple struct with public field
    struct PrivateType(i32); // private to module

    pub(crate) fn crate_visible() {}
    pub(super) fn parent_visible() {}
    pub(in crate::visibility_demo) fn in_path_visible() {}

    fn _use_private() { let _ = PrivateType(0); }
}

// ============================================================================
// SECTION 18: Constants vs Statics
// ============================================================================

const MAX_CONNECTIONS: usize = 1024; // inlined at compile time
static APP_VERSION: &str = "1.0.0"; // single memory location

fn constants_and_statics() {
    println!("max={}, version={}", MAX_CONNECTIONS, APP_VERSION);
}

// ============================================================================
// SECTION 19: Smart Pointers: `Box`, `Rc`, `Arc`, `RefCell`
// ============================================================================

fn smart_pointers() {
    // Box: heap allocation and indirection
    let b = Box::new(42);
    println!("boxed={}", *b);

    // Rc: single-threaded reference counting shared ownership
    let a = Rc::new(String::from("hello"));
    let a1 = a.clone();
    let a2 = Rc::clone(&a);
    println!("rc counts: strong={}", Rc::strong_count(&a));

    // RefCell: interior mutability with borrow checking at runtime
    let cell = RefCell::new(0);
    *cell.borrow_mut() += 1;
    println!("cell={}", cell.borrow());

    // Arc + Mutex: thread-safe shared mutable state
    let shared = Arc::new(Mutex::new(0));
    let s1 = Arc::clone(&shared);
    let t = thread::spawn(move || { let mut g = s1.lock().unwrap(); *g += 1; });
    let _ = t.join();
    println!("shared={}", *shared.lock().unwrap());
}

// ============================================================================
// SECTION 20: Concurrency: Threads and Channels
// ============================================================================

fn concurrency() {
    use std::sync::mpsc;
    let (tx, rx) = mpsc::channel::<String>();

    let handle = thread::spawn(move || {
        thread::sleep(Duration::from_millis(50));
        tx.send("hello from thread".to_string()).unwrap();
    });

    let msg = rx.recv().unwrap();
    println!("got: {}", msg);
    let _ = handle.join();
}

// ============================================================================
// SECTION 21: Error Handling Patterns and the `?` Operator
// ============================================================================

fn read_file_first_line(path: &str) -> Result<String, std::io::Error> {
    use std::io::{BufRead, BufReader};
    let f = std::fs::File::open(path)?; // propagate error
    let mut reader = BufReader::new(f);
    let mut line = String::new();
    let _n = reader.read_line(&mut line)?;
    Ok(line)
}

// ============================================================================
// SECTION 22: Attributes, Derives, Lints, and Doc Comments
// ============================================================================

/// Adds two numbers.
///
/// # Examples
///
/// ```
/// // simple example (doc test when part of a library crate)
/// // assert_eq!(2 + 3, 5);
/// ```
#[allow(clippy::needless_return)]
fn add_verbose(a: i32, b: i32) -> i32 {
    return a + b;
}

// File-level attributes like `#![allow(unused)]` can silence warnings in examples.

// ============================================================================
// SECTION 23: Testing (unit tests live next to code)
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() { assert_eq!(add(2, 3), 5); }

    #[test]
    fn test_point() {
        let mut p = Point::new(1, 2);
        assert_eq!(p.manhattan_len(), 3);
        p.translate(1, -3);
        assert_eq!(p, Point { x: 2, y: -1 });
    }
}

// ============================================================================
// SECTION 24: Miscellaneous Useful Traits and Conversions
// ============================================================================

fn conversions_misc() {
    // From / Into
    let s: String = String::from("abc");
    let t: String = "def".into();
    let u = String::from("123");

    // TryFrom / TryInto
    use std::convert::{TryFrom, TryInto};
    let b: u8 = 200u16.try_into().unwrap(); // ok, fits
    let _ = (s, t, u, b);
}

// ============================================================================
// SECTION 25: Why `{}` and why `::`
// ============================================================================

fn why_braces_and_colons() {
    // `{}` comes from the `format_args!` machinery. It uses `fmt::Display` to
    // turn values into human-readable strings. If a type doesn't implement
    // `Display`, use `{:?}` which uses `fmt::Debug`.
    println!("Display for integers: {} {}", 1, 2);
    println!("Debug for collections: {:?}", vec![1, 2, 3]);

    // `::` is the path separator to reach associated items and modules.
    // - Associated functions: `String::from`, `Vec::new` (no `self`).
    // - Enum variants: `Option::Some`, `Ordering::Less`.
    // - Modules/namespaces: `std::cmp::Ordering`.
    let _s = String::from("via associated function");
    use std::cmp::Ordering;
    let _o = Ordering::Less; // enum variant via `::`
}

// ============================================================================
// SECTION 26: Cargo and Crates (brief notes)
// ============================================================================
// - A project is a crate; binaries live in `src/main.rs` or `src/bin/*.rs`.
// - Libraries use `src/lib.rs`.
// - `cargo build`, `cargo run`, `cargo test`, `cargo doc --open`.
// - Dependencies are listed in `Cargo.toml` under `[dependencies]`.
// - Features, profiles, and workspaces are also configured in `Cargo.toml`.

// ============================================================================
// END
// ============================================================================


