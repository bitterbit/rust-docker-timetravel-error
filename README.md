
When run on a ubuntu docker image with a MacOS host, 
it seems we cannot trust the system clock as once in 2500~ iterations it returns an invalid time.
This means that when subtracting two times you cannot be sure it will not be negative. 

The ubuntu version is 20.04.2 LTS (Focal Fossa) and the host is macOS BigSur

```bash
$ ./target/debug/test_time
Hello, world!
.........................thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: SystemTimeError(23.5778ms)', src/main.rs:20:45
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```
