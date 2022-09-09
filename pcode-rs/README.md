# pcode-rs

Rust bindings and interface to the Sleigh/P-Code interface/libraries. This project is super in progress and wil probably
not work!


## Building

Get ghidra submodule:

```sh
$ git submodule update --init --recursive
```

Build:

```sh
$ cargo +stable build
```

Ostensibly, that is it!

## Using

We aren't here yet.

## Task List

- [x] Reliably build Ghidra sleigh compiler out of tree
- [x] Reliably build Ghidra libsleigh out of tree
- [x] Reliably build Ghidra libdecomp  out of tree
- [x] Invoke build via Cargo at package build time
- [x] Generate low level core bindings to libdecomp/libsleigh
- [ ] Test essential low level bindings (you are here)
- [ ] Write high level bindings for essential structures
- [ ] Test high level bindings for essential structures

## Test Failures

There are some failures already with the bindgen generated code.

For now, tracking these here (361 pass, 21 fail):


```
core::root::__bindgen_test_layout_partmap_open0_Address_ContextInternal_FreeArray_close0_instantiation
core::root::__bindgen_test_layout_partmap_open0_Address_TrackedSet_close0_instantiation
core::root::__bindgen_test_layout_partmap_open0_Address_uint4_close0_instantiation
core::root::__bindgen_test_layout_partmap_open0_Address_uint4_close0_instantiation_1
core::root::__bindgen_test_layout_partmap_open0_Address_uint4_close0_instantiation_2
core::root::__bindgen_test_layout_partmap_open0_Address_uint4_close0_instantiation_3
core::root::__bindgen_test_layout_rangemap_open0_ParamEntryRange_close0_instantiation
core::root::__bindgen_test_layout_rangemap_open0_ParamEntryRange_close0_instantiation_1
core::root::__bindgen_test_layout_rangemap_open0_ScopeMapper_close0_instantiation
core::root::__bindgen_test_layout_rangemap_open0_SymbolEntry_close0_instantiation
core::root::__bindgen_test_layout_rangemap_open0_SymbolEntry_close0_instantiation_1
core::root::__bindgen_test_layout_rangemap_open0_SymbolEntry_close0_instantiation_2
core::root::__bindgen_test_layout_rangemap_open0_SymbolEntry_close0_instantiation_3
core::root::__bindgen_test_layout_rangemap_open0_SymbolEntry_close0_instantiation_4
core::root::__bindgen_test_layout_rangemap_open0_SymbolEntry_close0_instantiation_5
core::root::__bindgen_test_layout_rangemap_open0_SymbolEntry_close0_instantiation_6
core::root::__bindgen_test_layout_rangemap_open0_SymbolEntry_close0_instantiation_7
core::root::__bindgen_test_layout_rangemap_open0_SymbolEntry_close0_instantiation_8
core::root::bindgen_test_layout_ContextField
core::root::bindgen_test_layout_Database
core::root::bindgen_test_layout_OperandValue
```