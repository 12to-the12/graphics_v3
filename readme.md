# Explanation
Welcome to the third iteration of my ray tracing journey!

## history
After first starting with graphic programming as a freshman in high school, the field has followed me around ever since. As I write these words in early 2025, I've spent the better part of a decade thinking about how images are created with math.
I started writing this version in 2023 as I was hanging out in southern India, in part because I wanted an excuse to learn Rust. Earlier iterations of this were exclusively written in Python, first in a web environment, then in cPython, then with a JIT compiler. That wasn't fast enough for me.

## efficiency
This is far from a resource efficient implementation. As it's software based, it will never come close to what a GPU can offer, even with recently implemented concurrency. I'm planning on taking advantage of various acceleration structures, and SIMD in the future, but don't have hopes that this will ever be as performant as is possible today.

## inspiration
I've taken heavy inspiration from the book Physically Based Rendering: from theory to implementation. The objective for this iteration of the project is to use as rigorous a definition as possible for all values being computed. I have to say, it's weird using the Boltzmann constant in a renderer, but I have my goals.

# How to run
If you're running this yourself, good luck, I've tried to make it as portable as possible.

## dependencies
To run this project you need to have Rust installed and visible on your path

## running
Clone the project with
```bash
git clone https://github.com/12to-the12/graphics_v3.git
```

cd into the folder and and run the project with
```
make
```

the renderer outputs the result as a png in the project root, that was simpler than trying to interface with a windowing system.


# Roadmap
- [x] ray/polygon intersection
- [x] matrix based tranformations
- [x] .obj import
- [x] Lambert's law falloff
- [x] Spectra based light propagation
- [x] concurrency
- [ ] full physically based lighting model
- [ ] spatial acceleration structures