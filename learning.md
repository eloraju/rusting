## Lifetimes

[The best lifetime tutorial I've found this far][1]

```rust
pub struct LifetimeExample<'time> {
  let nam: &'time str /* Lives as long as  */
}

impl LifetimeExample<'time> {
  pub fn new(name_ref: &'time str) -> Self{
    Self {
      name: name_ref
    }
  }
}

let ex = LifetimeExample::new(&"test");
```

In this pseudo context `ex.name` will be `test` but in this case we have told the
compiler when the value of `ex.name` will be wiped from memory and the compiler
can take that into account.

How does this relate to the engine? At the time of writing this down I haven't yet
checked what I can do to optimize the code, but I'm pretty sure that most of the
`String` variables can be turned into `&'a str`, since I fixed a lot of lifetime
related compiler warnings just by allocating new strings. I knew at the time that
it was not most likely the best way to go about it but didn't know better. 

[1]:https://www.youtube.com/watch?v=rAl-9HwD858&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa

## Macros

Could/Should I use macros to generate the pieces?
