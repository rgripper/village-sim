
Every week is Sa-Fr.

Previous week:



1. ~~Build hexagons, with positioning~~
2. ~~Seed trees (use existing tree svg, TODO texture)~~

Previous week:



1. ~~Seed villagers (Use a stub texture “textures/man.png”)~~
2. ~~Create star_pos struct~~
3. ~~Make trees grow (start with trees with random ages)~~
4. ~~Make trees seed other trees~~

Previous week:



1. ~~Make trees and villages drop shadows (a shadow looks like this: gray circular gradient placed underneath the main texture, sized according to the texture)~~

Previous week:



1. ~~Add environmental sound: birds.~~
2. ~~Housing~~
    1. ~~Game starts with every villager being Homeless~~
    2. ~~Assign villagers to houses~~
    3. ~~Keeps track of homelessness and vacant houses~~
3. ~~Figure out what is the center of an object, eg Villager - is it their legs or the center of their body? TODO: fix Tree and Villager rendering to be the same - they are rendered differently right now~~

Previous week:



1. ~~Implement movement of a Creature along a path of Vec2 nodes (starts movement, accelerates, walks with max speed, then decelerates and completely stops)~~
2. ~~A villager can to go for a walk/wander around (according to “Idling” section)~~
    1. ~~Pick a point to go to~~
    2. ~~Walk there~~
    3. ~~Pick a new point~~
    4. ~~Walk there~~
    5. ~~Repeat steps above~~

Previous week:



1. ~~Set up deployment to web (Github Pages)~~
    1. ~~Make it build into a standalone folder~~
    2. ~~Add a script to host on GitHub Pages~~
    3. ~~Make sure it’s working :-)~~
2. ~~Provide a “wooden log” texture~~

This week:



1. ~~(In progress, Vladimir) A villager can cut trees: Villager does the following:~~
    1. ~~Pick a tree to cut~~
    2. ~~Move to a tree~~
    3. Provide some cutting status animation (eg, a shader for the tree to overlay it with yellow color with 0.3 alpha channel)
    4. Pick wood (we should draw a “wooden log” texture on the ground)
    5. ~~Carry wood to the nearest Storage (use starting_point for now as a Storage) (also, draw the “wooden log” texture over the character’s torso)~~
    6. ~~Drop wood in the Stockpile~~
2. Day/night cycle
    1. ~~Track 24h day cycle (something like SimTime Resource)~~
    2. Calculate light level (as a Resource). (find some library or calc, e.g. pick any coordinates on the planet)
    3. ~~Display time at the top center of the game window~~

Backlog

1. Sleeping
    1. After 19:00 every second {(5..10)/600} gets added to {fatigue}. 
    2. Every second there is a {tiredness/600}% chance to go Sleeping. 
    3. When going to sleep, the character navigates to the house they are assigned to, or to a random spot on the ground nearby (if no house is built for them yet).
2. Refactor entity generation to respect collision boxes. Maybe: https://github.com/RustyStriker/bevy_physimple

Backlog (unrefined)



1. Building a House
2. Building a Storage

So far that is it for MVP of this microproject.


## Coming in the next microproject

Basic enemy AI

Fighting mechanics

Scrollable map (Maybe [https://crates.io/crates/bevy_fly_camera](https://crates.io/crates/bevy_fly_camera))

Tooling (part 1): Character animation tool


## Then coming in the next microproject

Convert current microproject to multiplayer!!! 🥳💪


## Future topics

Biomes (part 1): soil, vegetation types

(Possibly) Moving away from hex grid to squares

Camera zoom in/out


[https://bevyengine.org/news/bevy-0-5/#orthographic-camera-scaling-modes](https://bevyengine.org/news/bevy-0-5/#orthographic-camera-scaling-modes)

Player Resources UI

Character/Plant texture/art generators

Weather conditions