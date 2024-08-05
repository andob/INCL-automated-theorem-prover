use std::collections::BTreeSet;
use crate::formula::{Formula, FormulaExtras, PredicateArgument, PredicateArguments};
use crate::logic::rule_apply_factory::RuleApplyFactory;
use crate::tree::node::ProofTreeNode;
use crate::tree::subtree::ProofSubtree;

pub fn apply_existential_quantification(
    factory : &mut RuleApplyFactory, node : &ProofTreeNode,
    x : &PredicateArgument, p : &Formula, extras : &FormulaExtras,
) -> Option<ProofSubtree>
{
    let object_name_factory = get_object_name_factory(factory, node);
    let instantiated_p = p.instantiated(x, &object_name_factory, extras);
    let instantiated_p_node = factory.new_node(instantiated_p);

    return Some(ProofSubtree::with_middle_node(instantiated_p_node));
}

fn get_object_name_factory(factory : &mut RuleApplyFactory, node : &ProofTreeNode) -> Box<dyn Fn() -> String>
{
    let used_names = factory.tree.get_paths_that_goes_through_node(node).into_iter()
        .flat_map(|path| path.nodes.into_iter().map(|node| node.formula))
        .flat_map(|formula| formula.get_all_predicate_arguments().into_iter())
        .flat_map(|name| vec![name.object_name, name.variable_name])
        .collect::<BTreeSet<String>>();

    return Box::new(move ||
    {
        let mut char = 'a';
        let mut aux = 0u64;
        loop
        {
            let name = if aux==0 { char.to_string() }
            else { format!("{}{}", char, aux) };

            if !used_names.contains(&name) { return name; }

            if char < 'z' { char = ((char as u8) + 1) as char; }
            else { char = 'a'; aux += 1; }
        }
    });
}

impl PredicateArguments
{
    pub fn instantiated(&self, x : &PredicateArgument, object_name_factory : &Box<dyn Fn() -> String>) -> PredicateArguments
    {
        let mut instantiated_args : Vec<PredicateArgument> = vec![];
        for arg in self.iter()
        {
            if arg.variable_name != x.variable_name
            {
                //we need to instantiate x and this is y. skip.
                instantiated_args.push(arg.clone());
            }
            else if arg.is_instantiated()
            {
                //we need to instantiate x and this is already instantiated (a:x). keep the instantiation (a:x).
                instantiated_args.push(arg.clone());
            }
            else
            {
                //instantiate x into a:x
                let mut instantiated_arg = arg.clone();
                instantiated_arg.object_name = (*object_name_factory)();
                instantiated_args.push(instantiated_arg);
            }
        }

        return PredicateArguments::new(instantiated_args);
    }
}
