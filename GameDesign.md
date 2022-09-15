# Rusty Rogue :: Game Design Document

## Description
A dungeon crawler with procedurally generated levels, monsters of increasing difficulty, and turn.based movement.

## Story
The hero's hometown is suffering from a plague of monsters. Welling up from the deep, they seem unstoppable. Legend tells of the Amulet of Yala - Yet Another Lost Amulet - that can be used to stem the tide. After a long night in the tavern, the hero promises to save the day - and sets forth into the dungeon.

## Basic Game Loops
1. Enter the dungeon level
2. Explore, revealing the map
3. Encounter enemies that the player may fight or flee from
4. Find power-ups and use them to strengthen the player character
5. Locate the level exit - go to 1.

## Enemies

### Goblin
Angry, but cowardly, small green humanoids that attack in groups of at least two, but run away when alone. They prefer throwing stones from afar and tend to run away when you come too close; attention: Will try to overwhelm you in melee combat when in hordes of more then 5.

## MVP
- [x] Create a basic dungeon map
- [x] Place the player and let them walk around
- [ ] Spawn monsters, draw them, and let the player kill them by walking into them
- [ ] Add health and combat system
- [ ] Add health potions
- [ ] Display a "game over" screen when the player dies
- [ ] Add the Amulet of Yala to the level and let the player win by reaching it

## Stretch Goals
- [ ] Add Fields-of-View
- [ ] Add more interesting dungeon designs
- [ ] Add dungeon themes
- [ ] Add multiple layers to the dungeon (multiple levels), with the Amulet on the last one
- [ ] Add varied weapons to the game
- [ ] Move to a data-driven design for spawning monsters
- [ ] Consider some visual effects to make combat more visceral
- [ ] Consider keeping score
