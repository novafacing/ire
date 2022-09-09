/// Build script for C++ components of pcode-rs
extern crate bindgen;
use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=meson.build");
    println!("cargo:rerun-if-changed=src/main.rs");

    let ghidra_out_dir = PathBuf::from(env::var("OUT_DIR").unwrap()).join("ghidra");
    let ghidra_build_dir = PathBuf::from("ghidra-build");
    let ghidra_out_path = ghidra_out_dir.to_str().unwrap();
    let ghidra_build_path = ghidra_build_dir.to_str().unwrap();
    let ghidra_include_dir = ghidra_out_dir.join("include");
    let ghidra_lib_dir = ghidra_out_dir.join("lib");
    let ghidra_decomp_file = ghidra_out_dir.join("libdecomp.so");
    let ghidra_sleigh_file = ghidra_out_dir.join("libsleigh.so");
    let ghidra_wrapper_file = ghidra_include_dir.join("pcode-rs-wrapper.hh");
    let ghidra_include_path = ghidra_include_dir.to_str().unwrap();
    let ghidra_wrapper_path = ghidra_wrapper_file.to_str().unwrap();
    let ghidra_decomp_path = ghidra_decomp_file.to_str().unwrap();
    let ghidra_sleigh_path = ghidra_sleigh_file.to_str().unwrap();

    println!("cargo:rustc-link-search=native={}/lib/", ghidra_out_path);

    Command::new("meson")
        .arg(ghidra_build_path.clone())
        .arg(format!("-Dinstall_dir={}", ghidra_out_path))
        .status()
        .expect("Could not configure ghidra libraries.");

    Command::new("meson")
        .arg("compile")
        .arg("-C")
        .arg(ghidra_build_path.clone())
        .status()
        .expect("Could not build ghidra libraries.");

    Command::new("meson")
        .arg("install")
        .arg("-C")
        .arg(ghidra_build_path.clone())
        .status()
        .expect("Could not install ghidra libraries.");

    let bindings = bindgen::Builder::default()
        .header(ghidra_wrapper_path.clone())
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .clang_args(&[
            "-std=c++11",
            "-I",
            ghidra_include_path,
            "-x",
            "c++",
            "-fparse-all-comments",
            "-fretain-comments-from-system-headers",
        ])
        .generate_inline_functions(true)
        .dynamic_library_name("decomp")
        .dynamic_library_name("sleigh")
        // .link(ghidra_decomp_path)
        // .link(ghidra_sleigh_path)
        /* Allowlist all classes */
        .allowlist_type("CallGraphEdge")
        .allowlist_type("CallGraphNode")
        .allowlist_type("CallGraph")
        .allowlist_type("CoverBlock")
        .allowlist_type("Cover")
        .allowlist_type("FunctionTestProperty")
        .allowlist_type("FunctionTestCollection")
        .allowlist_type("SourceFileIndexer")
        .allowlist_type("Datatype")
        .allowlist_type("TypeField")
        .allowlist_type("TypeFactory")
        .allowlist_type("TypeOp")
        .allowlist_type("UnifyDatatype")
        .allowlist_type("RHSConstant")
        .allowlist_type("TraverseConstraint")
        .allowlist_type("UnifyConstraint")
        .allowlist_type("UnifyState")
        .allowlist_type("UnifyCPrinter")
        .allowlist_type("FileManage")
        .allowlist_type("ConstTpl")
        .allowlist_type("VarnodeTpl")
        .allowlist_type("HandleTpl")
        .allowlist_type("OpTpl")
        .allowlist_type("ConstructTpl")
        .allowlist_type("PcodeBuilder")
        .allowlist_type("ParamEntry")
        .allowlist_type("ParamEntryRange")
        .allowlist_type("ParamTrial")
        .allowlist_type("ParamActive")
        .allowlist_type("EffectRecord")
        .allowlist_type("ParamList")
        .allowlist_type("ProtoModel")
        .allowlist_type("ScoreProtoModel")
        .allowlist_type("ProtoParameter")
        .allowlist_type("ProtoStore")
        .allowlist_type("FuncProto")
        .allowlist_type("SleighSymbol")
        .allowlist_type("SymbolScope")
        .allowlist_type("SymbolTable")
        .allowlist_type("ContextChange")
        .allowlist_type("Constructor")
        .allowlist_type("DecisionProperties")
        .allowlist_type("DecisionNode")
        .allowlist_type("StackSolver")
        .allowlist_type("SectionVector")
        .allowlist_type("WithBlock")
        .allowlist_type("ConsistencyChecker")
        .allowlist_type("SymbolEntry")
        .allowlist_type("Symbol")
        .allowlist_type("SymbolCompareName")
        .allowlist_type("MapIterator")
        .allowlist_type("Scope")
        .allowlist_type("ScopeMapper")
        .allowlist_type("Database")
        .allowlist_type("ArchOption")
        .allowlist_type("OptionDatabase")
        .allowlist_type("rangemap")
        .allowlist_type("UserPcodeOp")
        .allowlist_type("UserOpManage")
        .allowlist_type("FlowInfo")
        .allowlist_type("LoadTable")
        .allowlist_type("PathMeld")
        .allowlist_type("GuardRecord")
        .allowlist_type("JumpValues")
        .allowlist_type("JumpModel")
        .allowlist_type("JumpTable")
        .allowlist_type("GrammarToken")
        .allowlist_type("GrammarLexer")
        .allowlist_type("TypeModifier")
        .allowlist_type("TypeDeclarator")
        .allowlist_type("CParse")
        .allowlist_type("PreferSplitManager")
        .allowlist_type("LocationMap")
        .allowlist_type("PriorityQueue")
        .allowlist_type("HeritageInfo")
        .allowlist_type("LoadGuard")
        .allowlist_type("Heritage")
        .allowlist_type("GhidraCommand")
        .allowlist_type("Varnode")
        .allowlist_type("VarnodeBank")
        .allowlist_type("PropagationState")
        .allowlist_type("AdditiveEdge")
        .allowlist_type("TermOrder")
        .allowlist_type("TokenPattern")
        .allowlist_type("PatternExpression")
        .allowlist_type("PatternEquation")
        .allowlist_type("ActionGroupList")
        .allowlist_type("Action")
        .allowlist_type("Rule")
        .allowlist_type("ActionDatabase")
        .allowlist_type("ContextBitRange")
        .allowlist_type("ContextDatabase")
        .allowlist_type("ContextCache")
        .allowlist_type("PcodeOpRaw")
        .allowlist_type("Token")
        .allowlist_type("ParserContext")
        .allowlist_type("ParserWalker")
        .allowlist_type("CPoolRecord")
        .allowlist_type("ConstantPool")
        .allowlist_type("TruncationTag")
        .allowlist_type("PcodeEmit")
        .allowlist_type("AssemblyEmit")
        .allowlist_type("AddressResolver")
        .allowlist_type("JoinRecord")
        .allowlist_type("AddrSpaceManager")
        .allowlist_type("CircleRange")
        .allowlist_type("ValueSet")
        .allowlist_type("Partition")
        .allowlist_type("ValueSetRead")
        .allowlist_type("Widener")
        .allowlist_type("ValueSetSolver")
        .allowlist_type("Location")
        .allowlist_type("ExprTree")
        .allowlist_type("PcodeCompile")
        .allowlist_type("FlowBlock")
        .allowlist_type("BlockMap")
        .allowlist_type("PatternBlock")
        .allowlist_type("Pattern")
        .allowlist_type("MemoryBank")
        .allowlist_type("MemoryState")
        .allowlist_type("partmap")
        .allowlist_type("PcodeLexer")
        .allowlist_type("RuleLexer")
        .allowlist_type("RuleCompile")
        .allowlist_type("InjectParameter")
        .allowlist_type("InjectContext")
        .allowlist_type("InjectPayload")
        .allowlist_type("PcodeInjectLibrary")
        .allowlist_type("CapabilityPoint")
        .allowlist_type("Funcdata")
        .allowlist_type("AncestorRealistic")
        .allowlist_type("ResolvedUnion")
        .allowlist_type("ResolveEdge")
        .allowlist_type("ScoreUnionFields")
        .allowlist_type("Comment")
        .allowlist_type("CommentDatabase")
        .allowlist_type("CommentSorter")
        .allowlist_type("OpToken")
        .allowlist_type("PrintLanguage")
        .allowlist_type("SplitVarnode")
        .allowlist_type("AddForm")
        .allowlist_type("SubForm")
        .allowlist_type("LogicalForm")
        .allowlist_type("Equal1Form")
        .allowlist_type("Equal2Form")
        .allowlist_type("Equal3Form")
        .allowlist_type("LessThreeWay")
        .allowlist_type("LessConstForm")
        .allowlist_type("ShiftForm")
        .allowlist_type("MultForm")
        .allowlist_type("PhiForm")
        .allowlist_type("IndirectForm")
        .allowlist_type("Statistics")
        .allowlist_type("BreakTable")
        .allowlist_type("BreakCallBack")
        .allowlist_type("Emulate")
        .allowlist_type("RemoteSocket")
        .allowlist_type("IfaceData")
        .allowlist_type("IfaceCommand")
        .allowlist_type("IfaceStatus")
        .allowlist_type("ConditionMarker")
        .allowlist_type("ConditionalExecution")
        .allowlist_type("AddTreeState")
        .allowlist_type("HighVariable")
        .allowlist_type("Endian")
        .allowlist_type("CompilerTag")
        .allowlist_type("LanguageDescription")
        .allowlist_type("CodeUnit")
        .allowlist_type("TargetHit")
        .allowlist_type("OpBehavior")
        .allowlist_type("StringManager")
        .allowlist_type("FloatFormat")
        .allowlist_type("LoadImage")
        .allowlist_type("Address")
        .allowlist_type("SeqNum")
        .allowlist_type("Range")
        .allowlist_type("RangeProperties")
        .allowlist_type("RangeList")
        .allowlist_type("PcodeCacher")
        .allowlist_type("DisassemblyCache")
        .allowlist_type("XmlScan")
        .allowlist_type("SubvariableFlow")
        .allowlist_type("NameRecommend")
        .allowlist_type("DynamicRecommend")
        .allowlist_type("TypeRecommend")
        .allowlist_type("RangeHint")
        .allowlist_type("AliasChecker")
        .allowlist_type("MapState")
        .allowlist_type("CastStrategy")
        .allowlist_type("Emit")
        .allowlist_type("TokenSplit")
        .allowlist_type("circularqueue")
        .allowlist_type("PendPrint")
        .allowlist_type("XmlScan")
        .allowlist_type("ToOpEdge")
        .allowlist_type("DynamicHash")
        .allowlist_type("Override")
        .allowlist_type("PcodeOp")
        .allowlist_type("PcodeOpBank")
        .allowlist_type("AddrSpace")
        .allowlist_type("AttributeId")
        .allowlist_type("ElementId")
        .allowlist_type("Decoder")
        .allowlist_type("Encoder")
        .allowlist_type("FloatingEdge")
        .allowlist_type("LoopBody")
        .allowlist_type("TraceDAG")
        .allowlist_type("CollapseStructure")
        .allowlist_type("ConditionalJoin")
        .allowlist_type("Attributes")
        .allowlist_type("ContentHandler")
        .allowlist_type("Element")
        .allowlist_type("DocumentStorage")
        .allowlist_type("TransformVar")
        .allowlist_type("TransformOp")
        .allowlist_type("LanedRegister")
        .allowlist_type("LaneDescription")
        .allowlist_type("TransformManager")
        .allowlist_type("HighEdge")
        .allowlist_type("BlockVarnode")
        .allowlist_type("Merge")
        .allowlist_type("ParamMeasure")
        .blocklist_type("const_pointer")
        .opaque_type("std::.*")
        .opaque_type("pointer")
        .enable_cxx_namespaces()
        .module_raw_line("root", "pub type const_pointer = u64;")
        .generate()
        .expect("Unable to generate ghidra bindings.");

    let bindings_src = PathBuf::from(env::var("OUT_DIR").unwrap()).join("ghidra_bindings.rs");

    bindings
        .write_to_file(bindings_src)
        .expect("Could not generate bindings file.");
}
