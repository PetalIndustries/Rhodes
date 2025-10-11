#[derive(Debug, Clone, Copy)]
pub struct Entity {
  id: u32,
  bitmask: u32
}

struct World {
  entity: HashMap<usize, Entity>,
  components: TypeIdMap, // id = struct

  components_map: TypeIdMap // id = Vec<Entity>
}

fn find_entity<Component, Component2>() {
  let id1 = TypeId::of::<Component>();
  let id2 = TypeId::of::<Component2>();

  let vec1 = world.components_map.get(id1);
  let vec2 = world.components_map.get(id2);

  let entities = vec2.extract(vec1);

}

// pos_id = 0
// gfx_id = 5

// pos bit: 1000 0000 0000 0000 0000 0000 0000 0000
// gfx bit: 0000 0100 0000 0000 0000 0000 0000 0000

// entity bitmasks
// 0 1 2 3
// -------
// 1 0 1 0
// 0 0 0 0
// 0 0 0 0
// 0 0 0 0
// 0 0 0 0
// 0 1 0 1
// . . . .
// . . . .

// pools

// comp_id=0
// ent_id=0 -> pos copy
// ent_id=2 -> pos copy

// comp_id=5
// ent_id=1 -> gfx copy
// ent_id=3 -> gfx copy