use crate::prelude::*;

#[system]
#[read_component(WantsToAttackk)]
#[write_component(Health)]
pub fn combat(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut attackers = <(Entity, &WantsToAttackk)>::query();
    let opponents: Vec<(Entity, Entity)> = attackers
        .iter(ecs)
        .map(|(entity, attack)| (*entity, attack.opponent))
        .collect();
    opponents.iter().for_each(|(message, opponent)| {
        if let Ok(health) = ecs
            .entry_mut(*opponent)
            .unwrap()
            .get_component_mut::<Health>()
        {
            println!("Health before attack: {}", health.current);
            health.current -= 1;

            if health.current < 1 {
                commands.remove(*opponent);
            }

            println!("Health after attack: {}", health.current);
        }
        commands.remove(*message);
    })
}
