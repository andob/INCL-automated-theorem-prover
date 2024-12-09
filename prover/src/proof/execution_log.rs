use std::cell::{RefCell, RefMut};
use std::collections::BTreeSet;
use std::fmt::{Display, Formatter};
use std::rc::Rc;
use crate::formula::PossibleWorld;
use crate::graph::GraphVertex;
use crate::tree::node_factory::ProofTreeNodeID;

pub struct ExecutionLog
{
    lines : Vec<String>
}

impl ExecutionLog
{
    thread_local!
    {
        static INSTANCE : Rc<RefCell<ExecutionLog>> =
            Rc::new(RefCell::new(ExecutionLog::new()));
    }

    fn new() -> ExecutionLog
    {
        return ExecutionLog { lines: Vec::new() };
    }

    pub fn clear() -> Vec<String>
    {
        return Self::INSTANCE.with(|log|
            log.replace_with(|_| ExecutionLog::new())).lines;
    }

    pub fn log(line : String)
    {
        Self::INSTANCE.with(|log|
            log.borrow_mut().lines.push(line));
    }
}

impl Display for ExecutionLog
{
    fn fmt(&self, f : &mut Formatter<'_>) -> std::fmt::Result
    {
        for line in &self.lines
        {
            write!(f, "{}", line).unwrap_or_default();
        }

        return write!(f, "");
    }
}

pub struct ExecutionLogHelperData
{
    pub new_graph_nodes : BTreeSet<PossibleWorld>,
    pub new_graph_vertices : BTreeSet<GraphVertex>,
    pub old_contradictions : BTreeSet<(ProofTreeNodeID, ProofTreeNodeID)>,
    pub new_contradictions : BTreeSet<(ProofTreeNodeID, ProofTreeNodeID)>,
}

impl ExecutionLogHelperData
{
    thread_local!
    {
        static INSTANCE : Rc<RefCell<ExecutionLogHelperData>> =
            Rc::new(RefCell::new(ExecutionLogHelperData::new()));
    }

    fn new() -> ExecutionLogHelperData
    {
        return ExecutionLogHelperData
        {
            new_graph_nodes: BTreeSet::new(),
            new_graph_vertices: BTreeSet::new(),
            old_contradictions: BTreeSet::new(),
            new_contradictions: BTreeSet::new(),
        }
    }

    #[inline(always)]
    pub fn with<F, R>(callback : F) -> R
    where F : FnOnce(RefMut<ExecutionLogHelperData>) -> R
    {
        return ExecutionLogHelperData::INSTANCE.with(|helper_data|
            callback(helper_data.borrow_mut()));
    }

    pub fn flush() -> ExecutionLogHelperData
    {
        return ExecutionLogHelperData::INSTANCE.with(|helper_data|
            helper_data.replace_with(|helper_data|
            {
                let mut old_contradictions = helper_data.old_contradictions.clone();
                old_contradictions.extend(helper_data.new_contradictions.clone());

                return ExecutionLogHelperData
                {
                    new_graph_nodes: BTreeSet::new(),
                    new_graph_vertices: BTreeSet::new(),
                    old_contradictions: old_contradictions,
                    new_contradictions: BTreeSet::new(),
                };
            }));
    }
}
