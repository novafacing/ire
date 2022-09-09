# Roadmap

## Interesting papers and literature:

* [https://5hadowblad3.github.io/files/ISSTA20-Trident.pdf](https://5hadowblad3.github.io/files/ISSTA20-Trident.pdf) SMT solver comparison

## Non-Goals

* Not an exploitation framework -- you could build one on top of this but this though
* Not monolithic -- should not be locked in if you want one feature from this
* Not "plug anything in", be semi-opinionated about choices (eg IR/IL backends)

## Feature Checklist

- [ ] Support userland apps and rootfs
- [ ] IL/IR Backends:
  - [ ] P-Code
  - [ ] Vex
  - [ ] LLVM LLIR, MLIR
  - [ ] VTIL
  - [ ] BNIL
- [ ] Static analysis:
  - [ ] Code identification
  - [ ] Data identification
  - [ ] Disassembly
  - [ ] Lifting to IR
  - [ ] Control Flow Graph
  - [ ] Data Flow Graph
  - [ ] Liveness analysis
  - [ ] Use/def chains
  - [ ] Reaching definitions analysis
  - [ ] Static pointer analysis
  - [ ] Indirect Jump Resolution
  - [ ] Dominators
  - [ ] Program Dependence Graph
  - [ ] Control Dependence Graph
  - [ ] Interprocedural callgraph
  - [ ] Code construct identification
  - [ ] Forward and backward slicing
  - [ ] Calling convention analysis/prototype recovery
  - [ ] Variable recovery
  - [ ] Local and global type recovery
  - [ ] Decompilation
  - [ ] Value-flow graph
  - [ ] Abstract Interpretation
  - [ ] Static symbolic execution
  - [ ] Veritesting
  - [ ] Affine relationship analysis
- [ ] Dynamic analysis:
  - [ ] Exhaustive state configuration
    - [ ] Filesystem
    - [ ] rootfs
    - [ ] inputs/output
  - [ ] Dynamic symbolic execution
  - [ ] Concrete emulation
  - [ ] Debugging
  - [ ] Dynamic taint tracking
  - [ ] Fuzzing
  - [ ] Snapshot capability for all dynamic modes and interface between modes
  - [ ] Kernel via rootfs emulation
- [ ] Binary modification and patching
  - [ ] Patches support
  - [ ] Reassembly
  - [ ] Binary optimization
- [ ] Utility functionality
  - [ ] Serialization of analysis databases
  - [ ] Serialization of dynamic states
  - [ ] Binary Diffing

## Prior Art

- LLVM Lifters
  - allin
  - bin2llvm
  - Dagger
  - fcd
  - Fracture
  - libbeauty
  - mctoll (mcsema)
  - rellume
  - remill
  - reopt
  - retdec
  - rev.ng
- Symbolic execution
  - Manticore
  - angr
- IL/IRs:
  - BIL (bap)
  - BNIL (binja)
  - Boogie (boogie)
  - Cas
  - Chunk IR
  - DBA
  - ESIL
  - Falcon IL
  - FalkerIL
  - GDSL
  - GRIRB
  - JEB IR
  - LowUIR
  - Miasm IR
  - Microcode
  - P-Code
  - RDIL
  - REIL
  - RREIL
  - Snowman IR
  - SSL
  - TSL
  - VEX
  - VINE
  - VTIL
  - AIL
  - LLVM IR