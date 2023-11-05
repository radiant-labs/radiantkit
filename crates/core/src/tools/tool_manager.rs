use crate::RadiantTool;
use std::collections::BTreeMap;

pub type ToolId = u32;

pub struct RadiantToolManager<M> {
    pub tools: BTreeMap<ToolId, Box<dyn RadiantTool<M>>>,
    pub active_tool_id: ToolId,
}

impl<M> RadiantToolManager<M> {
    pub fn new<T: RadiantTool<M> + 'static>(id: ToolId, tool: Box<T>) -> Self {
        Self {
            tools: BTreeMap::from([(id, tool as Box<dyn RadiantTool<M>>)]),
            active_tool_id: id,
        }
    }

    pub fn register_tool<T: RadiantTool<M> + 'static>(&mut self, tool_id: ToolId, tool: Box<T>) {
        self.tools.insert(tool_id, tool);
    }

    pub fn active_tool(&mut self) -> &mut dyn RadiantTool<M> {
        self.tools
            .get_mut(&self.active_tool_id)
            .expect("Active tool not found")
            .as_mut()
    }

    pub fn activate_tool(&mut self, id: u32) {
        if self.tools.len() > id as usize {
            self.active_tool_id = id;
        }
    }
}
