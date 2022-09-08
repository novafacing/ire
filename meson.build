# Meson build file for building pcode-rs C++ dependencies

project('pcode-rs', 'cpp', default_options: ['cpp_std=c++11'])


# Imports and dependencies
fs = import('fs')

flex = find_program('flex', required: true)
bison = find_program('bison', required: true)

libbfd = meson.get_compiler('cpp').find_library('bfd')

# Locations and definitions
install_path = meson.project_source_root() / get_option('install_dir')

ghidra_decompile_cpp_dir = './ghidra/Ghidra/Features/Decompiler/src/decompile/cpp'
ghidra_processors_dir = './ghidra/Ghidra/Processors'

# Sanity checks

if not fs.is_dir(ghidra_decompile_cpp_dir)
  error('Ghidra decompile C++ directory not found. Run `git submodule update --init --recursive` to fetch it.')
endif

if not fs.is_dir(ghidra_processors_dir)
  error('Ghidra processors directory not found. Run `git submodule update --init --recursive` to fetch it.')
endif

# Generated sources for sleigh and libraries

grammar_cc = custom_target(
    'grammar_cc',
    output: 'grammar.cc',
    input: ghidra_decompile_cpp_dir / 'grammar.y',
    command: [bison, '-p', 'cparse', '-o' , 'grammar.cc', '@INPUT@']
)

xml_cc = custom_target(
    'xml_cc',
    output: 'xml.cc',
    input: ghidra_decompile_cpp_dir / 'xml.y',
    command: [bison, '-p', 'xml', '-o' , 'xml.cc', '@INPUT@']
)
pcodeparse_cc = custom_target(
    'pcodeparse_cc',
    output: 'pcodeparse.cc',
    input: ghidra_decompile_cpp_dir / 'pcodeparse.y',
    command: [bison, '-p', 'pcode', '-o' , 'pcodeparse.cc', '@INPUT@']
)
slghparse_cc_hh = custom_target(
    'slghparse_cc_hh',
    output: [
        'slghparse.cc',
        'slghparse.hh'
    ],
    input: ghidra_decompile_cpp_dir / 'slghparse.y',
    command: [bison, '-d', '-o' , 'slghparse.cc', '@INPUT@']
)
# 0 is the CC file, 1 is the HH file
# meson generates them as an array if there are multiple outputs
slghparse_cc = slghparse_cc_hh[0]
slghparse_hh = slghparse_cc_hh[1]

ruleparse_cc_hh = custom_target(
    'ruleparse_cc_hh',
    output: [
        'ruleparse.cc',
        'ruleparse.hh'
    ],
    input: ghidra_decompile_cpp_dir / 'ruleparse.y',
    command: [bison, '-p', 'ruleparse', '-d' , '-o' , 'ruleparse.cc', '@INPUT@']
)
# 0 is the CC file, 1 is the HH file
# meson generates them as an array if there are multiple outputs
ruleparse_cc = ruleparse_cc_hh[0]
ruleparse_hh = ruleparse_cc_hh[1]

slghscan_cc = custom_target(
    'slghscan_cc',
    output: 'slghscan.cc',
    input: ghidra_decompile_cpp_dir / 'slghscan.l',
    command: [flex, '-o', '@OUTPUT@', '@INPUT@'],
    depends: [slghparse_cc_hh]
)

generated_sources = declare_dependency(
    sources: [
        grammar_cc,
        xml_cc,
        pcodeparse_cc,
        slghparse_cc,
        slghparse_hh,
        ruleparse_cc,
        ruleparse_hh,
        slghscan_cc
    ]
)

# End generated sources

# Source code for sleigh and libraries

ghidra_inc = include_directories(ghidra_decompile_cpp_dir)

core_src = [
    # ghidra_decompile_cpp_dir / 'xml.cc',
    xml_cc,
    ghidra_decompile_cpp_dir / 'marshal.cc',
    ghidra_decompile_cpp_dir / 'space.cc',
    ghidra_decompile_cpp_dir / 'float.cc',
    ghidra_decompile_cpp_dir / 'address.cc',
    ghidra_decompile_cpp_dir / 'pcoderaw.cc',
    ghidra_decompile_cpp_dir / 'translate.cc',
    ghidra_decompile_cpp_dir / 'opcodes.cc',
    ghidra_decompile_cpp_dir / 'globalcontext.cc',
]

