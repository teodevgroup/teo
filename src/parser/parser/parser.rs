use std::borrow::Borrow;
use snailquote::unescape;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::path::{Path, PathBuf};
use std::fs;
use maplit::{btreemap, btreeset, hashmap};
use pest::Parser as PestParser;
use pest::pratt_parser::PrattParser;
use to_mut::ToMut;
use to_mut_proc_macro::ToMut;
use once_cell::sync::Lazy;
use crate::core::result::Result;
use crate::core::interface::ResolvedInterfaceField;
use crate::parser::ast::action::{ActionDeclaration, ActionGroupDeclaration, ActionInputFormat};
use crate::parser::ast::argument::{Argument, ArgumentList};
use crate::parser::ast::arith_expr::{ArithExpr, Op};
use crate::parser::ast::client::ASTClient;
use crate::parser::ast::comment_block::CommentBlock;
use crate::parser::ast::config::ASTServer;
use crate::parser::ast::connector::ASTConnector;
use crate::parser::ast::constant::Constant;
use crate::parser::ast::data_set::{ASTDataSet, DataSetGroup, DataSetRecord};
use crate::parser::ast::debug_conf::ASTDebugConf;
use crate::parser::ast::decorator::ASTDecorator;
use crate::parser::ast::expression::{Expression, ExpressionKind, ArrayLiteral, BoolLiteral, DictionaryLiteral, EnumChoiceLiteral, NullLiteral, NumericLiteral, RangeLiteral, StringLiteral, TupleLiteral, RegExpLiteral, NullishCoalescing, Negation, BitwiseNegation };
use crate::parser::ast::field::ASTField;
use crate::parser::ast::generator::ASTEntity;
use crate::parser::ast::group::Group;
use crate::parser::ast::identifier::ASTIdentifier;
use crate::parser::ast::identifier_path::ASTIdentifierPath;
use crate::parser::ast::import::ASTImport;
use crate::parser::ast::interface::{InterfaceDeclaration, InterfaceItemDeclaration};
use crate::parser::ast::item::Item;
use crate::parser::ast::middleware::MiddlewareDeclaration;
use crate::parser::ast::model::ASTModel;
use crate::parser::ast::pipeline::ASTPipeline;
use crate::parser::ast::r#enum::{ASTEnum, EnumChoice};
use crate::parser::ast::r#type::{Arity, ASTFieldType};
use crate::parser::ast::source::ASTSource;
use crate::parser::ast::span::Span;
use crate::parser::ast::subscript::Subscript;
use crate::parser::ast::test_conf::ASTTestConf;
use crate::parser::ast::top::Top;
use crate::parser::ast::interface_type::InterfaceType;
use crate::parser::ast::namespace::ASTNamespace;
use crate::parser::ast::static_files::StaticFiles;
use crate::parser::ast::unit::Unit;
use crate::parser::diagnostics::diagnostics::{Diagnostics, DiagnosticsError};
use crate::parser::diagnostics::printer;
use crate::parser::parser::resolver::Resolver;

#[derive(pest_derive::Parser)]
#[grammar = "./src/parser/schema.pest"]
pub(crate) struct SchemaParser;

type Pair<'a> = pest::iterators::Pair<'a, Rule>;

static PRATT_PARSER: Lazy<PrattParser<Rule>> = Lazy::new(|| {
    use pest::pratt_parser::{Assoc::*, Op};
    use Rule::*;

    // Precedence is defined lowest to highest
    PrattParser::new()
        // Addition and subtract have equal precedence
        .op(Op::infix(BI_OR, Left))
        .op(Op::infix(BI_XOR, Left))
        .op(Op::infix(BI_AND, Left))
        .op(Op::infix(ADD, Left) | Op::infix(SUB, Left))
        .op(Op::infix(MUL, Left) | Op::infix(DIV, Left) | Op::infix(MOD, Left))
    // .op(Op::prefix(unary_minus))
});

#[derive(Debug, ToMut)]
pub(crate) struct ASTParser {
    pub(crate) sources: BTreeMap<usize, ASTSource>,
    pub(crate) enums: Vec<Vec<usize>>,
    pub(crate) models: Vec<Vec<usize>>,
    pub(crate) connector: Option<Vec<usize>>,
    pub(crate) server: Option<Vec<usize>>,
    pub(crate) entities: Vec<Vec<usize>>,
    pub(crate) clients: Vec<Vec<usize>>,
    pub(crate) data_sets: Vec<Vec<usize>>,
    pub(crate) debug_conf: Option<Vec<usize>>,
    pub(crate) test_conf: Option<Vec<usize>>,
    pub(crate) middlewares: Vec<Vec<usize>>,
    pub(crate) action_groups: Vec<Vec<usize>>,
    pub(crate) actions: Vec<Vec<usize>>,
    pub(crate) interfaces: Vec<Vec<usize>>,
    pub(crate) static_files: Vec<Vec<usize>>,
    pub(crate) namespaces: Vec<Vec<usize>>,
    pub(crate) next_id: usize,
    pub(crate) resolved: bool,
    pub(crate) resolved_action_inputs: HashMap<&'static str, HashMap<&'static str, ResolvedInterfaceField>>,
    pub(crate) current_source_path_bufs: Vec<PathBuf>,
    pub(crate) current_namespace_key: &'static str,
}

impl ASTParser {

    pub(crate) fn new() -> Self {
        Self {
            sources: btreemap!{},
            enums: vec![],
            models: vec![],
            connector: None,
            server: None,
            entities: vec![],
            clients: vec![],
            data_sets: vec![],
            debug_conf: None,
            test_conf: None,
            middlewares: vec![],
            action_groups: vec![],
            actions: vec![],
            interfaces: vec![],
            static_files: vec![],
            namespaces: vec![],
            next_id: 0,
            resolved: false,
            resolved_action_inputs: hashmap!{},
            current_source_path_bufs: vec![],
            current_namespace_key: "main",
        }
    }

    fn next_id(&mut self) -> usize {
        self.next_id += 1;
        self.next_id
    }

    pub(crate) fn parse(&mut self, main: Option<&str>, diagnostics: &mut Diagnostics) -> () {
        let main = if main.is_some() { main.unwrap() } else {
            let mut result: Option<&str> = None;
            for name in ["schema.teo", "src/schema.teo", "index.teo", "src/index.teo"] {
                let relative = PathBuf::from(name);
                let absolute = match fs::canonicalize(&relative) {
                    Ok(_path) => Some(name),
                    Err(_) => None,
                };
                if absolute.is_some() {
                    result = absolute;
                    break
                }
            }
            if result.is_some() {
                result.unwrap()
            } else {
                panic!("Cannot find a schema file.")
            }
        };
        let relative = PathBuf::from(main);
        let absolute = match fs::canonicalize(&relative) {
            Ok(path) => path,
            Err(_) => panic!("Schema file '{}' is not found.", relative.to_str().unwrap()),
        };
        self.parse_source(&absolute, diagnostics);
        let resolver = Resolver::new(self);
        resolver.resolve_parser(self, diagnostics);
    }

