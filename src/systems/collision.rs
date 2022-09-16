use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
pub fn collision(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    // CommandBuffer (Legion functionality):
    // will take instructions to perform after the system is finished
    // (at the end of a frame)
    // Used here to remove monsters from the game

    // get the Player position
    let mut player_pos = Point::zero();
    let mut players = <&Point>::query().filter(component::<Player>());

    players.iter(ecs).for_each(|pos| player_pos = *pos);

    // get all enemy positions
    let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());
    
    // Check if the player has moved atop an enemy and remove the enemy if so
    enemies.iter(ecs)
        .filter(|(_, pos)| **pos == player_pos) 
                   /* here ^ pos has the type &&Point, because it enters the 
                    * query as a reference and will be referenced again by the 
                    * iterator.
                    */
        .for_each(|(entity, _| {
            commands.remove(*entity);
        }
    );
}