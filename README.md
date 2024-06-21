<div align="center" >

<img style="height:100px;" src=".github/assets/book.png" > &nbsp;
<img style="height:100px;" src=".github/assets/rust.gif" >
<img style="height:100px;" src=".github/assets/cargo.png" >

# <img style="width:50px;" src=".github/assets/pencil.png" /><span>&nbsp;</span> Learn Everything about RUST

Comprehensive knowledge, learning resources, and practice materials for mastering RUST Language, from basic concepts to advanced techniques.
</div>

<div align="left" >

<h3> How to run rust file/files </h3>

**NOTE:** Rust must be installed on the system, before compiling the code.

```sh
cd ./folder/ ;
rustc ./file.rs ;
./file ;
```

<h3> How to create a new Cargo project </h3>

```sh
cargo new rust-project ;
```

<h3> How to run a Cargo project </h3>

```sh
cargo run ;
```

<h3> How to create a new Cargo binary </h3>

**Cargo.toml**

```
[[bin]]
name = "binary1"
path = "src/bin/binary1.rs"

[[bin]]
name = "binary2"
path = "src/another_folder/binary2.rs"
```

<h3> How to run a Cargo binary </h3>

```sh
cargo run --bin binary1 ;
```

```sh
cargo run -q --bin binary1 ;
```

<h3> Cargo project documentation </h3>

```sh
cargo doc ;
cargo doc --open ;

cargo rustdoc ;
rustdoc --help ;
```

</div>

<br />

<div align="center" >

<img style="width:130px;" src=".github/assets/code2.png" /> 

<h2>Fundamentals</h2>

In-depth exploration of core concepts and syntax of RUST, providing a strong foundation for more advanced topics.

</div>

<br />

<div align="center" >

<img style="width:80px;" src=".github/assets/ds.png" /> 

<h2>Data Structures</h2>

Detailed implementation and usage of essential data structures, enabling efficient data management and organization.

</div>

### 🔖 Primitive Types

### 🔖 Non-Primitive Types

<br />

<div align="center" >

<img style="width:80px;" src=".github/assets/oop3.png" /> 

<h2>Object Oriented Programming</h2>

The principles of object-oriented programming in RUST, including classes, inheritance, and encapsulation, to build modular and scalable applications.

</div>

<br />

<div align="center" >

<img style="width:80px;" src=".github/assets/design.png" /> 

<h2>Design Patterns</h2>

Comprehensive guide to best practices for designing reusable and maintainable code using established design patterns.

</div>

<br />

<div align="center" >

<img style="width:80px;" src=".github/assets/algo.png" /> 

<h2>Algorithms</h2>

Thorough solutions and explanations for common algorithmic challenges, aimed at enhancing problem-solving skills.

</div>