    fn parse_source(&mut self, path: &PathBuf, diagnostics: &mut Diagnostics) {
        let source_id = self.next_id();
        self.current_source_path_bufs.push(path.clone());
        let content = match fs::read_to_string(&path) {
            Ok(content) => content,
            Err(err) => panic!("Cannot read schema file content: {}", err)
        };
        let mut pairs = match SchemaParser::parse(Rule::schema, &content) {
            Ok(pairs) => pairs,
            Err(err) => panic!("{}", err)
        };
        let pairs = pairs.next().unwrap();
        let mut tops: BTreeMap<usize, Top> = btreemap![];
        let mut imports: BTreeSet<usize> = btreeset!{};
        let mut constants: BTreeSet<usize> = btreeset!{};
        let mut enums: BTreeSet<usize> = btreeset!{};
        let mut models: BTreeSet<usize> = btreeset!{};
        let mut namespaces: BTreeSet<usize> = btreeset!{};
        let mut action_groups: BTreeSet<usize> = btreeset!{};
        let mut data_sets: BTreeSet<usize> = btreeset!{};
        let mut pairs = pairs.into_inner().peekable();

        while let Some(current) = pairs.next() {
            let item_id = self.next_id();
            match current.as_rule() {
                Rule::import_statement => {
                    let import = self.parse_import(current, source_id, item_id, path.clone(), diagnostics);
                    tops.insert(item_id, import);
                    imports.insert(item_id);
                },
                Rule::let_declaration => {
                    let constant = self.parse_let_declaration(current, source_id, item_id, diagnostics);
                    tops.insert(item_id, constant);
                    constants.insert(item_id);
                },
                Rule::model_declaration => {
                    let model = self.parse_model(current, source_id, item_id, diagnostics, vec![], vec![source_id, item_id]);
                    tops.insert(item_id, model);
                    models.insert(item_id);
                    self.models.push(vec![source_id, item_id]);
                },
                Rule::enum_declaration => {
                    let r#enum = self.parse_enum(current, source_id, item_id, vec![], diagnostics);
                    tops.insert(item_id, r#enum);
                    enums.insert(item_id);
                    self.enums.push(vec![source_id, item_id]);
                },
                Rule::config_declaration => {
                    let config_block = self.parse_config_block(current, source_id, item_id, diagnostics);
                    tops.insert(item_id, config_block);
                },
                Rule::dataset_declaration => {
                    let dataset_block = self.parse_dataset_block(current, source_id, item_id, diagnostics);
                    tops.insert(item_id, dataset_block);
                    data_sets.insert(item_id);
                    self.data_sets.push(vec![source_id, item_id]);
                }
                Rule::EOI | Rule::EMPTY_LINES => {},
                Rule::comment_block => (),
                Rule::interface_declaration => {
                    let interface_declaration = self.parse_interface_declaration(current, source_id, item_id, diagnostics);
                    tops.insert(item_id, interface_declaration);
                    self.interfaces.push(vec![source_id, item_id]);
                },
                Rule::action_group_declaration => {
                    let action_group_declaration = self.parse_action_group_declaration(current, source_id, item_id, diagnostics);
                    tops.insert(item_id, action_group_declaration);
                    action_groups.insert(item_id);
                    self.action_groups.push(vec![source_id, item_id]);
                },
                Rule::middleware_declaration => {
                    let middleware_declaration = self.parse_middleware_declaration(current, source_id, item_id, diagnostics);
                    tops.insert(item_id, middleware_declaration);
                    self.middlewares.push(vec![source_id, item_id]);
                },
                Rule::static_files_declaration => {
                    let static_files_declaration = self.parse_static_files_declaration(current, source_id, item_id, diagnostics);
                    tops.insert(item_id, static_files_declaration);
                    self.static_files.push(vec![source_id, item_id]);
                },
                Rule::interface_enum_declaration => (),
                Rule::namespace => {
                    let namespace_declaration = self.parse_namespace_declaration(current, path, source_id, item_id, vec![], vec![], diagnostics);
                    tops.insert(item_id, Top::ASTNamespace(namespace_declaration));
                    namespaces.insert(item_id);
                    self.namespaces.push(vec![source_id, item_id]);
                },
                Rule::model_decorator_declaration => (),
                Rule::field_decorator_declaration => (),
                Rule::relation_decorator_declaration => (),
                Rule::property_decorator_declaration => (),
                Rule::pipeline_item_declaration => (),
                Rule::CATCH_ALL => self.insert_unparsed_rule_and_exit(diagnostics, Self::parse_span(&current)),
                _ => self.insert_unparsed_rule_and_exit(diagnostics, Self::parse_span(&current)),
            }
        }
        let result = ASTSource::new(source_id, path.clone(), tops, imports, constants, enums, models, namespaces, action_groups, data_sets);
        for import in result.borrow().imports() {
            let found = self.sources.values().find(|v| {
                (*v).borrow().path == import.path
            });
            if found.is_none() {
                self.parse_source(&import.path, diagnostics);
            }
        }
        self.sources.insert(source_id, result);
    }

    fn parse_import(&mut self, pair: Pair<'_>, source_id: usize, item_id: usize, path: PathBuf, diagnostics: &mut Diagnostics) -> Top {
        let mut identifiers = vec![];
        let span = Self::parse_span(&pair);
        let mut source: Option<StringLiteral> = None;
        for current in pair.into_inner() {
            match current.as_rule() {
                Rule::string_literal => {
                    let string_span = Self::parse_span(&current);
                    source = Some(StringLiteral { value: current.as_str().to_string(), span: string_span });
                },
                Rule::import_identifier_list => identifiers = self.parse_import_identifier_list(current, source_id, diagnostics),
                _ => self.insert_unparsed_rule_and_exit(diagnostics, Self::parse_span(&current)),
            }
        }
        let unescaped = unescape(source.as_ref().unwrap().value.as_str()).unwrap();
        let relative = PathBuf::from(unescaped);
        let mut dir = path.clone();
        dir.pop();
        let new = dir.join(&relative);
        let absolute = match Self::canonicalize(&new) {
            Some(path) => path,
            None => {
                self.insert_diagnostics_error_and_exit(format!("Schema file '{}' doesn't exist", relative.to_str().unwrap()), diagnostics, source.unwrap().span);
                panic!()
            },
        };
        Top::Import(ASTImport::new(item_id, source_id, identifiers, source.unwrap(), absolute, span))
    }

    fn canonicalize(path_buf: &PathBuf) -> Option<PathBuf> {
        if let Ok(found) = fs::canonicalize(&path_buf) {
            if !fs::metadata(&found).unwrap().is_dir() {
                return Some(found);
            }
        }
        let mut with_extension = path_buf.clone();
        Self::add_extension(&mut with_extension, "teo");
        if let Ok(found) = fs::canonicalize(&with_extension) {
            return Some(found);
        }
        let mut folder_index = path_buf.clone();
        folder_index.push("index.teo");
        if let Ok(found) = fs::canonicalize(&folder_index) {
            return Some(found);
        }
        None
    }

    fn add_extension(path: &mut PathBuf, extension: impl AsRef<std::path::Path>) {
        match path.extension() {
            Some(ext) => {
                let mut ext = ext.to_os_string();
                ext.push(".");
                ext.push(extension.as_ref());
                path.set_extension(ext)
            }
            None => path.set_extension(extension.as_ref()),
        };
    }

    fn parse_comment_block(&self, pair: Pair<'_>, source_id: usize, diagnostics: &mut Diagnostics) -> CommentBlock {
        let mut name = "".to_owned();
        let mut desc = "".to_owned();
        let span = Self::parse_span(&pair);
        for current in pair.into_inner() {
            match current.as_rule() {
                Rule::triple_comment => {
                    let (token, doc) = self.parse_comment_line(current, source_id, diagnostics);
                    if let Some(token) = token {
                        if &token == "@name" {
                            name = doc;
                        } else if &token == "@description" {
                            desc = Self::append_doc_desc(desc, doc)
                        }
                    } else {
                        desc = Self::append_doc_desc(desc, doc)
                    }
                },
                Rule::double_comment_block => {},
                Rule::double_comment => {},
                _ => self.insert_unparsed_rule_and_exit(diagnostics, Self::parse_span(&current)),
            }
        }
        CommentBlock {
            name: if name.is_empty() { None } else { Some(name) },
            desc: if desc.is_empty() { None } else { Some(desc) },
            span,
        }
    }

    fn append_doc_desc(desc: String, doc: String) -> String {
        if desc.is_empty() {
            doc.trim().to_owned()
        } else {
            desc + " " + doc.trim()
        }
    }

    fn parse_comment_line(&self, pair: Pair<'_>, source_id: usize, diagnostics: &mut Diagnostics) -> (Option<String>, String) {
        let mut token = None;
        let mut content = "".to_owned();
        for current in pair.into_inner() {
            match current.as_rule() {
                Rule::comment_token => token = Some(current.as_str().to_string()),
                Rule::doc_content => content = current.as_str().to_string(),
                _ => self.insert_unparsed_rule_and_exit(diagnostics, Self::parse_span(&current)),
            }
        }
        (token, content)
    }

    fn parse_model(&mut self, pair: Pair<'_>, source_id: usize, item_id: usize, diagnostics: &mut Diagnostics, ns_path: Vec<String>, id_path: Vec<usize>) -> Top {
        let mut comment_block = None;
        let mut identifier: Option<ASTIdentifier> = None;
        let mut fields: Vec<ASTField> = vec![];
        let mut decorators: Vec<ASTDecorator> = vec![];
        let span = Self::parse_span(&pair);
        for current in pair.into_inner() {
            match current.as_rule() {
                Rule::MODEL_KEYWORD | Rule::BLOCK_OPEN | Rule::BLOCK_CLOSE | Rule::EMPTY_LINES | Rule::double_comment_block | Rule::comment_block => {}
                Rule::identifier => identifier = Some(Self::parse_identifier(&current)),
                Rule::field_declaration => fields.push(self.parse_field(current, diagnostics, source_id)),
                Rule::item_decorator => decorators.push(self.parse_decorator(current, source_id, diagnostics)),
                Rule::triple_comment_block => comment_block = Some(self.parse_comment_block(current, source_id, diagnostics)),
                _ => self.insert_unparsed_rule_and_exit(diagnostics, Self::parse_span(&current)),
            }
        }
        Top::Model(ASTModel::new(
            item_id,
            source_id,
            id_path,
            identifier.unwrap(),
            ns_path,
            comment_block,
            fields,
            decorators,
            span,
        ))
    }

    fn parse_field(&self, pair: Pair<'_>, diagnostics: &mut Diagnostics, source_id: usize) -> ASTField {
        let mut comment_block = None;
        let mut identifier: Option<ASTIdentifier> = None;
        let mut r#type: Option<ASTFieldType> = None;
        let mut decorators: Vec<ASTDecorator> = vec![];
        let span = Self::parse_span(&pair);
        for current in pair.into_inner() {
            match current.as_rule() {
                Rule::COLON => {},
                Rule::triple_comment_block => comment_block = Some(self.parse_comment_block(current, source_id, diagnostics)),
                Rule::identifier => identifier = Some(Self::parse_identifier(&current)),
                Rule::field_type => r#type = Some(self.parse_field_type(current, diagnostics, source_id)),
                Rule::item_decorator => decorators.push(self.parse_decorator(current, source_id, diagnostics)),
                Rule::double_comment_block => {},
                _ => self.insert_unparsed_rule_and_exit(diagnostics, Self::parse_span(&current)),
            }
        }
        ASTField::new(
            source_id,
            comment_block,
            identifier.unwrap(),
            r#type.unwrap(),
            decorators,
            span,
        )
    }

    fn parse_enum(&mut self, pair: Pair<'_>, source_id: usize, item_id: usize, ns_path: Vec<String>, diagnostics: &mut Diagnostics) -> Top {
        let mut comment_block = None;
        let mut identifier: Option<ASTIdentifier> = None;
        let mut choices: Vec<EnumChoice> = vec![];
        let mut decorators: Vec<ASTDecorator> = vec![];
        let span = Self::parse_span(&pair);
        for current in pair.into_inner() {
            match current.as_rule() {
                Rule::ENUM_KEYWORD | Rule::COLON | Rule::EMPTY_LINES | Rule::BLOCK_OPEN | Rule::BLOCK_CLOSE => {},
                Rule::comment_block => comment_block = Some(self.parse_comment_block(current, source_id, diagnostics)),
                Rule::identifier => identifier = Some(Self::parse_identifier(&current)),
                Rule::enum_value_declaration => choices.push(self.parse_enum_value(current, source_id, diagnostics)),
                Rule::triple_comment_block => comment_block = Some(self.parse_comment_block(current, source_id, diagnostics)),
                _ => self.insert_unparsed_rule_and_exit(diagnostics, Self::parse_span(&current)),
            }
        }
        Top::Enum(ASTEnum::new(
            item_id,
            source_id,
            comment_block,
            identifier.unwrap(),
            ns_path,
            decorators,
            choices,
            span,
        ))
    }

    fn parse_enum_value(&mut self, pair: Pair<'_>, source_id: usize, diagnostics: &mut Diagnostics) -> EnumChoice {
        let mut comment_block = None;
        let mut identifier: Option<ASTIdentifier> = None;
        let mut decorators: Vec<ASTDecorator> = vec![];
        let span = Self::parse_span(&pair);
        for current in pair.into_inner() {
            match current.as_rule() {
                Rule::COLON | Rule::EMPTY_LINES | Rule::comment_block => {},
                Rule::identifier => identifier = Some(Self::parse_identifier(&current)),
                Rule::item_decorator => decorators.push(self.parse_decorator(current, source_id, diagnostics)),
                Rule::triple_comment_block => comment_block = Some(self.parse_comment_block(current, source_id, diagnostics)),
                _ => self.insert_unparsed_rule_and_exit(diagnostics, Self::parse_span(&current)),
            }
        }
        EnumChoice::new(identifier.unwrap(),comment_block,decorators, span)
    }

    fn parse_let_declaration(&mut self, pair: Pair<'_>, source_id: usize, item_id: usize, diagnostics: &mut Diagnostics) -> Top {
        let span = Self::parse_span(&pair);
        let mut identifier: Option<ASTIdentifier> = None;
        let mut expression: Option<Expression> = None;
        for current in pair.into_inner() {
            match current.as_rule() {
                Rule::identifier => identifier = Some(Self::parse_identifier(&current)),
                Rule::expression => expression = Some(self.parse_expression(current, source_id, diagnostics)),
                _ => self.insert_unparsed_rule_and_exit(diagnostics, Self::parse_span(&current)),
            }
        }
        Top::Constant(Constant::new(item_id, source_id, identifier.unwrap(), expression.unwrap(), span))
    }

    fn parse_dataset_block(&mut self, pair: Pair<'_>, source_id: usize, item_id: usize, diagnostics: &mut Diagnostics) -> Top {
        let mut identifier: Option<ASTIdentifier> = None;
        let mut auto_seed = false;
        let mut notrack = false;
        let mut groups = vec![];
        let span = Self::parse_span(&pair);
        for current in pair.into_inner() {
            match current.as_rule() {
                Rule::BLOCK_OPEN | Rule::BLOCK_CLOSE | Rule::EMPTY_LINES => (),
                Rule::AUTOSEED_KEYWORD => auto_seed = true,
                Rule::NOTRACK_KEYWORD => notrack = true,
                Rule::identifier => identifier = Some(Self::parse_identifier(&current)),
                Rule::dataset_group_declaration => {
                    let next_id = self.next_id();
                    groups.push(self.parse_dataset_group(current, source_id, next_id, diagnostics));
                },
                Rule::comment_block => (),
                _ => self.insert_unparsed_rule_and_exit(diagnostics, Self::parse_span(&current)),
            }
        }
        Top::DataSet(ASTDataSet::new(span, source_id, item_id, identifier.unwrap(), auto_seed, notrack, groups))
    }

    fn parse_dataset_group(&mut self, pair: Pair<'_>, source_id: usize, item_id: usize, diagnostics: &mut Diagnostics) -> DataSetGroup {
        let mut identifier: Option<ASTIdentifier> = None;
        let mut records = vec![];
        let span = Self::parse_span(&pair);
        for current in pair.into_inner() {
            match current.as_rule() {
                Rule::BLOCK_OPEN | Rule::BLOCK_CLOSE | Rule::EMPTY_LINES => (),
                Rule::identifier => identifier = Some(Self::parse_identifier(&current)),
                Rule::dataset_group_record_declaration => {
                    let next_id = self.next_id();
                    records.push(self.parse_dataset_record_declaration(current, source_id, next_id, diagnostics));
                },
                Rule::comment_block => (),
                _ => self.insert_unparsed_rule_and_exit(diagnostics, Self::parse_span(&current)),
            }
        }
        DataSetGroup::new(source_id, item_id, identifier.unwrap(), span, records)
    }

    fn parse_dataset_record_declaration(&mut self, pair: Pair<'_>, source_id: usize, item_id: usize, diagnostics: &mut Diagnostics) -> DataSetRecord {
        let mut identifier: Option<ASTIdentifier> = None;
        let mut dictionary: Option<DictionaryLiteral> = None;
        let span = Self::parse_span(&pair);
        for current in pair.into_inner() {
            match current.as_rule() {
                Rule::identifier => identifier = Some(Self::parse_identifier(&current)),
                Rule::dictionary_literal => dictionary = Some(self.parse_dictionary_literal(current, source_id, diagnostics)),
                _ => (),
            }
        }
        DataSetRecord::new(source_id, item_id, identifier.unwrap(), span, dictionary.unwrap())
    }

    fn parser_interface_type(&self, pair: Pair<'_>, source_id: usize, diagnostics: &mut Diagnostics) -> InterfaceType {
        let span = Self::parse_span(&pair);
        let mut name: Option<ASTIdentifier> = None;
        let mut args: Vec<InterfaceType> = vec![];
        let mut arity: Arity = Arity::Scalar;
        let mut collection_optionality: bool = false;
        let mut optionality: bool = false;
        for current in pair.into_inner() {
            match current.as_rule() {
                Rule::identifier => {
                    let identifier = Self::parse_identifier(&current);
                    name = Some(identifier);
                },
                Rule::interface_type => {
                    let identifier = self.parser_interface_type(current, source_id, diagnostics);
                    args.push(identifier);
                }
                Rule::arity => if current.as_str() == "[]" { arity = Arity::Array; } else { arity = Arity::Dictionary; },
                Rule::optionality => {
                    if arity == Arity::Scalar {
                        optionality = true;
                    } else {
                        collection_optionality = true;
                    }
                },
                _ => self.insert_unparsed_rule_and_exit(diagnostics, Self::parse_span(&current)),
            }
        }
        InterfaceType {
            name: name.unwrap(),
            args,
            span,
            arity,
            collection_optional: collection_optionality,
            optional: optionality,
        }
    }

    fn parse_namespace_declaration(&mut self, pair: Pair<'_>, path: &Path, source_id: usize, item_id: usize, parent_ids: Vec<usize>, ns_path: Vec<String>, diagnostics: &mut Diagnostics) -> ASTNamespace {
        let span = Self::parse_span(&pair);
        let mut name = None;
        let mut tops: BTreeMap<usize, Top> = btreemap![];
        let mut imports: BTreeSet<usize> = btreeset!{};
        let mut constants: BTreeSet<usize> = btreeset!{};
        let mut enums: BTreeSet<usize> = btreeset!{};
        let mut models: BTreeSet<usize> = btreeset!{};
        let mut namespaces: BTreeSet<usize> = btreeset!{};
        let mut data_sets: BTreeSet<usize> = btreeset!{};
        let mut content_parent_ids = parent_ids.clone();
        content_parent_ids.push(item_id);
        for current in pair.into_inner() {
            match current.as_rule() {
                Rule::identifier => {
                    let identifier = Self::parse_identifier(&current);
                    name = Some(identifier);
                },
                Rule::import_statement => {
                    let content_item_id = self.next_id();
                    let import = self.parse_import(current, source_id, content_item_id, path.to_owned(), diagnostics);
                    tops.insert(content_item_id, import);
                    imports.insert(content_item_id);
                },
                Rule::let_declaration => {
                    let content_item_id = self.next_id();
                    let constant = self.parse_let_declaration(current, source_id, content_item_id, diagnostics);
                    tops.insert(content_item_id, constant);
                    constants.insert(content_item_id);
                },
                Rule::model_declaration => {
                    let content_item_id = self.next_id();
                    let model = self.parse_model(current, source_id, content_item_id, diagnostics, {
                        let mut new_path = ns_path.clone();
                        new_path.push(name.clone().unwrap().name.clone());
                        new_path
                    }, {
                        let mut ids = content_parent_ids.clone();
                        ids.push(content_item_id);
                        ids
                    });
                    tops.insert(content_item_id, model);
                    models.insert(content_item_id);
                    self.models.push(vec_join(source_id, &content_parent_ids, content_item_id));
                },
                Rule::enum_declaration => {
                    let content_item_id = self.next_id();
                    let r#enum = self.parse_enum(current, source_id, content_item_id, {
                        let mut new_path = ns_path.clone();
                        new_path.push(name.clone().unwrap().name.clone());
                        new_path
                    }, diagnostics);
                    tops.insert(content_item_id, r#enum);
                    enums.insert(content_item_id);
                    self.enums.push(vec_join(source_id, &content_parent_ids, content_item_id));
                },
                Rule::config_declaration => {
                    let content_item_id = self.next_id();
                    let config_block = self.parse_config_block(current, source_id, content_item_id, diagnostics);
                    tops.insert(content_item_id, config_block);
                },
                Rule::dataset_declaration => {
                    let content_item_id = self.next_id();
                    let dataset_block = self.parse_dataset_block(current, source_id, content_item_id, diagnostics);
                    tops.insert(content_item_id, dataset_block);
                    data_sets.insert(content_item_id);
                    self.data_sets.push(vec_join(source_id, &content_parent_ids, content_item_id));
                }
                Rule::EOI | Rule::EMPTY_LINES => {},
                Rule::comment_block => (),
                Rule::interface_declaration => {
                    let content_item_id = self.next_id();
                    let interface_declaration = self.parse_interface_declaration(current, source_id, content_item_id, diagnostics);
                    tops.insert(content_item_id, interface_declaration);
                    self.interfaces.push(vec_join(source_id, &content_parent_ids, content_item_id));
                },
                Rule::action_group_declaration => {
                    let content_item_id = self.next_id();
                    let action_group_declaration = self.parse_action_group_declaration(current, source_id, content_item_id, diagnostics);
                    tops.insert(content_item_id, action_group_declaration);
                    self.action_groups.push(vec_join(source_id, &content_parent_ids, content_item_id));
                },
                Rule::middleware_declaration => {
                    let content_item_id = self.next_id();
                    let middleware_declaration = self.parse_middleware_declaration(current, source_id, content_item_id, diagnostics);
                    tops.insert(content_item_id, middleware_declaration);
                    self.middlewares.push(vec_join(source_id, &content_parent_ids, content_item_id));
                },
                Rule::static_files_declaration => {
                    let content_item_id = self.next_id();
                    let static_files_declaration = self.parse_static_files_declaration(current, source_id, content_item_id, diagnostics);
                    tops.insert(content_item_id, static_files_declaration);
                    self.static_files.push(vec_join(source_id, &content_parent_ids, content_item_id));
                },
                Rule::interface_enum_declaration => (),
                Rule::namespace => {
                    let content_item_id = self.next_id();
                    let mut ns_content_parent_ids = parent_ids.clone();
                    ns_content_parent_ids.push(item_id);
                    let namespace_declaration = self.parse_namespace_declaration(current, path, source_id, content_item_id, ns_content_parent_ids, {
                        let mut new_path = ns_path.clone();
                        new_path.push(name.clone().unwrap().name.clone());
                        new_path
                    }, diagnostics);
                    tops.insert(content_item_id, Top::ASTNamespace(namespace_declaration));
                    namespaces.insert(content_item_id);
                    self.namespaces.push(vec_join(source_id, &content_parent_ids, content_item_id));
                },
                Rule::model_decorator_declaration => (),
                Rule::field_decorator_declaration => (),
                Rule::relation_decorator_declaration => (),
                Rule::property_decorator_declaration => (),
                Rule::pipeline_item_declaration => (),
                Rule::NAMESPACE_KEYWORD | Rule::BLOCK_OPEN | Rule::BLOCK_CLOSE => (),
                Rule::BLOCK_LEVEL_CATCH_ALL => self.insert_unparsed_rule_and_exit(diagnostics, Self::parse_span(&current)),
                _ => self.insert_unparsed_rule_and_exit(diagnostics, Self::parse_span(&current)),
            }
        }

        let result = ASTNamespace::new(source_id, parent_ids, item_id, span, name.clone().unwrap().name.clone(), tops, imports, constants, enums, models, namespaces, data_sets);
        for import in result.borrow().imports() {
            let found = self.sources.values().find(|v| {
                (*v).borrow().path == import.path
            });
            if found.is_none() {
                self.parse_source(&import.path, diagnostics);
            }
        }
        result
    }

