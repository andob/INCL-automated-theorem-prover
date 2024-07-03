use str_macro::str;
use crate::formula::{PredicateArgument, PredicateArguments};
use crate::logic::rule_apply_factory::RuleApplyFactory;

impl PredicateArguments
{
    pub fn instantiated(&self, factory : &mut RuleApplyFactory, x : &PredicateArgument) -> PredicateArguments
    {
        let mut instantiated_args : Vec<PredicateArgument> = vec![];
        for arg in &self.args
        {
            if arg.type_name != x.type_name
            {
                //do not instantiate, skip
                instantiated_args.push(arg.clone());
            }
            else if let Some(instance_name) = &arg.instance_name
            {
                //argument already instantiated, reuse the instance
                let mut instantiated_arg = arg.clone();
                instantiated_arg.instance_name = Some(instance_name.clone());
                instantiated_args.push(instantiated_arg);
            }
            else
            {
                //instantiate the argument
                let mut instantiated_arg = arg.clone();
                let instance_name = factory.new_predicate_argument_instance_name();
                instantiated_arg.instance_name = Some(instance_name);
                instantiated_args.push(instantiated_arg);
            }
        }

        return PredicateArguments::new(instantiated_args);
    }
}

impl PredicateArgument
{
    pub fn is_instantiated(&self) -> bool
    {
        return self.instance_name.is_some();
    }
}

pub struct PredicateArgInstanceNameSequence {}
impl PredicateArgInstanceNameSequence
{
    pub fn new() -> PredicateArgInstanceNameSequence
    {
        return PredicateArgInstanceNameSequence {};
    }

    pub fn next(&mut self) -> String
    {
        return str!("Socrates");
    }
}
