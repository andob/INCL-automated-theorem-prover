use prover::formula::notations::OperatorNotations;
use prover::formula::to_string::FormulaFormatOptions;
use prover::logic::first_order_logic::FirstOrderLogicDomainType::{ConstantDomain, VariableDomain};
use prover::logic::first_order_logic::FirstOrderLogicIdentityType::{ContingentIdentity, NecessaryIdentity};
use prover::logic::first_order_logic::{VariableDomainFlags, FIRST_ORDER_LOGIC_NAME_PREFIX};
use prover::logic::propositional_logic::PropositionalLogic;
use prover::logic::Logic;
use prover::logic::LogicFactory;
use prover::parser::token_types::TokenTypeID;
use prover::problem::catalog::get_demo_problem_catalog;
use prover::problem::json::ProblemJSON;
use std::error::Error;
use strum::IntoEnumIterator;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsError;

pub const PROPOSITIONAL_LOGIC_CATEGORY_NAME : &str = "PropositionalLogic";

#[wasm_bindgen]
pub fn setup_console_error_panic_hook()
{
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn get_operator_notations() -> Vec<String>
{
    return OperatorNotations::iter().map(|n| n.to_string()).collect();
}

#[wasm_bindgen]
pub fn set_operator_notations(operator_notations_id : String)
{
    let operator_notations = OperatorNotations::iter()
        .find(|notations| notations.to_string() == operator_notations_id)
        .unwrap_or(OperatorNotations::BookNotations);

    FormulaFormatOptions::DEFAULT_NOTATIONS.with(|default_notations|
        { *(default_notations.borrow_mut()) = operator_notations });
}

#[wasm_bindgen]
pub fn get_logics() -> Vec<String>
{
    return LogicFactory::get_logic_theories().iter()
        .map(|logic| logic.get_name().to_string()).collect();
}

#[wasm_bindgen]
pub fn get_logics_categories() -> Vec<String>
{
    let mut categories: Vec<String> = Vec::new();
    categories.push(String::from(PROPOSITIONAL_LOGIC_CATEGORY_NAME));

    let domain_types =
    [
        ConstantDomain,
        VariableDomain(VariableDomainFlags { has_domain_increasing_constraint:false }),
        VariableDomain(VariableDomainFlags { has_domain_increasing_constraint:true }),
    ];

    for domain_type in domain_types
    {
        for identity_type in [NecessaryIdentity, ContingentIdentity]
        {
            let category = format!("{}+{}+{}", FIRST_ORDER_LOGIC_NAME_PREFIX, domain_type, identity_type);
            categories.push(category);
        }
    }

    return categories;
}

#[wasm_bindgen]
pub fn get_default_logic_category() -> String
{
    return String::from(PROPOSITIONAL_LOGIC_CATEGORY_NAME);
}

#[wasm_bindgen]
pub fn get_operator_symbols(logic_name : String) -> Vec<String>
{
    return FormulaFormatOptions::DEFAULT_NOTATIONS.with(|operator_notations|
    {
        if let Some(logic) = LogicFactory::get_logic_theories().into_iter()
            .find(|logic| logic.get_name().to_string() == logic_name)
        {
            return get_operator_symbols_impl(logic.get_parser_syntax());
        }

        return get_operator_symbols_impl(TokenTypeID::iter().collect());
    })
}

fn get_operator_symbols_impl(token_type_ids : Vec<TokenTypeID>) -> Vec<String>
{
    return FormulaFormatOptions::DEFAULT_NOTATIONS.with(|operator_notations|
    {
        return token_type_ids.into_iter()
            .map(|token_type_id| operator_notations.borrow().get_operator_character(token_type_id))
            .filter(|operator_character| *operator_character != ' ')
            .map(|operator_character| operator_character.to_string()).collect();
    })
}

#[wasm_bindgen]
pub fn get_problem_catalog() -> String
{
    return FormulaFormatOptions::DEFAULT_NOTATIONS.with(|operator_notations|
    {
        let book_chapters = get_demo_problem_catalog().unwrap();
        let book_chapters_json = serde_json::to_string(&book_chapters).unwrap();
        return book_chapters_json;
    })
}

macro_rules! format_error
{
    ($err : expr) => { format!("{}\n{}", $err.to_string(), $err.source().map(|s| s.to_string()).unwrap_or_default()).as_str() };
}

#[wasm_bindgen]
pub fn solve_problem(problem_raw_json : String) -> Result<String, JsError>
{
    return FormulaFormatOptions::DEFAULT_NOTATIONS.with(|operator_notations|
    {
        let problem_parsed_json = serde_json::from_str::<ProblemJSON>(problem_raw_json.as_str())
            .map_err(|err| JsError::new(format_error!(err)))?;

        let problem = problem_parsed_json.to_problem()
            .map_err(|err| JsError::new(format_error!(err)))?;

        let formula_format_options = FormulaFormatOptions::recommended_for(&problem.logic);

        let proof_tree = problem.prove();
        let proof_tree_json = proof_tree.to_json(&formula_format_options)
            .map_err(|err| JsError::new(format_error!(err)))?;

        return Ok(proof_tree_json);
    })
}

#[wasm_bindgen]
pub fn should_skip_rendering_modality_graph(logic_name_raw : String) -> bool
{
    let logic_name = LogicFactory::get_logic_theories().iter()
        .map(|logic| logic.get_name())
        .find(|logic_name| logic_name.to_string() == logic_name_raw)
        .unwrap_or(PropositionalLogic{}.get_name());

    return !logic_name.is_modal_logic();
}
