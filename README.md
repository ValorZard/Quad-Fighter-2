# Quad Fighter II: The Globe Trotter

Quad Fighter 2, a "sequel" to the extremely incomplete Quad Fighter, is a 2D fighting game with rollback netcode currently made in Rust with the [macroquad](https://github.com/not-fl3/macroquad) library and using [GGRS](https://github.com/gschup/ggrs) for the rollback netcode backend.

***<u>PULL REQUESTS AND ISSUES WELCOME!</u>***

## Game Goals

For now, here are the main goals for this game. 

- This will be a three button (Light, Medium, Heavy) fighter with quarter circles and "dragon punch" motions. There might possibly be a fourth button (Launcher/Special) similar to Dust in Guilty Gear.
- Despite the obvious reference to Street Fighter in the title, I'm actually much more influenced by anime fighters like Guilty Gear and Blazblue. As such, there will be a cancel system that lets you change moves from light - medium - heavy, with the ability to mix and match so you can go from light to heavy or medium to heavy for instance.
- There will also be Roman/Rapid cancels, as based on Guilty Gear Strive's RC system.
- This will be a super stripped down game. There will only be two characters at the start, and one of them is going to be [Kung Fu Man from MUGEN/IKEMEN](https://github.com/ikemen-engine/Ikemen_GO-Elecbyte-Screenpack) (whose sprites are licensed under creative commons.) The second character will probably be some character that's a mix of Ky Kiske/Melee Marth (swordie shoto with large grab range).

## Tech Stack

As mentioned before, this project uses a combination of technologies.

- We're currently using Macroquad as the backend for rendering graphics and sound. However, I'm looking at [the Bevy Engine](https://github.com/bevyengine/bevy/) and am thinking of switching to it once support for it using GGRS matures. I really think the people working on the Bevy engine are smart and talented, and know way more about making a game engine than I do.
- GGRS is a straight rust port of [GGPO](https://github.com/pond3r/ggpo/) that's really well made and documented quite well. Originally I was going to use [backroll-rs](https://github.com/HouraiTeahouse/backroll-rs) instead, as it has steamworks support and forthcoming web support, but I ultimately decided on using a more lightweight solution for now as we are still in the prototyping stage.
- The physics used for this project is based off of [a fork of the resphys physics library](https://github.com/ValorZard/Resphys-Fixed) which I added fixed point support to for complete determinism. Currently, we're using the serde branch for better serialization support.
- Currently, there isn't a definite code structure for this project as I'm fully focusing on trying to get a functioning minimum viable product working for prototyping. However, I'm planning on using an ecs later on to organize the code better (as well as to make serializing the game state much less a pain in the behind.) If and when we switch to Bevy, we can use the built in systems in it, but for now, I will probably use [Plank ECS](https://github.com/jojolepro/planck_ecs) and other tools from [minigene](https://github.com/jojolepro/minigene) for ECS and gamestate management.

## Contact

- The easiest way to contact me is on Discord. I'm the GGPO discord, the Rust Gamedev discord, the macroquad discord, the Bevy discord, and the FGC Devs discord under the name of ValorZard.
- Else, just post a github issue and I'll try to come back to it.

