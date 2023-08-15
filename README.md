# Making a Container in ~500 lines of Rust
A project to recreate a container in ~500 lines of Rust, following several different tutorials, exploring what a container is and how you can make one using Rust

### Introduction

The title may be slightly misleading but is a nod towards the original blog 
post which can be found [here](https://blog.lizzie.io/linux-containers-in-500-loc.html). This post inspired another series of posts by [Litchi Pi](https://litchipi.github.io/) - which can be found [here](https://litchipi.github.io/series/container_in_rust), or in Markdown form [here](https://github.com/litchipi/litchipi.github.io/blob/main/_posts/tutorials/containers_in_rust/2021-09-13-container-in-rust-part1.md). These posts will form the basis for the structure of this project but some parts are out of date (using crates that have since been superceeded by better maintained alternatives, for example), so it will likely be that the project looks different from the original "Crabcan".

### What this project is for
This project is to help learn about containers and the way they interact with the kernel. It might be useful to help with creating parts, if you are stuck on a particular component for example. It should also be a good way to get an introduction to crates available in the Rust eco-system, and trying to demonstrate best practice for Rust code.

### What this project is not
A good way to make containers, you shouldn't be making containers this way to release to production, there are better, more secure, and more thoroughly tested mechanisms for containers, use those for production releases.

### Where to find out more
This project will likely be worked on as a series of blog posts available on the [Electro Cat Website](https://electrocatstudios.com). It is also hoped that there will be enough interesting content to create a YouTube series to accompany this project. The Electo Cat Studios You Tube channel can be found [here](https://www.youtube.com/channel/UC1m6P72nySpB3lKWDYGVipw).