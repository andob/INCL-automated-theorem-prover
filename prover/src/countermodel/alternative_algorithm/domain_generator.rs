use std::cmp::max;
use std::collections::BTreeSet;
use std::rc::Rc;
use smol_str::{format_smolstr, SmolStr, ToSmolStr};
use crate::formula::PredicateArgument;
use crate::logic::Logic;

pub struct CountermodelDomainGenerator
{
    pub logic : Rc<dyn Logic>,
    pub predicate_arguments : BTreeSet<PredicateArgument>,
}

impl CountermodelDomainGenerator
{
    pub fn generate_domains(&self, min_domain_size : u8, max_domain_size : u8) -> Vec<BTreeSet<SmolStr>>
    {
        if self.logic.get_name().is_first_order_logic()
        {
            return (min_domain_size..=max_domain_size).into_iter()
                    .map(|domain_size| self.generate_domain(domain_size))
                    .collect();
        }

        return vec![BTreeSet::new()];
    }

    fn generate_domain(&self, number_of_elements : u8) -> BTreeSet<SmolStr>
    {
        let mut result = BTreeSet::new();

        let mut used_names = self.predicate_arguments.clone().into_iter()
            .flat_map(|arg| vec![arg.object_name, arg.variable_name])
            .collect::<BTreeSet<SmolStr>>();

        for _ in 0..max(1, number_of_elements)
        {
            let name = self.generate_next_unique_name(&used_names);
            used_names.insert(name.clone());
            result.insert(name);
        }
        
        return result;
    }
    
    fn generate_next_unique_name(&self, used_names : &BTreeSet<SmolStr>) -> SmolStr
    {
        let mut char = 'a';
        let mut aux = 0u64;
        loop
        {
            let name = if aux==0 { char.to_smolstr() }
            else { format_smolstr!("{}{}", char, aux) };

            if !used_names.contains(&name) { return name; }

            if char < 'z' { char = ((char as u8) + 1) as char; }
            else { char = 'a'; aux += 1; }
        }
    }
}
