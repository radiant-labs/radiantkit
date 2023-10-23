use crate::{RadiantTool, RadiantToolType};

pub struct RadiantToolManager {
    pub active_tool: Box<dyn RadiantTool>,
}

impl RadiantToolManager {
    pub fn new() -> Self {
        Self {
            active_tool: RadiantToolType::Selection.get_tool(),
        }
    }

    pub fn activate_tool(&mut self, tool: RadiantToolType) {
        self.active_tool = tool.get_tool();
    }
}
