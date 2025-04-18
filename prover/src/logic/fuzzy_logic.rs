use std::any::Any;
use std::collections::BTreeSet;
use box_macro::bx;
use smol_str::{format_smolstr, SmolStr, ToSmolStr};
use crate::formula::Formula::{And, Atomic, BiImply, GreaterOrEqualThan, Imply, LessThan, Non, Or};
use crate::formula::{FuzzyTag, FuzzyTags};
use crate::formula::Sign::{Minus, Plus};
use crate::logic::{Logic, LogicName, LogicRule, LogicRuleCollection};
use crate::logic::common_modal_logic::ModalityRef;
use crate::logic::rule_apply_factory::RuleApplyFactory;
use crate::parser::token_types::TokenTypeID;
use crate::semantics::fuzzy_logic_semantics::FuzzyLogicSemantics;
use crate::semantics::Semantics;
use crate::tree::node::ProofTreeNode;
use crate::tree::subtree::ProofSubtree;

pub struct LukasiewiczFuzzyLogic {}

impl Logic for LukasiewiczFuzzyLogic
{
    fn get_name(&self) -> LogicName { LogicName::of("LukasiewiczFuzzyLogic") }
    fn as_any(&self) -> &dyn Any { self }

    fn get_semantics(&self) -> Box<dyn Semantics>
    {
        return Box::new(FuzzyLogicSemantics {});
    }

    fn get_parser_syntax(&self) -> Vec<TokenTypeID>
    {
        return vec!
        [
            TokenTypeID::AtomicWithoutArgs,
            TokenTypeID::Non, TokenTypeID::And, TokenTypeID::Or,
            TokenTypeID::Imply, TokenTypeID::BiImply,
            TokenTypeID::OpenParenthesis, TokenTypeID::ClosedParenthesis
        ];
    }

    fn get_rules(&self) -> LogicRuleCollection
    {
        return LogicRuleCollection::of(vec!
        [
            Box::new(LukasiewiczFuzzyLogicRules {})
        ]);
    }

    fn get_modality_ref(&self) -> Option<ModalityRef> { None }
}

struct LukasiewiczFuzzyLogicRules {}

