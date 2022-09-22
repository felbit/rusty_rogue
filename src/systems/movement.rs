use crate::prelude::*;

/* Explain: system(for_each) (Legion)
 * Syntactic sugar for querying and iterating over elements;
 * derives the query from the parameters of the function (here: Entity and 
 * WantsToMove) and runs the function for every matching query result.
 */
#[system(for_each)]
#[read_component(Player)]
pub fn movement(
    entity: &Entity,
    want_move: &WantsToMove,
    #[resource] map: &Map,
    #[resource] camera: &mut Camera,
    ecs: &mut SubWorld,
    command: &mut CommandBuffer,
) {
    if map.can_enter_tile(want_move.destination) {
        /* It's more efficient to use commands, because Legion will batch the
         * updates and run them all at once (faster & more efficient) 
         */
        commands.add_component(want_move.entity, want_move.destination);

        if ecs.entry_ref(want_move.entity)
            .unwrap()
            .get_component::<Player>().is_ok() 
        {
            camera.on_player_move(want_move.destination);
        }
    }
    commands.remove(*entity);
}