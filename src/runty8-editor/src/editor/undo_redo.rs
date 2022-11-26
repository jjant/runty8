use runty8_core::SpriteSheet;

use super::notification;

#[derive(Debug)]
pub(crate) struct Commands {
    commands: Vec<Command>,
    current: usize,
}

impl Commands {
    pub fn new() -> Self {
        Self {
            commands: vec![],
            current: 0,
        }
    }
    pub fn push(&mut self, command: Command) {
        self.commands.drain(self.current..self.commands.len());

        self.commands.push(command);
        self.current += 1;
    }

    pub fn undo(&mut self, notification: &mut notification::State, sprite_sheet: &mut SpriteSheet) {
        if self.current > 0 {
            let command = &self.commands[self.current - 1];
            self.current -= 1;

            command.undo(sprite_sheet);
        } else {
            notification.alert("NOTHING TO UNDO".to_owned());
        }
    }

    pub fn redo(&mut self, notification: &mut notification::State, sprite_sheet: &mut SpriteSheet) {
        if self.current < self.commands.len() {
            let command = &self.commands[self.current];
            self.current += 1;

            command.redo(sprite_sheet);
        } else {
            notification.alert("NOTHING TO REDO".to_owned());
        }
    }
}

// Undoable actions
#[derive(Debug)]
pub enum Command {
    // TODO: This currently treats each edited pixel as its own action.
    // Pico8 instead tracks "strokes", i.e, drawing with the pen until you lift it
    // counts as a single command/undoable action. We should do that.
    PixelChanged(PixelChanged),
}

impl Command {
    pub fn pixel_changed(
        sprite: usize,
        x: isize,
        y: isize,
        previous_color: u8,
        new_color: u8,
    ) -> Self {
        Self::PixelChanged(PixelChanged {
            sprite,
            x,
            y,
            previous_color,
            new_color,
        })
    }

    fn undo(&self, sprite_sheet: &mut SpriteSheet) {
        match self {
            Command::PixelChanged(pixel_changed) => pixel_changed.undo(sprite_sheet),
        }
    }

    fn redo(&self, sprite_sheet: &mut SpriteSheet) {
        match self {
            Command::PixelChanged(pixel_changed) => pixel_changed.redo(sprite_sheet),
        }
    }
}

#[derive(Debug)]
pub struct PixelChanged {
    sprite: usize,
    x: isize,
    y: isize,
    previous_color: u8,
    new_color: u8,
}

impl PixelChanged {
    fn undo(&self, sprite_sheet: &mut SpriteSheet) {
        let sprite = sprite_sheet.get_sprite_mut(self.sprite);

        sprite.pset(self.x, self.y, self.previous_color);
    }

    fn redo(&self, sprite_sheet: &mut SpriteSheet) {
        let sprite = sprite_sheet.get_sprite_mut(self.sprite);

        sprite.pset(self.x, self.y, self.new_color);
    }
}

#[cfg(test)]
mod tests {
    use crate::editor::notification;

    use super::*;

    #[test]
    fn undo_empty() {
        let mut commands = Commands::new();
        let mut notification = notification::State::new();
        let mut sprite_sheet = SpriteSheet::new();

        commands.undo(&mut notification, &mut sprite_sheet);

        assert_eq!(commands.current, 0);
        assert_eq!(notification.content(), "NOTHING TO UNDO");
    }

    #[test]
    fn redo_empty() {
        let mut commands = Commands::new();
        let mut notification = notification::State::new();
        let mut sprite_sheet = SpriteSheet::new();

        commands.redo(&mut notification, &mut sprite_sheet);

        assert_eq!(commands.current, 0);
        assert_eq!(notification.content(), "NOTHING TO REDO");
    }

    #[test]
    fn undo_many() {
        let mut commands = Commands::new();
        let mut notification = notification::State::new();
        let mut sprite_sheet = SpriteSheet::new();

        commands.push(Command::pixel_changed(20, 0, 0, 1, 2));
        commands.push(Command::pixel_changed(20, 0, 0, 2, 3));
        commands.push(Command::pixel_changed(20, 0, 0, 3, 4));
        commands.push(Command::pixel_changed(20, 0, 0, 4, 5));
        sprite_sheet.get_sprite_mut(20).pset(0, 0, 5);

        fn get_pixel(sprite_sheet: &mut SpriteSheet) -> u8 {
            sprite_sheet.get_sprite_mut(20).pget(0, 0)
        }

        assert_eq!(get_pixel(&mut sprite_sheet), 5);
        commands.undo(&mut notification, &mut sprite_sheet);
        assert_eq!(get_pixel(&mut sprite_sheet), 4);
        commands.undo(&mut notification, &mut sprite_sheet);
        assert_eq!(get_pixel(&mut sprite_sheet), 3);
        commands.undo(&mut notification, &mut sprite_sheet);
        assert_eq!(get_pixel(&mut sprite_sheet), 2);
        commands.undo(&mut notification, &mut sprite_sheet);
        assert_eq!(get_pixel(&mut sprite_sheet), 1);
        commands.undo(&mut notification, &mut sprite_sheet);
        assert_eq!(get_pixel(&mut sprite_sheet), 1);
    }
}