impl LogicRule for LukasiewiczFuzzyLogicRules
{
    fn apply(&self, factory : &mut RuleApplyFactory, node : &ProofTreeNode) -> Option<ProofSubtree>
    {
        return match &node.formula
        {
            Non(box Non(box p, _), extras) =>
            {
                let tagged_p = p.with_sign(extras.sign).with_fuzzy_tags(extras.fuzzy_tags.clone());
                let tagged_p_node = factory.new_node(tagged_p.clone());

                return Some(ProofSubtree::with_middle_node(tagged_p_node));
            }

            And(box p, box q, extras) if extras.sign == Plus =>
            {
                let plus_q = q.with_sign(Plus).with_fuzzy_tags(extras.fuzzy_tags.clone());
                let plus_q_node = factory.new_node(plus_q);

                let plus_p = p.with_sign(Plus).with_fuzzy_tags(extras.fuzzy_tags.clone());
                let plus_p_node = factory.new_node_with_subnode(plus_p, plus_q_node);

                return Some(ProofSubtree::with_middle_node(plus_p_node));
            }

            And(box p, box q, extras) if extras.sign == Minus =>
            {
                let minus_p = p.with_sign(Minus).with_fuzzy_tags(extras.fuzzy_tags.clone());
                let minus_p_node = factory.new_node(minus_p);

                let minus_q = q.with_sign(Minus).with_fuzzy_tags(extras.fuzzy_tags.clone());
                let minus_q_node = factory.new_node(minus_q);

                return Some(ProofSubtree::with_left_right_nodes(minus_p_node, minus_q_node));
            }

            Non(box And(box p, box q, _), extras) =>
            {
                let non_p = Non(bx!(p.clone()), extras.clone());
                let non_q = Non(bx!(q.clone()), extras.clone());

                let non_p_or_non_q = Or(bx!(non_p), bx!(non_q), extras.clone());
                let non_p_or_non_q_node = factory.new_node(non_p_or_non_q);

                return Some(ProofSubtree::with_middle_node(non_p_or_non_q_node));
            }

            Or(box p, box q, extras) if extras.sign == Plus =>
            {
                let plus_p = p.with_sign(Plus).with_fuzzy_tags(extras.fuzzy_tags.clone());
                let plus_p_node = factory.new_node(plus_p);

                let plus_q = q.with_sign(Plus).with_fuzzy_tags(extras.fuzzy_tags.clone());
                let plus_q_node = factory.new_node(plus_q);

                return Some(ProofSubtree::with_left_right_nodes(plus_p_node, plus_q_node));
            }

            Or(box p, box q, extras) if extras.sign == Minus =>
            {
                let minus_q = q.with_sign(Minus).with_fuzzy_tags(extras.fuzzy_tags.clone());
                let minus_q_node = factory.new_node(minus_q);

                let minus_p = p.with_sign(Minus).with_fuzzy_tags(extras.fuzzy_tags.clone());
                let minus_p_node = factory.new_node_with_subnode(minus_p, minus_q_node);

                return Some(ProofSubtree::with_middle_node(minus_p_node));
            }

            Non(box Or(box p, box q, _), extras) =>
            {
                let non_p = Non(bx!(p.clone()), extras.clone());
                let non_q = Non(bx!(q.clone()), extras.clone());

                let non_p_and_non_q = And(bx!(non_p), bx!(non_q), extras.clone());
                let non_p_and_non_q_node = factory.new_node(non_p_and_non_q);

                return Some(ProofSubtree::with_middle_node(non_p_and_non_q_node));
            }

            Imply(box p, box q, extras) if extras.sign == Plus =>
            {
                let new_tag = self.create_new_fuzzy_tag(factory, node);

                let minus_p_tags = FuzzyTags::empty().plus(new_tag.clone());
                let minus_p = p.with_sign(Minus).with_fuzzy_tags(minus_p_tags);
                let minus_p_node = factory.new_node(minus_p);

                let plus_q_tags = extras.fuzzy_tags.plus(new_tag);
                let plus_q = q.with_sign(Plus).with_fuzzy_tags(plus_q_tags);
                let plus_q_node = factory.new_node(plus_q);

                return Some(ProofSubtree::with_left_right_nodes(minus_p_node, plus_q_node));
            }

            Imply(box p, box q, extras) if extras.sign == Minus =>
            {
                let new_tag = self.create_new_fuzzy_tag(factory, node);

                let minus_q_tags = extras.fuzzy_tags.plus(new_tag.clone());
                let minus_q = q.with_sign(Minus).with_fuzzy_tags(minus_q_tags);
                let minus_q_node = factory.new_node(minus_q);

                let plus_p_tags = FuzzyTags::empty().plus(new_tag);
                let plus_p = p.with_sign(Plus).with_fuzzy_tags(plus_p_tags);
                let plus_p_node = factory.new_node_with_subnode(plus_p, minus_q_node);

                return Some(ProofSubtree::with_middle_node(plus_p_node));
            }

            Non(box p_imply_q@Imply(..), extras) =>
            {
                let p_imply_q_minus = p_imply_q.with_sign(extras.sign * Minus);
                let p_imply_q_minus_node = factory.new_node(p_imply_q_minus);

                return Some(ProofSubtree::with_middle_node(p_imply_q_minus_node));
            }

            BiImply(box p, box q, extras) =>
            {
                let p_imply_q = Imply(bx!(p.clone()), bx!(q.clone()), extras.clone());
                let q_imply_p = Imply(bx!(q.clone()), bx!(p.clone()), extras.clone());
                let conjunction = And(bx!(p_imply_q), bx!(q_imply_p), extras.clone());
                let conjunction_node = factory.new_node(conjunction);

                return Some(ProofSubtree::with_middle_node(conjunction_node));
            }

            Non(box BiImply(box p, box q, _), extras) =>
            {
                let p_imply_q = Imply(bx!(p.clone()), bx!(q.clone()), extras.clone());
                let q_imply_p = Imply(bx!(q.clone()), bx!(p.clone()), extras.clone());
                let conjunction = And(bx!(p_imply_q), bx!(q_imply_p), extras.clone());
                let non_conjunction = Non(bx!(conjunction), extras.clone());
                let non_conjunction_node = factory.new_node(non_conjunction);

                return Some(ProofSubtree::with_middle_node(non_conjunction_node));
            }

            Atomic(p_name, extras) if extras.sign == Plus =>
            {
                let mu_tag = FuzzyTag::new(format_smolstr!("µ:{}", p_name));
                let new_extras = extras.with_fuzzy_tags(extras.fuzzy_tags.plus(mu_tag.clone()));

                let x = extras.fuzzy_tags.clone();
                let y = FuzzyTags::new(vec![mu_tag]);
                let x_greater_than_y = GreaterOrEqualThan(x.clone(), y.clone(), new_extras.to_formula_extras());
                let x_greater_than_y_node = factory.new_node(x_greater_than_y);

                return Some(ProofSubtree::with_middle_node(x_greater_than_y_node));
            }

            Atomic(p_name, extras) if extras.sign == Minus =>
            {
                let mu_tag = FuzzyTag::new(format_smolstr!("µ:{}", p_name));
                let new_extras = extras.with_fuzzy_tags(extras.fuzzy_tags.plus(mu_tag.clone()));

                let x = extras.fuzzy_tags.clone();
                let y = FuzzyTags::new(vec![mu_tag]);
                let x_less_than_y = LessThan(x.clone(), y.clone(), new_extras.to_formula_extras());
                let x_less_than_y_node = factory.new_node(x_less_than_y);

                return Some(ProofSubtree::with_middle_node(x_less_than_y_node));
            }

            Non(box Atomic(p_name, _), extras) if extras.sign == Plus =>
            {
                let minus_mu_tag = FuzzyTag { object_name:format_smolstr!("µ:{}", p_name), sign:Minus };
                let one_minus_mu_tags = vec![FuzzyTag::one(), minus_mu_tag];
                let new_extras = extras.with_fuzzy_tags(extras.fuzzy_tags.plus_vec(&one_minus_mu_tags));

                let x = extras.fuzzy_tags.clone();
                let y = FuzzyTags::new(one_minus_mu_tags);
                let x_greater_than_y = GreaterOrEqualThan(x.clone(), y.clone(), new_extras);
                let x_greater_than_y_node = factory.new_node(x_greater_than_y);

                return Some(ProofSubtree::with_middle_node(x_greater_than_y_node));
            }

            Non(box Atomic(p_name, _), extras) if extras.sign == Minus =>
            {
                let minus_mu_tag = FuzzyTag { object_name:format_smolstr!("µ:{}", p_name), sign:Minus };
                let one_minus_mu_tags = vec![FuzzyTag::one(), minus_mu_tag];
                let new_extras = extras.with_fuzzy_tags(extras.fuzzy_tags.plus_vec(&one_minus_mu_tags));

                let x = extras.fuzzy_tags.clone();
                let y = FuzzyTags::new(one_minus_mu_tags);
                let x_less_than_y = LessThan(x.clone(), y.clone(), new_extras);
                let x_less_than_y_node = factory.new_node(x_less_than_y);

                return Some(ProofSubtree::with_middle_node(x_less_than_y_node));
            }

            _ => None
        }
    }
}

impl LukasiewiczFuzzyLogicRules
{
    pub fn create_new_fuzzy_tag(&self, factory : &RuleApplyFactory, node : &ProofTreeNode) -> FuzzyTag
    {
        let used_names = factory.tree.get_paths_that_goes_through_node(node).into_iter()
            .flat_map(|path| path.nodes.into_iter().map(|node| node.formula))
            .flat_map(|formula| formula.get_fuzzy_tags().into_iter())
            .map(|fuzzy_tag| fuzzy_tag.object_name)
            .collect::<BTreeSet<SmolStr>>();

        let mut char = 'α';
        let mut aux = 0u64;
        loop
        {
            let name = if aux==0 { char.to_smolstr() }
            else { format_smolstr!("{}{}", char, aux) };

            if !used_names.contains(&name)
            {
                return FuzzyTag::new(name);
            }

            char = match char
            {
                'α' => 'β', 'β' => 'γ', 'γ' => 'ρ', 'ρ' => 'σ',
                'σ' => 'τ', 'τ' => 'φ', 'φ' => 'ψ', 'ψ' => 'ω',
                _ => { aux += 1; 'α' }
            };
        }
    }
}