dec_core_src = [
    ghidra_decompile_cpp_dir / 'capability.cc',
    ghidra_decompile_cpp_dir / 'architecture.cc',
    ghidra_decompile_cpp_dir / 'options.cc',
    ghidra_decompile_cpp_dir / 'graph.cc',
    ghidra_decompile_cpp_dir / 'cover.cc',
    ghidra_decompile_cpp_dir / 'block.cc',
    ghidra_decompile_cpp_dir / 'cast.cc',
    ghidra_decompile_cpp_dir / 'typeop.cc',
    ghidra_decompile_cpp_dir / 'database.cc',
    ghidra_decompile_cpp_dir / 'cpool.cc',
    ghidra_decompile_cpp_dir / 'comment.cc',
    ghidra_decompile_cpp_dir / 'stringmanage.cc',
    ghidra_decompile_cpp_dir / 'fspec.cc',
    ghidra_decompile_cpp_dir / 'action.cc',
    ghidra_decompile_cpp_dir / 'loadimage.cc',
    # ghidra_decompile_cpp_dir / 'grammar.cc',
    grammar_cc,
    ghidra_decompile_cpp_dir / 'varnode.cc',
    ghidra_decompile_cpp_dir / 'op.cc',
    ghidra_decompile_cpp_dir / 'type.cc',
    ghidra_decompile_cpp_dir / 'variable.cc',
    ghidra_decompile_cpp_dir / 'varmap.cc',
    ghidra_decompile_cpp_dir / 'jumptable.cc',
    ghidra_decompile_cpp_dir / 'emulate.cc',
    ghidra_decompile_cpp_dir / 'emulateutil.cc',
    ghidra_decompile_cpp_dir / 'flow.cc',
    ghidra_decompile_cpp_dir / 'userop.cc',
    ghidra_decompile_cpp_dir / 'funcdata.cc',
    ghidra_decompile_cpp_dir / 'funcdata_block.cc',
    ghidra_decompile_cpp_dir / 'funcdata_op.cc',
    ghidra_decompile_cpp_dir / 'funcdata_varnode.cc',
    ghidra_decompile_cpp_dir / 'unionresolve.cc',
    ghidra_decompile_cpp_dir / 'pcodeinject.cc',
    ghidra_decompile_cpp_dir / 'heritage.cc',
    ghidra_decompile_cpp_dir / 'prefersplit.cc',
    ghidra_decompile_cpp_dir / 'rangeutil.cc',
    ghidra_decompile_cpp_dir / 'ruleaction.cc',
    ghidra_decompile_cpp_dir / 'subflow.cc',
    ghidra_decompile_cpp_dir / 'blockaction.cc',
    ghidra_decompile_cpp_dir / 'merge.cc',
    ghidra_decompile_cpp_dir / 'double.cc',
    ghidra_decompile_cpp_dir / 'transform.cc',
    ghidra_decompile_cpp_dir / 'coreaction.cc',
    ghidra_decompile_cpp_dir / 'condexe.cc',
    ghidra_decompile_cpp_dir / 'override.cc',
    ghidra_decompile_cpp_dir / 'dynamic.cc',
    ghidra_decompile_cpp_dir / 'crc32.cc',
    ghidra_decompile_cpp_dir / 'prettyprint.cc',
    ghidra_decompile_cpp_dir / 'printlanguage.cc',
    ghidra_decompile_cpp_dir / 'printc.cc',
    ghidra_decompile_cpp_dir / 'printjava.cc',
    ghidra_decompile_cpp_dir / 'memstate.cc',
    ghidra_decompile_cpp_dir / 'opbehavior.cc',
    ghidra_decompile_cpp_dir / 'paramid.cc',
]

sleigh_src = [
    ghidra_decompile_cpp_dir / 'sleigh.cc',
    # ghidra_decompile_cpp_dir / 'pcodeparse.cc',
    pcodeparse_cc,
    ghidra_decompile_cpp_dir / 'pcodecompile.cc',
    ghidra_decompile_cpp_dir / 'sleighbase.cc',
    ghidra_decompile_cpp_dir / 'slghsymbol.cc',
    ghidra_decompile_cpp_dir / 'slghpatexpress.cc',
    ghidra_decompile_cpp_dir / 'slghpattern.cc',
    ghidra_decompile_cpp_dir / 'semantics.cc',
    ghidra_decompile_cpp_dir / 'context.cc',
    ghidra_decompile_cpp_dir / 'filemanage.cc',
]

ghidra_src = [
    ghidra_decompile_cpp_dir / 'ghidra_arch.cc',
    ghidra_decompile_cpp_dir / 'inject_ghidra.cc',
    ghidra_decompile_cpp_dir / 'ghidra_translate.cc',
    ghidra_decompile_cpp_dir / 'loadimage_ghidra.cc',
    ghidra_decompile_cpp_dir / 'typegrp_ghidra.cc',
    ghidra_decompile_cpp_dir / 'database_ghidra.cc',
    ghidra_decompile_cpp_dir / 'ghidra_context.cc',
    ghidra_decompile_cpp_dir / 'cpool_ghidra.cc',
    ghidra_decompile_cpp_dir / 'ghidra_process.cc',
    ghidra_decompile_cpp_dir / 'comment_ghidra.cc',
    ghidra_decompile_cpp_dir / 'string_ghidra.cc'
]

sleigh_compiler_src = [
    ghidra_decompile_cpp_dir / 'slgh_compile.cc',
    # ghidra_decompile_cpp_dir / 'slghparse.cc',
    slghparse_cc,
    # ghidra_decompile_cpp_dir / 'slghscan.cc',
    slghscan_cc,
]

special_src = [
    ghidra_decompile_cpp_dir / 'consolemain.cc',
    ghidra_decompile_cpp_dir / 'sleighexample.cc',
    ghidra_decompile_cpp_dir / 'test.cc'
]

extra_src = [
    ghidra_decompile_cpp_dir / 'bfd_arch.cc',
    ghidra_decompile_cpp_dir / 'callgraph.cc',
    ghidra_decompile_cpp_dir / 'codedata.cc',
    ghidra_decompile_cpp_dir / 'ifacedecomp.cc',
    ghidra_decompile_cpp_dir / 'ifaceterm.cc',
    ghidra_decompile_cpp_dir / 'inject_sleigh.cc',
    ghidra_decompile_cpp_dir / 'interface.cc',
    ghidra_decompile_cpp_dir / 'libdecomp.cc',
    ghidra_decompile_cpp_dir / 'loadimage_bfd.cc',
    ghidra_decompile_cpp_dir / 'loadimage_xml.cc',
    ghidra_decompile_cpp_dir / 'raw_arch.cc',
    ghidra_decompile_cpp_dir / 'rulecompile.cc',
    # ghidra_decompile_cpp_dir / 'ruleparse.cc',
    ruleparse_cc,
    ghidra_decompile_cpp_dir / 'sleigh_arch.cc',
    ghidra_decompile_cpp_dir / 'testfunction.cc',
    ghidra_decompile_cpp_dir / 'unify.cc',
    ghidra_decompile_cpp_dir / 'xml_arch.cc'
]