    fn parse_static_files_declaration(&mut self, pair: Pair<'_>, source_id: usize, item_id: usize, diagnostics: &mut Diagnostics) -> Top {
        let span = Self::parse_span(&pair);
        let mut map_expr = None;
        let mut path_expr = None;
        for current in pair.into_inner() {
            match current.as_rule() {
                Rule::expression => if path_expr.is_none() {
                    path_expr = Some(self.parse_expression(current, source_id, diagnostics));
                } else {
                    map_expr = Some(self.parse_expression(current, source_id, diagnostics));
                }
                Rule::STATIC_FILES_KEYWORD | Rule::FAT_ARROW_KEYWORD => (),
                _ => self.insert_unparsed_rule_and_exit(diagnostics, Self::parse_span(&current)),
            }
        }
        Top::StaticFiles(StaticFiles::new(source_id, item_id, span, path_expr.unwrap(), map_expr.unwrap()))
    }

    fn parse_middleware_declaration(&mut self, pair: Pair<'_>, source_id: usize, item_id: usize, diagnostics: &mut Diagnostics) -> Top {
        let mut name: Option<ASTIdentifier> = None;
        let span = Self::parse_span(&pair);
        for current in pair.into_inner() {
            match current.as_rule() {
                Rule::identifier => name = Some(Self::parse_identifier(&current)),
                Rule::argument_list_declaration => (),
                _ => self.insert_unparsed_rule_and_exit(diagnostics, Self::parse_span(&current)),
            }
        }
        Top::MiddlewareDeclaration(MiddlewareDeclaration {
            id: item_id,
            source_id,
            identifier: name.unwrap(),
            span,
        })
    }

