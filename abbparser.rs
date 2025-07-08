// Generated from abbParser.g4 by ANTLR 4.8
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use antlr4rust::PredictionContextCache;
use antlr4rust::parser::{Parser, BaseParser, ParserRecog, ParserNodeType};
use antlr4rust::token_stream::TokenStream;
use antlr4rust::TokenSource;
use antlr4rust::parser_atn_simulator::ParserATNSimulator;
use antlr4rust::errors::*;
use antlr4rust::rule_context::{BaseRuleContext, CustomRuleContext, RuleContext};
use antlr4rust::recognizer::{Recognizer,Actions};
use antlr4rust::atn_deserializer::ATNDeserializer;
use antlr4rust::dfa::DFA;
use antlr4rust::atn::{ATN, INVALID_ALT};
use antlr4rust::error_strategy::{ErrorStrategy, DefaultErrorStrategy};
use antlr4rust::parser_rule_context::{BaseParserRuleContext, ParserRuleContext,cast,cast_mut};
use antlr4rust::tree::*;
use antlr4rust::token::{TOKEN_EOF,OwningToken,Token};
use antlr4rust::int_stream::EOF;
use antlr4rust::vocabulary::{Vocabulary,VocabularyImpl};
use antlr4rust::token_factory::{CommonTokenFactory,TokenFactory, TokenAware};
use super::abbparserlistener::*;
use antlr4rust::lazy_static;
use antlr4rust::{TidAble,TidExt};

use std::marker::PhantomData;
use std::sync::Arc;
use std::rc::Rc;
use std::convert::TryFrom;
use std::cell::RefCell;
use std::ops::{DerefMut, Deref};
use std::borrow::{Borrow,BorrowMut};
use std::any::{Any,TypeId};

		pub const MODULE:isize=1; 
		pub const ENDMODULE:isize=2; 
		pub const PROC:isize=3; 
		pub const ENDPROC:isize=4; 
		pub const LOCAL:isize=5; 
		pub const CONST:isize=6; 
		pub const PERS:isize=7; 
		pub const VAR:isize=8; 
		pub const TOOLDATA:isize=9; 
		pub const WOBJDATA:isize=10; 
		pub const SPEEDDATA:isize=11; 
		pub const ZONEDATA:isize=12; 
		pub const CLOCK:isize=13; 
		pub const BOOL:isize=14; 
		pub const ON_CALL:isize=15; 
		pub const OFF_CALL:isize=16; 
		pub const SLASH:isize=17; 
		pub const EQUALS:isize=18; 
		pub const COMMA:isize=19; 
		pub const CURLY_OPEN:isize=20; 
		pub const CURLY_CLOSE:isize=21; 
		pub const COLON:isize=22; 
		pub const SEMICOLON:isize=23; 
		pub const BRACKET_OPEN:isize=24; 
		pub const BRACKET_CLOSE:isize=25; 
		pub const SQUARE_OPEN:isize=26; 
		pub const SQUARE_CLOSE:isize=27; 
		pub const DOT:isize=28; 
		pub const DOUBLEDOT:isize=29; 
		pub const REL_BIGGER:isize=30; 
		pub const REL_BIGGER_OR_EQUAL:isize=31; 
		pub const REL_SMALLER:isize=32; 
		pub const REL_SMALLER_OR_EQUAL:isize=33; 
		pub const REL_EQUAL:isize=34; 
		pub const REL_NOTEQUAL:isize=35; 
		pub const PLUS:isize=36; 
		pub const MINUS:isize=37; 
		pub const MULTIPLY:isize=38; 
		pub const PERCENT:isize=39; 
		pub const HASH:isize=40; 
		pub const WS:isize=41; 
		pub const NEWLINE:isize=42; 
		pub const LINE_COMMENT:isize=43; 
		pub const BOOLLITERAL:isize=44; 
		pub const CHARLITERAL:isize=45; 
		pub const STRINGLITERAL:isize=46; 
		pub const FLOATLITERAL:isize=47; 
		pub const INTLITERAL:isize=48; 
		pub const IDENTIFIER:isize=49;
	pub const RULE_module_:usize = 0; 
	pub const RULE_moduleData:usize = 1; 
	pub const RULE_moduleName:usize = 2; 
	pub const RULE_dataList:usize = 3; 
	pub const RULE_procedure:usize = 4; 
	pub const RULE_procCall:usize = 5; 
	pub const RULE_procName:usize = 6; 
	pub const RULE_procParameter:usize = 7; 
	pub const RULE_functionCall:usize = 8; 
	pub const RULE_functionParameter:usize = 9; 
	pub const RULE_declaration:usize = 10; 
	pub const RULE_type_:usize = 11; 
	pub const RULE_init_:usize = 12; 
	pub const RULE_expression:usize = 13; 
	pub const RULE_array_:usize = 14; 
	pub const RULE_primitive:usize = 15;
	pub const ruleNames: [&'static str; 16] =  [
		"module_", "moduleData", "moduleName", "dataList", "procedure", "procCall", 
		"procName", "procParameter", "functionCall", "functionParameter", "declaration", 
		"type_", "init_", "expression", "array_", "primitive"
	];


	pub const _LITERAL_NAMES: [Option<&'static str>;41] = [
		None, Some("'module'"), Some("'endmodule'"), Some("'PROC'"), Some("'ENDPROC'"), 
		Some("'LOCAL'"), Some("'CONST'"), Some("'PERS'"), Some("'VAR'"), Some("'TOOLDATA'"), 
		Some("'WOBJDATA'"), Some("'SPEEDDATA'"), Some("'ZONEDATA'"), Some("'CLOCK'"), 
		Some("'BOOL'"), Some("'\\ON'"), Some("'\\OFF'"), Some("'/'"), Some("':='"), 
		Some("','"), Some("'{'"), Some("'}'"), Some("':'"), Some("';'"), Some("'('"), 
		Some("')'"), Some("'['"), Some("']'"), Some("'.'"), Some("'..'"), Some("'>'"), 
		Some("'>='"), Some("'<'"), Some("'<='"), Some("'=='"), Some("'<>'"), Some("'+'"), 
		Some("'-'"), Some("'*'"), Some("'%'"), Some("'#'")
	];
	pub const _SYMBOLIC_NAMES: [Option<&'static str>;50]  = [
		None, Some("MODULE"), Some("ENDMODULE"), Some("PROC"), Some("ENDPROC"), 
		Some("LOCAL"), Some("CONST"), Some("PERS"), Some("VAR"), Some("TOOLDATA"), 
		Some("WOBJDATA"), Some("SPEEDDATA"), Some("ZONEDATA"), Some("CLOCK"), 
		Some("BOOL"), Some("ON_CALL"), Some("OFF_CALL"), Some("SLASH"), Some("EQUALS"), 
		Some("COMMA"), Some("CURLY_OPEN"), Some("CURLY_CLOSE"), Some("COLON"), 
		Some("SEMICOLON"), Some("BRACKET_OPEN"), Some("BRACKET_CLOSE"), Some("SQUARE_OPEN"), 
		Some("SQUARE_CLOSE"), Some("DOT"), Some("DOUBLEDOT"), Some("REL_BIGGER"), 
		Some("REL_BIGGER_OR_EQUAL"), Some("REL_SMALLER"), Some("REL_SMALLER_OR_EQUAL"), 
		Some("REL_EQUAL"), Some("REL_NOTEQUAL"), Some("PLUS"), Some("MINUS"), 
		Some("MULTIPLY"), Some("PERCENT"), Some("HASH"), Some("WS"), Some("NEWLINE"), 
		Some("LINE_COMMENT"), Some("BOOLLITERAL"), Some("CHARLITERAL"), Some("STRINGLITERAL"), 
		Some("FLOATLITERAL"), Some("INTLITERAL"), Some("IDENTIFIER")
	];
	lazy_static!{
	    static ref _shared_context_cache: Arc<PredictionContextCache> = Arc::new(PredictionContextCache::new());
		static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None));
	}


type BaseParserType<'input, I> =
	BaseParser<'input,abbParserExt<'input>, I, abbParserContextType , dyn abbParserListener<'input> + 'input >;

type TokenType<'input> = <LocalTokenFactory<'input> as TokenFactory<'input>>::Tok;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

pub type abbParserTreeWalker<'input,'a> =
	ParseTreeWalker<'input, 'a, abbParserContextType , dyn abbParserListener<'input> + 'a>;

