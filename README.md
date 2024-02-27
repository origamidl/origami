## Origami Definition Language
The **Origami Definition Language** (OrigamiDL for short) is a domain-specific language used to describe folding instructions used in the context of Origami. It allows to declaratively define Origami models, which can then be automatically converted into crease patterns, folding diagrams or interactive instructions.

## OrigamiDL setup
- Interpreter `origamic`
  - reads `.origami` (OrigamiDL) definitions and outputs `.fold` IR files with extensive origamidl-specific metadata
  - Compiler/interpreter in one step
  - Includes debugging features
- OrigamiDL ecosystem
  - Make use of origamidl-specific `.fold` files
  - Generate crease patterns
  - Generate diagrams
  - Generate interactive folding instructions
  - ...

## Develop
If you're using the [Nix package manager][nix] you can start a shell that contains all dependencies required to develop the compiler using `nix develop`. That's it, no manual installations needed.

[nix]: https://nixos.org/
