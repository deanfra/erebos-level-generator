```
------------------------------------------------
███████╗██████╗░███████╗██████╗░░█████╗░░██████╗
██╔════╝██╔══██╗██╔════╝██╔══██╗██╔══██╗██╔════╝
█████╗░░██████╔╝█████╗░░██████╦╝██║░░██║╚█████╗ 
██╔══╝░░██╔══██╗██╔══╝░░██╔══██╗██║░░██║░╚═══██╗
███████╗██║░░██║███████╗██████╦╝╚█████╔╝██████╔╝
╚══════╝╚═╝  ╚═╝╚══════╝╚═════╝  ╚════╝ ╚═════╝ 
------------- 𝓑𝓸𝓻𝓷 𝓯𝓻𝓸𝓶 𝓬𝓱𝓪𝓸𝓼 -------------
```

# Erebos Level Generator

A Rust attempt at a graph-based rogue-like/lite level generator, heavily inspired by [this blog article](https://ondra.nepozitek.cz/blog/graph-based-dungeon-generator-basics-1/) by Ondřej Nepožitek.

This is built to serve as a level generator for a Bevy based sidescroller project I am working on. Most roguelike dungeon generators focus on a single holistic map, but I want a more metroid-like map where paths and difficulty are based on a directed graph with precalculated connections.

## Running
```
cargo build
cargo run --release
```

---

## How does it work?

![](https://user-images.githubusercontent.com/2309383/159177723-222c327e-d587-48c2-a8bd-b842ce8dffd4.png)

1. Generates a random algorithmic graph
2. Assigns weights to each graph node (for difficulty and progress)
3. Procedurally steps through the graph and assigns a random fitting room shape
4. Assigns connecting doors
5. Prints out a nice map

## What's left to do?

- [x] Graph algorithms
- [x] Room randomiser
- [x] Room shape fitting
- [x] Start room + Boss room
- [ ] Unit tests
- [ ] Making this a crate
- [ ] Distinctive areas
- [ ] Portals/Stairs to different areas
- [ ] Backtracking for more optimal connections


## Specs
- `rustc 1.58.0`
- `cargo 1.58.0`

## Code & Disclaimer

This is a hobby project to learn a little about the following:
- Graph theory
- Level generation algorithms
- More Rust 

Quality is definitely not up to scratch because of my very limited time and a beginner's understanding of algorithms.