use vitruvian_types::{entity::Entity, prelude::{Damage, Dice, Name}};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let mut entity = Entity::new();
    entity.add(Name("Test".to_string()));
    entity.add(Damage(Dice::D6));
    
    
    
    Ok(())
}
