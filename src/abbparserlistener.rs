#![allow(nonstandard_style)]
// Generated from abbParser.g4 by ANTLR 4.8
use antlr4rust::tree::ParseTreeListener;
use super::abbparser::*;

pub trait abbParserListener<'input> : ParseTreeListener<'input,abbParserContextType>{
/**
 * Enter a parse tree produced by {@link abbParser#module_}.
 * @param ctx the parse tree
 */
fn enter_module_(&mut self, _ctx: &Module_Context<'input>) { }
/**
 * Exit a parse tree produced by {@link abbParser#module_}.
 * @param ctx the parse tree
 */
fn exit_module_(&mut self, _ctx: &Module_Context<'input>) { }
/**
 * Enter a parse tree produced by {@link abbParser#moduleData}.
 * @param ctx the parse tree
 */
fn enter_moduleData(&mut self, _ctx: &ModuleDataContext<'input>) { }
/**
 * Exit a parse tree produced by {@link abbParser#moduleData}.
 * @param ctx the parse tree
 */
fn exit_moduleData(&mut self, _ctx: &ModuleDataContext<'input>) { }
/**
 * Enter a parse tree produced by {@link abbParser#moduleName}.
 * @param ctx the parse tree
 */
fn enter_moduleName(&mut self, _ctx: &ModuleNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link abbParser#moduleName}.
 * @param ctx the parse tree
 */
fn exit_moduleName(&mut self, _ctx: &ModuleNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link abbParser#dataList}.
 * @param ctx the parse tree
 */
fn enter_dataList(&mut self, _ctx: &DataListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link abbParser#dataList}.
 * @param ctx the parse tree
 */
fn exit_dataList(&mut self, _ctx: &DataListContext<'input>) { }
/**
 * Enter a parse tree produced by {@link abbParser#procedure}.
 * @param ctx the parse tree
 */
fn enter_procedure(&mut self, _ctx: &ProcedureContext<'input>) { }
/**
 * Exit a parse tree produced by {@link abbParser#procedure}.
 * @param ctx the parse tree
 */
fn exit_procedure(&mut self, _ctx: &ProcedureContext<'input>) { }
/**
 * Enter a parse tree produced by {@link abbParser#procCall}.
 * @param ctx the parse tree
 */
fn enter_procCall(&mut self, _ctx: &ProcCallContext<'input>) { }
/**
 * Exit a parse tree produced by {@link abbParser#procCall}.
 * @param ctx the parse tree
 */
fn exit_procCall(&mut self, _ctx: &ProcCallContext<'input>) { }
/**
 * Enter a parse tree produced by {@link abbParser#procName}.
 * @param ctx the parse tree
 */
fn enter_procName(&mut self, _ctx: &ProcNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link abbParser#procName}.
 * @param ctx the parse tree
 */
fn exit_procName(&mut self, _ctx: &ProcNameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link abbParser#procParameter}.
 * @param ctx the parse tree
 */
fn enter_procParameter(&mut self, _ctx: &ProcParameterContext<'input>) { }
/**
 * Exit a parse tree produced by {@link abbParser#procParameter}.
 * @param ctx the parse tree
 */
fn exit_procParameter(&mut self, _ctx: &ProcParameterContext<'input>) { }
/**
 * Enter a parse tree produced by {@link abbParser#functionCall}.
 * @param ctx the parse tree
 */
fn enter_functionCall(&mut self, _ctx: &FunctionCallContext<'input>) { }
/**
 * Exit a parse tree produced by {@link abbParser#functionCall}.
 * @param ctx the parse tree
 */
fn exit_functionCall(&mut self, _ctx: &FunctionCallContext<'input>) { }
/**
 * Enter a parse tree produced by {@link abbParser#functionParameter}.
 * @param ctx the parse tree
 */
fn enter_functionParameter(&mut self, _ctx: &FunctionParameterContext<'input>) { }
/**
 * Exit a parse tree produced by {@link abbParser#functionParameter}.
 * @param ctx the parse tree
 */
fn exit_functionParameter(&mut self, _ctx: &FunctionParameterContext<'input>) { }
/**
 * Enter a parse tree produced by {@link abbParser#declaration}.
 * @param ctx the parse tree
 */
fn enter_declaration(&mut self, _ctx: &DeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link abbParser#declaration}.
 * @param ctx the parse tree
 */
fn exit_declaration(&mut self, _ctx: &DeclarationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link abbParser#type_}.
 * @param ctx the parse tree
 */
fn enter_type_(&mut self, _ctx: &Type_Context<'input>) { }
/**
 * Exit a parse tree produced by {@link abbParser#type_}.
 * @param ctx the parse tree
 */
fn exit_type_(&mut self, _ctx: &Type_Context<'input>) { }
/**
 * Enter a parse tree produced by {@link abbParser#init_}.
 * @param ctx the parse tree
 */
fn enter_init_(&mut self, _ctx: &Init_Context<'input>) { }
/**
 * Exit a parse tree produced by {@link abbParser#init_}.
 * @param ctx the parse tree
 */
fn exit_init_(&mut self, _ctx: &Init_Context<'input>) { }
/**
 * Enter a parse tree produced by {@link abbParser#expression}.
 * @param ctx the parse tree
 */
fn enter_expression(&mut self, _ctx: &ExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link abbParser#expression}.
 * @param ctx the parse tree
 */
fn exit_expression(&mut self, _ctx: &ExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link abbParser#array_}.
 * @param ctx the parse tree
 */
fn enter_array_(&mut self, _ctx: &Array_Context<'input>) { }
/**
 * Exit a parse tree produced by {@link abbParser#array_}.
 * @param ctx the parse tree
 */
fn exit_array_(&mut self, _ctx: &Array_Context<'input>) { }
/**
 * Enter a parse tree produced by {@link abbParser#primitive}.
 * @param ctx the parse tree
 */
fn enter_primitive(&mut self, _ctx: &PrimitiveContext<'input>) { }
/**
 * Exit a parse tree produced by {@link abbParser#primitive}.
 * @param ctx the parse tree
 */
fn exit_primitive(&mut self, _ctx: &PrimitiveContext<'input>) { }

}

antlr4rust::coerce_from!{ 'input : abbParserListener<'input> }