    fn parse_action_group_declaration(&mut self, pair: Pair<'_>, source_id: usize, item_id: usize, diagnostics: &mut Diagnostics) -> Top {
        let mut name: Option<ASTIdentifier> = None;
        let span = Self::parse_span(&pair);
        let mut actions: Vec<ActionDeclaration> = vec![];
        for current in pair.into_inner() {
            match current.as_rule() {
                Rule::identifier => name = Some(Self::parse_identifier(&current)),
                Rule::comment_block => (),
                Rule::action_declaration => {
                    let action_id = self.next_id();
                    actions.push(self.parse_action_declaration(current, source_id, action_id, item_id, diagnostics));
                },
                Rule::EMPTY_LINES | Rule::BLOCK_OPEN | Rule::BLOCK_CLOSE => (),
                _ => self.insert_unparsed_rule_and_exit(diagnostics, Self::parse_span(&current)),
            }
        }
        Top::ActionGroupDeclaration(ActionGroupDeclaration {
            id: item_id,
            source_id,
            identifier: name.unwrap(),
            actions,
            span,
        })
    }

    fn parse_action_declaration(&mut self, pair: Pair<'_>, source_id: usize, item_id: usize, group_id: usize, diagnostics: &mut Diagnostics) -> ActionDeclaration {
        let mut identifier: Option<ASTIdentifier> = None;
        let mut input_type: Option<InterfaceType> = None;
        let mut output_type: Option<InterfaceType> = None;
        let mut input_format: ActionInputFormat = ActionInputFormat::Json;
        let span = Self::parse_span(&pair);
        for current in pair.into_inner() {
            match current.as_rule() {
                Rule::identifier => identifier = Some(Self::parse_identifier(&current)),
                Rule::interface_type => if input_type.is_some() {
                    output_type = Some(self.parser_interface_type(current, source_id, diagnostics));
                } else {
                    input_type = Some(self.parser_interface_type(current, source_id, diagnostics));
                },
                Rule::COLON => (),
                Rule::req_type => if current.as_str() == "form" {
                    input_format = ActionInputFormat::Form
                },
                _ => self.insert_unparsed_rule_and_exit(diagnostics, Self::parse_span(&current)),
            }
        }
        ActionDeclaration {
            id: item_id,
            source_id,
            group_id,
            identifier: identifier.unwrap(),
            input_type: input_type.unwrap(),
            output_type: output_type.unwrap(),
            input_format,
            span,
            resolved_input_interface: None,
            resolved_input_shape: None,
        }
    }

