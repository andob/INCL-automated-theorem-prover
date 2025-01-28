use std::rc::Rc;
use crate::logic::first_order_logic::{FirstOrderLogic, FirstOrderLogicDomainType, FirstOrderLogicIdentityType};
use crate::logic::{Logic, LogicName};
use crate::logic::normal_modal_logic::NormalModalLogic;
use crate::logic::propositional_logic::PropositionalLogic;

pub struct AlternativeCountermodelFinderAvailability {}
impl AlternativeCountermodelFinderAvailability
{
    pub fn get_available_logics() -> Vec<Rc<dyn Logic>>
    {
        return
        [
            Self::get_available_propositional_logics(),
            Self::get_available_first_order_logics(),
        ].concat();
    }

    pub fn get_available_logic_names() -> Vec<LogicName>
    {
        return Self::get_available_logics().iter()
            .map(|logic| logic.get_name()).collect();
    }

    pub fn get_available_propositional_logics() -> Vec<Rc<dyn Logic>>
    {
        return vec!
        [
            Rc::new(PropositionalLogic{}),
            Rc::new(NormalModalLogic::K()),
            Rc::new(NormalModalLogic::T()),
            Rc::new(NormalModalLogic::B()),
            Rc::new(NormalModalLogic::S4()),
            Rc::new(NormalModalLogic::S5()),
        ];
    }

    pub fn get_available_first_order_logics() -> Vec<Rc<dyn Logic>>
    {
        return Self::get_available_propositional_logics().iter()
            .map(|propositional_logic| Rc::new(FirstOrderLogic
            {
                domain_type: FirstOrderLogicDomainType::ConstantDomain,
                identity_type: FirstOrderLogicIdentityType::ContingentIdentity,
                base_logic: propositional_logic.clone(),
            }) as Rc<dyn Logic>).collect();
    }
}
