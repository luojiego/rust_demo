pub trait Draw {
    fn draw(&self);
}

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

// 以下实现的问题在于无法在 components 中放入不同的类型
// 即 Box::new(Button) 和 Box::new(SelectBox) 并不一样
// pub struct Screen<T: Draw> {
//     pub components: Vec<T>,
// }
// 
// impl<T> Screen<T>
// where 
//     T: Draw 
// {
//     pub fn run(&self) {
//         for component in self.components.iter() {
//             component.draw();
//         }
//     }
// }

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {

    }
}
