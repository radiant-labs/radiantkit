
use std::collections::BTreeMap;
use std::any::TypeId;
use crate::RadiantTool;

pub struct RadiantToolManager<M> {
    pub tools: BTreeMap<TypeId, Box<dyn RadiantTool<M>>>,
    pub active_tool_id: TypeId,
}

impl<M> RadiantToolManager<M> {
    pub fn new<T: RadiantTool<M> + 'static>(tool: Box<T>) -> Self {
        Self {
            tools: BTreeMap::from([
                (TypeId::of::<T>(), tool as Box<dyn RadiantTool<M>>),
            ]),
            active_tool_id: TypeId::of::<T>(),
        }
    }

    pub fn register_tool<T: RadiantTool<M> + 'static>(&mut self, tool: Box<T>) {
        self.tools.insert(TypeId::of::<T>(), tool);
    }

    pub fn active_tool(&mut self) -> &mut dyn RadiantTool<M> {
        self.tools
            .get_mut(&self.active_tool_id)
            .expect("Active tool not found")
            .as_mut()
    }

    pub fn activate_tool(&mut self, id: u32) {
        if self.tools.len() > id as usize {
            self.active_tool_id = *self.tools.keys().nth(id as usize).unwrap();
        }
    }
}
