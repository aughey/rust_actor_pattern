use anyhow::Result;
use rust_actor_pattern::{
    alice,
    mine::{self, Generator},
};

#[tokio::main]
async fn main() -> Result<()> {
    {
        let actor = alice::MyActorHandle::new();

        let id1 = actor.get_unique_id().await;
        let id2 = actor.get_unique_id().await;
        println!("Alice: id1 = {}, id2 = {}", id1, id2);
    }
    {
        let mut generator = mine::create_u32_id_generator();
        let id1 = generator.get_unique_id().await?;
        let id2 = generator.get_unique_id().await?;
        println!("Const: id1 = {}, id2 = {}", id1, id2);
    }
    {
        let mut generator = Some(42);
        let id1 = generator.get_unique_id().await;
        let id2 = generator.get_unique_id().await;
        println!("Optional: id1 = {:?}, id2 = {:?}", id1, id2);
    }
    {
        let mut generator = mine::create_locked_id_generator();
        let id1 = generator.get_unique_id().await?;
        let id2 = generator.get_unique_id().await?;
        println!("Locked: id1 = {}, id2 = {}", id1, id2);
    }
    {
        let (actor, mut generator) = mine::create_actor_id_generator();
        tokio::spawn(actor);
        let id1 = generator.get_unique_id().await?;
        let id2 = generator.get_unique_id().await?;
        println!("Actor: id1 = {}, id2 = {}", id1, id2);
    }
    {
        let mut generator = mine::create_iterator_id_generator((0..).filter(|x| x % 2 == 0));
        for _ in 0..20 {
            let id = generator.get_unique_id().await;
            println!("Iterator: id = {:?}", id);
        }
    }

    Ok(())
}
