use splat::engine::{Button, Camera, Drawable, Input, ScreenCoord, ScreenPos, Sprite};

enum State {
    Normal,
    Hovered,
    ClickedThisFrame,
    Pressed,
}

struct UiButton {
    screen_pos: ScreenPos,
    sprite: Sprite,
    hover_sprite: Sprite,
    press_sprite: Sprite,
    state: State,
}

impl UiButton {
    fn update(&mut self, input: &Input) {
        let width = self.sprite.len() as ScreenCoord;
        if width == 0 {
            return;
        }

        let height = self.sprite[0].len() as ScreenCoord / 2;
        if height == 0 {
            return;
        }

        let offset = input.mouse_screen_pos - self.screen_pos;
        if 0 <= offset.x && offset.x <= width && 0 <= offset.y && offset.y <= height {
            if input.pressed_this_frame(Button::LeftMouse) {
                self.state = State::ClickedThisFrame;
            } else if input.pressed(Button::LeftMouse) {
                self.state = State::Pressed;
            } else {
                self.state = State::Hovered;
            }
        } else {
            self.state = State::Normal;
        }
    }

    fn clicked_this_frame(&self) -> bool {
        matches!(self.state, State::ClickedThisFrame)
    }
}

impl Drawable for UiButton {
    fn draw(&self, camera: &splat::engine::Camera, renderer: &mut splat::engine::Renderer) {
        let sprite = match self.state {
            State::Normal => &self.sprite,
            State::Hovered => &self.hover_sprite,
            State::ClickedThisFrame | State::Pressed => &self.press_sprite,
        };
        camera.paint_sprite(sprite, self.screen_pos.into(), renderer);
    }
}

pub struct UiButtons {
    left_button: UiButton,
    right_button: UiButton,
    up_button: UiButton,
    down_button: UiButton,
    msg_button: UiButton,
    msg_button_counter: usize,
}

impl Drawable for UiButtons {
    fn draw(&self, camera: &splat::engine::Camera, renderer: &mut splat::engine::Renderer) {
        self.left_button.draw(camera, renderer);
        self.right_button.draw(camera, renderer);
        self.up_button.draw(camera, renderer);
        self.down_button.draw(camera, renderer);
        self.msg_button.draw(camera, renderer);
    }
}

impl UiButtons {
    pub fn new() -> Self {
        let base_pos = ScreenPos::new(10, 5);
        Self {
            left_button: UiButton {
                screen_pos: base_pos + ScreenPos::new(-6, 0),
                sprite: vec![
                    vec!['┌', '─', '─', '─', '┐'],
                    vec!['│', ' ', '◀', ' ', '│'],
                    vec!['└', '─', '─', '─', '┘'],
                ],
                hover_sprite: vec![
                    vec!['┏', '━', '━', '━', '┓'],
                    vec!['┃', ' ', '◀', ' ', '┃'],
                    vec!['┗', '━', '━', '━', '┛'],
                ],
                press_sprite: vec![
                    vec!['╔', '═', '═', '═', '╗'],
                    vec!['║', ' ', '◀', ' ', '║'],
                    vec!['╚', '═', '═', '═', '╝'],
                ],
                state: State::Normal,
            },
            right_button: UiButton {
                screen_pos: base_pos + ScreenPos::new(6, 0),
                sprite: vec![
                    vec!['┌', '─', '─', '─', '┐'],
                    vec!['│', ' ', '▶', ' ', '│'],
                    vec!['└', '─', '─', '─', '┘'],
                ],
                hover_sprite: vec![
                    vec!['┏', '━', '━', '━', '┓'],
                    vec!['┃', ' ', '▶', ' ', '┃'],
                    vec!['┗', '━', '━', '━', '┛'],
                ],
                press_sprite: vec![
                    vec!['╔', '═', '═', '═', '╗'],
                    vec!['║', ' ', '▶', ' ', '║'],
                    vec!['╚', '═', '═', '═', '╝'],
                ],
                state: State::Normal,
            },
            up_button: UiButton {
                screen_pos: base_pos + ScreenPos::new(0, -3),
                sprite: vec![
                    vec!['┌', '─', '─', '─', '┐'],
                    vec!['│', ' ', '▲', ' ', '│'],
                    vec!['└', '─', '─', '─', '┘'],
                ],
                hover_sprite: vec![
                    vec!['┏', '━', '━', '━', '┓'],
                    vec!['┃', ' ', '▲', ' ', '┃'],
                    vec!['┗', '━', '━', '━', '┛'],
                ],
                press_sprite: vec![
                    vec!['╔', '═', '═', '═', '╗'],
                    vec!['║', ' ', '▲', ' ', '║'],
                    vec!['╚', '═', '═', '═', '╝'],
                ],
                state: State::Normal,
            },
            down_button: UiButton {
                screen_pos: base_pos + ScreenPos::new(0, 3),
                sprite: vec![
                    vec!['┌', '─', '─', '─', '┐'],
                    vec!['│', ' ', '▼', ' ', '│'],
                    vec!['└', '─', '─', '─', '┘'],
                ],
                hover_sprite: vec![
                    vec!['┏', '━', '━', '━', '┓'],
                    vec!['┃', ' ', '▼', ' ', '┃'],
                    vec!['┗', '━', '━', '━', '┛'],
                ],
                press_sprite: vec![
                    vec!['╔', '═', '═', '═', '╗'],
                    vec!['║', ' ', '▼', ' ', '║'],
                    vec!['╚', '═', '═', '═', '╝'],
                ],
                state: State::Normal,
            },
            msg_button: UiButton {
                screen_pos: base_pos,
                sprite: vec![
                    vec!['┌', '─', '─', '─', '┐'],
                    vec!['│', ' ', '?', ' ', '│'],
                    vec!['└', '─', '─', '─', '┘'],
                ],
                hover_sprite: vec![
                    vec!['┏', '━', '━', '━', '┓'],
                    vec!['┃', ' ', '?', ' ', '┃'],
                    vec!['┗', '━', '━', '━', '┛'],
                ],
                press_sprite: vec![
                    vec!['╔', '═', '═', '═', '╗'],
                    vec!['║', ' ', '?', ' ', '║'],
                    vec!['╚', '═', '═', '═', '╝'],
                ],
                state: State::Normal,
            },
            msg_button_counter: 0,
        }
    }

    pub fn update(&mut self, input: &Input, camera: &mut Camera) {
        self.left_button.update(input);
        self.right_button.update(input);
        self.up_button.update(input);
        self.down_button.update(input);
        self.msg_button.update(input);

        if self.left_button.clicked_this_frame() {
            camera.pos.x += 10.0;
        }

        if self.right_button.clicked_this_frame() {
            camera.pos.x -= 10.0;
        }

        if self.up_button.clicked_this_frame() {
            camera.pos.y += 5.0;
        }

        if self.down_button.clicked_this_frame() {
            camera.pos.y -= 5.0;
        }

        if self.msg_button.clicked_this_frame() {
            let msg = format!(
                "that cheese {}stinks",
                "really ".repeat(self.msg_button_counter)
            );
            splat::dbg!(msg);
            self.msg_button_counter += 1;
        }
    }
}
