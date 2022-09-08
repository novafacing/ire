# Roadmap

## Interesting papers and literature:

* [https://5hadowblad3.github.io/files/ISSTA20-Trident.pdf](https://5hadowblad3.github.io/files/ISSTA20-Trident.pdf) SMT solver comparison

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
  - [ ] Program Dependence Graph
  - [ ] Control Dependence Graph
  - [ ] Dominators
  - [ ] Liveness analysis
  - [ ] Use/def chains
  - [ ] Reaching definitions analysis
  - [ ] Static pointer analysis
  - [ ] Interprocedural callgraph
  - [ ] Code construct identification
  - [ ] Forward and backward slicing
  - [ ] Calling convention analysis/prototype recovery
  - [ ] Local and global type recovery
  - [ ] Indirect Jump Resolution
  - [ ] Binary Diffing
  - [ ] Variable recovery
  - [ ] Decompilation
  - [ ] Value-flow graph
- [ ] Binary modification and patching
  - [ ] Patches support
  - [ ] Reassembly
  - [ ] Binary optimization
- [ ] Utility functionality
  - [ ] Serialization of analysis databases
  - [ ] Serialization of dynamic states

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