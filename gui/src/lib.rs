pub trait Draw {
    fn draw(&self);
}

// if this was implemented with a generic param and trait bounds,
// we would limit the values of components to only one type of obj
// (i.e. must all be of type button)
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
    }
}
