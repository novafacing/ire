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

# Sleigh compilation steps

sleigh_processor_sources = {
    'MIPS': [
        'mipsfloat.sinc',
        'mips64.pspec',
        'mips.sinc',
        'mips.dwarf',
        'mips16.sinc',
        'mips32le.cspec',
        'mips32_fp64.cspec',
        'mips32micro.pspec',
        'MIPS.opinion',
        'mips64_32_o32.cspec',
        'mips64.cspec',
        'mips64micro.pspec',
        'mips64R6.pspec',
        'mips32be.cspec',
        'mips32R6.pspec',
        'mips32.pspec',
        'mips64_32_o64.cspec',
        'mipsmicro.sinc',
        'mips.ldefs',
        'mips64_32_n32.cspec',
        'mips64Instructions.sinc',
        'mips32Instructions.sinc'
    ],
    'TI_MSP430': [
        'TI430X.sinc',
        'TI430Common.sinc',
        'TI_MSP430.pspec',
        'ti_msp430.opinion',
        'TI_MSP430.ldefs',
        'TI_MSP430X.cspec',
        'TI_MSP430.cspec'
    ],
    'x86': [
        'avx_manual.sinc',
        'fma.sinc',
        'x86-16.cspec',
        'x86borland.cspec',
        'lzcnt.sinc',
        'x86-64-win.cspec',
        'x86.ldefs',
        'x86.dwarf',
        'smx.sinc',
        'x86gcc.cspec',
        'x86-64.dwarf',
        'avx2.sinc',
        'x86.opinion',
        'x86delphi.cspec',
        'x86-16.gdis',
        'pclmulqdq.sinc',
        'mpx.sinc',
        'x86win.cspec',
        'x86-64.pspec',
        'sgx.sinc',
        'adx.sinc',
        'x86.pspec',
        'avx2_manual.sinc',
        'rdrand.sinc',
        'bmi1.sinc',
        'bmi2.sinc',
        'macros.sinc',
        'x86-16.pspec',
        'ia.sinc',
        'x86-16-real.pspec',
        'cet.sinc',
        'clwb.sinc',
        'sha.sinc',
        'avx.sinc',
        'x86-64-gcc.cspec'
    ],
    'Sparc': [
        'SparcV9.pspec',
        'SparcV9_32.cspec',
        'SparcVIS.sinc',
        'Sparc.opinion',
        'SparcV9_64.cspec',
        'SparcV9.ldefs',
        'SparcV9.sinc'
    ],
    # TODO: V850 disabled due to subdirectory include
    # 'V850': [
    #     'V850.pspec',
    #     'V850.cspec',
    #     'V850.opinion',
    #     'V850.ldefs',
    # ],
    '6502': [
        '6502.ldefs',
        '6502.cspec',
        '6502.pspec'
    ],
    'MC6800': [
        '6x09.sinc',
        '6805.ldefs',
        '6800.ldefs',
        '6809.cspec',
        '6x09_exg_tfr.sinc',
        '6805.cspec',
        '6805.pspec',
        '6x09_push.sinc',
        '6809.pspec',
        '6x09_pull.sinc'
    ],
    'M8C': [
        'm8c.cspec',
        'm8c.ldefs',
        'm8c.opinion',
        'm8c.pspec'
    ],
    'SuperH4': [
        'SuperH4.sinc',
        'SuperH4_le.cspec',
        'SuperH4.pspec',
        'SuperH4.opinion',
        'SuperH4_be.cspec',
        'SuperH4.ldefs'
    ],
    'HCS12': [
        'HCS12X.pspec',
        'HCS12.cspec',
        'HC12.cspec',
        'HCS12.ldefs',
        'HCS12.opinion',
        'XGATE.sinc',
        'HC12.pspec',
        'HCS12.pspec',
        'HCS12X.cspec',
        'HCS_HC12.sinc'
    ],
    'CP1600': [
        'CP1600.ldefs',
        'CP1600.cspec',
        'CP1600.opinion',
        'CP1600.pspec'
    ],
    'Dalvik': [
        'Dalvik_OpCode_FE_FF_dex.sinc',
        'Dalvik_OpCode_FA_FD_dex.sinc',
        'Dalvik_Base.pspec',
        'Dalvik_OpCode_FE_unused.sinc',
        'Dalvik_OpCode_EB_F2_iput_iget.sinc',
        'Dalvik_OpCode_F8_unused.sinc',
        'Dalvik_Base.sinc',
        'Dalvik_OpCode_F6_unused.sinc',
        'Dalvik_OpCode_F4_unused.sinc',
        'Dalvik_OpCode_F7_unused.sinc',
        'Dalvik_OpCode_73_unused.sinc',
        'Dalvik_Base.cspec',
        'Dalvik_OpCode_FC_unused.sinc',
        'Dalvik_OpCode_F9_unused.sinc',
        'Dalvik_OpCode_FD_unused.sinc',
        'Dalvik.ldefs',
        'Dalvik_OpCode_7A_unused.sinc',
        'Dalvik_OpCode_F5_unused.sinc',
        'Dalvik_OpCode_E3_EA_unused.sinc',
        'Dalvik_OpCode_E3_EA_dex.sinc',
        'Dalvik_OpCode_EB_F2_unused.sinc',
        'Dalvik_OpCode_73_return_void_no_barrier.sinc',
        'Dalvik_OpCode_3E_43_unused.sinc',
        'Dalvik_OpCode_FA_unused.sinc',
        'Dalvik_OpCode_FF_unused.sinc',
        'Dalvik_OpCode_73_return_void_barrier.sinc',
        'Dalvik_OpCode_FB_unused.sinc',
        'Dalvik_OpCode_F3_unused.sinc',
        'Dalvik_OpCode_79_unused.sinc',
        'Dalvik.opinion'
    ],
    '8048': [
        '8048.pspec',
        '8048.cspec',
        '8048.ldefs'
    ],
    'RISCV': [
        'riscv.rv64k.sinc',
        'riscv.rv64q.sinc',
        'riscv.rv32i.sinc',
        'RV64IC.pspec',
        'RV32GC.pspec',
        'riscv.rv32d.sinc',
        'riscv.ldefs',
        'riscv.priv.sinc',
        'RV64I.pspec',
        'riscv.table.sinc',
        'riscv.custom.sinc',
        'riscv64-fp.cspec',
        'riscv.rv32a.sinc',
        'riscv32.cspec',
        'RV32G.pspec',
        'riscv.rv64b.sinc',
        'riscv.rv64f.sinc',
        'riscv.zi.sinc',
        'riscv.rv32q.sinc',
        'riscv.csr.sinc',
        'riscv.rv64i.sinc',
        'riscv.rv32p.sinc',
        'RV64G.pspec',
        'RV32I.pspec',
        'RV64GC.pspec',
        'riscv.rvv.sinc',
        'riscv.rv32k.sinc',
        'riscv.opinion',
        'riscv64.cspec',
        'riscv64.dwarf',
        'riscv.rv64d.sinc',
        'riscv.rv32b.sinc',
        'riscv.rv32m.sinc',
        'riscv32.dwarf',
        'riscv.rv64a.sinc',
        'riscv32-fp.cspec',
        'riscv.rv32f.sinc',
        'riscv.rv64p.sinc',
        'riscv.rv64m.sinc',
        'RV32IMC.pspec',
        'riscv.rvc.sinc',
        'riscv.instr.sinc',
        'riscv.reg.sinc',
        'RV32IC.pspec'
    ],
    '8051': [
        '80251.sinc',
        'mx51.cspec',
        'mx51.sinc',
        '8051.cspec',
        '8051_main.sinc',
        '80251.cspec',
        '8051.pspec',
        '80390.cspec',
        'mx51.pspec',
        '8051.ldefs',
        '8051_archimedes.cspec',
        '80251.pspec'
    ],
    'SuperH': [
        'superh2a.cspec',
        'superh.ldefs',
        'superh.cspec',
        'superh.pspec',
        'superh.sinc'
    ],
    'ARM': [
        'ARM.gdis',
        'ARMCortex.pspec',
        'ARMt.pspec',
        'ARM.cspec',
        'ARMv8.sinc',
        'ARMt_v6.pspec',
        'ARM.dwarf',
        'ARM.sinc',
        'ARMneon.sinc',
        'ARM_win.cspec',
        'ARMt_v45.pspec',
        'ARMneon.dwarf',
        'ARMTHUMBinstructions.sinc',
        'ARM_v45.cspec',
        'ARM_v45.pspec',
        'ARMinstructions.sinc',
        'ARM.ldefs',
        'ARMtTHUMB.pspec',
        'ARM.opinion'
    ],
    '68000': [
        '68000.cspec',
        '68000.dwarf',
        '68000.pspec',
        '68000.sinc',
        '68000.opinion',
        '68000.ldefs'
    ],
    'Z80': [
        'z80.cspec',
        'z80.pspec',
        'z80.ldefs',
        'z182.pspec',
        'z8401x.pspec',
        'z180.pspec'
    ],
    'HCS08': [
        'HC08-MC68HC908QY4.pspec',
        'HC08.pspec',
        'HC05-M68HC05TB.pspec',
        'HCS08.pspec',
        'HCS_HC.sinc',
        'HCS08.cspec',
        'HCS08.opinion',
        'HC05.ldefs',
        'HCS08.ldefs',
        'HC05.pspec',
        'HC08.ldefs',
        'HCS08-MC9S08GB60.pspec'
    ],
    'DATA': [
        'data-ptr64.cspec',
        'data.pspec',
        'data-ptr16.cspec',
        'data.sinc',
        'data-ptr32.cspec',
        'data.ldefs'
    ],
    'PIC': [
        'pic16f.pspec',
        'pic17c7xx.ldefs',
        'pic16.ldefs',
        'pic18_instructions.sinc',
        'PIC24.ldefs',
        'pic12c5xx.cspec',
        'pic18.cspec',
        'pic16.cspec',
        'pic17c7xx.pspec',
        'pic16c5x.pspec',
        'PIC24.cspec',
        'pic16f.cspec',
        'pic18.pspec',
        'pic16_instructions.sinc',
        'pic17c7xx_instructions.sinc',
        'pic12_instructions.sinc',
        'pic17c7xx.sinc',
        'PIC33.dwarf',
        'PIC24.sinc',
        'pic18.sinc',
        'pic12c5xx.pspec',
        'PIC30.dwarf',
        'pic16c5x.cspec',
        'pic16.pspec',
        'pic16.sinc',
        'pic17c7xx.cspec',
        'pic12.sinc',
        'PIC24.opinion',
        'pic16c5x.ldefs',
        'pic12c5xx.ldefs',
        'pic18.ldefs',
        'PIC24.pspec'
    ],
    'Toy': [
        'toy64-long8.cspec',
        'toyInstructions.sinc',
        'toy_harvard.pspec',
        'toy.ldefs',
        'toyPosStack.cspec',
        'toy64.cspec',
        'toy_builder.sinc',
        'toy.pspec',
        'toy.cspec',
        'toy.sinc'
    ],
    'tricore': [
        'tricore.dwarf',
        'tricore.pcp.sinc',
        'tricore.opinion',
        'tc176x.pspec',
        'tricore.cspec',
        'tc29x.pspec',
        'tricore.sinc',
        'tricore.pspec',
        'tricore.ldefs',
        'tc172x.pspec'
    ],
    '8085': [
        '8085.ldefs',
        '8085.pspec',
        '8085.cspec'
    ],
    'Atmel': [
        'avr32a_logic_operations.sinc',
        'avr32a_instruction_flow.sinc',
        'avr32a_multiplication_operations.sinc',
        'avr8gcc.cspec',
        'avr32a_bit_operations.sinc',
        'avr32.opinion',
        'avr32a_shift_operations.sinc',
        'avr32a_arithmetic_operations.sinc',
        'avr8.pspec',
        'avr8xmega.pspec',
        'avr32a.pspec',
        'avr32a_data_transfer.sinc',
        'avr32a.cspec',
        'avr32a_system_control.sinc',
        'avr32a_simd_operations.sinc',
        'avr32a_dsp_operations2.sinc',
        'atmega256.pspec',
        'avr8imgCraftV8.cspec',
        'avr32a_autogen.sinc',
        'avr8.sinc',
        'avr8.opinion',
        'avr32a_dsp_operations.sinc',
        'avr32a_coprocessor_interface.sinc',
        'avr32a.ldefs',
        'avr8iarV1.cspec',
        'avr8.ldefs',
        'avr8egcc.cspec'
    ],
    'CR16': [
        'CR16.ldefs',
        'CR16.cspec',
        'CR16B.sinc',
        'CR16.opinion',
        'CR16.pspec',
        'CR16C.sinc'
    ],
    'JVM': [
        'JVM.pspec',
        'JVM.opinion',
        'JVM.cspec',
        'JVM.ldefs'
    ],
    'PowerPC': [
        'lmwInstructions.sinc',
        'ppc_64.cspec',
        'lswInstructions.sinc',
        'Scalar_SPFP.sinc',
        'ppc.ldefs',
        'ppc_isa.sinc',
        'vsx.sinc',
        'altivec.sinc',
        'ppc_instructions.sinc',
        'SPE_EFV.sinc',
        'ppc_64_32.cspec',
        '4xx.sinc',
        'SPE_APU.sinc',
        'ppc_32_mpc8270.pspec',
        'ppc_vle.sinc',
        'SPE_EFSD.sinc',
        'ppc_64.pspec',
        'ppc_common.sinc',
        'ppc_32_e500_le.cspec',
        'ppc_64_be_Mac.cspec',
        'ppc_32.pspec',
        'stmwInstructions.sinc',
        'ppc_embedded.sinc',
        'PowerPC.opinion',
        'ppc_a2.sinc',
        'stswiInstructions.sinc',
        'ppc.dwarf',
        'ppc_32_e500_be.cspec',
        'ppc_32_be.cspec',
        'g2.sinc',
        'ppc_32_be_Mac.cspec',
        'FPRC.sinc',
        'SPEF_SCR.sinc',
        'ppc_32_le.cspec',
        'quicciii.sinc',
        'evx.sinc'
    ],
    'MCS96': [
        'MCS96.cspec',
        'MCS96.pspec',
        'MCS96.ldefs',
        'MCS96.sinc'
    ],
    'PA-RISC': [
        'pa-risc32.cspec',
        'pa-risc.sinc',
        'pa-risc.opinion',
        'pa-risc.ldefs',
        'pa-riscInstructions.sinc',
        'pa-risc32.pspec'
    ],
    'AARCH64': [
        'AppleSilicon.ldefs',
        'AARCH64sve.sinc',
        'AARCH64.dwarf',
        'AARCH64base.sinc',
        'AARCH64_ilp32.cspec',
        'AARCH64_AMXext.sinc',
        'AARCH64.pspec',
        'AARCH64ldst.sinc',
        'AARCH64.ldefs',
        'AARCH64_win.cspec',
        'AARCH64.opinion',
        'AARCH64_base_PACoptions.sinc',
        'AARCH64neon.sinc',
        'AARCH64instructions.sinc',
        'AARCH64.cspec'
    ]
} 