    fn parse_interface_declaration(&mut self, pair: Pair<'_>, source_id: usize, item_id: usize, diagnostics: &mut Diagnostics) -> Top {
        let mut name: Option<InterfaceType> = None;
        let mut extends: Vec<InterfaceType> = vec![];
        let mut items: Vec<InterfaceItemDeclaration> = vec![];
        let span = Self::parse_span(&pair);
        for current in pair.into_inner() {
            match current.as_rule() {
                Rule::interface_type => {
                    let interface_type = self.parser_interface_type(current, source_id, diagnostics);
                    if name.is_some() {
                        extends.push(interface_type);
                    } else {
                        name = Some(interface_type);
                    }
                }
                Rule::interface_item => {
                    let interface_item_decl = self.parse_interface_item_declaration(current, source_id, diagnostics);
                    items.push(interface_item_decl);
                }
                _ => (),
            }
        }
        Top::InterfaceDeclaration(InterfaceDeclaration {
            id: item_id,
            source_id,
            name: name.unwrap(),
            extends,
            items,
            span,
        })
    }

    fn parse_interface_item_declaration(&mut self, pair: Pair<'_>, source_id: usize, diagnostics: &mut Diagnostics) -> InterfaceItemDeclaration {
        let mut name: Option<ASTIdentifier> = None;
        let mut kind: Option<InterfaceType> = None;
        let span = Self::parse_span(&pair);
        for current in pair.into_inner() {
            match current.as_rule() {
                Rule::identifier => name = Some(Self::parse_identifier(&current)),
                Rule::interface_type => kind = Some(self.parser_interface_type(current, source_id, diagnostics)),
                _ => (),
            }
        }
        InterfaceItemDeclaration { name: name.unwrap(), kind: kind.unwrap(), span }
    }

    fn parse_config_block(&mut self, pair: Pair<'_>, source_id: usize, item_id: usize, diagnostics: &mut Diagnostics) -> Top {
        let mut identifier: Option<ASTIdentifier> = None;
        let mut items: Vec<Item> = vec![];
        let mut keyword = "";
        let mut keyword_span = None;
        let span = Self::parse_span(&pair);
        for current in pair.into_inner() {
            match current.as_rule() {
                Rule::BLOCK_OPEN | Rule::BLOCK_CLOSE | Rule::EMPTY_LINES => (),
                Rule::config_keywords => {
                    keyword = current.as_str();
                    keyword_span = Some(Self::parse_span(&current));
                },
                Rule::identifier => identifier = Some(Self::parse_identifier(&current)),
                Rule::config_item => items.push(self.parse_config_item(current, source_id, diagnostics)),
                Rule::comment_block => (),
                Rule::BLOCK_LEVEL_CATCH_ALL => self.insert_unparsed_rule_and_exit(diagnostics, Self::parse_span(&current)),
                _ => self.insert_unparsed_rule_and_exit(diagnostics, Self::parse_span(&current)),
            }
        }
        match keyword {
            "server" => {
                if self.server.is_some() {
                    self.insert_diagnostics_error_and_exit("Duplicated configuration found.", diagnostics, keyword_span.unwrap());
                }
                self.server = Some(vec![source_id, item_id]);
                Top::ServerConfig(ASTServer::new(item_id, source_id, items, span))
            },
            "connector" => {
                if self.connector.is_some() {
                    self.insert_diagnostics_error_and_exit("Duplicated configuration found.", diagnostics, keyword_span.unwrap());
                }
                self.connector = Some(vec![source_id, item_id]);
                Top::Connector(ASTConnector::new(items, span, source_id, item_id))
            },
            "entity" => {
                self.entities.push(vec![source_id, item_id]);
                Top::Generator(ASTEntity::new(item_id, source_id, identifier, items, span))
            },
            "client" => {
                self.clients.push(vec![source_id, item_id]);
                Top::Client(ASTClient::new(item_id, source_id, identifier, items, span))
            },
            "debug" => {
                self.debug_conf = Some(vec![source_id, item_id]);
                Top::DebugConf(ASTDebugConf::new(items, span, source_id, item_id))
            },
            "test" => {
                self.test_conf = Some(vec![source_id, item_id]);
                Top::TestConf(ASTTestConf::new(items, span, source_id, item_id))
            },
            _ => {
                self.insert_diagnostics_error_and_exit(format!("Undefined configuration '{}'.", keyword), diagnostics, keyword_span.unwrap());
                panic!()
            }
        }
    }

