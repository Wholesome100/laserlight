# Tech Stack

## Introduction

Laserlight will be made with the Bevy game engine, written in Rust.

For me, this will be the first true game I have made in a while, and to that end I want to make sure:
  - I keep the focus on the core game
  - I learn the fundamental concepts of an FPS
  - I keep it self-contained in the beginning

## Crates

For the game to exist, in its most basic form, I need to be able to move my character, have them interact with the world, and shoot.

### Avian3d

This crate will handle the physics of Laserlight. A dynamic character controller will be created using the features of this crate, and will be the main way players interact with the game world.

### Bevy Enhanced Input

This crate will provide a portable and reusuable input management setup for Laserlight, ideally allowing it to run across a variety of control schemes without issue, and standardizing actions.

### rand

Every FPS has some degree of randomness to it, and this crate provides for it.

## Considerations

Bevy-Replicon was planned for this crate, as multiplayer implementation is a future goal. However, I decided it would be more important to focus on the basics such as setting up the character controller and shooting as opposed to setting up a game server. While this does mean that I will have to write AI bots to have some opponents, there are multiple crates that exist for pathfinding, and the scripts beyond that will be manageable.

A dynamic character controller is simpler, but loses some of the finer tuning of a kinematic character controller. However, it is going from a simpler implementation to a more complex one if a kinematic character controller is needed, and for the time being, a dynamic character controller will serve well without losing any significant progress should we move to kinematic.

## Future Plans

Refactoring the game to be multiplayer will be a significant shift, but will be able to reuse existing assets and systems.
