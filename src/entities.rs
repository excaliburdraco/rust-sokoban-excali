use crate::components::*;
use specs::{Builder, World, WorldExt};


// Create a wall entity
pub fn create_wall(world: &mut World, position: Position) {
    world
        .create_entity()
        .with(Position { z: 10, ..position })
        .with(Renderable {
            path: "/images/wall.png".to_string(),
        })
        .with(Wall {})
        .with(Immovable)
        .build();
}
// Create a floor entity，assume the floor is at z=5,and the wall is at z=10
pub fn create_floor(world: &mut World, position: Position) {
    world
        .create_entity()
        .with(Position { z: 5, ..position })
        .with(Renderable {
            path: "/images/floor.png".to_string(),
        })
        .build();
}
// create a box entity,include position、renderable、box component,we have 
// to create a box spot entity,include position、renderable、boxspot component
pub fn create_box(world: &mut World, position: Position) {
    world
        .create_entity()
        .with(Position { z: 10, ..position })
        .with(Renderable {
            path: "/images/box.png".to_string(),
        })
        .with(Box {})
        .with(Movable)
        .build();
}

// create a box spot entity,include position、renderable、boxspot component
// use EntityBuilder to create the box spot entity
// EntityBuilder implements with method,which can be chained.

pub fn create_box_spot(world: &mut World, position: Position) {
    world
        .create_entity()
        .with(Position { z: 9, ..position })
        .with(Renderable {
            path: "/images/box_spot.png".to_string(),
        })
        .with(BoxSpot {})
        .build();
}
// EntityBuilder实现了with方法，可以链式调用,调用的路径是，world.create_entity().with(Position{z:10,..position})
// world实现了create_entity方法，返回一个EntityBuilder，EntityBuilder实现了with方法，可以链式调用.
pub fn create_player(world: &mut World, position: Position) {
    world
        .create_entity()
        .with(Position { z: 10, ..position })
        .with(Renderable {
            path: "/images/player.png".to_string(),
        })
        .with(Player {})
        .with(Movable)
        .build();
}