    fn parse_config_item(&self, pair: Pair<'_>, source_id: usize, diagnostics: &mut Diagnostics) -> Item {
        let span = Self::parse_span(&pair);
        let mut identifier: Option<ASTIdentifier> = None;
        let mut expression: Option<Expression> = None;
        for current in pair.into_inner() {
            match current.as_rule() {
                Rule::identifier => identifier = Some(Self::parse_identifier(&current)),
                Rule::expression => expression = Some(self.parse_expression(current, source_id, diagnostics)),
                _ => self.insert_unparsed_rule_and_exit(diagnostics, Self::parse_span(&current)),
            }
        }
        Item { identifier: identifier.unwrap(), expression: expression.unwrap(), span }
    }

    fn parse_decorator(&self, pair: Pair<'_>, source_id: usize, diagnostics: &mut Diagnostics) -> ASTDecorator {
        let span = Self::parse_span(&pair);
        let mut unit: Option<ExpressionKind> = None;
        for current in pair.into_inner() {
            match current.as_rule() {
                Rule::identifier_unit => unit = Some(self.parse_unit(current, source_id, diagnostics)),
                _ => self.insert_unparsed_rule_and_exit(diagnostics, Self::parse_span(&current)),
            }
        }
        ASTDecorator::new(unit.unwrap(), span)
    }

    fn parse_pipeline(&self, pair: Pair<'_>, source_id: usize, diagnostics: &mut Diagnostics) -> ASTPipeline {
        let span = Self::parse_span(&pair);
        let mut unit: Option<ExpressionKind> = None;
        for current in pair.into_inner() {
            match current.as_rule() {
                Rule::identifier_unit => unit = Some(self.parse_unit(current, source_id, diagnostics)),
                _ => self.insert_unparsed_rule_and_exit(diagnostics, Self::parse_span(&current)),
            }
        }
        ASTPipeline {
            expression: Box::new(unit.unwrap()),
            span,
        }
    }

    fn parse_argument(&self, pair: Pair<'_>, source_id: usize, diagnostics: &mut Diagnostics) -> Argument {
        let span = Self::parse_span(&pair);
        let name: Option<ASTIdentifier> = None;
        let mut value: Option<ExpressionKind> = None;
        for current in pair.into_inner() {
            match current.as_rule() {
                Rule::named_argument => {
                    return self.parse_named_argument(current, source_id, diagnostics);
                },
                Rule::expression => value = Some(self.parse_expression(current, source_id, diagnostics).kind),
                Rule::empty_argument => panic!("Empty argument found."),
                _ => self.insert_unparsed_rule_and_exit(diagnostics, Self::parse_span(&current)),
            }
        }
        Argument { name, value: value.unwrap(), span, resolved: None }
    }

    fn parse_named_argument(&self, pair: Pair<'_>, source_id: usize, diagnostics: &mut Diagnostics) -> Argument {
        let span = Self::parse_span(&pair);
        let mut name: Option<ASTIdentifier> = None;
        let mut value: Option<ExpressionKind> = None;
        for current in pair.into_inner() {
            match current.as_rule() {
                Rule::identifier => name = Some(Self::parse_identifier(&current)),
                Rule::expression => value = Some(self.parse_expression(current, source_id, diagnostics).kind),
                Rule::empty_argument => panic!("Empty argument found."),
                _ => self.insert_unparsed_rule_and_exit(diagnostics, Self::parse_span(&current)),
            }
        }
        Argument { name, value: value.unwrap(), span, resolved: None }
    }

    fn parse_expression(&self, pair: Pair<'_>, source_id: usize, diagnostics: &mut Diagnostics) -> Expression {
        for current in pair.into_inner() {
            match current.as_rule() {
                Rule::nullish_coalescing => return Expression::new(ExpressionKind::NullishCoalescing(self.parse_nullish_coalescing(current, source_id, diagnostics))),
                Rule::negation => return Expression::new(ExpressionKind::Negation(self.parse_negation(current, source_id, diagnostics))),
                Rule::bitwise_negation => return Expression::new(ExpressionKind::BitwiseNegation(self.parse_bitwise_negation(current, source_id, diagnostics))),
                Rule::arith_expr => return Expression::new(ExpressionKind::ArithExpr(self.parse_arith_expr(current, source_id, diagnostics))),
                Rule::unit => return Expression::new(self.parse_unit(current, source_id, diagnostics)),
                Rule::pipeline => return Expression::new(ExpressionKind::Pipeline(self.parse_pipeline(current, source_id, diagnostics))),
                _ => self.insert_unparsed_rule_and_exit(diagnostics, Self::parse_span(&current)),
            }
        }
        panic!();
    }

    fn parse_unit(&self, pair: Pair<'_>, source_id: usize, diagnostics: &mut Diagnostics) -> ExpressionKind {
        let span = Self::parse_span(&pair);
        let mut unit = Unit { expressions: vec![], span };
        for current in pair.into_inner() {
            match current.as_rule() {
                Rule::group => unit.expressions.push(ExpressionKind::Group(self.parse_group(current, source_id, diagnostics))),
                Rule::null_literal => unit.expressions.push(ExpressionKind::NullLiteral(NullLiteral { value: current.as_str().to_string(), span })),
                Rule::bool_literal => unit.expressions.push(ExpressionKind::BoolLiteral(BoolLiteral { value: current.as_str().to_string(), span })),
                Rule::numeric_literal => unit.expressions.push(ExpressionKind::NumericLiteral(NumericLiteral { value: current.as_str().to_string(), span })),
                Rule::string_literal => unit.expressions.push(ExpressionKind::StringLiteral(StringLiteral { value: current.as_str().to_string(), span })),
                Rule::regexp_literal => unit.expressions.push(ExpressionKind::RegExpLiteral(self.parse_regexp_literal(current, source_id, diagnostics))),
                Rule::enum_choice_literal => unit.expressions.push(ExpressionKind::EnumChoiceLiteral(self.parse_enum_choice_literal(current, source_id, diagnostics))),
                Rule::tuple_literal => unit.expressions.push(ExpressionKind::TupleLiteral(self.parse_tuple_literal(current, source_id, diagnostics))),
                Rule::array_literal => unit.expressions.push(ExpressionKind::ArrayLiteral(self.parse_array_literal(current, source_id, diagnostics))),
                Rule::dictionary_literal => unit.expressions.push(ExpressionKind::DictionaryLiteral(self.parse_dictionary_literal(current, source_id, diagnostics))),
                Rule::range_literal => unit.expressions.push(ExpressionKind::RangeLiteral(self.parse_range_literal(current, source_id, diagnostics))),
                Rule::identifier => unit.expressions.push(ExpressionKind::Identifier(Self::parse_identifier(&current))),
                Rule::subscript => unit.expressions.push(ExpressionKind::Subscript(self.parse_subscript(current, source_id, diagnostics))),
                Rule::argument_list => unit.expressions.push(ExpressionKind::ArgumentList(self.parse_argument_list(current, source_id, diagnostics))),
                _ => self.insert_unparsed_rule_and_exit(diagnostics, Self::parse_span(&current)),
            }
        }
        if unit.expressions.len() == 1 {
            return unit.expressions.get(0).unwrap().clone()
        } else {
            return ExpressionKind::Unit(unit);
        }
    }

    fn parse_enum_choice_literal(&self, pair: Pair<'_>, source_id: usize, diagnostics: &mut Diagnostics) -> EnumChoiceLiteral {
        let span = Self::parse_span(&pair);
        let mut arg_list: Option<ArgumentList> = None;
        let mut value: Option<String> = None;
        for current in pair.into_inner() {
            match current.as_rule() {
                Rule::identifier => value = Some(current.as_str().to_owned()),
                Rule::argument_list => arg_list = Some(self.parse_argument_list(current, source_id, diagnostics)),
                _ => self.insert_unparsed_rule_and_exit(diagnostics, Self::parse_span(&current)),
            }
        }
        EnumChoiceLiteral { value: value.unwrap(), span, argument_list: arg_list }
    }