libsla_src = [
    ghidra_decompile_cpp_dir / 'loadimage.cc',
    ghidra_decompile_cpp_dir / 'sleigh.cc',
    ghidra_decompile_cpp_dir / 'memstate.cc',
    ghidra_decompile_cpp_dir / 'emulate.cc',
    ghidra_decompile_cpp_dir / 'opbehavior.cc',
]


sleigh = executable(
    'sleigh',
    core_src + sleigh_src + sleigh_compiler_src,
    cpp_args: ['-Wno-sign-compare'],
    include_directories: [ghidra_decompile_cpp_dir],
    install_dir: install_path / 'bin',
    install: true,
)

libsleigh = library(
    'sleigh',
    core_src + sleigh_src + libsla_src,
    cpp_args: ['-Wno-sign-compare'],
    include_directories: [ghidra_decompile_cpp_dir],
    install_dir: install_path / 'lib',
    install: true,
)

libdecomp = library(
    'decomp',
    core_src + dec_core_src + extra_src + sleigh_src,
    cpp_args: ['-Wno-sign-compare'],
    include_directories: [ghidra_decompile_cpp_dir],
    dependencies: libbfd,
    install_dir: install_path / 'lib',
    install: true,
)

###############################AUTOMATICALLY GENERATED##################################
# You can re-generate this section of the script by running the following command:  #
# python3 scripts/generate_meson_sleigh.py                                     #
########################################################################################
sleigh_processor_sources = {
    'MIPS': [
        'mips64.pspec',
        'mips32_fp64.cspec',
        'mips.sinc',
        'mipsmicro.sinc',
        'mipsfloat.sinc',
        'mips.ldefs',
        'mips32be.cspec',
        'mips32R6.pspec',
        'mips64_32_o64.cspec',
        'mips32.pspec',
        'mips.dwarf',
        'mips64micro.pspec',
        'mips32le.cspec',
        'mips32micro.pspec',
        'mips64R6.pspec',
        'mips64_32_o32.cspec',
        'mips64_32_n32.cspec',
        'mips64.cspec',
        'MIPS.opinion',
        'mips64Instructions.sinc',
        'mips32Instructions.sinc',
        'mips16.sinc'
    ],
    'TI_MSP430': [
        'TI_MSP430.ldefs',
        'ti_msp430.opinion',
        'TI430X.sinc',
        'TI_MSP430.pspec',
        'TI_MSP430X.cspec',
        'TI430Common.sinc',
        'TI_MSP430.cspec'
    ],
    'x86': [
        'x86-64.dwarf',
        'x86.opinion',
        'fma.sinc',
        'sha.sinc',
        'bmi1.sinc',
        'x86-16.cspec',
        'avx2.sinc',
        'lzcnt.sinc',
        'avx2_manual.sinc',
        'cet.sinc',
        'x86borland.cspec',
        'x86-16-real.pspec',
        'x86-16.pspec',
        'smx.sinc',
        'bmi2.sinc',
        'x86.dwarf',
        'x86-64-gcc.cspec',
        'x86delphi.cspec',
        'x86.ldefs',
        'x86-16.gdis',
        'sgx.sinc',
        'x86gcc.cspec',
        'clwb.sinc',
        'macros.sinc',
        'ia.sinc',
        'pclmulqdq.sinc',
        'mpx.sinc',
        'avx.sinc',
        'adx.sinc',
        'x86-64.pspec',
        'x86.pspec',
        'x86-64-win.cspec',
        'x86win.cspec',
        'avx_manual.sinc',
        'rdrand.sinc'
    ],
    'Sparc': [
        'SparcV9_32.cspec',
        'SparcV9.pspec',
        'SparcVIS.sinc',
        'SparcV9.ldefs',
        'SparcV9_64.cspec',
        'Sparc.opinion',
        'SparcV9.sinc'
    ],
    '6502': [
        '6502.pspec',
        '6502.cspec',
        '6502.ldefs'
    ],
    'MC6800': [
        '6800.ldefs',
        '6x09_push.sinc',
        '6809.pspec',
        '6x09.sinc',
        '6805.ldefs',
        '6805.cspec',
        '6805.pspec',
        '6809.cspec',
        '6x09_exg_tfr.sinc',
        '6x09_pull.sinc'
    ],
    'M8C': [
        'm8c.pspec',
        'm8c.ldefs',
        'm8c.opinion',
        'm8c.cspec'
    ],
    'SuperH4': [
        'SuperH4.ldefs',
        'SuperH4_le.cspec',
        'SuperH4_be.cspec',
        'SuperH4.sinc',
        'SuperH4.pspec',
        'SuperH4.opinion'
    ],
    'HCS12': [
        'XGATE.sinc',
        'HCS12.opinion',
        'HCS12X.pspec',
        'HCS_HC12.sinc',
        'HC12.cspec',
        'HCS12.cspec',
        'HCS12.pspec',
        'HCS12X.cspec',
        'HC12.pspec',
        'HCS12.ldefs'
    ],
    'CP1600': [
        'CP1600.opinion',
        'CP1600.cspec',
        'CP1600.pspec',
        'CP1600.ldefs'
    ],
    'Dalvik': [
        'Dalvik_OpCode_F6_unused.sinc',
        'Dalvik_OpCode_F8_unused.sinc',
        'Dalvik_OpCode_73_return_void_no_barrier.sinc',
        'Dalvik_OpCode_FF_unused.sinc',
        'Dalvik_OpCode_E3_EA_dex.sinc',
        'Dalvik_Base.pspec',
        'Dalvik_OpCode_F7_unused.sinc',
        'Dalvik_Base.cspec',
        'Dalvik_Base.sinc',
        'Dalvik_OpCode_FB_unused.sinc',
        'Dalvik_OpCode_F3_unused.sinc',
        'Dalvik_OpCode_F4_unused.sinc',
        'Dalvik_OpCode_FE_FF_dex.sinc',
        'Dalvik_OpCode_79_unused.sinc',
        'Dalvik_OpCode_FA_FD_dex.sinc',
        'Dalvik_OpCode_EB_F2_iput_iget.sinc',
        'Dalvik_OpCode_3E_43_unused.sinc',
        'Dalvik_OpCode_FC_unused.sinc',
        'Dalvik_OpCode_73_return_void_barrier.sinc',
        'Dalvik_OpCode_FA_unused.sinc',
        'Dalvik_OpCode_7A_unused.sinc',
        'Dalvik_OpCode_FE_unused.sinc',
        'Dalvik_OpCode_F5_unused.sinc',
        'Dalvik_OpCode_FD_unused.sinc',
        'Dalvik_OpCode_F9_unused.sinc',
        'Dalvik.ldefs',
        'Dalvik_OpCode_EB_F2_unused.sinc',
        'Dalvik_OpCode_73_unused.sinc',
        'Dalvik_OpCode_E3_EA_unused.sinc',
        'Dalvik.opinion'
    ],
    '8048': [
        '8048.ldefs',
        '8048.cspec',
        '8048.pspec'
    ],
    'RISCV': [
        'riscv64-fp.cspec',
        'riscv.opinion',
        'riscv32.dwarf',
        'riscv.rvv.sinc',
        'riscv.priv.sinc',
        'riscv.rv64m.sinc',
        'riscv.instr.sinc',
        'riscv.rv64a.sinc',
        'RV64GC.pspec',
        'riscv64.cspec',
        'riscv.reg.sinc',
        'RV32G.pspec',
        'riscv.rv64f.sinc',
        'riscv.rv64b.sinc',
        'RV64I.pspec',
        'riscv.rv32f.sinc',
        'riscv.rv32i.sinc',
        'RV64IC.pspec',
        'riscv.rv32a.sinc',
        'riscv.rv32m.sinc',
        'riscv.rv64i.sinc',
        'riscv.rv64p.sinc',
        'riscv32-fp.cspec',
        'riscv.custom.sinc',
        'riscv64.dwarf',
        'riscv.rv32q.sinc',
        'riscv.rv32b.sinc',
        'riscv.rv64d.sinc',
        'RV64G.pspec',
        'riscv.csr.sinc',
        'riscv.table.sinc',
        'riscv.rv32k.sinc',
        'riscv32.cspec',
        'riscv.rvc.sinc',
        'riscv.rv64q.sinc',
        'riscv.rv32d.sinc',
        'RV32IMC.pspec',
        'RV32I.pspec',
        'riscv.zi.sinc',
        'riscv.rv64k.sinc',
        'RV32IC.pspec',
        'riscv.ldefs',
        'riscv.rv32p.sinc',
        'RV32GC.pspec'
    ],
    '8051': [
        '8051.ldefs',
        '8051.pspec',
        '8051_archimedes.cspec',
        '8051_main.sinc',
        'mx51.sinc',
        '80251.pspec',
        'mx51.pspec',
        '80390.cspec',
        '8051.cspec',
        'mx51.cspec',
        '80251.sinc',
        '80251.cspec'
    ],
    'SuperH': [
        'superh.sinc',
        'superh.cspec',
        'superh.ldefs',
        'superh.pspec',
        'superh2a.cspec'
    ],
    'ARM': [
        'ARMt_v45.pspec',
        'ARMTHUMBinstructions.sinc',
        'ARMtTHUMB.pspec',
        'ARMneon.sinc',
        'ARM.ldefs',
        'ARM.sinc',
        'ARM.opinion',
        'ARM.gdis',
        'ARMt.pspec',
        'ARMneon.dwarf',
        'ARM.dwarf',
        'ARM_v45.cspec',
        'ARM_v45.pspec',
        'ARM.cspec',
        'ARMCortex.pspec',
        'ARMv8.sinc',
        'ARMt_v6.pspec',
        'ARM_win.cspec',
        'ARMinstructions.sinc'
    ],
    '68000': [
        '68000.pspec',
        '68000.dwarf',
        '68000.ldefs',
        '68000.opinion',
        '68000.sinc',
        '68000.cspec'
    ],
    'Z80': [
        'z80.cspec',
        'z180.pspec',
        'z8401x.pspec',
        'z182.pspec',
        'z80.pspec',
        'z80.ldefs'
    ],
    'HCS08': [
        'HC08.pspec',
        'HCS08.opinion',
        'HC08-MC68HC908QY4.pspec',
        'HC08.ldefs',
        'HCS08.ldefs',
        'HCS_HC.sinc',
        'HC05-M68HC05TB.pspec',
        'HC05.ldefs',
        'HC05.pspec',
        'HCS08.pspec',
        'HCS08-MC9S08GB60.pspec',
        'HCS08.cspec'
    ],
    'DATA': [
        'data-ptr64.cspec',
        'data-ptr32.cspec',
        'data-ptr16.cspec',
        'data.ldefs',
        'data.pspec',
        'data.sinc'
    ],
    'PIC': [
        'PIC30.dwarf',
        'pic17c7xx.cspec',
        'pic17c7xx.pspec',
        'pic18.ldefs',
        'pic12c5xx.ldefs',
        'pic16f.cspec',
        'pic17c7xx.ldefs',
        'PIC24.cspec',
        'pic16c5x.pspec',
        'pic16.pspec',
        'PIC33.dwarf',
        'pic16.cspec',
        'PIC24.ldefs',
        'pic18.sinc',
        'pic16c5x.ldefs',
        'pic16_instructions.sinc',
        'pic12c5xx.pspec',
        'pic16.ldefs',
        'pic16c5x.cspec',
        'pic18.cspec',
        'pic12c5xx.cspec',
        'PIC24.pspec',
        'PIC24.opinion',
        'PIC24.sinc',
        'pic16f.pspec',
        'pic18.pspec',
        'pic12.sinc',
        'pic16.sinc',
        'pic17c7xx_instructions.sinc',
        'pic18_instructions.sinc',
        'pic17c7xx.sinc',
        'pic12_instructions.sinc'
    ],
    'Toy': [
        'toy.pspec',
        'toy64.cspec',
        'toy64-long8.cspec',
        'toyInstructions.sinc',
        'toy_builder.sinc',
        'toy.ldefs',
        'toyPosStack.cspec',
        'toy.cspec',
        'toy.sinc',
        'toy_harvard.pspec'
    ],
    'tricore': [
        'tricore.pcp.sinc',
        'tricore.pspec',
        'tc172x.pspec',
        'tc176x.pspec',
        'tricore.dwarf',
        'tricore.cspec',
        'tricore.opinion',
        'tricore.ldefs',
        'tc29x.pspec',
        'tricore.sinc'
    ],
    '8085': [
        '8085.pspec',
        '8085.cspec',
        '8085.ldefs'
    ],
    'Atmel': [
        'avr8.ldefs',
        'avr32a.ldefs',
        'avr32a_system_control.sinc',
        'avr32a_logic_operations.sinc',
        'avr32a_arithmetic_operations.sinc',
        'atmega256.pspec',
        'avr32a_dsp_operations.sinc',
        'avr32a_autogen.sinc',
        'avr8gcc.cspec',
        'avr32a_coprocessor_interface.sinc',
        'avr32.opinion',
        'avr32a_data_transfer.sinc',
        'avr8.pspec',
        'avr8xmega.pspec',
        'avr32a_shift_operations.sinc',
        'avr8egcc.cspec',
        'avr8iarV1.cspec',
        'avr32a_instruction_flow.sinc',
        'avr32a_multiplication_operations.sinc',
        'avr32a_dsp_operations2.sinc',
        'avr32a_simd_operations.sinc',
        'avr8imgCraftV8.cspec',
        'avr32a.cspec',
        'avr32a_bit_operations.sinc',
        'avr8.opinion',
        'avr32a.pspec',
        'avr8.sinc'
    ],
    'CR16': [
        'CR16.ldefs',
        'CR16.pspec',
        'CR16C.sinc',
        'CR16.opinion',
        'CR16B.sinc',
        'CR16.cspec'
    ],
    'JVM': [
        'JVM.ldefs',
        'JVM.pspec',
        'JVM.cspec',
        'JVM.opinion'
    ],
    'PowerPC': [
        'Scalar_SPFP.sinc',
        'ppc_32_mpc8270.pspec',
        'ppc.dwarf',
        'ppc_32.pspec',
        'ppc_32_le.cspec',
        'vsx.sinc',
        'ppc_32_e500_le.cspec',
        'ppc_isa.sinc',
        'ppc_vle.sinc',
        'ppc_64_32.cspec',
        'lmwInstructions.sinc',
        'stmwInstructions.sinc',
        'ppc_64_be_Mac.cspec',
        'SPE_EFV.sinc',
        'ppc_common.sinc',
        'ppc_32_be.cspec',
        'FPRC.sinc',
        'ppc_64.pspec',
        'ppc_32_e500_be.cspec',
        'ppc_embedded.sinc',
        'PowerPC.opinion',
        'quicciii.sinc',
        'SPEF_SCR.sinc',
        'ppc_64.cspec',
        '4xx.sinc',
        'SPE_APU.sinc',
        'g2.sinc',
        'ppc_instructions.sinc',
        'ppc_32_be_Mac.cspec',
        'lswInstructions.sinc',
        'altivec.sinc',
        'stswiInstructions.sinc',
        'ppc.ldefs',
        'ppc_a2.sinc',
        'evx.sinc',
        'SPE_EFSD.sinc'
    ],
    'MCS96': [
        'MCS96.pspec',
        'MCS96.ldefs',
        'MCS96.sinc',
        'MCS96.cspec'
    ],
    'PA-RISC': [
        'pa-riscInstructions.sinc',
        'pa-risc32.cspec',
        'pa-risc32.pspec',
        'pa-risc.ldefs',
        'pa-risc.sinc',
        'pa-risc.opinion'
    ],
    'AARCH64': [
        'AARCH64.opinion',
        'AARCH64ldst.sinc',
        'AARCH64sve.sinc',
        'AARCH64_win.cspec',
        'AARCH64.pspec',
        'AARCH64base.sinc',
        'AppleSilicon.ldefs',
        'AARCH64neon.sinc',
        'AARCH64.cspec',
        'AARCH64_AMXext.sinc',
        'AARCH64_base_PACoptions.sinc',
        'AARCH64_ilp32.cspec',
        'AARCH64.ldefs',
        'AARCH64instructions.sinc',
        'AARCH64.dwarf'
    ]
}


