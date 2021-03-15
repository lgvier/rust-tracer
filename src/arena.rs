use crate::shapes::Shape;

// Arena is a pattern to simplify self-referencing structs (see the Group shape)
// https://dev.to/deciduously/no-more-tears-no-more-knots-arena-allocated-trees-in-rust-44k6

pub struct Arena {
    pub objects: Vec<Option<Shape>>,
}

impl Arena {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }
    pub fn add(&mut self, object: Shape) -> usize {
        let id = self.objects.len();
        self.objects.push(Some(object));
        id
    }
    // When creating groups, we need the id before adding the object to the arena
    pub fn next_id(&mut self) -> usize {
        let id = self.objects.len();
        self.objects.push(None);
        id
    }
    pub fn add_with_id(&mut self, id: usize, object: Shape) {
        if id >= self.objects.len() {
            panic!("Invalid id: {}", id);
        }
        if self.objects[id].is_some() {
            panic!("Id {} is already in use", id);
        }
        self.objects[id] = Some(object);
    }

    pub fn get(&self, id: usize) -> &Shape {
        &self.objects[id].as_ref().unwrap()
    }

    pub fn apply_changes(&mut self, id: usize, c: impl Fn(&mut Shape)) {
        match self.objects[id].as_mut() {
            Some(object) => {
                c(object);
            }
            None => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{material::MaterialBuilder, sphere};

    #[test]
    fn change_something() {
        let mut arena = Arena::new();
        let id = arena.add(sphere!());
        arena.apply_changes(id, |shape| {
            shape.set_material(MaterialBuilder::default().ambient(0.42).build().unwrap())
        });
        assert_eq!(0.42, arena.get(id).material().ambient);
    }
}