    fn parse_nullish_coalescing(&self, pair: Pair<'_>, source_id: usize, diagnostics: &mut Diagnostics) -> NullishCoalescing {
        let span = Self::parse_span(&pair);
        let mut expressions = vec![];
        for current in pair.into_inner() {
            match current.as_rule() {
                Rule::unit => expressions.push(self.parse_unit(current, source_id, diagnostics)),
                _ => self.insert_unparsed_rule_and_exit(diagnostics, Self::parse_span(&current)),
            }
        }
        NullishCoalescing { expressions, span }
    }

    fn parse_negation(&self, pair: Pair<'_>, source_id: usize, diagnostics: &mut Diagnostics) -> Negation {
        let span = Self::parse_span(&pair);
        let mut expression = None;
        for current in pair.into_inner() {
            match current.as_rule() {
                Rule::unit => expression = Some(self.parse_unit(current, source_id, diagnostics)),
                _ => self.insert_unparsed_rule_and_exit(diagnostics, Self::parse_span(&current)),
            }
        }
        Negation { expression: Box::new(expression.unwrap()), span }
    }

    fn parse_bitwise_negation(&self, pair: Pair<'_>, source_id: usize, diagnostics: &mut Diagnostics) -> BitwiseNegation {
        let span = Self::parse_span(&pair);
        let mut expression = None;
        for current in pair.into_inner() {
            match current.as_rule() {
                Rule::unit => expression = Some(self.parse_unit(current, source_id, diagnostics)),
                _ => self.insert_unparsed_rule_and_exit(diagnostics, Self::parse_span(&current)),
            }
        }
        BitwiseNegation { expression: Box::new(expression.unwrap()), span }
    }

    fn parse_arith_expr(&self, pair: Pair<'_>, source_id: usize, diagnostics: &mut Diagnostics) -> ArithExpr {
        let result = PRATT_PARSER.map_primary(|primary| match primary.as_rule() {
            Rule::operand => ArithExpr::Expression(Box::new(self.parse_expression(primary, source_id, diagnostics).kind)),
            _ => {
                let error_span = Self::parse_span(&primary);
                self.insert_diagnostics_error_and_exit(format!("Unexpected operand or operator"), diagnostics, error_span);
                panic!()
            },
        }).map_infix(|lhs, op, rhs| {
            let ourop = match op.as_rule() {
                Rule::ADD => Op::Add,
                Rule::SUB => Op::Sub,
                Rule::MUL => Op::Mul,
                Rule::DIV => Op::Div,
                Rule::MOD => Op::Mod,
                Rule::BI_AND => Op::BitAnd,
                Rule::BI_XOR => Op::BitXor,
                Rule::BI_OR => Op::BitOr,
                _ => {
                    panic!()
                },
            };
            ArithExpr::BinaryOp {
                lhs: Box::new(lhs),
                op: ourop,
                rhs: Box::new(rhs),
            }
        }).parse(pair.into_inner());
        result
    }

    fn parse_subscript(&self, pair: Pair<'_>, source_id: usize, diagnostics: &mut Diagnostics) -> Subscript {
        let span = Self::parse_span(&pair);
        for current in pair.into_inner() {
            match current.as_rule() {
                Rule::expression => return Subscript { expression: Box::new(self.parse_expression(current, source_id, diagnostics).kind), span },
                _ => self.insert_unparsed_rule_and_exit(diagnostics, Self::parse_span(&current)),
            }
        }
        panic!()
    }

    fn parse_regexp_literal(&self, pair: Pair<'_>, source_id: usize, diagnostics: &mut Diagnostics) -> RegExpLiteral {
        let span = Self::parse_span(&pair);
        for current in pair.into_inner() {
            match current.as_rule() {
                Rule::regexp_content => return RegExpLiteral { value: current.as_str().to_string(), span },
                _ => self.insert_unparsed_rule_and_exit(diagnostics, Self::parse_span(&current)),
            }
        }
        panic!()
    }

    fn parse_argument_list(&self, pair: Pair<'_>, source_id: usize, diagnostics: &mut Diagnostics) -> ArgumentList {
        let span = Self::parse_span(&pair);
        let mut arguments: Vec<Argument> = vec![];
        for current in pair.into_inner() {
            match current.as_rule() {
                Rule::argument => arguments.push(self.parse_argument(current, source_id, diagnostics)),
                _ => self.insert_unparsed_rule_and_exit(diagnostics, Self::parse_span(&current)),
            }
        }
        ArgumentList { arguments, span, resolved: false }
    }

    fn parse_group(&self, pair: Pair<'_>, source_id: usize, diagnostics: &mut Diagnostics) -> Group {
        let span = Self::parse_span(&pair);
        for current in pair.into_inner() {
            match current.as_rule() {
                Rule::expression => return Group { expression: Box::new(self.parse_expression(current, source_id, diagnostics).kind), span },
                _ => self.insert_unparsed_rule_and_exit(diagnostics, Self::parse_span(&current)),
            }
        }
        panic!()
    }

    fn parse_range_literal(&self, pair: Pair<'_>, source_id: usize, diagnostics: &mut Diagnostics) -> RangeLiteral {
        let span = Self::parse_span(&pair);
        let mut expressions: Vec<ExpressionKind> = vec![];
        let mut closed = false;
        for current in pair.into_inner() {
            match current.as_rule() {
                Rule::range_end => expressions.push(self.parse_range_end(current, source_id, diagnostics)),
                Rule::RANGE_OPEN => closed = false,
                Rule::RANGE_CLOSE => closed = true,
                _ => self.insert_unparsed_rule_and_exit(diagnostics, Self::parse_span(&current)),
            }
        }
        RangeLiteral { closed, expressions, span }
    }

    fn parse_range_end(&self, pair: Pair<'_>, source_id: usize, diagnostics: &mut Diagnostics) -> ExpressionKind {
        let span = Self::parse_span(&pair);
        for current in pair.into_inner() {
            match current.as_rule() {
                Rule::numeric_literal => return ExpressionKind::NumericLiteral(NumericLiteral { value: current.as_str().to_string(), span }),
                Rule::unit_without_range_literal => return self.parse_unit(current, source_id, diagnostics),
                _ => self.insert_unparsed_rule_and_exit(diagnostics, Self::parse_span(&current)),
            }
        }
        panic!()
    }

    fn parse_tuple_literal(&self, pair: Pair<'_>, source_id: usize, diagnostics: &mut Diagnostics) -> TupleLiteral {
        let span = Self::parse_span(&pair);
        let mut expressions: Vec<ExpressionKind> = vec![];
        for current in pair.into_inner() {
            match current.as_rule() {
                Rule::expression => expressions.push(self.parse_expression(current, source_id, diagnostics).kind),
                _ => self.insert_unparsed_rule_and_exit(diagnostics, Self::parse_span(&current)),
            }
        }
        TupleLiteral { expressions, span }
    }

    fn parse_array_literal(&self, pair: Pair<'_>, source_id: usize, diagnostics: &mut Diagnostics) -> ArrayLiteral {
        let span = Self::parse_span(&pair);
        let mut expressions: Vec<ExpressionKind> = vec![];
        for current in pair.into_inner() {
            match current.as_rule() {
                Rule::expression => expressions.push(self.parse_expression(current, source_id, diagnostics).kind),
                _ => self.insert_unparsed_rule_and_exit(diagnostics, Self::parse_span(&current)),
            }
        }
        ArrayLiteral { expressions, span }
    }

    fn parse_dictionary_literal(&self, pair: Pair<'_>, source_id: usize, diagnostics: &mut Diagnostics) -> DictionaryLiteral {
        let span = Self::parse_span(&pair);
        let mut expressions: Vec<(ExpressionKind, ExpressionKind)> = vec![];
        for current in pair.into_inner() {
            match current.as_rule() {
                Rule::named_expression => expressions.push(self.parse_named_expression(current, source_id, diagnostics)),
                Rule::BLOCK_OPEN | Rule::BLOCK_CLOSE => (),
                _ => self.insert_unparsed_rule_and_exit(diagnostics, Self::parse_span(&current)),
            }
        }
        DictionaryLiteral { expressions, span }
    }

