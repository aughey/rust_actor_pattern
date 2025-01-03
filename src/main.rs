use anyhow::Result;
use rust_actor_pattern::{
    his,
    mine::{self, Generator as _},
};
#[tokio::main]
async fn main() -> Result<()> {
    {
        let actor = his::MyActorHandle::new();

        let id1 = actor.get_unique_id().await;
        let id2 = actor.get_unique_id().await;
        println!("id1: {}, id2: {}", id1, id2);
    }
    {
        let generator = mine::create_const_id_generator();
        let id1 = generator.get_unique_id().await?;
        let id2 = generator.get_unique_id().await?;
        println!("id1: {}, id2: {}", id1, id2);
    }
    {
        let generator = mine::create_locked_id_generator();
        let id1 = generator.get_unique_id().await?;
        let id2 = generator.get_unique_id().await?;
        println!("id1: {}, id2: {}", id1, id2);
    }
    {
        let (actor, generator) = mine::create_actor_id_generator();
        let id1 = generator.get_unique_id().await?;
        let id2 = generator.get_unique_id().await?;
        println!("id1: {}, id2: {}", id1, id2);
    }
    Ok(())
}
