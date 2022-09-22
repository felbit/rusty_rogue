use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
pub fn player_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState,
) {
    let mut players = <(Entity, &Point)>::query()
        .filter(component::<Player>());

    if let Some(key) = *key {
        let delta = match key {
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            _ => Point::new(0, 0),
        };

        let (player_entity, destination) = players.iter(ecs)
            // map over the query results and return the first result
            .find_map(|(entity, pos)| Some((*entity, *pos+delta)))
            // we know that we have exactly one player entity, 
            // therefore we can unwrap safely (or crash)
            .unwrap();
        
        // is there an enemy?
        let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());
        if delta.x != 0 || delta.y != 0 {
            let mut hit_some = false;
            enemies.iter(ecs)
                .filter(|(_, pos)| **pos == destination)
                .for_each(|(entity, _)| {
                    hit_some = true;
                    commands.push(((), WantsToAttack {
                        attacker: player_entity,
                        victim: *entity,
                    }));
                });

                if !hit_some {
                    commands.push(((), WantsToMove {
                        entity: player_entity,
                        destination,
                    }));
                }
        }

        *turn_state = TurnState::PlayerTurn;
    }

}