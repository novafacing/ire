# Meson build file for building pcode-rs C++ dependencies

project('pcode-rs', 'cpp')

fs = import('fs')

ghidra_decompile_cpp_dir = './ghidra/Ghidra/Features/Decompiler/src/decompile/cpp'

if not fs.is_dir(ghidra_decompile_cpp_dir)
  error('Ghidra decompile C++ directory not found. Run `git submodule update --init --recursive` to fetch it.')
endif

# Generated Sources - Scanner & Lexer Files

flex = find_program('flex', required: true)
bison = find_program('bison', required: true)

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
    command: [flex, '-o', '@OUTPUT@', '@INPUT@']
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


sleigh = executable(
    'sleighcc',
    core_src + sleigh_src + sleigh_compiler_src,
    include_directories: [ghidra_decompile_cpp_dir],
    c_args: ['-Wno-sign-compare']
)