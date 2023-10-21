use crate::ui::button;

#[derive(Debug, Clone)]
pub(crate) struct ToolButton {
    pub button: button::State,
    pub tool: Tool,
}

impl ToolButton {
    pub fn buttons() -> Vec<ToolButton> {
        const TOOLS: &[Tool] = &[
            Tool::Pencil,
            Tool::Stamp,
            Tool::Selection,
            Tool::Pan,
            Tool::Fill,
            Tool::Shape(ShapeTool::Circle),
        ];

        TOOLS
            .iter()
            .copied()
            .map(|tool| Self {
                button: button::State::new(),
                tool,
            })
            .collect()
    }

    pub fn update(&mut self, newly_selected_tool: Tool) {
        self.tool = match (self.tool, newly_selected_tool) {
            (Tool::Shape(_), Tool::Shape(_)) => newly_selected_tool,
            _ => self.tool,
        };
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub(crate) enum Tool {
    Pencil,
    Stamp,
    Selection,
    Pan,
    Fill,
    Shape(ShapeTool),
}

impl Tool {
    pub fn unselected_sprite(self) -> usize {
        match self {
            Tool::Pencil => 45,
            Tool::Stamp => 37,
            Tool::Selection => 38,
            Tool::Pan => 39,
            Tool::Fill => 40,
            Tool::Shape(shape_tool) => shape_tool.unselected_sprite(),
        }
    }

    // Returns which tool should be selected next when this one is clicked.
    // This exists (rather than just being `self`) because the buttons for the
    // shape tools change to the next tool when clicked again.
    pub fn selected_tool_on_click(self, currently_selected_tool: Tool) -> Self {
        match self {
            Tool::Shape(shape_tool) => {
                Tool::Shape(shape_tool.next_shape_tool(currently_selected_tool))
            }
            _ => self,
        }
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub(crate) enum ShapeTool {
    Circle,
    Rectangle,
    Line,
}

impl ShapeTool {
    // TODO: Icons change for Circle and Rectangle if holding Ctrl,
    // to show the filled variants.
    fn unselected_sprite(self) -> usize {
        match self {
            ShapeTool::Circle => 41,
            ShapeTool::Rectangle => 43,
            ShapeTool::Line => 46,
        }
    }

    fn next_shape_tool(self, currently_selected_tool: Tool) -> Self {
        match currently_selected_tool {
            Tool::Shape(_) => {}
            _ => return self,
        };

        match self {
            ShapeTool::Circle => ShapeTool::Rectangle,
            ShapeTool::Rectangle => ShapeTool::Line,
            ShapeTool::Line => ShapeTool::Circle,
        }
    }
}
