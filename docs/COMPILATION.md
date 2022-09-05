# Compiling Ghidra Decompiler

## Understanding the Makefile

There's an easy way to get all the make variables:
```makefile
BFDHOME=/usr

MAKE_STATIC=
ARCH_TYPE=
ADDITIONAL_FLAGS=
SLEIGHVERSION=sleigh-2.1.0

EXTENSION_POINT=../../../../../../../ghidra.ext-u/Ghidra/Features/DecompilerExtensions/src/decompile/cpp
GHIDRA_BIN=../../../../../../../ghidra.bin

OS = $(shell uname -s)
CPU = $(shell uname -m)

# TODO: need to revise to support arm64/aarch64 arch - improve on both OS and arch detection 

ifeq ($(OS),Linux)
# Allow ARCH to be specified externally so we can build for 32-bit from a 64-bit Linux
ifndef ARCH
  ARCH=$(CPU)
endif
ifeq ($(ARCH),x86_64)
  ARCH_TYPE=-m64
  OSDIR=linux_x86_64
else
  ARCH_TYPE=-m32
  OSDIR=linux_x86_32
endif
endif

ifeq ($(OS),Darwin)
  MAKE_STATIC=
  ARCH_TYPE=-arch x86_64
  ADDITIONAL_FLAGS=-mmacosx-version-min=10.6 -w
  OSDIR=mac_x86_64
endif

CC=gcc
CXX=g++

# Debug flags
DBG_CXXFLAGS=-g -std=c++11 -Wall -Wno-sign-compare
#DBG_CXXFLAGS=-g -pg -Wall -Wno-sign-compare
#DBG_CXXFLAGS=-g -fprofile-arcs -ftest-coverage -Wall -Wno-sign-compare

# Optimization flags
OPT_CXXFLAGS=-O2 -std=c++11 -Wall -Wno-sign-compare

YACC=bison

# libraries
#INCLUDES=-I$(BFDHOME)/include
INCLUDES=
BFDLIB=-lbfd -lz

LNK=

# Source files
ALL_SOURCE= $(wildcard *.cc)
ALL_NAMES=$(subst .cc,,$(ALL_SOURCE))
UNITTEST_SOURCE= $(wildcard ../unittests/*.cc)
UNITTEST_NAMES=$(subst .cc,,$(UNITTEST_SOURCE))
UNITTEST_STRIP=$(subst ../unittests/,,$(UNITTEST_NAMES))

COREEXT_SOURCE= $(wildcard coreext_*.cc)
COREEXT_NAMES=$(subst .cc,,$(COREEXT_SOURCE))

GHIDRAEXT_SOURCE= $(wildcard ghidraext_*.cc)
GHIDRAEXT_NAMES=$(subst .cc,,$(GHIDRAEXT_SOURCE))

EXTERNAL_COREEXT_SOURCE= $(wildcard $(EXTENSION_POINT)/coreext_*.cc)
EXTERNAL_GHIDRAEXT_SOURCE= $(wildcard $(EXTENSION_POINT)/ghidraext_*.cc)
EXTERNAL_CONSOLEEXT_SOURCE= $(wildcard $(EXTENSION_POINT)/consoleext_*.cc)
EXTERNAL_COREEXT_NAMES=$(subst .cc,,$(notdir $(EXTERNAL_COREEXT_SOURCE)))
EXTERNAL_GHIDRAEXT_NAMES=$(subst .cc,,$(notdir $(EXTERNAL_GHIDRAEXT_SOURCE)))
EXTERNAL_CONSOLEEXT_NAMES=$(subst .cc,,$(notdir $(EXTERNAL_CONSOLEEXT_SOURCE)))

# The following macros partition all the source files, there should be no overlaps
# Some core source files used in all projects
CORE=	xml marshal space float address pcoderaw translate opcodes globalcontext
# Additional core files for any projects that decompile
DECCORE=capability architecture options graph cover block cast typeop database cpool \
	comment stringmanage fspec action loadimage grammar varnode op \
	type variable varmap jumptable emulate emulateutil flow userop \
	funcdata funcdata_block funcdata_op funcdata_varnode unionresolve pcodeinject \
	heritage prefersplit rangeutil ruleaction subflow blockaction merge double \
	transform coreaction condexe override dynamic crc32 prettyprint \
	printlanguage printc printjava memstate opbehavior paramid $(COREEXT_NAMES)
# Files used for any project that use the sleigh decoder
SLEIGH=	sleigh pcodeparse pcodecompile sleighbase slghsymbol \
	slghpatexpress slghpattern semantics context filemanage
# Additional files for the GHIDRA specific build
GHIDRA=	ghidra_arch inject_ghidra ghidra_translate loadimage_ghidra \
	typegrp_ghidra database_ghidra ghidra_context cpool_ghidra \
	ghidra_process comment_ghidra string_ghidra $(GHIDRAEXT_NAMES)
# Additional files specific to the sleigh compiler
SLACOMP=slgh_compile slghparse slghscan
# Additional special files that should not be considered part of the library
SPECIAL=consolemain sleighexample test
# Any additional modules for the command line decompiler
EXTRA= $(filter-out $(CORE) $(DECCORE) $(SLEIGH) $(GHIDRA) $(SLACOMP) $(SPECIAL),$(ALL_NAMES))

EXECS=decomp_dbg decomp_opt ghidra_test_dbg ghidra_dbg ghidra_opt sleigh_dbg sleigh_opt libdecomp_dbg.a libdecomp.a

# Possible conditional compilation flags
#     __TERMINAL__             # Turn on terminal support for console mode
#     CPUI_STATISTICS          # Turn on collection of cover and cast statistics
#     CPUI_RULECOMPILE         # Allow user defined dynamic rules

# Debug compilation flags
#     OPACTION_DEBUG           # Turns on all the action tracing facilities
#     MERGEMULTI_DEBUG         # Check for MULTIEQUAL and INDIRECT intersections
#     BLOCKCONSISTENT_DEBUG    # Check that block graph structure is consistent
#     DFSVERIFY_DEBUG          # make sure that the block ordering algorithm produces
#                                a true depth first traversal of the dominator tree
#     CPUI_DEBUG               # This is the one controlling switch for all the other debug switches

COMMANDLINE_NAMES=$(CORE) $(DECCORE) $(EXTRA) $(SLEIGH) consolemain
COMMANDLINE_DEBUG=-DCPUI_DEBUG -D__TERMINAL__
COMMANDLINE_OPT=-D__TERMINAL__

TEST_NAMES=$(CORE) $(DECCORE) $(SLEIGH) $(EXTRA) test 
TEST_DEBUG=-D__TERMINAL__

GHIDRA_NAMES=$(CORE) $(DECCORE) $(GHIDRA)
GHIDRA_NAMES_DBG=$(GHIDRA_NAMES) callgraph ifacedecomp testfunction ifaceterm interface
GHIDRA_DEBUG=-DCPUI_DEBUG
GHIDRA_OPT=

SLEIGH_NAMES=$(CORE) $(SLEIGH) $(SLACOMP)
SLEIGH_DEBUG=-DYYDEBUG
SLEIGH_OPT=

# The SLEIGH library is built with console mode objects and it
# uses the COMMANDLINE_* options
LIBSLA_NAMES=$(CORE) $(SLEIGH) loadimage sleigh memstate emulate opbehavior

# The Decompiler library is built with console mode objects and it uses the COMMANDLINE_* options
LIBDECOMP_NAMES=$(CORE) $(DECCORE) $(EXTRA) $(SLEIGH)

# object file macros
COMMANDLINE_DBG_OBJS=$(COMMANDLINE_NAMES:%=com_dbg/%.o)
COMMANDLINE_OPT_OBJS=$(COMMANDLINE_NAMES:%=com_opt/%.o)
TEST_DEBUG_OBJS=$(TEST_NAMES:%=test_dbg/%.o) $(UNITTEST_STRIP:%=test_dbg/%.o)
GHIDRA_DBG_OBJS=$(GHIDRA_NAMES_DBG:%=ghi_dbg/%.o)
GHIDRA_OPT_OBJS=$(GHIDRA_NAMES:%=ghi_opt/%.o)
SLEIGH_DBG_OBJS=$(SLEIGH_NAMES:%=sla_dbg/%.o)
SLEIGH_OPT_OBJS=$(SLEIGH_NAMES:%=sla_opt/%.o)
LIBSLA_DBG_OBJS=$(LIBSLA_NAMES:%=com_dbg/%.o)
LIBSLA_OPT_OBJS=$(LIBSLA_NAMES:%=com_opt/%.o)
LIBSLA_SOURCE=$(LIBSLA_NAMES:%=%.cc) $(LIBSLA_NAMES:%=%.hh) \
	$(SLACOMP:%=%.cc) slgh_compile.hh slghparse.hh types.h \
	partmap.hh error.hh slghparse.y pcodeparse.y xml.y slghscan.l loadimage_bfd.hh loadimage_bfd.cc
LIBDECOMP_DBG_OBJS=$(LIBDECOMP_NAMES:%=com_dbg/%.o)
LIBDECOMP_OPT_OBJS=$(LIBDECOMP_NAMES:%=com_opt/%.o)

# conditionals to determine which dependency files to build
DEPNAMES=com_dbg/depend com_opt/depend

all:
	@echo "."
	@echo "."
	@echo "ALL_SOURCE=" $(ALL_SOURCE)
	@echo "."
	@echo "."
	@echo "ALL_NAMES=" $(ALL_NAMES)
	@echo "."
	@echo "."
	@echo "UNITTEST_SOURCE=" $(UNITTEST_SOURCE)
	@echo "."
	@echo "."
	@echo "UNITTEST_NAMES=" $(UNITTEST_NAMES)
	@echo "."
	@echo "."
	@echo "UNITTEST_STRIP=" $(UNITTEST_STRIP)
	@echo "."
	@echo "."
	@echo "COREEXT_SOURCE=" $(COREEXT_SOURCE)
	@echo "."
	@echo "."
	@echo "COREEXT_NAMES=" $(COREEXT_NAMES)
	@echo "."
	@echo "."
	@echo "GHIDRAEXT_SOURCE=" $(GHIDRAEXT_SOURCE)
	@echo "."
	@echo "."
	@echo "GHIDRAEXT_NAMES=" $(GHIDRAEXT_NAMES)
	@echo "."
	@echo "."
	@echo "EXTERNAL_COREEXT_SOURCE=" $(EXTERNAL_COREEXT_SOURCE)
	@echo "."
	@echo "."
	@echo "EXTERNAL_GHIDRAEXT_SOURCE=" $(EXTERNAL_GHIDRAEXT_SOURCE)
	@echo "."
	@echo "."
	@echo "EXTERNAL_CONSOLEEXT_SOURCE=" $(EXTERNAL_CONSOLEEXT_SOURCE)
	@echo "."
	@echo "."
	@echo "EXTERNAL_COREEXT_NAMES=" $(EXTERNAL_COREEXT_NAMES)
	@echo "."
	@echo "."
	@echo "EXTERNAL_GHIDRAEXT_NAMES=" $(EXTERNAL_GHIDRAEXT_NAMES)
	@echo "."
	@echo "."
	@echo "EXTERNAL_CONSOLEEXT_NAMES=" $(EXTERNAL_CONSOLEEXT_NAMES)
	@echo "."
	@echo "."
	@echo "CORE=" $(CORE)
	@echo "."
	@echo "."
	@echo "DECCORE=" $(DECCORE)
	@echo "."
	@echo "."
	@echo "SLEIGH=" $(SLEIGH)
	@echo "."
	@echo "."
	@echo "GHIDRA=" $(GHIDRA)
	@echo "."
	@echo "."
	@echo "SLACOMP=" $(SLACOMP)
	@echo "."
	@echo "."
	@echo "SPECIAL=" $(SPECIAL)
	@echo "."
	@echo "."
	@echo "EXTRA=" $(EXTRA)
	@echo "."
	@echo "."
	@echo "EXECS=" $(EXECS)
	@echo "."
	@echo "."
	@echo "COMMANDLINE_NAMES=" $(COMMANDLINE_NAMES)
	@echo "."
	@echo "."
	@echo "COMMANDLINE_DEBUG=" $(COMMANDLINE_DEBUG)
	@echo "."
	@echo "."
	@echo "COMMANDLINE_OPT=" $(COMMANDLINE_OPT)
	@echo "."
	@echo "."
	@echo "TEST_NAMES=" $(TEST_NAMES)
	@echo "."
	@echo "."
	@echo "TEST_DEBUG=" $(TEST_DEBUG)
	@echo "."
	@echo "."
	@echo "GHIDRA_NAMES=" $(GHIDRA_NAMES)
	@echo "."
	@echo "."
	@echo "GHIDRA_NAMES_DBG=" $(GHIDRA_NAMES_DBG)
	@echo "."
	@echo "."
	@echo "GHIDRA_DEBUG=" $(GHIDRA_DEBUG)
	@echo "."
	@echo "."
	@echo "GHIDRA_OPT=" $(GHIDRA_OPT)
	@echo "."
	@echo "."
	@echo "SLEIGH_NAMES=" $(SLEIGH_NAMES)
	@echo "."
	@echo "."
	@echo "SLEIGH_DEBUG=" $(SLEIGH_DEBUG)
	@echo "."
	@echo "."
	@echo "SLEIGH_OPT=" $(SLEIGH_OPT)
	@echo "."
	@echo "."
	@echo "LIBSLA_NAMES=" $(LIBSLA_NAMES)
	@echo "."
	@echo "."
	@echo "LIBDECOMP_NAMES=" $(LIBDECOMP_NAMES)
	@echo "."
	@echo "."
	@echo "COMMANDLINE_DBG_OBJS=" $(COMMANDLINE_DBG_OBJS)
	@echo "."
	@echo "."
	@echo "COMMANDLINE_OPT_OBJS=" $(COMMANDLINE_OPT_OBJS)
	@echo "."
	@echo "."
	@echo "TEST_DEBUG_OBJS=" $(TEST_DEBUG_OBJS)
	@echo "."
	@echo "."
	@echo "GHIDRA_DBG_OBJS=" $(GHIDRA_DBG_OBJS)
	@echo "."
	@echo "."
	@echo "GHIDRA_OPT_OBJS=" $(GHIDRA_OPT_OBJS)
	@echo "."
	@echo "."
	@echo "SLEIGH_DBG_OBJS=" $(SLEIGH_DBG_OBJS)
	@echo "."
	@echo "."
	@echo "SLEIGH_OPT_OBJS=" $(SLEIGH_OPT_OBJS)
	@echo "."
	@echo "."
	@echo "LIBSLA_DBG_OBJS=" $(LIBSLA_DBG_OBJS)
	@echo "."
	@echo "."
	@echo "LIBSLA_OPT_OBJS=" $(LIBSLA_OPT_OBJS)
	@echo "."
	@echo "."
	@echo "LIBSLA_SOURCE=" $(LIBSLA_SOURCE)
	@echo "."
	@echo "."
	@echo "LIBDECOMP_DBG_OBJS=" $(LIBDECOMP_DBG_OBJS)
	@echo "."
	@echo "."
	@echo "LIBDECOMP_OPT_OBJS=" $(LIBDECOMP_OPT_OBJS)
	@echo "."
	@echo "."
	@echo "DEPNAMES=" $(DEPNAMES)
```