sleigh_processor_specs = {
    'MIPS': {
        'mips64be.sla': ['mips64be.slaspec'] + sleigh_processor_sources.get('MIPS'),
        'mips32R6le.sla': ['mips32R6le.slaspec'] + sleigh_processor_sources.get('MIPS'),
        'mips64le.sla': ['mips64le.slaspec'] + sleigh_processor_sources.get('MIPS'),
        'mips32be.sla': ['mips32be.slaspec'] + sleigh_processor_sources.get('MIPS'),
        'mips32R6be.sla': ['mips32R6be.slaspec'] + sleigh_processor_sources.get('MIPS'),
        'mips32le.sla': ['mips32le.slaspec'] + sleigh_processor_sources.get('MIPS'),
    },
    'TI_MSP430': {
        'TI_MSP430X.sla': ['TI_MSP430X.slaspec'] + sleigh_processor_sources.get('TI_MSP430'),
        'TI_MSP430.sla': ['TI_MSP430.slaspec'] + sleigh_processor_sources.get('TI_MSP430'),
    },
    'x86': {
        'x86.sla': ['x86.slaspec'] + sleigh_processor_sources.get('x86'),
        'x86-64.sla': ['x86-64.slaspec'] + sleigh_processor_sources.get('x86'),
    },
    'Sparc': {
        'SparcV9_32.sla': ['SparcV9_32.slaspec'] + sleigh_processor_sources.get('Sparc'),
        'SparcV9_64.sla': ['SparcV9_64.slaspec'] + sleigh_processor_sources.get('Sparc'),
    },
    '6502': {
        '65c02.sla': ['65c02.slaspec'] + sleigh_processor_sources.get('6502'),
        '6502.sla': ['6502.slaspec'] + sleigh_processor_sources.get('6502'),
    },
    'MC6800': {
        '6805.sla': ['6805.slaspec'] + sleigh_processor_sources.get('MC6800'),
        'H6309.sla': ['H6309.slaspec'] + sleigh_processor_sources.get('MC6800'),
        '6809.sla': ['6809.slaspec'] + sleigh_processor_sources.get('MC6800'),
    },
    'M8C': {
        'm8c.sla': ['m8c.slaspec'] + sleigh_processor_sources.get('M8C'),
    },
    'SuperH4': {
        'SuperH4_be.sla': ['SuperH4_be.slaspec'] + sleigh_processor_sources.get('SuperH4'),
        'SuperH4_le.sla': ['SuperH4_le.slaspec'] + sleigh_processor_sources.get('SuperH4'),
    },
    'HCS12': {
        'HCS12.sla': ['HCS12.slaspec'] + sleigh_processor_sources.get('HCS12'),
        'HC12.sla': ['HC12.slaspec'] + sleigh_processor_sources.get('HCS12'),
        'HCS12X.sla': ['HCS12X.slaspec'] + sleigh_processor_sources.get('HCS12'),
    },
    'CP1600': {
        'CP1600.sla': ['CP1600.slaspec'] + sleigh_processor_sources.get('CP1600'),
    },
    'Dalvik': {
        'Dalvik_DEX_Lollipop.sla': ['Dalvik_DEX_Lollipop.slaspec'] + sleigh_processor_sources.get('Dalvik'),
        'Dalvik_DEX_Android10.sla': ['Dalvik_DEX_Android10.slaspec'] + sleigh_processor_sources.get('Dalvik'),
        'Dalvik_Base.sla': ['Dalvik_Base.slaspec'] + sleigh_processor_sources.get('Dalvik'),
        'Dalvik_ODEX_KitKat.sla': ['Dalvik_ODEX_KitKat.slaspec'] + sleigh_processor_sources.get('Dalvik'),
        'Dalvik_DEX_Android11.sla': ['Dalvik_DEX_Android11.slaspec'] + sleigh_processor_sources.get('Dalvik'),
        'Dalvik_DEX_Nougat.sla': ['Dalvik_DEX_Nougat.slaspec'] + sleigh_processor_sources.get('Dalvik'),
        'Dalvik_DEX_Android12.sla': ['Dalvik_DEX_Android12.slaspec'] + sleigh_processor_sources.get('Dalvik'),
        'Dalvik_DEX_Marshmallow.sla': ['Dalvik_DEX_Marshmallow.slaspec'] + sleigh_processor_sources.get('Dalvik'),
        'Dalvik_DEX_KitKat.sla': ['Dalvik_DEX_KitKat.slaspec'] + sleigh_processor_sources.get('Dalvik'),
        'Dalvik_DEX_Pie.sla': ['Dalvik_DEX_Pie.slaspec'] + sleigh_processor_sources.get('Dalvik'),
        'Dalvik_DEX_Oreo.sla': ['Dalvik_DEX_Oreo.slaspec'] + sleigh_processor_sources.get('Dalvik'),
    },
    '8048': {
        '8048.sla': ['8048.slaspec'] + sleigh_processor_sources.get('8048'),
    },
    'RISCV': {
        'riscv.lp64d.sla': ['riscv.lp64d.slaspec'] + sleigh_processor_sources.get('RISCV'),
        'riscv.ilp32d.sla': ['riscv.ilp32d.slaspec'] + sleigh_processor_sources.get('RISCV'),
    },
    '8051': {
        '80251.sla': ['80251.slaspec'] + sleigh_processor_sources.get('8051'),
        'mx51.sla': ['mx51.slaspec'] + sleigh_processor_sources.get('8051'),
        '8051.sla': ['8051.slaspec'] + sleigh_processor_sources.get('8051'),
        '80390.sla': ['80390.slaspec'] + sleigh_processor_sources.get('8051'),
    },
    'SuperH': {
        'sh-2.sla': ['sh-2.slaspec'] + sleigh_processor_sources.get('SuperH'),
        'sh-2a.sla': ['sh-2a.slaspec'] + sleigh_processor_sources.get('SuperH'),
        'sh-1.sla': ['sh-1.slaspec'] + sleigh_processor_sources.get('SuperH'),
    },
    'ARM': {
        'ARM4t_be.sla': ['ARM4t_be.slaspec'] + sleigh_processor_sources.get('ARM'),
        'ARM6_be.sla': ['ARM6_be.slaspec'] + sleigh_processor_sources.get('ARM'),
        'ARM7_le.sla': ['ARM7_le.slaspec'] + sleigh_processor_sources.get('ARM'),
        'ARM5_be.sla': ['ARM5_be.slaspec'] + sleigh_processor_sources.get('ARM'),
        'ARM5t_be.sla': ['ARM5t_be.slaspec'] + sleigh_processor_sources.get('ARM'),
        'ARM7_be.sla': ['ARM7_be.slaspec'] + sleigh_processor_sources.get('ARM'),
        'ARM4_be.sla': ['ARM4_be.slaspec'] + sleigh_processor_sources.get('ARM'),
        'ARM6_le.sla': ['ARM6_le.slaspec'] + sleigh_processor_sources.get('ARM'),
        'ARM4t_le.sla': ['ARM4t_le.slaspec'] + sleigh_processor_sources.get('ARM'),
        'ARM8_le.sla': ['ARM8_le.slaspec'] + sleigh_processor_sources.get('ARM'),
        'ARM4_le.sla': ['ARM4_le.slaspec'] + sleigh_processor_sources.get('ARM'),
        'ARM8_be.sla': ['ARM8_be.slaspec'] + sleigh_processor_sources.get('ARM'),
        'ARM5t_le.sla': ['ARM5t_le.slaspec'] + sleigh_processor_sources.get('ARM'),
        'ARM5_le.sla': ['ARM5_le.slaspec'] + sleigh_processor_sources.get('ARM'),
    },
    '68000': {
        '68030.sla': ['68030.slaspec'] + sleigh_processor_sources.get('68000'),
        'coldfire.sla': ['coldfire.slaspec'] + sleigh_processor_sources.get('68000'),
        '68020.sla': ['68020.slaspec'] + sleigh_processor_sources.get('68000'),
        '68040.sla': ['68040.slaspec'] + sleigh_processor_sources.get('68000'),
    },
    'Z80': {
        'z180.sla': ['z180.slaspec'] + sleigh_processor_sources.get('Z80'),
        'z80.sla': ['z80.slaspec'] + sleigh_processor_sources.get('Z80'),
    },
    'HCS08': {
        'HC05.sla': ['HC05.slaspec'] + sleigh_processor_sources.get('HCS08'),
        'HCS08.sla': ['HCS08.slaspec'] + sleigh_processor_sources.get('HCS08'),
        'HC08.sla': ['HC08.slaspec'] + sleigh_processor_sources.get('HCS08'),
    },
    'DATA': {
        'data-be-64.sla': ['data-be-64.slaspec'] + sleigh_processor_sources.get('DATA'),
        'data-le-64.sla': ['data-le-64.slaspec'] + sleigh_processor_sources.get('DATA'),
    },
    'PIC': {
        'dsPIC30F.sla': ['dsPIC30F.slaspec'] + sleigh_processor_sources.get('PIC'),
        'PIC24F.sla': ['PIC24F.slaspec'] + sleigh_processor_sources.get('PIC'),
        'pic12c5xx.sla': ['pic12c5xx.slaspec'] + sleigh_processor_sources.get('PIC'),
        'dsPIC33E.sla': ['dsPIC33E.slaspec'] + sleigh_processor_sources.get('PIC'),
        'pic18.sla': ['pic18.slaspec'] + sleigh_processor_sources.get('PIC'),
        'PIC24H.sla': ['PIC24H.slaspec'] + sleigh_processor_sources.get('PIC'),
        'dsPIC33F.sla': ['dsPIC33F.slaspec'] + sleigh_processor_sources.get('PIC'),
        'pic17c7xx.sla': ['pic17c7xx.slaspec'] + sleigh_processor_sources.get('PIC'),
        'pic16c5x.sla': ['pic16c5x.slaspec'] + sleigh_processor_sources.get('PIC'),
        'pic16.sla': ['pic16.slaspec'] + sleigh_processor_sources.get('PIC'),
        'pic16f.sla': ['pic16f.slaspec'] + sleigh_processor_sources.get('PIC'),
        'PIC24E.sla': ['PIC24E.slaspec'] + sleigh_processor_sources.get('PIC'),
        'dsPIC33C.sla': ['dsPIC33C.slaspec'] + sleigh_processor_sources.get('PIC'),
    },
    'Toy': {
        'toy_builder_le_align2.sla': ['toy_builder_le_align2.slaspec'] + sleigh_processor_sources.get('Toy'),
        'toy_le.sla': ['toy_le.slaspec'] + sleigh_processor_sources.get('Toy'),
        'toy_builder_be.sla': ['toy_builder_be.slaspec'] + sleigh_processor_sources.get('Toy'),
        'toy_wsz_le.sla': ['toy_wsz_le.slaspec'] + sleigh_processor_sources.get('Toy'),
        'toy64_be_harvard.sla': ['toy64_be_harvard.slaspec'] + sleigh_processor_sources.get('Toy'),
        'toy64_be.sla': ['toy64_be.slaspec'] + sleigh_processor_sources.get('Toy'),
        'toy_wsz_be.sla': ['toy_wsz_be.slaspec'] + sleigh_processor_sources.get('Toy'),
        'toy_be_posStack.sla': ['toy_be_posStack.slaspec'] + sleigh_processor_sources.get('Toy'),
        'toy_builder_le.sla': ['toy_builder_le.slaspec'] + sleigh_processor_sources.get('Toy'),
        'toy_builder_be_align2.sla': ['toy_builder_be_align2.slaspec'] + sleigh_processor_sources.get('Toy'),
        'toy_be.sla': ['toy_be.slaspec'] + sleigh_processor_sources.get('Toy'),
        'toy64_le.sla': ['toy64_le.slaspec'] + sleigh_processor_sources.get('Toy'),
    },
    'tricore': {
        'tricore.sla': ['tricore.slaspec'] + sleigh_processor_sources.get('tricore'),
    },
    '8085': {
        '8085.sla': ['8085.slaspec'] + sleigh_processor_sources.get('8085'),
    },
    'Atmel': {
        'avr8eind.sla': ['avr8eind.slaspec'] + sleigh_processor_sources.get('Atmel'),
        'avr8e.sla': ['avr8e.slaspec'] + sleigh_processor_sources.get('Atmel'),
        'avr32a.sla': ['avr32a.slaspec'] + sleigh_processor_sources.get('Atmel'),
        'avr8xmega.sla': ['avr8xmega.slaspec'] + sleigh_processor_sources.get('Atmel'),
        'avr8.sla': ['avr8.slaspec'] + sleigh_processor_sources.get('Atmel'),
    },
    'CR16': {
        'CR16C.sla': ['CR16C.slaspec'] + sleigh_processor_sources.get('CR16'),
        'CR16B.sla': ['CR16B.slaspec'] + sleigh_processor_sources.get('CR16'),
    },
    'JVM': {
        'JVM.sla': ['JVM.slaspec'] + sleigh_processor_sources.get('JVM'),
    },
    'PowerPC': {
        'ppc_32_e500_le.sla': ['ppc_32_e500_le.slaspec'] + sleigh_processor_sources.get('PowerPC'),
        'ppc_32_le.sla': ['ppc_32_le.slaspec'] + sleigh_processor_sources.get('PowerPC'),
        'ppc_64_be.sla': ['ppc_64_be.slaspec'] + sleigh_processor_sources.get('PowerPC'),
        'ppc_32_quicciii_be.sla': ['ppc_32_quicciii_be.slaspec'] + sleigh_processor_sources.get('PowerPC'),
        'ppc_32_4xx_be.sla': ['ppc_32_4xx_be.slaspec'] + sleigh_processor_sources.get('PowerPC'),
        'ppc_32_be.sla': ['ppc_32_be.slaspec'] + sleigh_processor_sources.get('PowerPC'),
        'ppc_64_isa_le.sla': ['ppc_64_isa_le.slaspec'] + sleigh_processor_sources.get('PowerPC'),
        'ppc_64_le.sla': ['ppc_64_le.slaspec'] + sleigh_processor_sources.get('PowerPC'),
        'ppc_64_isa_be.sla': ['ppc_64_isa_be.slaspec'] + sleigh_processor_sources.get('PowerPC'),
        'ppc_64_isa_altivec_be.sla': ['ppc_64_isa_altivec_be.slaspec'] + sleigh_processor_sources.get('PowerPC'),
        'ppc_32_4xx_le.sla': ['ppc_32_4xx_le.slaspec'] + sleigh_processor_sources.get('PowerPC'),
        'ppc_64_isa_vle_be.sla': ['ppc_64_isa_vle_be.slaspec'] + sleigh_processor_sources.get('PowerPC'),
        'ppc_32_e500_be.sla': ['ppc_32_e500_be.slaspec'] + sleigh_processor_sources.get('PowerPC'),
        'ppc_32_quicciii_le.sla': ['ppc_32_quicciii_le.slaspec'] + sleigh_processor_sources.get('PowerPC'),
        'ppc_64_isa_altivec_vle_be.sla': ['ppc_64_isa_altivec_vle_be.slaspec'] + sleigh_processor_sources.get('PowerPC'),
        'ppc_64_isa_altivec_le.sla': ['ppc_64_isa_altivec_le.slaspec'] + sleigh_processor_sources.get('PowerPC'),
    },
    'MCS96': {
        'MCS96.sla': ['MCS96.slaspec'] + sleigh_processor_sources.get('MCS96'),
    },
    'PA-RISC': {
        'pa-risc32be.sla': ['pa-risc32be.slaspec'] + sleigh_processor_sources.get('PA-RISC'),
    },
    'AARCH64': {
        'AARCH64_AppleSilicon.sla': ['AARCH64_AppleSilicon.slaspec'] + sleigh_processor_sources.get('AARCH64'),
        'AARCH64.sla': ['AARCH64.slaspec'] + sleigh_processor_sources.get('AARCH64'),
        'AARCH64BE.sla': ['AARCH64BE.slaspec'] + sleigh_processor_sources.get('AARCH64'),
    }
}

###################################END AUTOMATICALLY GENERATED##########################

sleigh_input_files = {}
foreach sleigh_processor, sleigh_languages : sleigh_processor_specs
    processor_dir = ghidra_processors_dir / sleigh_processor / 'data' / 'languages'
    install_dir = install_path / 'share' / 'sleigh' / sleigh_processor

    foreach sleigh_language, sleigh_language_srcs : sleigh_languages
        language_deps = []

        foreach language_srcfile : sleigh_language_srcs
            if sleigh_input_files.has_key(language_srcfile)
                language_deps += sleigh_input_files.get(language_srcfile)

            else
                sleigh_input_file = configure_file(
                    input: processor_dir / language_srcfile,
                    output: language_srcfile,
                    copy: true,
                    install_dir: install_dir,
                    install: true
                )

                sleigh_input_files += {language_srcfile: sleigh_input_file}
                language_deps += sleigh_input_file
            endif
        endforeach

        sleigh_language_target = custom_target(
            sleigh_language,
            output: sleigh_language,
            input: language_deps,
            command: [sleigh, language_deps[0]],
            install_dir: install_dir,
            install: true,
        )
    endforeach
endforeach