/// Parser for abbParser grammar
pub struct abbParser<'input,I,H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	base:BaseParserType<'input,I>,
	interpreter:Arc<ParserATNSimulator>,
	_shared_context_cache: Box<PredictionContextCache>,
    pub err_handler: H,
}

impl<'input, I, H> abbParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn get_serialized_atn() -> &'static str { _serializedATN }

    pub fn set_error_strategy(&mut self, strategy: H) {
        self.err_handler = strategy
    }

    pub fn with_strategy(input: I, strategy: H) -> Self {
		antlr4rust::recognizer::check_version("0","3");
		let interpreter = Arc::new(ParserATNSimulator::new(
			_ATN.clone(),
			_decision_to_DFA.clone(),
			_shared_context_cache.clone(),
		));
		Self {
			base: BaseParser::new_base_parser(
				input,
				Arc::clone(&interpreter),
				abbParserExt{
					_pd: Default::default(),
				}
			),
			interpreter,
            _shared_context_cache: Box::new(PredictionContextCache::new()),
            err_handler: strategy,
        }
    }

}

type DynStrategy<'input,I> = Box<dyn ErrorStrategy<'input,BaseParserType<'input,I>> + 'input>;

impl<'input, I> abbParser<'input, I, DynStrategy<'input,I>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn with_dyn_strategy(input: I) -> Self{
    	Self::with_strategy(input,Box::new(DefaultErrorStrategy::new()))
    }
}

impl<'input, I> abbParser<'input, I, DefaultErrorStrategy<'input,abbParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn new(input: I) -> Self{
    	Self::with_strategy(input,DefaultErrorStrategy::new())
    }
}

/// Trait for monomorphized trait object that corresponds to the nodes of parse tree generated for abbParser
pub trait abbParserContext<'input>:
	for<'x> Listenable<dyn abbParserListener<'input> + 'x > + 
	ParserRuleContext<'input, TF=LocalTokenFactory<'input>, Ctx=abbParserContextType>
{}

antlr4rust::coerce_from!{ 'input : abbParserContext<'input> }

impl<'input> abbParserContext<'input> for TerminalNode<'input,abbParserContextType> {}
impl<'input> abbParserContext<'input> for ErrorNode<'input,abbParserContextType> {}

antlr4rust::tid! { impl<'input> TidAble<'input> for dyn abbParserContext<'input> + 'input }

antlr4rust::tid! { impl<'input> TidAble<'input> for dyn abbParserListener<'input> + 'input }

pub struct abbParserContextType;
antlr4rust::tid!{abbParserContextType}

impl<'input> ParserNodeType<'input> for abbParserContextType{
	type TF = LocalTokenFactory<'input>;
	type Type = dyn abbParserContext<'input> + 'input;
}

impl<'input, I, H> Deref for abbParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    type Target = BaseParserType<'input,I>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'input, I, H> DerefMut for abbParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct abbParserExt<'input>{
	_pd: PhantomData<&'input str>,
}

impl<'input> abbParserExt<'input>{
}
antlr4rust::tid! { abbParserExt<'a> }

impl<'input> TokenAware<'input> for abbParserExt<'input>{
	type TF = LocalTokenFactory<'input>;
}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> ParserRecog<'input, BaseParserType<'input,I>> for abbParserExt<'input>{}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> Actions<'input, BaseParserType<'input,I>> for abbParserExt<'input>{
	fn get_grammar_file_name(&self) -> & str{ "abbParser.g4"}

   	fn get_rule_names(&self) -> &[& str] {&ruleNames}

   	fn get_vocabulary(&self) -> &dyn Vocabulary { &**VOCABULARY }
}
//------------------- module_ ----------------
pub type Module_ContextAll<'input> = Module_Context<'input>;


pub type Module_Context<'input> = BaseParserRuleContext<'input,Module_ContextExt<'input>>;

#[derive(Clone)]
pub struct Module_ContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> abbParserContext<'input> for Module_Context<'input>{}

