
use std::collections::HashMap;

use crate::{RadiantTool, SelectionTool, RectangleTool};

pub type RadiantToolId = u32;

pub struct RadiantToolManager {
    pub tools: HashMap<RadiantToolId, Box<dyn RadiantTool>>,
    pub active_tool_id: RadiantToolId,
}

impl RadiantToolManager {
    pub fn new() -> Self {
        Self {
            tools: HashMap::from([
                (0u32, Box::new(SelectionTool::new()) as Box<dyn RadiantTool>),
                (1u32, Box::new(RectangleTool::new()) as Box<dyn RadiantTool>),
            ]),
            active_tool_id: 0,
        }
    }

    pub fn register_tool(&mut self, tool: Box<dyn RadiantTool>) {
        self.tools.insert(tool.tool_id(), tool);
    }

    pub fn active_tool(&mut self) -> &mut dyn RadiantTool {
        self.tools
            .get_mut(&self.active_tool_id)
            .expect("Active tool not found")
            .as_mut()
    }

    pub fn activate_tool(&mut self, tool_id: RadiantToolId) {
        if self.tools.contains_key(&tool_id) {
            self.active_tool_id = tool_id;
        }
    }
}
