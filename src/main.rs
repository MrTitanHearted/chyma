mod ast;
mod lexer;
mod parser;
mod token;

mod compiler;

mod resolvers;
use llvm_sys::analysis::{LLVMVerifierFailureAction, LLVMVerifyModule};
use llvm_sys::core::{
    LLVMContextCreate, LLVMContextDispose, LLVMCreateBuilderInContext, LLVMDisposeBuilder,
    LLVMDisposeMessage, LLVMDisposeModule, LLVMGetBufferSize, LLVMGetBufferStart,
    LLVMModuleCreateWithNameInContext,
};
use llvm_sys::target::{
    LLVM_InitializeNativeAsmPrinter, LLVM_InitializeNativeTarget, LLVMSetModuleDataLayout,
};
use llvm_sys::target_machine::{
    LLVMCodeGenFileType, LLVMCodeGenOptLevel, LLVMCodeModel, LLVMCreateTargetDataLayout,
    LLVMCreateTargetMachine, LLVMDisposeTargetMachine, LLVMGetDefaultTargetTriple,
    LLVMGetTargetFromTriple, LLVMRelocMode, LLVMTargetMachineEmitToMemoryBuffer,
};
use llvm_sys::transforms::pass_builder::{
    LLVMCreatePassBuilderOptions, LLVMDisposePassBuilderOptions, LLVMRunPasses,
};
use resolvers::*;

use crate::ast::{FlatASTFormatter, TreeASTFormatter};
use crate::parser::{ParserError, ParserErrorFormatter};
use crate::token::TokenStringInterner;
use clap::Parser;
use lexer::Lexer;
use std::ffi::CString;
use std::{fs::File, io::Read};

#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, help = "Path to the input file")]
    input: String,
}

fn main() -> std::io::Result<()> {
    let mut interner = TokenStringInterner::new();

    if let Err(err) = main_entry(&mut interner) {
        if let Some(err) = err.downcast_ref::<ParserError>() {
            panic!("{}", ParserErrorFormatter::new(&interner, err));
        }
        panic!("{err}");
    }

    Ok(())
}

fn main_entry(interner: &mut TokenStringInterner) -> Result<(), Box<dyn std::error::Error>> {
    let mut source = String::new();

    let args = Args::parse();
    File::open(args.input)?.read_to_string(&mut source)?;

    let tokens = Lexer::lex(interner, &source)?;

    let ast = parser::Parser::parse(interner, &tokens)?;

    println!("{}", TreeASTFormatter::new(&ast));
    println!("{}", FlatASTFormatter::new(&ast));

    SemanticResolver::resolve(&ast)?;
    let expression_type_map = TypeResolver::resolve(&ast)?;

    unsafe {
        LLVM_InitializeNativeTarget();
        LLVM_InitializeNativeAsmPrinter();

        let context = LLVMContextCreate();
        let module_name = CString::new("add")?;
        let module = LLVMModuleCreateWithNameInContext(module_name.as_ptr(), context);
        let builder = LLVMCreateBuilderInContext(context);

        compiler::Compiler::compile(&ast, &expression_type_map, context, module, builder)?;

        let mut err_str = std::ptr::null_mut();
        if LLVMVerifyModule(
            module,
            LLVMVerifierFailureAction::LLVMPrintMessageAction,
            &mut err_str,
        ) != 0
        {
            eprintln!("Error verifying module: {:?}", err_str);
        }

        let triple = LLVMGetDefaultTargetTriple();
        let mut target = std::ptr::null_mut();
        if LLVMGetTargetFromTriple(triple, &mut target, &mut err_str) != 0 {
            eprintln!("Error getting target from triple: {:?}", err_str);
        }

        let target_machine = LLVMCreateTargetMachine(
            target,
            triple,
            CString::new("generic")?.as_ptr(),
            CString::new("")?.as_ptr(),
            LLVMCodeGenOptLevel::LLVMCodeGenLevelAggressive,
            LLVMRelocMode::LLVMRelocDefault,
            LLVMCodeModel::LLVMCodeModelDefault,
        );
        LLVMDisposeMessage(triple);

        let pass_builder_options = LLVMCreatePassBuilderOptions();
        LLVMRunPasses(
            module,
            CString::new("default<O3>")?.as_ptr(),
            target_machine,
            pass_builder_options,
        );

        LLVMDisposePassBuilderOptions(pass_builder_options);

        LLVMSetModuleDataLayout(module, LLVMCreateTargetDataLayout(target_machine));

        let mut memory_buffer = std::ptr::null_mut();
        if LLVMTargetMachineEmitToMemoryBuffer(
            target_machine,
            module,
            LLVMCodeGenFileType::LLVMObjectFile,
            &mut err_str,
            &mut memory_buffer,
        ) != 0
        {
            eprintln!("Error getting .o file to memory: {:?}", err_str);
        }

        let obj_data = LLVMGetBufferStart(memory_buffer);
        let obj_size = LLVMGetBufferSize(memory_buffer);

        let slice = std::slice::from_raw_parts(obj_data as *const u8, obj_size);
        std::fs::write("add.o", slice)?;

        LLVMDisposeTargetMachine(target_machine);
        LLVMDisposeBuilder(builder);
        LLVMDisposeModule(module);
        LLVMContextDispose(context);
    }

    Ok(())
}
