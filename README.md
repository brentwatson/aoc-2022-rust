# Advent Of Code 2022

Used rust as part of 5+ to try these out.

Also read through rust book: https://doc.rust-lang.org/book

### Thoughts on Rust:
- Syntax is easy
- Dealing with ownership is hard
- Compiler puts scoping issues right in your face
  - Eg: moving items between arrays takes lots of code because you can't have immutable ref & muttable ref
  - Trees are near impossible without `Rc`/`RefCell` types because you can't do 2-way modifications that are needed
- Would be perfect to write something system-level that has lots of safty guarnatees
- Can see why it's a good choice over C - forces you to write very safe code even it it takes more code
- No nulls is great. Almost everthing get wrapped in a `Result` type (which can be `if let`ed)
- AOC challanges took more time to fight with the compiler about how things are referenced vs solving the problem at hand
