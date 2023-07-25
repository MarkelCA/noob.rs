# Learnings

## Shadowing
### Temporal
Rust:
```rust
fn foo(str: &str) {
    println!("{}", str);

    for str in 0..2 {
        println!("{}", str);
    }

    println!("{}", str);
}

fn main() {
    foo("My text")
}

// Outputs:
// My text
// 0
// 1
// My text
```
Go:
```go
package main
import "fmt"

func foo(str string) {
    fmt.Println(str)

    for str := 0 ; str < 2 ; str += 1 {
        fmt.Println(str)
    }

    fmt.Println(str)
}

func main() {
    foo("My text")
}

// Outputs:
// My text
// 0
// 1
// My text
```
### Permament
Python:
```python
def foo(str):
    print(str)

    for str in range(2):
        print(str)

    print(str)

if __name__ == "__main__":
    foo("My text")

## Outputs:
## My text
## 0
## 1
## 1
```
