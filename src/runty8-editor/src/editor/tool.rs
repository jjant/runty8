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
    pub const TOOLS: &[Tool] = &[
        Tool::Pencil,
        Tool::Stamp,
        Tool::Selection,
        Tool::Pan,
        Tool::Fill,
        Tool::Shape(ShapeTool::Circle),
    ];

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
}
