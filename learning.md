## Lifetimes

[The best lifetime tutorial I've found this far][1]

[Rust book chapter on lifetimes][2]

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
Here we are telling the compiler that the `name_ref` we provide has a lifetime of
`'time` - which can be anything from a microsecond to the lifetime of the program
(static). This lifetime is inherited from the argument we pass to the function so
the compiler knows how long the memeber variable `name` will be valid for and
knows to raise an error on compile time should we try and write code that would
violate the lifetime constraint.

I tried to create an example of the lifetime violation here but at first I failed with

```rust
fn main() {
    let n1 = "test1";
    let mut test = LTTest::new(&n1);
    
    println!("{}",test.name);
    
    {
        let n2 = "dead";
        test.name = &n2;
        println!("{}",test.name);
    }
    
    println!("{}",test.name);
}

pub struct LTTest<'a> {
    name: &'a str,
}

impl<'a> LTTest<'a> {
    pub fn new(name: &'a str) -> Self {
        Self {
            name
        } 
    }
}
```

I assumed that this would not compile, since we try to use `n2` after it should have
been destroyed. Then I realised remembered that when you create a string with just 
`""`, you actually create a string in your binary and that by definition has a static
lifetime. The pointer I gave to `LTTest` was a pointer to a static string in the binary
itself! So I "fixed" the code by changing the declaration of `n2`

```rust
// From this
let n2 = "dead";

// To this
let n2 = String::from("dead");

```
And lo and behold!

```shell
error[E0597]: `n2` does not live long enough
  --> src/main.rs:9:21
   |
9  |         test.name = &n2;
   |                     ^^^ borrowed value does not live long enough
10 |         println!("{}",test.name);
11 |     }
   |     - `n2` dropped here while still borrowed
12 |     
13 |     println!("{}",test.name);
   |                   --------- borrow later used here
```

Now we allocate the string and runtime and do indeed get the error that we should :)

How does this relate to the engine? At the time of writing this down I haven't yet
checked what I can do to optimize the code, but I'm pretty sure that most of the
`String` variables can be turned into `&'a str`, since I fixed a lot of lifetime
related compiler warnings just by allocating new strings. I knew at the time that
it was not most likely the best way to go about it but didn't know better.

Allocating stings is a lot slower than just returning a pointer to a already existing
string and even though the engine doesn't play around with strings _that_ much it's
still extremely good practice. Helps one to think in the terms of lifetimes.

[1]:https://www.youtube.com/watch?v=rAl-9HwD858&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa
[2]:https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html

## Macros

Could/Should I use macros to generate the pieces?
