# No spam

Executes `f` only a maximum number of times per time. For example, to execute only 5 times per second:

```rust
use no_spam::NoSpam;

fn main() {
    let mut collect: Vec<Option<u32>> = Vec::new();
    let mut no_spam = NoSpam::new_per_second(5);
    for _ in 0..10 {
        no_spam.on(|_| collect.push(None));
    }
    std::thread::sleep(std::time::Duration::from_secs(2));
    for _ in 0..10 {
        no_spam.on(|_| collect.push(None));
    }
    assert!(collect.len() == 10, "collect size: {}", collect.len());
}
```

Also has a function to call for spam control on average. That is, it does not guarantee that `f` will be caled a maximum of `x` times per second, but it should, on average, call it `x` times per second. This is useful if you cannot/don't want to create an object for each spam control

```rust
use NoSpam::per_time;
fn main () {
    let t = 500;
    for _ in 0..t {
        per_time(10, t, |_|{println!("called")});
    }
}
```