impl<'input,'a> Listenable<dyn abbParserListener<'input> + 'a> for Module_Context<'input>{
		fn enter(&self,listener: &mut (dyn abbParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_module_(self);
		}fn exit(&self,listener: &mut (dyn abbParserListener<'input> + 'a)) {
			listener.exit_module_(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Module_ContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = abbParserContextType;
	fn get_rule_index(&self) -> usize { RULE_module_ }
	//fn type_rule_index() -> usize where Self: Sized { RULE_module_ }
}
antlr4rust::tid!{Module_ContextExt<'a>}

impl<'input> Module_ContextExt<'input>{
	fn new(parent: Option<Rc<dyn abbParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Module_ContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Module_ContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Module_ContextAttrs<'input>: abbParserContext<'input> + BorrowMut<Module_ContextExt<'input>>{

fn moduleData(&self) -> Option<Rc<ModuleDataContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,abbParserContextType>>> where Self:Sized{
	self.get_token(EOF, 0)
}

}

impl<'input> Module_ContextAttrs<'input> for Module_Context<'input>{}

impl<'input, I, H> abbParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn module_(&mut self,)
	-> Result<Rc<Module_ContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Module_ContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 0, RULE_module_);
        let mut _localctx: Rc<Module_ContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule moduleData*/
			recog.base.set_state(32);
			recog.moduleData()?;

			recog.base.set_state(33);
			recog.base.match_token(EOF,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- moduleData ----------------
pub type ModuleDataContextAll<'input> = ModuleDataContext<'input>;


pub type ModuleDataContext<'input> = BaseParserRuleContext<'input,ModuleDataContextExt<'input>>;

#[derive(Clone)]
pub struct ModuleDataContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> abbParserContext<'input> for ModuleDataContext<'input>{}

impl<'input,'a> Listenable<dyn abbParserListener<'input> + 'a> for ModuleDataContext<'input>{
		fn enter(&self,listener: &mut (dyn abbParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_moduleData(self);
		}fn exit(&self,listener: &mut (dyn abbParserListener<'input> + 'a)) {
			listener.exit_moduleData(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for ModuleDataContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = abbParserContextType;
	fn get_rule_index(&self) -> usize { RULE_moduleData }
	//fn type_rule_index() -> usize where Self: Sized { RULE_moduleData }
}
antlr4rust::tid!{ModuleDataContextExt<'a>}

impl<'input> ModuleDataContextExt<'input>{
	fn new(parent: Option<Rc<dyn abbParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ModuleDataContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ModuleDataContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ModuleDataContextAttrs<'input>: abbParserContext<'input> + BorrowMut<ModuleDataContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token MODULE
/// Returns `None` if there is no child corresponding to token MODULE
fn MODULE(&self) -> Option<Rc<TerminalNode<'input,abbParserContextType>>> where Self:Sized{
	self.get_token(MODULE, 0)
}
fn moduleName(&self) -> Option<Rc<ModuleNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves all `TerminalNode`s corresponding to token NEWLINE in current rule
fn NEWLINE_all(&self) -> Vec<Rc<TerminalNode<'input,abbParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token NEWLINE, starting from 0.
/// Returns `None` if number of children corresponding to token NEWLINE is less or equal than `i`.
fn NEWLINE(&self, i: usize) -> Option<Rc<TerminalNode<'input,abbParserContextType>>> where Self:Sized{
	self.get_token(NEWLINE, i)
}
fn dataList(&self) -> Option<Rc<DataListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token ENDMODULE
/// Returns `None` if there is no child corresponding to token ENDMODULE
fn ENDMODULE(&self) -> Option<Rc<TerminalNode<'input,abbParserContextType>>> where Self:Sized{
	self.get_token(ENDMODULE, 0)
}

}

impl<'input> ModuleDataContextAttrs<'input> for ModuleDataContext<'input>{}

impl<'input, I, H> abbParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn moduleData(&mut self,)
	-> Result<Rc<ModuleDataContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ModuleDataContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 2, RULE_moduleData);
        let mut _localctx: Rc<ModuleDataContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(35);
			recog.base.match_token(MODULE,&mut recog.err_handler)?;

			/*InvokeRule moduleName*/
			recog.base.set_state(36);
			recog.moduleName()?;

			recog.base.set_state(37);
			recog.base.match_token(NEWLINE,&mut recog.err_handler)?;

			/*InvokeRule dataList*/
			recog.base.set_state(38);
			recog.dataList()?;

			recog.base.set_state(42);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==NEWLINE {
				{
				{
				recog.base.set_state(39);
				recog.base.match_token(NEWLINE,&mut recog.err_handler)?;

				}
				}
				recog.base.set_state(44);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(45);
			recog.base.match_token(ENDMODULE,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- moduleName ----------------
pub type ModuleNameContextAll<'input> = ModuleNameContext<'input>;


pub type ModuleNameContext<'input> = BaseParserRuleContext<'input,ModuleNameContextExt<'input>>;

#[derive(Clone)]
pub struct ModuleNameContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> abbParserContext<'input> for ModuleNameContext<'input>{}

impl<'input,'a> Listenable<dyn abbParserListener<'input> + 'a> for ModuleNameContext<'input>{
		fn enter(&self,listener: &mut (dyn abbParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_moduleName(self);
		}fn exit(&self,listener: &mut (dyn abbParserListener<'input> + 'a)) {
			listener.exit_moduleName(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for ModuleNameContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = abbParserContextType;
	fn get_rule_index(&self) -> usize { RULE_moduleName }
	//fn type_rule_index() -> usize where Self: Sized { RULE_moduleName }
}
antlr4rust::tid!{ModuleNameContextExt<'a>}

impl<'input> ModuleNameContextExt<'input>{
	fn new(parent: Option<Rc<dyn abbParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ModuleNameContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ModuleNameContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ModuleNameContextAttrs<'input>: abbParserContext<'input> + BorrowMut<ModuleNameContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,abbParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
fn procCall(&self) -> Option<Rc<ProcCallContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ModuleNameContextAttrs<'input> for ModuleNameContext<'input>{}

impl<'input, I, H> abbParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn moduleName(&mut self,)
	-> Result<Rc<ModuleNameContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ModuleNameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 4, RULE_moduleName);
        let mut _localctx: Rc<ModuleNameContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(49);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(1,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(47);
					recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule procCall*/
					recog.base.set_state(48);
					recog.procCall()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- dataList ----------------
pub type DataListContextAll<'input> = DataListContext<'input>;


pub type DataListContext<'input> = BaseParserRuleContext<'input,DataListContextExt<'input>>;

#[derive(Clone)]
pub struct DataListContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> abbParserContext<'input> for DataListContext<'input>{}

impl<'input,'a> Listenable<dyn abbParserListener<'input> + 'a> for DataListContext<'input>{
		fn enter(&self,listener: &mut (dyn abbParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_dataList(self);
		}fn exit(&self,listener: &mut (dyn abbParserListener<'input> + 'a)) {
			listener.exit_dataList(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for DataListContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = abbParserContextType;
	fn get_rule_index(&self) -> usize { RULE_dataList }
	//fn type_rule_index() -> usize where Self: Sized { RULE_dataList }
}
antlr4rust::tid!{DataListContextExt<'a>}

impl<'input> DataListContextExt<'input>{
	fn new(parent: Option<Rc<dyn abbParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<DataListContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DataListContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait DataListContextAttrs<'input>: abbParserContext<'input> + BorrowMut<DataListContextExt<'input>>{

/// Retrieves all `TerminalNode`s corresponding to token NEWLINE in current rule
fn NEWLINE_all(&self) -> Vec<Rc<TerminalNode<'input,abbParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token NEWLINE, starting from 0.
/// Returns `None` if number of children corresponding to token NEWLINE is less or equal than `i`.
fn NEWLINE(&self, i: usize) -> Option<Rc<TerminalNode<'input,abbParserContextType>>> where Self:Sized{
	self.get_token(NEWLINE, i)
}
fn declaration_all(&self) ->  Vec<Rc<DeclarationContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn declaration(&self, i: usize) -> Option<Rc<DeclarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn procedure_all(&self) ->  Vec<Rc<ProcedureContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn procedure(&self, i: usize) -> Option<Rc<ProcedureContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> DataListContextAttrs<'input> for DataListContext<'input>{}

impl<'input, I, H> abbParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn dataList(&mut self,)
	-> Result<Rc<DataListContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = DataListContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 6, RULE_dataList);
        let mut _localctx: Rc<DataListContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(60);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(3,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					recog.base.set_state(58);
					recog.err_handler.sync(&mut recog.base)?;
					match recog.base.input.la(1) {
					 NEWLINE 
						=> {
							{
							recog.base.set_state(51);
							recog.base.match_token(NEWLINE,&mut recog.err_handler)?;

							}
						}

					 LOCAL | CONST | PERS | VAR 
						=> {
							{
							/*InvokeRule declaration*/
							recog.base.set_state(52);
							recog.declaration()?;

							recog.base.set_state(53);
							recog.base.match_token(NEWLINE,&mut recog.err_handler)?;

							}
						}

					 PROC 
						=> {
							{
							/*InvokeRule procedure*/
							recog.base.set_state(55);
							recog.procedure()?;

							recog.base.set_state(56);
							recog.base.match_token(NEWLINE,&mut recog.err_handler)?;

							}
						}

						_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
					}
					} 
				}
				recog.base.set_state(62);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(3,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- procedure ----------------
pub type ProcedureContextAll<'input> = ProcedureContext<'input>;


pub type ProcedureContext<'input> = BaseParserRuleContext<'input,ProcedureContextExt<'input>>;

#[derive(Clone)]
pub struct ProcedureContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> abbParserContext<'input> for ProcedureContext<'input>{}

impl<'input,'a> Listenable<dyn abbParserListener<'input> + 'a> for ProcedureContext<'input>{
		fn enter(&self,listener: &mut (dyn abbParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_procedure(self);
		}fn exit(&self,listener: &mut (dyn abbParserListener<'input> + 'a)) {
			listener.exit_procedure(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for ProcedureContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = abbParserContextType;
	fn get_rule_index(&self) -> usize { RULE_procedure }
	//fn type_rule_index() -> usize where Self: Sized { RULE_procedure }
}
antlr4rust::tid!{ProcedureContextExt<'a>}

impl<'input> ProcedureContextExt<'input>{
	fn new(parent: Option<Rc<dyn abbParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ProcedureContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ProcedureContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ProcedureContextAttrs<'input>: abbParserContext<'input> + BorrowMut<ProcedureContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token PROC
/// Returns `None` if there is no child corresponding to token PROC
fn PROC(&self) -> Option<Rc<TerminalNode<'input,abbParserContextType>>> where Self:Sized{
	self.get_token(PROC, 0)
}
fn procCall(&self) -> Option<Rc<ProcCallContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves all `TerminalNode`s corresponding to token NEWLINE in current rule
fn NEWLINE_all(&self) -> Vec<Rc<TerminalNode<'input,abbParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token NEWLINE, starting from 0.
/// Returns `None` if number of children corresponding to token NEWLINE is less or equal than `i`.
fn NEWLINE(&self, i: usize) -> Option<Rc<TerminalNode<'input,abbParserContextType>>> where Self:Sized{
	self.get_token(NEWLINE, i)
}
/// Retrieves first TerminalNode corresponding to token ENDPROC
/// Returns `None` if there is no child corresponding to token ENDPROC
fn ENDPROC(&self) -> Option<Rc<TerminalNode<'input,abbParserContextType>>> where Self:Sized{
	self.get_token(ENDPROC, 0)
}
fn functionCall_all(&self) ->  Vec<Rc<FunctionCallContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn functionCall(&self, i: usize) -> Option<Rc<FunctionCallContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> ProcedureContextAttrs<'input> for ProcedureContext<'input>{}

impl<'input, I, H> abbParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn procedure(&mut self,)
	-> Result<Rc<ProcedureContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ProcedureContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 8, RULE_procedure);
        let mut _localctx: Rc<ProcedureContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(63);
			recog.base.match_token(PROC,&mut recog.err_handler)?;

			/*InvokeRule procCall*/
			recog.base.set_state(64);
			recog.procCall()?;

			recog.base.set_state(65);
			recog.base.match_token(NEWLINE,&mut recog.err_handler)?;

			recog.base.set_state(71);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==IDENTIFIER {
				{
				{
				/*InvokeRule functionCall*/
				recog.base.set_state(66);
				recog.functionCall()?;

				recog.base.set_state(67);
				recog.base.match_token(NEWLINE,&mut recog.err_handler)?;

				}
				}
				recog.base.set_state(73);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(74);
			recog.base.match_token(ENDPROC,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- procCall ----------------
pub type ProcCallContextAll<'input> = ProcCallContext<'input>;


pub type ProcCallContext<'input> = BaseParserRuleContext<'input,ProcCallContextExt<'input>>;

#[derive(Clone)]
pub struct ProcCallContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> abbParserContext<'input> for ProcCallContext<'input>{}

impl<'input,'a> Listenable<dyn abbParserListener<'input> + 'a> for ProcCallContext<'input>{
		fn enter(&self,listener: &mut (dyn abbParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_procCall(self);
		}fn exit(&self,listener: &mut (dyn abbParserListener<'input> + 'a)) {
			listener.exit_procCall(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for ProcCallContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = abbParserContextType;
	fn get_rule_index(&self) -> usize { RULE_procCall }
	//fn type_rule_index() -> usize where Self: Sized { RULE_procCall }
}
antlr4rust::tid!{ProcCallContextExt<'a>}

impl<'input> ProcCallContextExt<'input>{
	fn new(parent: Option<Rc<dyn abbParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ProcCallContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ProcCallContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ProcCallContextAttrs<'input>: abbParserContext<'input> + BorrowMut<ProcCallContextExt<'input>>{

fn procName(&self) -> Option<Rc<ProcNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn procParameter(&self) -> Option<Rc<ProcParameterContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ProcCallContextAttrs<'input> for ProcCallContext<'input>{}

impl<'input, I, H> abbParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn procCall(&mut self,)
	-> Result<Rc<ProcCallContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ProcCallContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 10, RULE_procCall);
        let mut _localctx: Rc<ProcCallContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule procName*/
			recog.base.set_state(76);
			recog.procName()?;

			recog.base.set_state(78);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==BRACKET_OPEN {
				{
				/*InvokeRule procParameter*/
				recog.base.set_state(77);
				recog.procParameter()?;

				}
			}

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- procName ----------------
pub type ProcNameContextAll<'input> = ProcNameContext<'input>;


pub type ProcNameContext<'input> = BaseParserRuleContext<'input,ProcNameContextExt<'input>>;

#[derive(Clone)]
pub struct ProcNameContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> abbParserContext<'input> for ProcNameContext<'input>{}

impl<'input,'a> Listenable<dyn abbParserListener<'input> + 'a> for ProcNameContext<'input>{
		fn enter(&self,listener: &mut (dyn abbParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_procName(self);
		}fn exit(&self,listener: &mut (dyn abbParserListener<'input> + 'a)) {
			listener.exit_procName(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for ProcNameContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = abbParserContextType;
	fn get_rule_index(&self) -> usize { RULE_procName }
	//fn type_rule_index() -> usize where Self: Sized { RULE_procName }
}
antlr4rust::tid!{ProcNameContextExt<'a>}

impl<'input> ProcNameContextExt<'input>{
	fn new(parent: Option<Rc<dyn abbParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ProcNameContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ProcNameContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ProcNameContextAttrs<'input>: abbParserContext<'input> + BorrowMut<ProcNameContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,abbParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}

}

impl<'input> ProcNameContextAttrs<'input> for ProcNameContext<'input>{}

impl<'input, I, H> abbParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn procName(&mut self,)
	-> Result<Rc<ProcNameContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ProcNameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 12, RULE_procName);
        let mut _localctx: Rc<ProcNameContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(80);
			recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- procParameter ----------------
pub type ProcParameterContextAll<'input> = ProcParameterContext<'input>;


pub type ProcParameterContext<'input> = BaseParserRuleContext<'input,ProcParameterContextExt<'input>>;

#[derive(Clone)]
pub struct ProcParameterContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> abbParserContext<'input> for ProcParameterContext<'input>{}

impl<'input,'a> Listenable<dyn abbParserListener<'input> + 'a> for ProcParameterContext<'input>{
		fn enter(&self,listener: &mut (dyn abbParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_procParameter(self);
		}fn exit(&self,listener: &mut (dyn abbParserListener<'input> + 'a)) {
			listener.exit_procParameter(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for ProcParameterContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = abbParserContextType;
	fn get_rule_index(&self) -> usize { RULE_procParameter }
	//fn type_rule_index() -> usize where Self: Sized { RULE_procParameter }
}
antlr4rust::tid!{ProcParameterContextExt<'a>}

impl<'input> ProcParameterContextExt<'input>{
	fn new(parent: Option<Rc<dyn abbParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ProcParameterContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ProcParameterContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ProcParameterContextAttrs<'input>: abbParserContext<'input> + BorrowMut<ProcParameterContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token BRACKET_OPEN
/// Returns `None` if there is no child corresponding to token BRACKET_OPEN
fn BRACKET_OPEN(&self) -> Option<Rc<TerminalNode<'input,abbParserContextType>>> where Self:Sized{
	self.get_token(BRACKET_OPEN, 0)
}
/// Retrieves first TerminalNode corresponding to token BRACKET_CLOSE
/// Returns `None` if there is no child corresponding to token BRACKET_CLOSE
fn BRACKET_CLOSE(&self) -> Option<Rc<TerminalNode<'input,abbParserContextType>>> where Self:Sized{
	self.get_token(BRACKET_CLOSE, 0)
}
/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,abbParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}

}

impl<'input> ProcParameterContextAttrs<'input> for ProcParameterContext<'input>{}

impl<'input, I, H> abbParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn procParameter(&mut self,)
	-> Result<Rc<ProcParameterContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ProcParameterContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 14, RULE_procParameter);
        let mut _localctx: Rc<ProcParameterContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(82);
			recog.base.match_token(BRACKET_OPEN,&mut recog.err_handler)?;

			recog.base.set_state(84);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==IDENTIFIER {
				{
				recog.base.set_state(83);
				recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

				}
			}

			recog.base.set_state(86);
			recog.base.match_token(BRACKET_CLOSE,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- functionCall ----------------
pub type FunctionCallContextAll<'input> = FunctionCallContext<'input>;


pub type FunctionCallContext<'input> = BaseParserRuleContext<'input,FunctionCallContextExt<'input>>;

#[derive(Clone)]
pub struct FunctionCallContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> abbParserContext<'input> for FunctionCallContext<'input>{}

impl<'input,'a> Listenable<dyn abbParserListener<'input> + 'a> for FunctionCallContext<'input>{
		fn enter(&self,listener: &mut (dyn abbParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_functionCall(self);
		}fn exit(&self,listener: &mut (dyn abbParserListener<'input> + 'a)) {
			listener.exit_functionCall(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for FunctionCallContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = abbParserContextType;
	fn get_rule_index(&self) -> usize { RULE_functionCall }
	//fn type_rule_index() -> usize where Self: Sized { RULE_functionCall }
}
antlr4rust::tid!{FunctionCallContextExt<'a>}

impl<'input> FunctionCallContextExt<'input>{
	fn new(parent: Option<Rc<dyn abbParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<FunctionCallContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,FunctionCallContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait FunctionCallContextAttrs<'input>: abbParserContext<'input> + BorrowMut<FunctionCallContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,abbParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
fn functionParameter_all(&self) ->  Vec<Rc<FunctionParameterContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn functionParameter(&self, i: usize) -> Option<Rc<FunctionParameterContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token SEMICOLON
/// Returns `None` if there is no child corresponding to token SEMICOLON
fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input,abbParserContextType>>> where Self:Sized{
	self.get_token(SEMICOLON, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,abbParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,abbParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}

}

impl<'input> FunctionCallContextAttrs<'input> for FunctionCallContext<'input>{}

impl<'input, I, H> abbParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn functionCall(&mut self,)
	-> Result<Rc<FunctionCallContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = FunctionCallContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 16, RULE_functionCall);
        let mut _localctx: Rc<FunctionCallContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(88);
			recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

			recog.base.set_state(94);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(7,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					/*InvokeRule functionParameter*/
					recog.base.set_state(89);
					recog.functionParameter()?;

					recog.base.set_state(90);
					recog.base.match_token(COMMA,&mut recog.err_handler)?;

					}
					} 
				}
				recog.base.set_state(96);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(7,&mut recog.base)?;
			}
			/*InvokeRule functionParameter*/
			recog.base.set_state(97);
			recog.functionParameter()?;

			recog.base.set_state(98);
			recog.base.match_token(SEMICOLON,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- functionParameter ----------------
pub type FunctionParameterContextAll<'input> = FunctionParameterContext<'input>;


pub type FunctionParameterContext<'input> = BaseParserRuleContext<'input,FunctionParameterContextExt<'input>>;

#[derive(Clone)]
pub struct FunctionParameterContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> abbParserContext<'input> for FunctionParameterContext<'input>{}

impl<'input,'a> Listenable<dyn abbParserListener<'input> + 'a> for FunctionParameterContext<'input>{
		fn enter(&self,listener: &mut (dyn abbParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_functionParameter(self);
		}fn exit(&self,listener: &mut (dyn abbParserListener<'input> + 'a)) {
			listener.exit_functionParameter(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for FunctionParameterContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = abbParserContextType;
	fn get_rule_index(&self) -> usize { RULE_functionParameter }
	//fn type_rule_index() -> usize where Self: Sized { RULE_functionParameter }
}
antlr4rust::tid!{FunctionParameterContextExt<'a>}

impl<'input> FunctionParameterContextExt<'input>{
	fn new(parent: Option<Rc<dyn abbParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<FunctionParameterContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,FunctionParameterContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait FunctionParameterContextAttrs<'input>: abbParserContext<'input> + BorrowMut<FunctionParameterContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token ON_CALL
/// Returns `None` if there is no child corresponding to token ON_CALL
fn ON_CALL(&self) -> Option<Rc<TerminalNode<'input,abbParserContextType>>> where Self:Sized{
	self.get_token(ON_CALL, 0)
}
/// Retrieves first TerminalNode corresponding to token OFF_CALL
/// Returns `None` if there is no child corresponding to token OFF_CALL
fn OFF_CALL(&self) -> Option<Rc<TerminalNode<'input,abbParserContextType>>> where Self:Sized{
	self.get_token(OFF_CALL, 0)
}
fn primitive(&self) -> Option<Rc<PrimitiveContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,abbParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}

}

impl<'input> FunctionParameterContextAttrs<'input> for FunctionParameterContext<'input>{}

impl<'input, I, H> abbParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn functionParameter(&mut self,)
	-> Result<Rc<FunctionParameterContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = FunctionParameterContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 18, RULE_functionParameter);
        let mut _localctx: Rc<FunctionParameterContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(104);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 ON_CALL 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(100);
					recog.base.match_token(ON_CALL,&mut recog.err_handler)?;

					}
				}

			 OFF_CALL 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(101);
					recog.base.match_token(OFF_CALL,&mut recog.err_handler)?;

					}
				}

			 PLUS | MINUS | BOOLLITERAL | CHARLITERAL | STRINGLITERAL | FLOATLITERAL |
			 INTLITERAL 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule primitive*/
					recog.base.set_state(102);
					recog.primitive()?;

					}
				}

			 IDENTIFIER 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					recog.base.set_state(103);
					recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- declaration ----------------
pub type DeclarationContextAll<'input> = DeclarationContext<'input>;


pub type DeclarationContext<'input> = BaseParserRuleContext<'input,DeclarationContextExt<'input>>;

#[derive(Clone)]
pub struct DeclarationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> abbParserContext<'input> for DeclarationContext<'input>{}

impl<'input,'a> Listenable<dyn abbParserListener<'input> + 'a> for DeclarationContext<'input>{
		fn enter(&self,listener: &mut (dyn abbParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_declaration(self);
		}fn exit(&self,listener: &mut (dyn abbParserListener<'input> + 'a)) {
			listener.exit_declaration(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for DeclarationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = abbParserContextType;
	fn get_rule_index(&self) -> usize { RULE_declaration }
	//fn type_rule_index() -> usize where Self: Sized { RULE_declaration }
}
antlr4rust::tid!{DeclarationContextExt<'a>}

impl<'input> DeclarationContextExt<'input>{
	fn new(parent: Option<Rc<dyn abbParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<DeclarationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DeclarationContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait DeclarationContextAttrs<'input>: abbParserContext<'input> + BorrowMut<DeclarationContextExt<'input>>{

fn init_(&self) -> Option<Rc<Init_ContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn type_(&self) -> Option<Rc<Type_ContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,abbParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
/// Retrieves first TerminalNode corresponding to token SEMICOLON
/// Returns `None` if there is no child corresponding to token SEMICOLON
fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input,abbParserContextType>>> where Self:Sized{
	self.get_token(SEMICOLON, 0)
}
/// Retrieves first TerminalNode corresponding to token EQUALS
/// Returns `None` if there is no child corresponding to token EQUALS
fn EQUALS(&self) -> Option<Rc<TerminalNode<'input,abbParserContextType>>> where Self:Sized{
	self.get_token(EQUALS, 0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> DeclarationContextAttrs<'input> for DeclarationContext<'input>{}

impl<'input, I, H> abbParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn declaration(&mut self,)
	-> Result<Rc<DeclarationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = DeclarationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 20, RULE_declaration);
        let mut _localctx: Rc<DeclarationContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule init_*/
			recog.base.set_state(106);
			recog.init_()?;

			/*InvokeRule type_*/
			recog.base.set_state(107);
			recog.type_()?;

			recog.base.set_state(108);
			recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

			recog.base.set_state(111);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==EQUALS {
				{
				recog.base.set_state(109);
				recog.base.match_token(EQUALS,&mut recog.err_handler)?;

				/*InvokeRule expression*/
				recog.base.set_state(110);
				recog.expression()?;

				}
			}

			recog.base.set_state(113);
			recog.base.match_token(SEMICOLON,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- type_ ----------------
pub type Type_ContextAll<'input> = Type_Context<'input>;


pub type Type_Context<'input> = BaseParserRuleContext<'input,Type_ContextExt<'input>>;

#[derive(Clone)]
pub struct Type_ContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> abbParserContext<'input> for Type_Context<'input>{}

impl<'input,'a> Listenable<dyn abbParserListener<'input> + 'a> for Type_Context<'input>{
		fn enter(&self,listener: &mut (dyn abbParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_type_(self);
		}fn exit(&self,listener: &mut (dyn abbParserListener<'input> + 'a)) {
			listener.exit_type_(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Type_ContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = abbParserContextType;
	fn get_rule_index(&self) -> usize { RULE_type_ }
	//fn type_rule_index() -> usize where Self: Sized { RULE_type_ }
}
antlr4rust::tid!{Type_ContextExt<'a>}

impl<'input> Type_ContextExt<'input>{
	fn new(parent: Option<Rc<dyn abbParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Type_ContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Type_ContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Type_ContextAttrs<'input>: abbParserContext<'input> + BorrowMut<Type_ContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token TOOLDATA
/// Returns `None` if there is no child corresponding to token TOOLDATA
fn TOOLDATA(&self) -> Option<Rc<TerminalNode<'input,abbParserContextType>>> where Self:Sized{
	self.get_token(TOOLDATA, 0)
}
/// Retrieves first TerminalNode corresponding to token WOBJDATA
/// Returns `None` if there is no child corresponding to token WOBJDATA
fn WOBJDATA(&self) -> Option<Rc<TerminalNode<'input,abbParserContextType>>> where Self:Sized{
	self.get_token(WOBJDATA, 0)
}
/// Retrieves first TerminalNode corresponding to token SPEEDDATA
/// Returns `None` if there is no child corresponding to token SPEEDDATA
fn SPEEDDATA(&self) -> Option<Rc<TerminalNode<'input,abbParserContextType>>> where Self:Sized{
	self.get_token(SPEEDDATA, 0)
}
/// Retrieves first TerminalNode corresponding to token ZONEDATA
/// Returns `None` if there is no child corresponding to token ZONEDATA
fn ZONEDATA(&self) -> Option<Rc<TerminalNode<'input,abbParserContextType>>> where Self:Sized{
	self.get_token(ZONEDATA, 0)
}
/// Retrieves first TerminalNode corresponding to token CLOCK
/// Returns `None` if there is no child corresponding to token CLOCK
fn CLOCK(&self) -> Option<Rc<TerminalNode<'input,abbParserContextType>>> where Self:Sized{
	self.get_token(CLOCK, 0)
}
/// Retrieves first TerminalNode corresponding to token BOOL
/// Returns `None` if there is no child corresponding to token BOOL
fn BOOL(&self) -> Option<Rc<TerminalNode<'input,abbParserContextType>>> where Self:Sized{
	self.get_token(BOOL, 0)
}

}

impl<'input> Type_ContextAttrs<'input> for Type_Context<'input>{}

impl<'input, I, H> abbParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn type_(&mut self,)
	-> Result<Rc<Type_ContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Type_ContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 22, RULE_type_);
        let mut _localctx: Rc<Type_ContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(115);
			_la = recog.base.input.la(1);
			if { !((((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << TOOLDATA) | (1usize << WOBJDATA) | (1usize << SPEEDDATA) | (1usize << ZONEDATA) | (1usize << CLOCK) | (1usize << BOOL))) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- init_ ----------------
pub type Init_ContextAll<'input> = Init_Context<'input>;


pub type Init_Context<'input> = BaseParserRuleContext<'input,Init_ContextExt<'input>>;

#[derive(Clone)]
pub struct Init_ContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> abbParserContext<'input> for Init_Context<'input>{}

impl<'input,'a> Listenable<dyn abbParserListener<'input> + 'a> for Init_Context<'input>{
		fn enter(&self,listener: &mut (dyn abbParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_init_(self);
		}fn exit(&self,listener: &mut (dyn abbParserListener<'input> + 'a)) {
			listener.exit_init_(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Init_ContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = abbParserContextType;
	fn get_rule_index(&self) -> usize { RULE_init_ }
	//fn type_rule_index() -> usize where Self: Sized { RULE_init_ }
}
antlr4rust::tid!{Init_ContextExt<'a>}

impl<'input> Init_ContextExt<'input>{
	fn new(parent: Option<Rc<dyn abbParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Init_ContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Init_ContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Init_ContextAttrs<'input>: abbParserContext<'input> + BorrowMut<Init_ContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token CONST
/// Returns `None` if there is no child corresponding to token CONST
fn CONST(&self) -> Option<Rc<TerminalNode<'input,abbParserContextType>>> where Self:Sized{
	self.get_token(CONST, 0)
}
/// Retrieves first TerminalNode corresponding to token PERS
/// Returns `None` if there is no child corresponding to token PERS
fn PERS(&self) -> Option<Rc<TerminalNode<'input,abbParserContextType>>> where Self:Sized{
	self.get_token(PERS, 0)
}
/// Retrieves first TerminalNode corresponding to token VAR
/// Returns `None` if there is no child corresponding to token VAR
fn VAR(&self) -> Option<Rc<TerminalNode<'input,abbParserContextType>>> where Self:Sized{
	self.get_token(VAR, 0)
}
/// Retrieves first TerminalNode corresponding to token LOCAL
/// Returns `None` if there is no child corresponding to token LOCAL
fn LOCAL(&self) -> Option<Rc<TerminalNode<'input,abbParserContextType>>> where Self:Sized{
	self.get_token(LOCAL, 0)
}

}

impl<'input> Init_ContextAttrs<'input> for Init_Context<'input>{}

impl<'input, I, H> abbParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn init_(&mut self,)
	-> Result<Rc<Init_ContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Init_ContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 24, RULE_init_);
        let mut _localctx: Rc<Init_ContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(118);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==LOCAL {
				{
				recog.base.set_state(117);
				recog.base.match_token(LOCAL,&mut recog.err_handler)?;

				}
			}

			recog.base.set_state(120);
			_la = recog.base.input.la(1);
			if { !((((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << CONST) | (1usize << PERS) | (1usize << VAR))) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- expression ----------------
pub type ExpressionContextAll<'input> = ExpressionContext<'input>;


pub type ExpressionContext<'input> = BaseParserRuleContext<'input,ExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct ExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> abbParserContext<'input> for ExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn abbParserListener<'input> + 'a> for ExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn abbParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_expression(self);
		}fn exit(&self,listener: &mut (dyn abbParserListener<'input> + 'a)) {
			listener.exit_expression(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for ExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = abbParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}
antlr4rust::tid!{ExpressionContextExt<'a>}

impl<'input> ExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn abbParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ExpressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ExpressionContextAttrs<'input>: abbParserContext<'input> + BorrowMut<ExpressionContextExt<'input>>{

fn array_(&self) -> Option<Rc<Array_ContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn primitive(&self) -> Option<Rc<PrimitiveContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ExpressionContextAttrs<'input> for ExpressionContext<'input>{}

impl<'input, I, H> abbParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn expression(&mut self,)
	-> Result<Rc<ExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 26, RULE_expression);
        let mut _localctx: Rc<ExpressionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(124);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 SQUARE_OPEN 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule array_*/
					recog.base.set_state(122);
					recog.array_()?;

					}
				}

			 PLUS | MINUS | BOOLLITERAL | CHARLITERAL | STRINGLITERAL | FLOATLITERAL |
			 INTLITERAL 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule primitive*/
					recog.base.set_state(123);
					recog.primitive()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- array_ ----------------
pub type Array_ContextAll<'input> = Array_Context<'input>;


pub type Array_Context<'input> = BaseParserRuleContext<'input,Array_ContextExt<'input>>;

#[derive(Clone)]
pub struct Array_ContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> abbParserContext<'input> for Array_Context<'input>{}

impl<'input,'a> Listenable<dyn abbParserListener<'input> + 'a> for Array_Context<'input>{
		fn enter(&self,listener: &mut (dyn abbParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_array_(self);
		}fn exit(&self,listener: &mut (dyn abbParserListener<'input> + 'a)) {
			listener.exit_array_(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for Array_ContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = abbParserContextType;
	fn get_rule_index(&self) -> usize { RULE_array_ }
	//fn type_rule_index() -> usize where Self: Sized { RULE_array_ }
}
antlr4rust::tid!{Array_ContextExt<'a>}

impl<'input> Array_ContextExt<'input>{
	fn new(parent: Option<Rc<dyn abbParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Array_ContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Array_ContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Array_ContextAttrs<'input>: abbParserContext<'input> + BorrowMut<Array_ContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token SQUARE_OPEN
/// Returns `None` if there is no child corresponding to token SQUARE_OPEN
fn SQUARE_OPEN(&self) -> Option<Rc<TerminalNode<'input,abbParserContextType>>> where Self:Sized{
	self.get_token(SQUARE_OPEN, 0)
}
fn expression_all(&self) ->  Vec<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token SQUARE_CLOSE
/// Returns `None` if there is no child corresponding to token SQUARE_CLOSE
fn SQUARE_CLOSE(&self) -> Option<Rc<TerminalNode<'input,abbParserContextType>>> where Self:Sized{
	self.get_token(SQUARE_CLOSE, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,abbParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,abbParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}

}

impl<'input> Array_ContextAttrs<'input> for Array_Context<'input>{}

impl<'input, I, H> abbParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn array_(&mut self,)
	-> Result<Rc<Array_ContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Array_ContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 28, RULE_array_);
        let mut _localctx: Rc<Array_ContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(126);
			recog.base.match_token(SQUARE_OPEN,&mut recog.err_handler)?;

			recog.base.set_state(132);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(12,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					/*InvokeRule expression*/
					recog.base.set_state(127);
					recog.expression()?;

					recog.base.set_state(128);
					recog.base.match_token(COMMA,&mut recog.err_handler)?;

					}
					} 
				}
				recog.base.set_state(134);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(12,&mut recog.base)?;
			}
			/*InvokeRule expression*/
			recog.base.set_state(135);
			recog.expression()?;

			recog.base.set_state(136);
			recog.base.match_token(SQUARE_CLOSE,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- primitive ----------------
pub type PrimitiveContextAll<'input> = PrimitiveContext<'input>;


pub type PrimitiveContext<'input> = BaseParserRuleContext<'input,PrimitiveContextExt<'input>>;

#[derive(Clone)]
pub struct PrimitiveContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> abbParserContext<'input> for PrimitiveContext<'input>{}

impl<'input,'a> Listenable<dyn abbParserListener<'input> + 'a> for PrimitiveContext<'input>{
		fn enter(&self,listener: &mut (dyn abbParserListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_primitive(self);
		}fn exit(&self,listener: &mut (dyn abbParserListener<'input> + 'a)) {
			listener.exit_primitive(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for PrimitiveContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = abbParserContextType;
	fn get_rule_index(&self) -> usize { RULE_primitive }
	//fn type_rule_index() -> usize where Self: Sized { RULE_primitive }
}
antlr4rust::tid!{PrimitiveContextExt<'a>}

impl<'input> PrimitiveContextExt<'input>{
	fn new(parent: Option<Rc<dyn abbParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PrimitiveContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PrimitiveContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait PrimitiveContextAttrs<'input>: abbParserContext<'input> + BorrowMut<PrimitiveContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token BOOLLITERAL
/// Returns `None` if there is no child corresponding to token BOOLLITERAL
fn BOOLLITERAL(&self) -> Option<Rc<TerminalNode<'input,abbParserContextType>>> where Self:Sized{
	self.get_token(BOOLLITERAL, 0)
}
/// Retrieves first TerminalNode corresponding to token CHARLITERAL
/// Returns `None` if there is no child corresponding to token CHARLITERAL
fn CHARLITERAL(&self) -> Option<Rc<TerminalNode<'input,abbParserContextType>>> where Self:Sized{
	self.get_token(CHARLITERAL, 0)
}
/// Retrieves first TerminalNode corresponding to token STRINGLITERAL
/// Returns `None` if there is no child corresponding to token STRINGLITERAL
fn STRINGLITERAL(&self) -> Option<Rc<TerminalNode<'input,abbParserContextType>>> where Self:Sized{
	self.get_token(STRINGLITERAL, 0)
}
/// Retrieves first TerminalNode corresponding to token FLOATLITERAL
/// Returns `None` if there is no child corresponding to token FLOATLITERAL
fn FLOATLITERAL(&self) -> Option<Rc<TerminalNode<'input,abbParserContextType>>> where Self:Sized{
	self.get_token(FLOATLITERAL, 0)
}
/// Retrieves first TerminalNode corresponding to token PLUS
/// Returns `None` if there is no child corresponding to token PLUS
fn PLUS(&self) -> Option<Rc<TerminalNode<'input,abbParserContextType>>> where Self:Sized{
	self.get_token(PLUS, 0)
}
/// Retrieves first TerminalNode corresponding to token MINUS
/// Returns `None` if there is no child corresponding to token MINUS
fn MINUS(&self) -> Option<Rc<TerminalNode<'input,abbParserContextType>>> where Self:Sized{
	self.get_token(MINUS, 0)
}
/// Retrieves first TerminalNode corresponding to token INTLITERAL
/// Returns `None` if there is no child corresponding to token INTLITERAL
fn INTLITERAL(&self) -> Option<Rc<TerminalNode<'input,abbParserContextType>>> where Self:Sized{
	self.get_token(INTLITERAL, 0)
}

}

impl<'input> PrimitiveContextAttrs<'input> for PrimitiveContext<'input>{}

impl<'input, I, H> abbParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn primitive(&mut self,)
	-> Result<Rc<PrimitiveContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PrimitiveContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 30, RULE_primitive);
        let mut _localctx: Rc<PrimitiveContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(149);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(15,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(138);
					recog.base.match_token(BOOLLITERAL,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(139);
					recog.base.match_token(CHARLITERAL,&mut recog.err_handler)?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(140);
					recog.base.match_token(STRINGLITERAL,&mut recog.err_handler)?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					recog.base.set_state(142);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==PLUS || _la==MINUS {
						{
						recog.base.set_state(141);
						_la = recog.base.input.la(1);
						if { !(_la==PLUS || _la==MINUS) } {
							recog.err_handler.recover_inline(&mut recog.base)?;

						}
						else {
							if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
							recog.err_handler.report_match(&mut recog.base);
							recog.base.consume(&mut recog.err_handler);
						}
						}
					}

					recog.base.set_state(144);
					recog.base.match_token(FLOATLITERAL,&mut recog.err_handler)?;

					}
				}
			,
				5 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 5);
					recog.base.enter_outer_alt(None, 5);
					{
					recog.base.set_state(146);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==PLUS || _la==MINUS {
						{
						recog.base.set_state(145);
						_la = recog.base.input.la(1);
						if { !(_la==PLUS || _la==MINUS) } {
							recog.err_handler.recover_inline(&mut recog.base)?;

						}
						else {
							if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
							recog.err_handler.report_match(&mut recog.base);
							recog.base.consume(&mut recog.err_handler);
						}
						}
					}

					recog.base.set_state(148);
					recog.base.match_token(INTLITERAL,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}

lazy_static! {
    static ref _ATN: Arc<ATN> =
        Arc::new(ATNDeserializer::new(None).deserialize(_serializedATN.chars()));
    static ref _decision_to_DFA: Arc<Vec<antlr4rust::RwLock<DFA>>> = {
        let mut dfa = Vec::new();
        let size = _ATN.decision_to_state.len();
        for i in 0..size {
            dfa.push(DFA::new(
                _ATN.clone(),
                _ATN.get_decision_state(i),
                i as isize,
            ).into())
        }
        Arc::new(dfa)
    };
}



const _serializedATN:&'static str =
	"\x03\u{608b}\u{a72a}\u{8133}\u{b9ed}\u{417c}\u{3be7}\u{7786}\u{5964}\x03\
	\x33\u{9a}\x04\x02\x09\x02\x04\x03\x09\x03\x04\x04\x09\x04\x04\x05\x09\x05\
	\x04\x06\x09\x06\x04\x07\x09\x07\x04\x08\x09\x08\x04\x09\x09\x09\x04\x0a\
	\x09\x0a\x04\x0b\x09\x0b\x04\x0c\x09\x0c\x04\x0d\x09\x0d\x04\x0e\x09\x0e\
	\x04\x0f\x09\x0f\x04\x10\x09\x10\x04\x11\x09\x11\x03\x02\x03\x02\x03\x02\
	\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x07\x03\x2b\x0a\x03\x0c\x03\x0e\
	\x03\x2e\x0b\x03\x03\x03\x03\x03\x03\x04\x03\x04\x05\x04\x34\x0a\x04\x03\
	\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x07\x05\x3d\x0a\x05\
	\x0c\x05\x0e\x05\x40\x0b\x05\x03\x06\x03\x06\x03\x06\x03\x06\x03\x06\x03\
	\x06\x07\x06\x48\x0a\x06\x0c\x06\x0e\x06\x4b\x0b\x06\x03\x06\x03\x06\x03\
	\x07\x03\x07\x05\x07\x51\x0a\x07\x03\x08\x03\x08\x03\x09\x03\x09\x05\x09\
	\x57\x0a\x09\x03\x09\x03\x09\x03\x0a\x03\x0a\x03\x0a\x03\x0a\x07\x0a\x5f\
	\x0a\x0a\x0c\x0a\x0e\x0a\x62\x0b\x0a\x03\x0a\x03\x0a\x03\x0a\x03\x0b\x03\
	\x0b\x03\x0b\x03\x0b\x05\x0b\x6b\x0a\x0b\x03\x0c\x03\x0c\x03\x0c\x03\x0c\
	\x03\x0c\x05\x0c\x72\x0a\x0c\x03\x0c\x03\x0c\x03\x0d\x03\x0d\x03\x0e\x05\
	\x0e\x79\x0a\x0e\x03\x0e\x03\x0e\x03\x0f\x03\x0f\x05\x0f\x7f\x0a\x0f\x03\
	\x10\x03\x10\x03\x10\x03\x10\x07\x10\u{85}\x0a\x10\x0c\x10\x0e\x10\u{88}\
	\x0b\x10\x03\x10\x03\x10\x03\x10\x03\x11\x03\x11\x03\x11\x03\x11\x05\x11\
	\u{91}\x0a\x11\x03\x11\x03\x11\x05\x11\u{95}\x0a\x11\x03\x11\x05\x11\u{98}\
	\x0a\x11\x03\x11\x02\x02\x12\x02\x04\x06\x08\x0a\x0c\x0e\x10\x12\x14\x16\
	\x18\x1a\x1c\x1e\x20\x02\x05\x03\x02\x0b\x10\x03\x02\x08\x0a\x03\x02\x26\
	\x27\x02\u{9f}\x02\x22\x03\x02\x02\x02\x04\x25\x03\x02\x02\x02\x06\x33\x03\
	\x02\x02\x02\x08\x3e\x03\x02\x02\x02\x0a\x41\x03\x02\x02\x02\x0c\x4e\x03\
	\x02\x02\x02\x0e\x52\x03\x02\x02\x02\x10\x54\x03\x02\x02\x02\x12\x5a\x03\
	\x02\x02\x02\x14\x6a\x03\x02\x02\x02\x16\x6c\x03\x02\x02\x02\x18\x75\x03\
	\x02\x02\x02\x1a\x78\x03\x02\x02\x02\x1c\x7e\x03\x02\x02\x02\x1e\u{80}\x03\
	\x02\x02\x02\x20\u{97}\x03\x02\x02\x02\x22\x23\x05\x04\x03\x02\x23\x24\x07\
	\x02\x02\x03\x24\x03\x03\x02\x02\x02\x25\x26\x07\x03\x02\x02\x26\x27\x05\
	\x06\x04\x02\x27\x28\x07\x2c\x02\x02\x28\x2c\x05\x08\x05\x02\x29\x2b\x07\
	\x2c\x02\x02\x2a\x29\x03\x02\x02\x02\x2b\x2e\x03\x02\x02\x02\x2c\x2a\x03\
	\x02\x02\x02\x2c\x2d\x03\x02\x02\x02\x2d\x2f\x03\x02\x02\x02\x2e\x2c\x03\
	\x02\x02\x02\x2f\x30\x07\x04\x02\x02\x30\x05\x03\x02\x02\x02\x31\x34\x07\
	\x33\x02\x02\x32\x34\x05\x0c\x07\x02\x33\x31\x03\x02\x02\x02\x33\x32\x03\
	\x02\x02\x02\x34\x07\x03\x02\x02\x02\x35\x3d\x07\x2c\x02\x02\x36\x37\x05\
	\x16\x0c\x02\x37\x38\x07\x2c\x02\x02\x38\x3d\x03\x02\x02\x02\x39\x3a\x05\
	\x0a\x06\x02\x3a\x3b\x07\x2c\x02\x02\x3b\x3d\x03\x02\x02\x02\x3c\x35\x03\
	\x02\x02\x02\x3c\x36\x03\x02\x02\x02\x3c\x39\x03\x02\x02\x02\x3d\x40\x03\
	\x02\x02\x02\x3e\x3c\x03\x02\x02\x02\x3e\x3f\x03\x02\x02\x02\x3f\x09\x03\
	\x02\x02\x02\x40\x3e\x03\x02\x02\x02\x41\x42\x07\x05\x02\x02\x42\x43\x05\
	\x0c\x07\x02\x43\x49\x07\x2c\x02\x02\x44\x45\x05\x12\x0a\x02\x45\x46\x07\
	\x2c\x02\x02\x46\x48\x03\x02\x02\x02\x47\x44\x03\x02\x02\x02\x48\x4b\x03\
	\x02\x02\x02\x49\x47\x03\x02\x02\x02\x49\x4a\x03\x02\x02\x02\x4a\x4c\x03\
	\x02\x02\x02\x4b\x49\x03\x02\x02\x02\x4c\x4d\x07\x06\x02\x02\x4d\x0b\x03\
	\x02\x02\x02\x4e\x50\x05\x0e\x08\x02\x4f\x51\x05\x10\x09\x02\x50\x4f\x03\
	\x02\x02\x02\x50\x51\x03\x02\x02\x02\x51\x0d\x03\x02\x02\x02\x52\x53\x07\
	\x33\x02\x02\x53\x0f\x03\x02\x02\x02\x54\x56\x07\x1a\x02\x02\x55\x57\x07\
	\x33\x02\x02\x56\x55\x03\x02\x02\x02\x56\x57\x03\x02\x02\x02\x57\x58\x03\
	\x02\x02\x02\x58\x59\x07\x1b\x02\x02\x59\x11\x03\x02\x02\x02\x5a\x60\x07\
	\x33\x02\x02\x5b\x5c\x05\x14\x0b\x02\x5c\x5d\x07\x15\x02\x02\x5d\x5f\x03\
	\x02\x02\x02\x5e\x5b\x03\x02\x02\x02\x5f\x62\x03\x02\x02\x02\x60\x5e\x03\
	\x02\x02\x02\x60\x61\x03\x02\x02\x02\x61\x63\x03\x02\x02\x02\x62\x60\x03\
	\x02\x02\x02\x63\x64\x05\x14\x0b\x02\x64\x65\x07\x19\x02\x02\x65\x13\x03\
	\x02\x02\x02\x66\x6b\x07\x11\x02\x02\x67\x6b\x07\x12\x02\x02\x68\x6b\x05\
	\x20\x11\x02\x69\x6b\x07\x33\x02\x02\x6a\x66\x03\x02\x02\x02\x6a\x67\x03\
	\x02\x02\x02\x6a\x68\x03\x02\x02\x02\x6a\x69\x03\x02\x02\x02\x6b\x15\x03\
	\x02\x02\x02\x6c\x6d\x05\x1a\x0e\x02\x6d\x6e\x05\x18\x0d\x02\x6e\x71\x07\
	\x33\x02\x02\x6f\x70\x07\x14\x02\x02\x70\x72\x05\x1c\x0f\x02\x71\x6f\x03\
	\x02\x02\x02\x71\x72\x03\x02\x02\x02\x72\x73\x03\x02\x02\x02\x73\x74\x07\
	\x19\x02\x02\x74\x17\x03\x02\x02\x02\x75\x76\x09\x02\x02\x02\x76\x19\x03\
	\x02\x02\x02\x77\x79\x07\x07\x02\x02\x78\x77\x03\x02\x02\x02\x78\x79\x03\
	\x02\x02\x02\x79\x7a\x03\x02\x02\x02\x7a\x7b\x09\x03\x02\x02\x7b\x1b\x03\
	\x02\x02\x02\x7c\x7f\x05\x1e\x10\x02\x7d\x7f\x05\x20\x11\x02\x7e\x7c\x03\
	\x02\x02\x02\x7e\x7d\x03\x02\x02\x02\x7f\x1d\x03\x02\x02\x02\u{80}\u{86}\
	\x07\x1c\x02\x02\u{81}\u{82}\x05\x1c\x0f\x02\u{82}\u{83}\x07\x15\x02\x02\
	\u{83}\u{85}\x03\x02\x02\x02\u{84}\u{81}\x03\x02\x02\x02\u{85}\u{88}\x03\
	\x02\x02\x02\u{86}\u{84}\x03\x02\x02\x02\u{86}\u{87}\x03\x02\x02\x02\u{87}\
	\u{89}\x03\x02\x02\x02\u{88}\u{86}\x03\x02\x02\x02\u{89}\u{8a}\x05\x1c\x0f\
	\x02\u{8a}\u{8b}\x07\x1d\x02\x02\u{8b}\x1f\x03\x02\x02\x02\u{8c}\u{98}\x07\
	\x2e\x02\x02\u{8d}\u{98}\x07\x2f\x02\x02\u{8e}\u{98}\x07\x30\x02\x02\u{8f}\
	\u{91}\x09\x04\x02\x02\u{90}\u{8f}\x03\x02\x02\x02\u{90}\u{91}\x03\x02\x02\
	\x02\u{91}\u{92}\x03\x02\x02\x02\u{92}\u{98}\x07\x31\x02\x02\u{93}\u{95}\
	\x09\x04\x02\x02\u{94}\u{93}\x03\x02\x02\x02\u{94}\u{95}\x03\x02\x02\x02\
	\u{95}\u{96}\x03\x02\x02\x02\u{96}\u{98}\x07\x32\x02\x02\u{97}\u{8c}\x03\
	\x02\x02\x02\u{97}\u{8d}\x03\x02\x02\x02\u{97}\u{8e}\x03\x02\x02\x02\u{97}\
	\u{90}\x03\x02\x02\x02\u{97}\u{94}\x03\x02\x02\x02\u{98}\x21\x03\x02\x02\
	\x02\x12\x2c\x33\x3c\x3e\x49\x50\x56\x60\x6a\x71\x78\x7e\u{86}\u{90}\u{94}\
	\u{97}";