This will output:

```
.
.
ALL_SOURCE= action.cc address.cc architecture.cc bfd_arch.cc blockaction.cc block.cc callgraph.cc capability.cc cast.cc codedata.cc comment.cc comment_ghidra.cc condexe.cc consolemain.cc context.cc coreaction.cc cover.cc cpool.cc cpool_ghidra.cc crc32.cc database.cc database_ghidra.cc double.cc dynamic.cc emulate.cc emulateutil.cc filemanage.cc float.cc flow.cc fspec.cc funcdata_block.cc funcdata.cc funcdata_op.cc funcdata_varnode.cc ghidra_arch.cc ghidra_context.cc ghidra_process.cc ghidra_translate.cc globalcontext.cc grammar.cc graph.cc heritage.cc ifacedecomp.cc ifaceterm.cc inject_ghidra.cc inject_sleigh.cc interface.cc jumptable.cc libdecomp.cc loadimage_bfd.cc loadimage.cc loadimage_ghidra.cc loadimage_xml.cc marshal.cc memstate.cc merge.cc opbehavior.cc op.cc opcodes.cc options.cc override.cc paramid.cc pcodecompile.cc pcodeinject.cc pcodeparse.cc pcoderaw.cc prefersplit.cc prettyprint.cc printc.cc printjava.cc printlanguage.cc rangeutil.cc raw_arch.cc ruleaction.cc rulecompile.cc ruleparse.cc semantics.cc sleigh_arch.cc sleighbase.cc sleigh.cc sleighexample.cc slgh_compile.cc slghparse.cc slghpatexpress.cc slghpattern.cc slghscan.cc slghsymbol.cc space.cc string_ghidra.cc stringmanage.cc subflow.cc test.cc testfunction.cc transform.cc translate.cc type.cc typegrp_ghidra.cc typeop.cc unify.cc unionresolve.cc userop.cc variable.cc varmap.cc varnode.cc xml_arch.cc xml.cc
.
.
ALL_NAMES= action address architecture bfd_arch blockaction block callgraph capability cast codedata comment comment_ghidra condexe consolemain context coreaction cover cpool cpool_ghidra crc32 database database_ghidra double dynamic emulate emulateutil filemanage float flow fspec funcdata_block funcdata funcdata_op funcdata_varnode ghidra_arch ghidra_context ghidra_process ghidra_translate globalcontext grammar graph heritage ifacedecomp ifaceterm inject_ghidra inject_sleigh interface jumptable libdecomp loadimage_bfd loadimage loadimage_ghidra loadimage_xml marshal memstate merge opbehavior op opcodes options override paramid pcodecompile pcodeinject pcodeparse pcoderaw prefersplit prettyprint printc printjava printlanguage rangeutil raw_arch ruleaction rulecompile ruleparse semantics sleigh_arch sleighbase sleigh sleighexample slgh_compile slghparse slghpatexpress slghpattern slghscan slghsymbol space string_ghidra stringmanage subflow test testfunction transform translate type typegrp_ghidra typeop unify unionresolve userop variable varmap varnode xml_arch xml
.
.
UNITTEST_SOURCE= ../unittests/testcirclerange.cc ../unittests/testfloatemu.cc ../unittests/testmarshal.cc ../unittests/testtypes.cc
.
.
UNITTEST_NAMES= ../unittests/testcirclerange ../unittests/testfloatemu ../unittests/testmarshal ../unittests/testtypes
.
.
UNITTEST_STRIP= testcirclerange testfloatemu testmarshal testtypes
.
.
COREEXT_SOURCE=
.
.
COREEXT_NAMES=
.
.
GHIDRAEXT_SOURCE=
.
.
GHIDRAEXT_NAMES=
.
.
EXTERNAL_COREEXT_SOURCE=
.
.
EXTERNAL_GHIDRAEXT_SOURCE=
.
.
EXTERNAL_CONSOLEEXT_SOURCE=
.
.
EXTERNAL_COREEXT_NAMES=
.
.
EXTERNAL_GHIDRAEXT_NAMES=
.
.
EXTERNAL_CONSOLEEXT_NAMES=
.
.
CORE= xml marshal space float address pcoderaw translate opcodes globalcontext
.
.
DECCORE= capability architecture options graph cover block cast typeop database cpool comment stringmanage fspec action loadimage grammar varnode op type variable varmap jumptable emulate emulateutil flow userop funcdata funcdata_block funcdata_op funcdata_varnode unionresolve pcodeinject heritage prefersplit rangeutil ruleaction subflow blockaction merge double transform coreaction condexe override dynamic crc32 prettyprint printlanguage printc printjava memstate opbehavior paramid
.
.
SLEIGH= sleigh pcodeparse pcodecompile sleighbase slghsymbol slghpatexpress slghpattern semantics context filemanage
.
.
GHIDRA= ghidra_arch inject_ghidra ghidra_translate loadimage_ghidra typegrp_ghidra database_ghidra ghidra_context cpool_ghidra ghidra_process comment_ghidra string_ghidra
.
.
SLACOMP= slgh_compile slghparse slghscan
.
.
SPECIAL= consolemain sleighexample test
.
.
EXTRA= bfd_arch callgraph codedata ifacedecomp ifaceterm inject_sleigh interface libdecomp loadimage_bfd loadimage_xml raw_arch rulecompile ruleparse sleigh_arch testfunction unify xml_arch
.
.
EXECS= decomp_dbg decomp_opt ghidra_test_dbg ghidra_dbg ghidra_opt sleigh_dbg sleigh_opt libdecomp_dbg.a libdecomp.a
.
.
COMMANDLINE_NAMES= xml marshal space float address pcoderaw translate opcodes globalcontext capability architecture options graph cover block cast typeop database cpool comment stringmanage fspec action loadimage grammar varnode op type variable varmap jumptable emulate emulateutil flow userop funcdata funcdata_block funcdata_op funcdata_varnode unionresolve pcodeinject heritage prefersplit rangeutil ruleaction subflow blockaction merge double transform coreaction condexe override dynamic crc32 prettyprint printlanguage printc printjava memstate opbehavior paramid bfd_arch callgraph codedata ifacedecomp ifaceterm inject_sleigh interface libdecomp loadimage_bfd loadimage_xml raw_arch rulecompile ruleparse sleigh_arch testfunction unify xml_arch sleigh pcodeparse pcodecompile sleighbase slghsymbol slghpatexpress slghpattern semantics context filemanage consolemain
.
.
COMMANDLINE_DEBUG= -DCPUI_DEBUG -D__TERMINAL__
.
.
COMMANDLINE_OPT= -D__TERMINAL__
.
.
TEST_NAMES= xml marshal space float address pcoderaw translate opcodes globalcontext capability architecture options graph cover block cast typeop database cpool comment stringmanage fspec action loadimage grammar varnode op type variable varmap jumptable emulate emulateutil flow userop funcdata funcdata_block funcdata_op funcdata_varnode unionresolve pcodeinject heritage prefersplit rangeutil ruleaction subflow blockaction merge double transform coreaction condexe override dynamic crc32 prettyprint printlanguage printc printjava memstate opbehavior paramid sleigh pcodeparse pcodecompile sleighbase slghsymbol slghpatexpress slghpattern semantics context filemanage bfd_arch callgraph codedata ifacedecomp ifaceterm inject_sleigh interface libdecomp loadimage_bfd loadimage_xml raw_arch rulecompile ruleparse sleigh_arch testfunction unify xml_arch test
.
.
TEST_DEBUG= -D__TERMINAL__
.
.
GHIDRA_NAMES= xml marshal space float address pcoderaw translate opcodes globalcontext capability architecture options graph cover block cast typeop database cpool comment stringmanage fspec action loadimage grammar varnode op type variable varmap jumptable emulate emulateutil flow userop funcdata funcdata_block funcdata_op funcdata_varnode unionresolve pcodeinject heritage prefersplit rangeutil ruleaction subflow blockaction merge double transform coreaction condexe override dynamic crc32 prettyprint printlanguage printc printjava memstate opbehavior paramid ghidra_arch inject_ghidra ghidra_translate loadimage_ghidra typegrp_ghidra database_ghidra ghidra_context cpool_ghidra ghidra_process comment_ghidra string_ghidra
.
.
GHIDRA_NAMES_DBG= xml marshal space float address pcoderaw translate opcodes globalcontext capability architecture options graph cover block cast typeop database cpool comment stringmanage fspec action loadimage grammar varnode op type variable varmap jumptable emulate emulateutil flow userop funcdata funcdata_block funcdata_op funcdata_varnode unionresolve pcodeinject heritage prefersplit rangeutil ruleaction subflow blockaction merge double transform coreaction condexe override dynamic crc32 prettyprint printlanguage printc printjava memstate opbehavior paramid ghidra_arch inject_ghidra ghidra_translate loadimage_ghidra typegrp_ghidra database_ghidra ghidra_context cpool_ghidra ghidra_process comment_ghidra string_ghidra callgraph ifacedecomp testfunction ifaceterm interface
.
.
GHIDRA_DEBUG= -DCPUI_DEBUG
.
.
GHIDRA_OPT=
.
.
SLEIGH_NAMES= xml marshal space float address pcoderaw translate opcodes globalcontext sleigh pcodeparse pcodecompile sleighbase slghsymbol slghpatexpress slghpattern semantics context filemanage slgh_compile slghparse slghscan
.
.
SLEIGH_DEBUG= -DYYDEBUG
.
.
SLEIGH_OPT=
.
.
LIBSLA_NAMES= xml marshal space float address pcoderaw translate opcodes globalcontext sleigh pcodeparse pcodecompile sleighbase slghsymbol slghpatexpress slghpattern semantics context filemanage loadimage sleigh memstate emulate opbehavior
.
.
LIBDECOMP_NAMES= xml marshal space float address pcoderaw translate opcodes globalcontext capability architecture options graph cover block cast typeop database cpool comment stringmanage fspec action loadimage grammar varnode op type variable varmap jumptable emulate emulateutil flow userop funcdata funcdata_block funcdata_op funcdata_varnode unionresolve pcodeinject heritage prefersplit rangeutil ruleaction subflow blockaction merge double transform coreaction condexe override dynamic crc32 prettyprint printlanguage printc printjava memstate opbehavior paramid bfd_arch callgraph codedata ifacedecomp ifaceterm inject_sleigh interface libdecomp loadimage_bfd loadimage_xml raw_arch rulecompile ruleparse sleigh_arch testfunction unify xml_arch sleigh pcodeparse pcodecompile sleighbase slghsymbol slghpatexpress slghpattern semantics context filemanage
.
.
COMMANDLINE_DBG_OBJS= com_dbg/xml.o com_dbg/marshal.o com_dbg/space.o com_dbg/float.o com_dbg/address.o com_dbg/pcoderaw.o com_dbg/translate.o com_dbg/opcodes.o com_dbg/globalcontext.o com_dbg/capability.o com_dbg/architecture.o com_dbg/options.o com_dbg/graph.o com_dbg/cover.o com_dbg/block.o com_dbg/cast.o com_dbg/typeop.o com_dbg/database.o com_dbg/cpool.o com_dbg/comment.o com_dbg/stringmanage.o com_dbg/fspec.o com_dbg/action.o com_dbg/loadimage.o com_dbg/grammar.o com_dbg/varnode.o com_dbg/op.o com_dbg/type.o com_dbg/variable.o com_dbg/varmap.o com_dbg/jumptable.o com_dbg/emulate.o com_dbg/emulateutil.o com_dbg/flow.o com_dbg/userop.o com_dbg/funcdata.o com_dbg/funcdata_block.o com_dbg/funcdata_op.o com_dbg/funcdata_varnode.o com_dbg/unionresolve.o com_dbg/pcodeinject.o com_dbg/heritage.o com_dbg/prefersplit.o com_dbg/rangeutil.o com_dbg/ruleaction.o com_dbg/subflow.o com_dbg/blockaction.o com_dbg/merge.o com_dbg/double.o com_dbg/transform.o com_dbg/coreaction.o com_dbg/condexe.o com_dbg/override.o com_dbg/dynamic.o com_dbg/crc32.o com_dbg/prettyprint.o com_dbg/printlanguage.o com_dbg/printc.o com_dbg/printjava.o com_dbg/memstate.o com_dbg/opbehavior.o com_dbg/paramid.o com_dbg/bfd_arch.o com_dbg/callgraph.o com_dbg/codedata.o com_dbg/ifacedecomp.o com_dbg/ifaceterm.o com_dbg/inject_sleigh.o com_dbg/interface.o com_dbg/libdecomp.o com_dbg/loadimage_bfd.o com_dbg/loadimage_xml.o com_dbg/raw_arch.o com_dbg/rulecompile.o com_dbg/ruleparse.o com_dbg/sleigh_arch.o com_dbg/testfunction.o com_dbg/unify.o com_dbg/xml_arch.o com_dbg/sleigh.o com_dbg/pcodeparse.o com_dbg/pcodecompile.o com_dbg/sleighbase.o com_dbg/slghsymbol.o com_dbg/slghpatexpress.o com_dbg/slghpattern.o com_dbg/semantics.o com_dbg/context.o com_dbg/filemanage.o com_dbg/consolemain.o
.
.
COMMANDLINE_OPT_OBJS= com_opt/xml.o com_opt/marshal.o com_opt/space.o com_opt/float.o com_opt/address.o com_opt/pcoderaw.o com_opt/translate.o com_opt/opcodes.o com_opt/globalcontext.o com_opt/capability.o com_opt/architecture.o com_opt/options.o com_opt/graph.o com_opt/cover.o com_opt/block.o com_opt/cast.o com_opt/typeop.o com_opt/database.o com_opt/cpool.o com_opt/comment.o com_opt/stringmanage.o com_opt/fspec.o com_opt/action.o com_opt/loadimage.o com_opt/grammar.o com_opt/varnode.o com_opt/op.o com_opt/type.o com_opt/variable.o com_opt/varmap.o com_opt/jumptable.o com_opt/emulate.o com_opt/emulateutil.o com_opt/flow.o com_opt/userop.o com_opt/funcdata.o com_opt/funcdata_block.o com_opt/funcdata_op.o com_opt/funcdata_varnode.o com_opt/unionresolve.o com_opt/pcodeinject.o com_opt/heritage.o com_opt/prefersplit.o com_opt/rangeutil.o com_opt/ruleaction.o com_opt/subflow.o com_opt/blockaction.o com_opt/merge.o com_opt/double.o com_opt/transform.o com_opt/coreaction.o com_opt/condexe.o com_opt/override.o com_opt/dynamic.o com_opt/crc32.o com_opt/prettyprint.o com_opt/printlanguage.o com_opt/printc.o com_opt/printjava.o com_opt/memstate.o com_opt/opbehavior.o com_opt/paramid.o com_opt/bfd_arch.o com_opt/callgraph.o com_opt/codedata.o com_opt/ifacedecomp.o com_opt/ifaceterm.o com_opt/inject_sleigh.o com_opt/interface.o com_opt/libdecomp.o com_opt/loadimage_bfd.o com_opt/loadimage_xml.o com_opt/raw_arch.o com_opt/rulecompile.o com_opt/ruleparse.o com_opt/sleigh_arch.o com_opt/testfunction.o com_opt/unify.o com_opt/xml_arch.o com_opt/sleigh.o com_opt/pcodeparse.o com_opt/pcodecompile.o com_opt/sleighbase.o com_opt/slghsymbol.o com_opt/slghpatexpress.o com_opt/slghpattern.o com_opt/semantics.o com_opt/context.o com_opt/filemanage.o com_opt/consolemain.o
.
.
TEST_DEBUG_OBJS= test_dbg/xml.o test_dbg/marshal.o test_dbg/space.o test_dbg/float.o test_dbg/address.o test_dbg/pcoderaw.o test_dbg/translate.o test_dbg/opcodes.o test_dbg/globalcontext.o test_dbg/capability.o test_dbg/architecture.o test_dbg/options.o test_dbg/graph.o test_dbg/cover.o test_dbg/block.o test_dbg/cast.o test_dbg/typeop.o test_dbg/database.o test_dbg/cpool.o test_dbg/comment.o test_dbg/stringmanage.o test_dbg/fspec.o test_dbg/action.o test_dbg/loadimage.o test_dbg/grammar.o test_dbg/varnode.o test_dbg/op.o test_dbg/type.o test_dbg/variable.o test_dbg/varmap.o test_dbg/jumptable.o test_dbg/emulate.o test_dbg/emulateutil.o test_dbg/flow.o test_dbg/userop.o test_dbg/funcdata.o test_dbg/funcdata_block.o test_dbg/funcdata_op.o test_dbg/funcdata_varnode.o test_dbg/unionresolve.o test_dbg/pcodeinject.o test_dbg/heritage.o test_dbg/prefersplit.o test_dbg/rangeutil.o test_dbg/ruleaction.o test_dbg/subflow.o test_dbg/blockaction.o test_dbg/merge.o test_dbg/double.o test_dbg/transform.o test_dbg/coreaction.o test_dbg/condexe.o test_dbg/override.o test_dbg/dynamic.o test_dbg/crc32.o test_dbg/prettyprint.o test_dbg/printlanguage.o test_dbg/printc.o test_dbg/printjava.o test_dbg/memstate.o test_dbg/opbehavior.o test_dbg/paramid.o test_dbg/sleigh.o test_dbg/pcodeparse.o test_dbg/pcodecompile.o test_dbg/sleighbase.o test_dbg/slghsymbol.o test_dbg/slghpatexpress.o test_dbg/slghpattern.o test_dbg/semantics.o test_dbg/context.o test_dbg/filemanage.o test_dbg/bfd_arch.o test_dbg/callgraph.o test_dbg/codedata.o test_dbg/ifacedecomp.o test_dbg/ifaceterm.o test_dbg/inject_sleigh.o test_dbg/interface.o test_dbg/libdecomp.o test_dbg/loadimage_bfd.o test_dbg/loadimage_xml.o test_dbg/raw_arch.o test_dbg/rulecompile.o test_dbg/ruleparse.o test_dbg/sleigh_arch.o test_dbg/testfunction.o test_dbg/unify.o test_dbg/xml_arch.o test_dbg/test.o test_dbg/testcirclerange.o test_dbg/testfloatemu.o test_dbg/testmarshal.o test_dbg/testtypes.o
.
.
GHIDRA_DBG_OBJS= ghi_dbg/xml.o ghi_dbg/marshal.o ghi_dbg/space.o ghi_dbg/float.o ghi_dbg/address.o ghi_dbg/pcoderaw.o ghi_dbg/translate.o ghi_dbg/opcodes.o ghi_dbg/globalcontext.o ghi_dbg/capability.o ghi_dbg/architecture.o ghi_dbg/options.o ghi_dbg/graph.o ghi_dbg/cover.o ghi_dbg/block.o ghi_dbg/cast.o ghi_dbg/typeop.o ghi_dbg/database.o ghi_dbg/cpool.o ghi_dbg/comment.o ghi_dbg/stringmanage.o ghi_dbg/fspec.o ghi_dbg/action.o ghi_dbg/loadimage.o ghi_dbg/grammar.o ghi_dbg/varnode.o ghi_dbg/op.o ghi_dbg/type.o ghi_dbg/variable.o ghi_dbg/varmap.o ghi_dbg/jumptable.o ghi_dbg/emulate.o ghi_dbg/emulateutil.o ghi_dbg/flow.o ghi_dbg/userop.o ghi_dbg/funcdata.o ghi_dbg/funcdata_block.o ghi_dbg/funcdata_op.o ghi_dbg/funcdata_varnode.o ghi_dbg/unionresolve.o ghi_dbg/pcodeinject.o ghi_dbg/heritage.o ghi_dbg/prefersplit.o ghi_dbg/rangeutil.o ghi_dbg/ruleaction.o ghi_dbg/subflow.o ghi_dbg/blockaction.o ghi_dbg/merge.o ghi_dbg/double.o ghi_dbg/transform.o ghi_dbg/coreaction.o ghi_dbg/condexe.o ghi_dbg/override.o ghi_dbg/dynamic.o ghi_dbg/crc32.o ghi_dbg/prettyprint.o ghi_dbg/printlanguage.o ghi_dbg/printc.o ghi_dbg/printjava.o ghi_dbg/memstate.o ghi_dbg/opbehavior.o ghi_dbg/paramid.o ghi_dbg/ghidra_arch.o ghi_dbg/inject_ghidra.o ghi_dbg/ghidra_translate.o ghi_dbg/loadimage_ghidra.o ghi_dbg/typegrp_ghidra.o ghi_dbg/database_ghidra.o ghi_dbg/ghidra_context.o ghi_dbg/cpool_ghidra.o ghi_dbg/ghidra_process.o ghi_dbg/comment_ghidra.o ghi_dbg/string_ghidra.o ghi_dbg/callgraph.o ghi_dbg/ifacedecomp.o ghi_dbg/testfunction.o ghi_dbg/ifaceterm.o ghi_dbg/interface.o
.
.
GHIDRA_OPT_OBJS= ghi_opt/xml.o ghi_opt/marshal.o ghi_opt/space.o ghi_opt/float.o ghi_opt/address.o ghi_opt/pcoderaw.o ghi_opt/translate.o ghi_opt/opcodes.o ghi_opt/globalcontext.o ghi_opt/capability.o ghi_opt/architecture.o ghi_opt/options.o ghi_opt/graph.o ghi_opt/cover.o ghi_opt/block.o ghi_opt/cast.o ghi_opt/typeop.o ghi_opt/database.o ghi_opt/cpool.o ghi_opt/comment.o ghi_opt/stringmanage.o ghi_opt/fspec.o ghi_opt/action.o ghi_opt/loadimage.o ghi_opt/grammar.o ghi_opt/varnode.o ghi_opt/op.o ghi_opt/type.o ghi_opt/variable.o ghi_opt/varmap.o ghi_opt/jumptable.o ghi_opt/emulate.o ghi_opt/emulateutil.o ghi_opt/flow.o ghi_opt/userop.o ghi_opt/funcdata.o ghi_opt/funcdata_block.o ghi_opt/funcdata_op.o ghi_opt/funcdata_varnode.o ghi_opt/unionresolve.o ghi_opt/pcodeinject.o ghi_opt/heritage.o ghi_opt/prefersplit.o ghi_opt/rangeutil.o ghi_opt/ruleaction.o ghi_opt/subflow.o ghi_opt/blockaction.o ghi_opt/merge.o ghi_opt/double.o ghi_opt/transform.o ghi_opt/coreaction.o ghi_opt/condexe.o ghi_opt/override.o ghi_opt/dynamic.o ghi_opt/crc32.o ghi_opt/prettyprint.o ghi_opt/printlanguage.o ghi_opt/printc.o ghi_opt/printjava.o ghi_opt/memstate.o ghi_opt/opbehavior.o ghi_opt/paramid.o ghi_opt/ghidra_arch.o ghi_opt/inject_ghidra.o ghi_opt/ghidra_translate.o ghi_opt/loadimage_ghidra.o ghi_opt/typegrp_ghidra.o ghi_opt/database_ghidra.o ghi_opt/ghidra_context.o ghi_opt/cpool_ghidra.o ghi_opt/ghidra_process.o ghi_opt/comment_ghidra.o ghi_opt/string_ghidra.o
.
.
SLEIGH_DBG_OBJS= sla_dbg/xml.o sla_dbg/marshal.o sla_dbg/space.o sla_dbg/float.o sla_dbg/address.o sla_dbg/pcoderaw.o sla_dbg/translate.o sla_dbg/opcodes.o sla_dbg/globalcontext.o sla_dbg/sleigh.o sla_dbg/pcodeparse.o sla_dbg/pcodecompile.o sla_dbg/sleighbase.o sla_dbg/slghsymbol.o sla_dbg/slghpatexpress.o sla_dbg/slghpattern.o sla_dbg/semantics.o sla_dbg/context.o sla_dbg/filemanage.o sla_dbg/slgh_compile.o sla_dbg/slghparse.o sla_dbg/slghscan.o
.
.
SLEIGH_OPT_OBJS= sla_opt/xml.o sla_opt/marshal.o sla_opt/space.o sla_opt/float.o sla_opt/address.o sla_opt/pcoderaw.o sla_opt/translate.o sla_opt/opcodes.o sla_opt/globalcontext.o sla_opt/sleigh.o sla_opt/pcodeparse.o sla_opt/pcodecompile.o sla_opt/sleighbase.o sla_opt/slghsymbol.o sla_opt/slghpatexpress.o sla_opt/slghpattern.o sla_opt/semantics.o sla_opt/context.o sla_opt/filemanage.o sla_opt/slgh_compile.o sla_opt/slghparse.o sla_opt/slghscan.o
.
.
LIBSLA_DBG_OBJS= com_dbg/xml.o com_dbg/marshal.o com_dbg/space.o com_dbg/float.o com_dbg/address.o com_dbg/pcoderaw.o com_dbg/translate.o com_dbg/opcodes.o com_dbg/globalcontext.o com_dbg/sleigh.o com_dbg/pcodeparse.o com_dbg/pcodecompile.o com_dbg/sleighbase.o com_dbg/slghsymbol.o com_dbg/slghpatexpress.o com_dbg/slghpattern.o com_dbg/semantics.o com_dbg/context.o com_dbg/filemanage.o com_dbg/loadimage.o com_dbg/sleigh.o com_dbg/memstate.o com_dbg/emulate.o com_dbg/opbehavior.o
.
.
LIBSLA_OPT_OBJS= com_opt/xml.o com_opt/marshal.o com_opt/space.o com_opt/float.o com_opt/address.o com_opt/pcoderaw.o com_opt/translate.o com_opt/opcodes.o com_opt/globalcontext.o com_opt/sleigh.o com_opt/pcodeparse.o com_opt/pcodecompile.o com_opt/sleighbase.o com_opt/slghsymbol.o com_opt/slghpatexpress.o com_opt/slghpattern.o com_opt/semantics.o com_opt/context.o com_opt/filemanage.o com_opt/loadimage.o com_opt/sleigh.o com_opt/memstate.o com_opt/emulate.o com_opt/opbehavior.o
.
.
LIBSLA_SOURCE= xml.cc marshal.cc space.cc float.cc address.cc pcoderaw.cc translate.cc opcodes.cc globalcontext.cc sleigh.cc pcodeparse.cc pcodecompile.cc sleighbase.cc slghsymbol.cc slghpatexpress.cc slghpattern.cc semantics.cc context.cc filemanage.cc loadimage.cc sleigh.cc memstate.cc emulate.cc opbehavior.cc xml.hh marshal.hh space.hh float.hh address.hh pcoderaw.hh translate.hh opcodes.hh globalcontext.hh sleigh.hh pcodeparse.hh pcodecompile.hh sleighbase.hh slghsymbol.hh slghpatexpress.hh slghpattern.hh semantics.hh context.hh filemanage.hh loadimage.hh sleigh.hh memstate.hh emulate.hh opbehavior.hh slgh_compile.cc slghparse.cc slghscan.cc slgh_compile.hh slghparse.hh types.h partmap.hh error.hh slghparse.y pcodeparse.y xml.y slghscan.l loadimage_bfd.hh loadimage_bfd.cc
.
.
LIBDECOMP_DBG_OBJS= com_dbg/xml.o com_dbg/marshal.o com_dbg/space.o com_dbg/float.o com_dbg/address.o com_dbg/pcoderaw.o com_dbg/translate.o com_dbg/opcodes.o com_dbg/globalcontext.o com_dbg/capability.o com_dbg/architecture.o com_dbg/options.o com_dbg/graph.o com_dbg/cover.o com_dbg/block.o com_dbg/cast.o com_dbg/typeop.o com_dbg/database.o com_dbg/cpool.o com_dbg/comment.o com_dbg/stringmanage.o com_dbg/fspec.o com_dbg/action.o com_dbg/loadimage.o com_dbg/grammar.o com_dbg/varnode.o com_dbg/op.o com_dbg/type.o com_dbg/variable.o com_dbg/varmap.o com_dbg/jumptable.o com_dbg/emulate.o com_dbg/emulateutil.o com_dbg/flow.o com_dbg/userop.o com_dbg/funcdata.o com_dbg/funcdata_block.o com_dbg/funcdata_op.o com_dbg/funcdata_varnode.o com_dbg/unionresolve.o com_dbg/pcodeinject.o com_dbg/heritage.o com_dbg/prefersplit.o com_dbg/rangeutil.o com_dbg/ruleaction.o com_dbg/subflow.o com_dbg/blockaction.o com_dbg/merge.o com_dbg/double.o com_dbg/transform.o com_dbg/coreaction.o com_dbg/condexe.o com_dbg/override.o com_dbg/dynamic.o com_dbg/crc32.o com_dbg/prettyprint.o com_dbg/printlanguage.o com_dbg/printc.o com_dbg/printjava.o com_dbg/memstate.o com_dbg/opbehavior.o com_dbg/paramid.o com_dbg/bfd_arch.o com_dbg/callgraph.o com_dbg/codedata.o com_dbg/ifacedecomp.o com_dbg/ifaceterm.o com_dbg/inject_sleigh.o com_dbg/interface.o com_dbg/libdecomp.o com_dbg/loadimage_bfd.o com_dbg/loadimage_xml.o com_dbg/raw_arch.o com_dbg/rulecompile.o com_dbg/ruleparse.o com_dbg/sleigh_arch.o com_dbg/testfunction.o com_dbg/unify.o com_dbg/xml_arch.o com_dbg/sleigh.o com_dbg/pcodeparse.o com_dbg/pcodecompile.o com_dbg/sleighbase.o com_dbg/slghsymbol.o com_dbg/slghpatexpress.o com_dbg/slghpattern.o com_dbg/semantics.o com_dbg/context.o com_dbg/filemanage.o
.
.
LIBDECOMP_OPT_OBJS= com_opt/xml.o com_opt/marshal.o com_opt/space.o com_opt/float.o com_opt/address.o com_opt/pcoderaw.o com_opt/translate.o com_opt/opcodes.o com_opt/globalcontext.o com_opt/capability.o com_opt/architecture.o com_opt/options.o com_opt/graph.o com_opt/cover.o com_opt/block.o com_opt/cast.o com_opt/typeop.o com_opt/database.o com_opt/cpool.o com_opt/comment.o com_opt/stringmanage.o com_opt/fspec.o com_opt/action.o com_opt/loadimage.o com_opt/grammar.o com_opt/varnode.o com_opt/op.o com_opt/type.o com_opt/variable.o com_opt/varmap.o com_opt/jumptable.o com_opt/emulate.o com_opt/emulateutil.o com_opt/flow.o com_opt/userop.o com_opt/funcdata.o com_opt/funcdata_block.o com_opt/funcdata_op.o com_opt/funcdata_varnode.o com_opt/unionresolve.o com_opt/pcodeinject.o com_opt/heritage.o com_opt/prefersplit.o com_opt/rangeutil.o com_opt/ruleaction.o com_opt/subflow.o com_opt/blockaction.o com_opt/merge.o com_opt/double.o com_opt/transform.o com_opt/coreaction.o com_opt/condexe.o com_opt/override.o com_opt/dynamic.o com_opt/crc32.o com_opt/prettyprint.o com_opt/printlanguage.o com_opt/printc.o com_opt/printjava.o com_opt/memstate.o com_opt/opbehavior.o com_opt/paramid.o com_opt/bfd_arch.o com_opt/callgraph.o com_opt/codedata.o com_opt/ifacedecomp.o com_opt/ifaceterm.o com_opt/inject_sleigh.o com_opt/interface.o com_opt/libdecomp.o com_opt/loadimage_bfd.o com_opt/loadimage_xml.o com_opt/raw_arch.o com_opt/rulecompile.o com_opt/ruleparse.o com_opt/sleigh_arch.o com_opt/testfunction.o com_opt/unify.o com_opt/xml_arch.o com_opt/sleigh.o com_opt/pcodeparse.o com_opt/pcodecompile.o com_opt/sleighbase.o com_opt/slghsymbol.o com_opt/slghpatexpress.o com_opt/slghpattern.o com_opt/semantics.o com_opt/context.o com_opt/filemanage.o
.
.
DEPNAMES= com_dbg/depend com_opt/depend
```

Which we can use to create our meson build.