    fn parse_named_expression(&self, pair: Pair<'_>, source_id: usize, diagnostics: &mut Diagnostics) -> (ExpressionKind, ExpressionKind) {
        let mut key = None;
        let mut value = None;
        for current in pair.into_inner() {
            match current.as_rule() {
                Rule::expression => if key.is_none() {
                    key = Some(self.parse_expression(current, source_id, diagnostics).kind);
                } else {
                    value = Some(self.parse_expression(current, source_id, diagnostics).kind);
                }
                _ => self.insert_unparsed_rule_and_exit(diagnostics, Self::parse_span(&current)),
            }
        }
        return (key.unwrap(), value.unwrap())
    }

    fn parse_field_type(&self, pair: Pair<'_>, diagnostics: &mut Diagnostics, source_id: usize) -> ASTFieldType {
        let mut identifiers = None;
        let mut arity = Arity::Scalar;
        let mut item_required = true;
        let mut collection_required = true;
        let span = Self::parse_span(&pair);
        for current in pair.into_inner() {
            match current.as_rule() {
                Rule::COLON => {},
                Rule::identifier_path => identifiers = Some(self.parse_identifier_path(current, diagnostics)),
                Rule::arity => if current.as_str() == "[]" { arity = Arity::Array; } else { arity = Arity::Dictionary; },
                Rule::optionality => {
                    if arity == Arity::Scalar {
                        item_required = false;
                    } else {
                        collection_required = false;
                    }
                },
                _ => self.insert_unparsed_rule_and_exit(diagnostics, Self::parse_span(&current)),
            }
        }
        ASTFieldType::new(
            span,
            identifiers.unwrap(),
            arity,
            item_required,
            collection_required,
        )
    }

    fn parse_import_identifier_list(&self, pair: Pair<'_>, source_id: usize, diagnostics: &mut Diagnostics) -> Vec<ASTIdentifier> {
        let mut identifiers = vec![];
        for current in pair.into_inner() {
            match current.as_rule() {
                Rule::identifier => identifiers.push(Self::parse_identifier(&current)),
                Rule::TRAILING_COMMA | Rule::BLOCK_CLOSE => (),
                _ => self.insert_unparsed_rule_and_exit(diagnostics, Self::parse_span(&current)),
            }
        }
        identifiers
    }

    fn parse_identifier(pair: &Pair<'_>) -> ASTIdentifier {
        ASTIdentifier {
            name: pair.as_str().to_owned(),
            span: Self::parse_span(pair),
        }
    }

    fn parse_identifier_path(&self, pair: Pair<'_>, diagnostics: &mut Diagnostics) -> ASTIdentifierPath {
        let span = Self::parse_span(&pair);
        let mut identifiers = vec![];
        for current in pair.into_inner() {
            match current.as_rule() {
                Rule::identifier => identifiers.push(Self::parse_identifier(&current)),
                _ => self.insert_unparsed_rule_and_exit(diagnostics, Self::parse_span(&current)),
            }
        }
        ASTIdentifierPath {
            span,
            identifiers,
        }
    }

    fn parse_span(pair: &Pair<'_>) -> Span {
        let start_line_col = pair.line_col();
        let span = pair.as_span();
        let end_line_col = span.end_pos().line_col();
        Span {
            start: span.start(),
            end: span.end(),
            start_position: start_line_col,
            end_position: end_line_col,
        }
    }

    pub(crate) fn get_source(&self, id: usize) -> &ASTSource {
        self.sources.get(&id).unwrap()
    }

    pub(crate) fn connector(&self) -> Result<&ASTConnector> {
        match &self.connector {
            Some(connector) => {
                let source = self.get_source(*connector.get(0).unwrap());
                Ok(source.get_connector(*connector.get(1).unwrap()))
            }
            None => Err(crate::core::error::Error::fatal("Parser's connector is accessed while it's not set.")),
        }
    }

    pub(crate) fn server(&self) -> Result<&ASTServer> {
        match &self.server {
            Some(server) => {
                let source = self.get_source(*server.get(0).unwrap());
                Ok(source.get_server(*server.get(1).unwrap()))
            }
            None => Err(crate::core::error::Error::fatal("Parser's server is accessed while it's not set.")),
        }
    }

    pub(crate) fn debug(&self) -> Option<&ASTDebugConf> {
        self.debug_conf.as_ref().map(|debug| {
            let source = self.get_source(*debug.get(0).unwrap());
            source.get_debug_conf(*debug.get(1).unwrap())
        })
    }

    pub(crate) fn test(&self) -> Option<&ASTTestConf> {
        self.test_conf.as_ref().map(|test| {
            let source = self.get_source(*test.get(0).unwrap());
            source.get_test_conf(*test.get(1).unwrap())
        })
    }

    pub(crate) fn entities(&self) -> Vec<&ASTEntity> {
        self.entities.iter().map(|g| {
            let source = self.get_source(*g.get(0).unwrap());
            source.get_entity(*g.get(1).unwrap())
        }).collect()
    }

    pub(crate) fn clients(&self) -> Vec<&ASTClient> {
        self.clients.iter().map(|g| {
            let source = self.get_source(*g.get(0).unwrap());
            source.get_client(*g.get(1).unwrap())
        }).collect()
    }

    pub(crate) fn data_sets(&self) -> Vec<&ASTDataSet> {
        self.data_sets.iter().map(|d| {
            let source = self.get_source(*d.get(0).unwrap());
            source.get_data_set(*d.get(1).unwrap())
        }).collect()
    }

    pub(crate) fn enums(&self) -> Vec<&ASTEnum> {
        self.enums.iter().map(|e| {
            let source = self.get_source(*e.get(0).unwrap());
            source.get_enum(*e.get(1).unwrap())
        }).collect()
    }

    pub(crate) fn models(&self) -> Vec<&ASTModel> {
        self.models.iter().map(|m| {
            if m.len() == 2 {
                let source = self.get_source(*m.get(0).unwrap());
                source.get_model(*m.get(1).unwrap())
            } else {
                let namespace_path = m.as_slice()[..m.len() - 1].to_vec();
                let namespace = self.namespace(namespace_path);
                namespace.get_model(*m.last().unwrap())
            }
        }).collect()
    }

    pub(crate) fn namespace(&self, path: Vec<usize>) -> &ASTNamespace {
        let source = self.get_source(*path.get(0).unwrap());
        let mut ns = source.get_namespace(*path.get(1).unwrap());
        if path.len() > 2 {
            for ns_id in path.as_slice()[2..].iter() {
                ns = ns.get_namespace(*ns_id);
            }
        }
        ns
    }

    pub(crate) fn middlewares(&self) -> Vec<&MiddlewareDeclaration> {
        self.middlewares.iter().map(|m| {
            let source = self.get_source(*m.get(0).unwrap());
            source.get_middleware(*m.get(1).unwrap())
        }).collect()
    }

    pub(crate) fn action_groups(&self) -> Vec<&ActionGroupDeclaration> {
        self.action_groups.iter().map(|m| {
            let source = self.get_source(*m.get(0).unwrap());
            source.get_action_group(*m.get(1).unwrap())
        }).collect()
    }

    pub(crate) fn interfaces(&self) -> Vec<&InterfaceDeclaration> {
        self.interfaces.iter().map(|m| {
            let source = self.get_source(*m.get(0).unwrap());
            source.get_interface(*m.get(1).unwrap())
        }).collect()
    }

    pub(crate) fn static_files(&self) -> Vec<&StaticFiles> {
        self.static_files.iter().map(|m| {
            let source = self.get_source(*m.get(0).unwrap());
            source.get_static_files(*m.get(1).unwrap())
        }).collect()
    }

    fn insert_unparsed_rule_and_exit(&self, diagnostics: &mut Diagnostics, span: Span) {
        let source_path = self.current_source_path_bufs.last().unwrap().clone();
        diagnostics.insert_unparsed_rule(span, source_path);
        printer::print_diagnostics(diagnostics, true);
        std::process::exit(1);
    }

    fn insert_diagnostics_error_and_exit(&self, message: impl Into<String>, diagnostics: &mut Diagnostics, span: Span) {
        let source_path = self.current_source_path_bufs.last().unwrap().clone();
        diagnostics.insert(DiagnosticsError::new(span, message.into(), source_path));
        printer::print_diagnostics(diagnostics, true);
        std::process::exit(1);
    }
}

fn vec_join(first: usize, vec: &Vec<usize>, last: usize) -> Vec<usize> {
    let mut result = vec.clone();
    result.insert(0, first);
    result.push(last);
    result
}