# Basic Outline
This is a simple implementation of numbers in the complex plane. Includes addition, subtraction, and multiplication. The main module is `complex_numbers`, and the actual implementation of a complex number is defined within under `Complex`.

Complex numbers are defined generically, and can be initialized with any type satisfying:

```rust
impl<T> Complex<T> 
where T: Add<Output = T> + Mul<Output = T> + 
Sub<Output = T> + Clone + 
Default
```

The `Complex` implementation directly implements the above, so you can use the common operators `+, -, *` to work with them.