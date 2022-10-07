// Topic: Match guards & binding
//
// Summary:
// * A tile-based game requires different logic for different kinds
//   of tiles. Print different messages depending on the kind of
//   tile selected.
//
// Requirements:
// * Bricks:
//   * Colored bricks should print "The brick color is [color]"
//   * Other bricks should print "[Bricktype] brick"
// * Water:
//   * Pressure levels 10 and over should print "High water pressure!"
//   * Pressure levels under 10 should print "Water pressure level: [Pressure]"
// * Grass, Dirt, and Sand should all print "Ground tile"
// * Treasure Chests:
//   * If the treasure is Gold and the amount is at least 100, print "Lots of gold!"
// * Everything else shoud not print any messages
//
// Notes:
// * Use a single match expression utilizing guards to implement the program
// * Run the program and print the messages with at least 4 different tiles

#[derive(Debug)]
enum TreasureItem {
    Gold,
    SuperPower,
}

#[derive(Debug)]
struct TreasureChest {
    content: TreasureItem,
    amount: usize,
}

#[derive(Debug)]
struct Pressure(u16);

#[derive(Debug)]
enum BrickStyle {
    Dungeon,
    Gray,
    Red,
}

#[derive(Debug)]
enum Tile {
    Brick(BrickStyle),
    Dirt,
    Grass,
    Sand,
    Treasure(TreasureChest),
    Water(Pressure),
    Wood,
}

fn main() {
    let brick_tile1 = Tile::Brick(BrickStyle::Dungeon);
    match_tile(brick_tile1);
    let brick_tile2 = Tile::Brick(BrickStyle::Red);
    match_tile(brick_tile2);

    let dirt_tile = Tile::Dirt;
    match_tile(dirt_tile);
    let grass_tile = Tile::Grass;
    match_tile(grass_tile);
    let sand_tile = Tile::Sand;
    match_tile(sand_tile);

    let treasure_tile1 = Tile::Treasure(TreasureChest {
        content: TreasureItem::Gold,
        amount: 123,
    });
    match_tile(treasure_tile1);
    let treasure_tile2 = Tile::Treasure(TreasureChest {
        content: TreasureItem::SuperPower,
        amount: 412,
    });
    match_tile(treasure_tile2);

    let water_tile1 = Tile::Water(Pressure(12));
    match_tile(water_tile1);
    let water_tile2 = Tile::Water(Pressure(5));
    match_tile(water_tile2);
    let wood_tile = Tile::Wood;
    match_tile(wood_tile);
}

fn match_tile(tile: Tile) {
    use Tile::{Dirt, Grass, Sand};

    match tile {
        Tile::Brick(brick @ BrickStyle::Gray | brick @ BrickStyle::Red) => {
            println!("The brick color is {:?}", brick)
        }
        Tile::Brick(other) => {
            println!("{:?} brick", other)
        }
        Tile::Water(pressure) if pressure.0 >= 10 => {
            println!("High water pressure!")
        }
        Tile::Water(pressure) => {
            println!("Water pressure level: {}", pressure.0)
        }
        Grass | Dirt | Sand => {
            println!("Ground tile")
        }
        Tile::Treasure(TreasureChest {
            amount,
            content: TreasureItem::Gold,
        }) if amount >= 100 => {
            println!("Lots of gold!")
        }
        _ => (),
    }
}
