## There were Classes, interfaces, and GC before
During its development, there have been many features that have been added and
removed again (for example, a garbage collector, classes, and interfaces) to help
it become the fast and safe language that it is today.

## Objects and behaviour
In class based languages-
```java
class Door{
    private bool is_open = false;
    public void Open(){
        this.is_open = true;
    }
}
```

In Rust using traits- 
```rust
struct Door {
    is_open: bool,
}

/* 
impl Door {
    fn new(is_open: bool) -> Door {
        Door { is_open: is_open }
    }
}
*/
trait Openable {
    fn open(&mut self);
}

impl Openable for Door {
    fn open(&mut self) {
        self.is_open = true;
    }
}
fn main() {
    // let mut door1 = Door::new(false);
    let mut door = Door { is_open: false };
    door.open();
}

```
Rust required all its `Door` instance to be `mut` for this case unlike other traditional languages

## `Option<T>` and `Result<T,E>` instead of typical null

There is no `null` pointer/reference in rust
```rust
fn find(item: i32, stack: &Vec<i32>) -> Option<i32> {
    for i in stack {
        // println!("{:?}", i);
        if i == &item {
            return Option::Some(item);
        }
    }
    return None;
}
fn main() {
    let stack = vec![3, 2, 5, 8, 4, 1];

    //using exhaustive match statement
    match find(2, &stack) {
        Some(val) => println!("Found: {val}"),
        None => println!("NOT found"),
    }

    //using the `if let and else`
    if let Some(val) = find(0, &stack) {
        println!("Found using `if let statement`: {}", val);
    } else {
        println!("None: from the `if let else statement`")
    }
}
```

```rust
fn read_file(path: &str) -> Result<String, io::Error> {
// open the path as a file and read it
}
match read_file("/tmp/not/a/file") {
    Ok(content) => println!(content),
    Err(error) => println!("Oh no!")
}
```

## Macros
Helps to minimize the boilerplate code
- **Declarative macros-** work on patterns and run code if that pattern matches.  
 e.g- standard `vec![]` macro implementation
    ```rust
    #[macro_export]
    macro_rules! vec {
        ( $( $x:expr ),* ) => {
            {
                let mut temp_vec = Vec::new();
                 $( temp_vec.push($x); )*
                temp_vec
            }
        };
    }
    ```
- **Procedural macros-** often used to provide a default trait implementation. Example, ``#[derive(Clone,Debug)]`` statements found on top of structures helps to implement `Clone` and `Debug` trait automatically.