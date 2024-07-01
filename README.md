# RNP (Rust for NumPy)

This project is a standard implementation of numpy in rust. I took the inspiration from Maharshi Pandya's original project of making numpy in C. Check here: https://github.com/smolorg/smolar

As of now it contains the basic array operations such as addition, multiplication, transpose and some shape shifting xD.
This is a Rust project, which means you need to create an environment to run it.

`cargo new {project name}`

The above code creates a cargo project for the implementation part, and then, you can run it and maybe try off something new onto it!

I'm lazy with error handling that's why I went with rust.
Just run the `main.rs` file independently for now
All the datatypes are mostly in float (f32).

Comments in the code will help you navigate the understanding of syntax.
// first error that I made: never use ';' when returning a generic :`)

## Why Rust?

Rust is a systems and developer friendly language with great error handling unlike any other, and fast operatability like C.
