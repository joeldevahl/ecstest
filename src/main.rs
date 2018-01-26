trait Storage {}

#[derive(Default)]
struct VecStorage<T: Component> {
    pool: Vec<T>,
}

impl<T: Component> Storage for VecStorage<T> {}

trait Component {
    type Storage: Storage + Default;
}

#[derive(Default)]
struct Position {
    x: f32,
    y: f32,
    z: f32,
}

impl Component for Position {
    type Storage = VecStorage<Self>;
}

#[derive(Default)]
struct Renderer {
    mesh: u32,
    mat: u32,
}

impl Component for Renderer {
    type Storage = VecStorage<Self>;
}

struct Registry {
    components: Vec<Box<Storage>>,
}

impl Registry {
    pub fn new() -> Self {
        Registry {
            components: Vec::new(),
        }        
    }

    pub fn register_component<T: Component + 'static>(&mut self) -> usize
    where
        T::Storage: Default,
    {
        let id = self.components.len();
        let storage: T::Storage = Default::default();
        self.components.push(Box::new(storage));
        id
    }
}

fn main() {
    let mut registry = Registry::new();
    
    let position_id = registry.register_component::<Position>();
    let renderer_id = registry.register_component::<Renderer>();
}