sleigh_processor_specs = {
    'MIPS': {
        'mips64be.sla': [
            'mips64be.slaspec',
        ]
        + sleigh_processor_sources.get('MIPS'),
        'mips32R6le.sla': [
            'mips32R6le.slaspec',
        ]
        + sleigh_processor_sources.get('MIPS'),
        'mips64le.sla': [
            'mips64le.slaspec',
        ]
        + sleigh_processor_sources.get('MIPS'),
        'mips32be.sla': [
            'mips32be.slaspec',
        ]
        + sleigh_processor_sources.get('MIPS'),
        'mips32R6be.sla': [
            'mips32R6be.slaspec',
        ]
        + sleigh_processor_sources.get('MIPS'),
        'mips32le.sla': [
            'mips32le.slaspec',
        ]
        + sleigh_processor_sources.get('MIPS'),
    },
    'TI_MSP430': {
        'TI_MSP430X.sla': [
            'TI_MSP430X.slaspec',
        ]
        + sleigh_processor_sources.get('TI_MSP430'),
        'TI_MSP430.sla': [
            'TI_MSP430.slaspec',
        ]
        + sleigh_processor_sources.get('TI_MSP430'),
    },
    'x86': {
        'x86.sla': [
            'x86.slaspec',
        ]
        + sleigh_processor_sources.get('x86'),
        'x86-64.sla': [
            'x86-64.slaspec',
        ]
        + sleigh_processor_sources.get('x86'),
    },
    'Sparc': {
        'SparcV9_32.sla': [
            'SparcV9_32.slaspec',
        ]
        + sleigh_processor_sources.get('Sparc'),
        'SparcV9_64.sla': [
            'SparcV9_64.slaspec',
        ]
        + sleigh_processor_sources.get('Sparc'),
    },
    # TODO: V850 disabled due to subdirectory include
    # 'V850': {
    #     'V850.sla': [
    #         'V850.slaspec',
    #     ]
    #     + sleigh_processor_sources.get('V850'),
    # },
    '6502': {
        '65c02.sla': [
            '65c02.slaspec',
        ]
        + sleigh_processor_sources.get('6502'),
        '6502.sla': [
            '6502.slaspec',
        ]
        + sleigh_processor_sources.get('6502'),
    },
    'MC6800': {
        '6805.sla': [
            '6805.slaspec',
        ]
        + sleigh_processor_sources.get('MC6800'),
        'H6309.sla': [
            'H6309.slaspec',
        ]
        + sleigh_processor_sources.get('MC6800'),
        '6809.sla': [
            '6809.slaspec',
        ]
        + sleigh_processor_sources.get('MC6800'),
    },
    'M8C': {
        'm8c.sla': [
            'm8c.slaspec',
        ]
        + sleigh_processor_sources.get('M8C'),
    },
    'SuperH4': {
        'SuperH4_be.sla': [
            'SuperH4_be.slaspec',
        ]
        + sleigh_processor_sources.get('SuperH4'),
        'SuperH4_le.sla': [
            'SuperH4_le.slaspec',
        ]
        + sleigh_processor_sources.get('SuperH4'),
    },
    'HCS12': {
        'HCS12.sla': [
            'HCS12.slaspec',
        ]
        + sleigh_processor_sources.get('HCS12'),
        'HC12.sla': [
            'HC12.slaspec',
        ]
        + sleigh_processor_sources.get('HCS12'),
        'HCS12X.sla': [
            'HCS12X.slaspec',
        ]
        + sleigh_processor_sources.get('HCS12'),
    },
    'CP1600': {
        'CP1600.sla': [
            'CP1600.slaspec',
        ]
        + sleigh_processor_sources.get('CP1600'),
    },
    'Dalvik': {
        'Dalvik_DEX_Lollipop.sla': [
            'Dalvik_DEX_Lollipop.slaspec',
        ]
        + sleigh_processor_sources.get('Dalvik'),
        'Dalvik_DEX_Android10.sla': [
            'Dalvik_DEX_Android10.slaspec',
        ]
        + sleigh_processor_sources.get('Dalvik'),
        'Dalvik_Base.sla': [
            'Dalvik_Base.slaspec',
        ]
        + sleigh_processor_sources.get('Dalvik'),
        'Dalvik_ODEX_KitKat.sla': [
            'Dalvik_ODEX_KitKat.slaspec',
        ]
        + sleigh_processor_sources.get('Dalvik'),
        'Dalvik_DEX_Android11.sla': [
            'Dalvik_DEX_Android11.slaspec',
        ]
        + sleigh_processor_sources.get('Dalvik'),
        'Dalvik_DEX_Nougat.sla': [
            'Dalvik_DEX_Nougat.slaspec',
        ]
        + sleigh_processor_sources.get('Dalvik'),
        'Dalvik_DEX_Android12.sla': [
            'Dalvik_DEX_Android12.slaspec',
        ]
        + sleigh_processor_sources.get('Dalvik'),
        'Dalvik_DEX_Marshmallow.sla': [
            'Dalvik_DEX_Marshmallow.slaspec',
        ]
        + sleigh_processor_sources.get('Dalvik'),
        'Dalvik_DEX_KitKat.sla': [
            'Dalvik_DEX_KitKat.slaspec',
        ]
        + sleigh_processor_sources.get('Dalvik'),
        'Dalvik_DEX_Pie.sla': [
            'Dalvik_DEX_Pie.slaspec',
        ]
        + sleigh_processor_sources.get('Dalvik'),
        'Dalvik_DEX_Oreo.sla': [
            'Dalvik_DEX_Oreo.slaspec',
        ]
        + sleigh_processor_sources.get('Dalvik'),
    },
    '8048': {
        '8048.sla': [
            '8048.slaspec',
        ]
        + sleigh_processor_sources.get('8048'),
    },
    'RISCV': {
        'riscv.lp64d.sla': [
            'riscv.lp64d.slaspec',
        ]
        + sleigh_processor_sources.get('RISCV'),
        'riscv.ilp32d.sla': [
            'riscv.ilp32d.slaspec',
        ]
        + sleigh_processor_sources.get('RISCV'),
    },
    '8051': {
        '80251.sla': [
            '80251.slaspec',
        ]
        + sleigh_processor_sources.get('8051'),
        'mx51.sla': [
            'mx51.slaspec',
        ]
        + sleigh_processor_sources.get('8051'),
        '8051.sla': [
            '8051.slaspec',
        ]
        + sleigh_processor_sources.get('8051'),
        '80390.sla': [
            '80390.slaspec',
        ]
        + sleigh_processor_sources.get('8051'),
    },
    'SuperH': {
        'sh-2.sla': [
            'sh-2.slaspec',
        ]
        + sleigh_processor_sources.get('SuperH'),
        'sh-2a.sla': [
            'sh-2a.slaspec',
        ]
        + sleigh_processor_sources.get('SuperH'),
        'sh-1.sla': [
            'sh-1.slaspec',
        ]
        + sleigh_processor_sources.get('SuperH'),
    },
    'ARM': {
        'ARM4t_be.sla': [
            'ARM4t_be.slaspec',
        ]
        + sleigh_processor_sources.get('ARM'),
        'ARM6_be.sla': [
            'ARM6_be.slaspec',
        ]
        + sleigh_processor_sources.get('ARM'),
        'ARM7_le.sla': [
            'ARM7_le.slaspec',
        ]
        + sleigh_processor_sources.get('ARM'),
        'ARM5_be.sla': [
            'ARM5_be.slaspec',
        ]
        + sleigh_processor_sources.get('ARM'),
        'ARM5t_be.sla': [
            'ARM5t_be.slaspec',
        ]
        + sleigh_processor_sources.get('ARM'),
        'ARM7_be.sla': [
            'ARM7_be.slaspec',
        ]
        + sleigh_processor_sources.get('ARM'),
        'ARM4_be.sla': [
            'ARM4_be.slaspec',
        ]
        + sleigh_processor_sources.get('ARM'),
        'ARM6_le.sla': [
            'ARM6_le.slaspec',
        ]
        + sleigh_processor_sources.get('ARM'),
        'ARM4t_le.sla': [
            'ARM4t_le.slaspec',
        ]
        + sleigh_processor_sources.get('ARM'),
        'ARM8_le.sla': [
            'ARM8_le.slaspec',
        ]
        + sleigh_processor_sources.get('ARM'),
        'ARM4_le.sla': [
            'ARM4_le.slaspec',
        ]
        + sleigh_processor_sources.get('ARM'),
        'ARM8_be.sla': [
            'ARM8_be.slaspec',
        ]
        + sleigh_processor_sources.get('ARM'),
        'ARM5t_le.sla': [
            'ARM5t_le.slaspec',
        ]
        + sleigh_processor_sources.get('ARM'),
        'ARM5_le.sla': [
            'ARM5_le.slaspec',
        ]
        + sleigh_processor_sources.get('ARM'),
    },
    '68000': {
        '68030.sla': [
            '68030.slaspec',
        ]
        + sleigh_processor_sources.get('68000'),
        'coldfire.sla': [
            'coldfire.slaspec',
        ]
        + sleigh_processor_sources.get('68000'),
        '68020.sla': [
            '68020.slaspec',
        ]
        + sleigh_processor_sources.get('68000'),
        '68040.sla': [
            '68040.slaspec',
        ]
        + sleigh_processor_sources.get('68000'),
    },
    'Z80': {
        'z180.sla': [
            'z180.slaspec',
        ]
        + sleigh_processor_sources.get('Z80'),
        'z80.sla': [
            'z80.slaspec',
        ]
        + sleigh_processor_sources.get('Z80'),
    },
    'HCS08': {
        'HC05.sla': [
            'HC05.slaspec',
        ]
        + sleigh_processor_sources.get('HCS08'),
        'HCS08.sla': [
            'HCS08.slaspec',
        ]
        + sleigh_processor_sources.get('HCS08'),
        'HC08.sla': [
            'HC08.slaspec',
        ]
        + sleigh_processor_sources.get('HCS08'),
    },
    'DATA': {
        'data-be-64.sla': [
            'data-be-64.slaspec',
        ]
        + sleigh_processor_sources.get('DATA'),
        'data-le-64.sla': [
            'data-le-64.slaspec',
        ]
        + sleigh_processor_sources.get('DATA'),
    },
    'PIC': {
        'dsPIC30F.sla': [
            'dsPIC30F.slaspec',
        ]
        + sleigh_processor_sources.get('PIC'),
        'PIC24F.sla': [
            'PIC24F.slaspec',
        ]
        + sleigh_processor_sources.get('PIC'),
        'pic12c5xx.sla': [
            'pic12c5xx.slaspec',
        ]
        + sleigh_processor_sources.get('PIC'),
        'dsPIC33E.sla': [
            'dsPIC33E.slaspec',
        ]
        + sleigh_processor_sources.get('PIC'),
        'pic18.sla': [
            'pic18.slaspec',
        ]
        + sleigh_processor_sources.get('PIC'),
        'PIC24H.sla': [
            'PIC24H.slaspec',
        ]
        + sleigh_processor_sources.get('PIC'),
        'dsPIC33F.sla': [
            'dsPIC33F.slaspec',
        ]
        + sleigh_processor_sources.get('PIC'),
        'pic17c7xx.sla': [
            'pic17c7xx.slaspec',
        ]
        + sleigh_processor_sources.get('PIC'),
        'pic16c5x.sla': [
            'pic16c5x.slaspec',
        ]
        + sleigh_processor_sources.get('PIC'),
        'pic16.sla': [
            'pic16.slaspec',
        ]
        + sleigh_processor_sources.get('PIC'),
        'pic16f.sla': [
            'pic16f.slaspec',
        ]
        + sleigh_processor_sources.get('PIC'),
        'PIC24E.sla': [
            'PIC24E.slaspec',
        ]
        + sleigh_processor_sources.get('PIC'),
        'dsPIC33C.sla': [
            'dsPIC33C.slaspec',
        ]
        + sleigh_processor_sources.get('PIC'),
    },
    'Toy': {
        'toy_builder_le_align2.sla': [
            'toy_builder_le_align2.slaspec',
        ]
        + sleigh_processor_sources.get('Toy'),
        'toy_le.sla': [
            'toy_le.slaspec',
        ]
        + sleigh_processor_sources.get('Toy'),
        'toy_builder_be.sla': [
            'toy_builder_be.slaspec',
        ]
        + sleigh_processor_sources.get('Toy'),
        'toy_wsz_le.sla': [
            'toy_wsz_le.slaspec',
        ]
        + sleigh_processor_sources.get('Toy'),
        'toy64_be_harvard.sla': [
            'toy64_be_harvard.slaspec',
        ]
        + sleigh_processor_sources.get('Toy'),
        'toy64_be.sla': [
            'toy64_be.slaspec',
        ]
        + sleigh_processor_sources.get('Toy'),
        'toy_wsz_be.sla': [
            'toy_wsz_be.slaspec',
        ]
        + sleigh_processor_sources.get('Toy'),
        'toy_be_posStack.sla': [
            'toy_be_posStack.slaspec',
        ]
        + sleigh_processor_sources.get('Toy'),
        'toy_builder_le.sla': [
            'toy_builder_le.slaspec',
        ]
        + sleigh_processor_sources.get('Toy'),
        'toy_builder_be_align2.sla': [
            'toy_builder_be_align2.slaspec',
        ]
        + sleigh_processor_sources.get('Toy'),
        'toy_be.sla': [
            'toy_be.slaspec',
        ]
        + sleigh_processor_sources.get('Toy'),
        'toy64_le.sla': [
            'toy64_le.slaspec',
        ]
        + sleigh_processor_sources.get('Toy'),
    },
    'tricore': {
        'tricore.sla': [
            'tricore.slaspec',
        ]
        + sleigh_processor_sources.get('tricore'),
    },
    '8085': {
        '8085.sla': [
            '8085.slaspec',
        ]
        + sleigh_processor_sources.get('8085'),
    },
    'Atmel': {
        'avr8eind.sla': [
            'avr8eind.slaspec',
        ]
        + sleigh_processor_sources.get('Atmel'),
        'avr8e.sla': [
            'avr8e.slaspec',
        ]
        + sleigh_processor_sources.get('Atmel'),
        'avr32a.sla': [
            'avr32a.slaspec',
        ]
        + sleigh_processor_sources.get('Atmel'),
        'avr8xmega.sla': [
            'avr8xmega.slaspec',
        ]
        + sleigh_processor_sources.get('Atmel'),
        'avr8.sla': [
            'avr8.slaspec',
        ]
        + sleigh_processor_sources.get('Atmel'),
    },
    'CR16': {
        'CR16C.sla': [
            'CR16C.slaspec',
        ]
        + sleigh_processor_sources.get('CR16'),
        'CR16B.sla': [
            'CR16B.slaspec',
        ]
        + sleigh_processor_sources.get('CR16'),
    },
    'JVM': {
        'JVM.sla': [
            'JVM.slaspec',
        ]
        + sleigh_processor_sources.get('JVM'),
    },
    'PowerPC': {
        'ppc_32_e500_le.sla': [
            'ppc_32_e500_le.slaspec',
        ]
        + sleigh_processor_sources.get('PowerPC'),
        'ppc_32_le.sla': [
            'ppc_32_le.slaspec',
        ]
        + sleigh_processor_sources.get('PowerPC'),
        'ppc_64_be.sla': [
            'ppc_64_be.slaspec',
        ]
        + sleigh_processor_sources.get('PowerPC'),
        'ppc_32_quicciii_be.sla': [
            'ppc_32_quicciii_be.slaspec',
        ]
        + sleigh_processor_sources.get('PowerPC'),
        'ppc_32_4xx_be.sla': [
            'ppc_32_4xx_be.slaspec',
        ]
        + sleigh_processor_sources.get('PowerPC'),
        'ppc_32_be.sla': [
            'ppc_32_be.slaspec',
        ]
        + sleigh_processor_sources.get('PowerPC'),
        'ppc_64_isa_le.sla': [
            'ppc_64_isa_le.slaspec',
        ]
        + sleigh_processor_sources.get('PowerPC'),
        'ppc_64_le.sla': [
            'ppc_64_le.slaspec',
        ]
        + sleigh_processor_sources.get('PowerPC'),
        'ppc_64_isa_be.sla': [
            'ppc_64_isa_be.slaspec',
        ]
        + sleigh_processor_sources.get('PowerPC'),
        'ppc_64_isa_altivec_be.sla': [
            'ppc_64_isa_altivec_be.slaspec',
        ]
        + sleigh_processor_sources.get('PowerPC'),
        'ppc_32_4xx_le.sla': [
            'ppc_32_4xx_le.slaspec',
        ]
        + sleigh_processor_sources.get('PowerPC'),
        'ppc_64_isa_vle_be.sla': [
            'ppc_64_isa_vle_be.slaspec',
        ]
        + sleigh_processor_sources.get('PowerPC'),
        'ppc_32_e500_be.sla': [
            'ppc_32_e500_be.slaspec',
        ]
        + sleigh_processor_sources.get('PowerPC'),
        'ppc_32_quicciii_le.sla': [
            'ppc_32_quicciii_le.slaspec',
        ]
        + sleigh_processor_sources.get('PowerPC'),
        'ppc_64_isa_altivec_vle_be.sla': [
            'ppc_64_isa_altivec_vle_be.slaspec',
        ]
        + sleigh_processor_sources.get('PowerPC'),
        'ppc_64_isa_altivec_le.sla': [
            'ppc_64_isa_altivec_le.slaspec',
        ]
        + sleigh_processor_sources.get('PowerPC'),
    },
    'MCS96': {
        'MCS96.sla': [
            'MCS96.slaspec',
        ]
        + sleigh_processor_sources.get('MCS96'),
    },
    'PA-RISC': {
        'pa-risc32be.sla': [
            'pa-risc32be.slaspec',
        ]
        + sleigh_processor_sources.get('PA-RISC'),
    },
    'AARCH64': {
        'AARCH64_AppleSilicon.sla': [
            'AARCH64_AppleSilicon.slaspec',
        ]
        + sleigh_processor_sources.get('AARCH64'),
        'AARCH64.sla': [
            'AARCH64.slaspec',
        ]
        + sleigh_processor_sources.get('AARCH64'),
        'AARCH64BE.sla': [
            'AARCH64BE.slaspec',
        ]
        + sleigh_processor_sources.get('AARCH64'),
    },
}



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
