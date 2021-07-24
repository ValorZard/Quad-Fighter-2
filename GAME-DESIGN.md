# Game Design

The idea behind this project is to be a tutorial on how to make your own fighting game with rollback netcode.

This will be heavily, and when it's finished or at least in a basic state, I want it all translated into Japanese as well.

Since it has a three button layout and magic series, plus air dashes, I want a Marvel vs Capcom / Guilty Gear feel.
## Code base

Because it is ECS, it should be very modular. Entire moves and functions should be able to be shared between multiple characters, since all characters are just a collection of data. This could prevent having to have boilerplate attack code for each player, and only focus on implementing special functions for specials/supers that are unique.

One possible result of this is that you could have a character that copies moves, and it would be really easy to implement because you could just call the system associated with the character your  "stealing" moves from.

Rust should be First Party, and anyone should be able to make the character they want to make with the Rust API.

# MAKE SOMETHING WORK FIRST, MAKE IT PRETTY LATER.
## THIS INCLUDES ART

## Game Design

The player is able to either stay still (idle), crouch, jump, or move around on the ground or air.
They can also be knocked down, and then get up.
They can also dash on both the air and ground.

There should be two characters, to demonstrate you can make more than one. However, the other character should just be a semi clone (maybe Evil Kung Fu Man)

List of moves (Kung Fu Man):
KFM sprites are CC-BY-NC
Sprite Sheet: https://www.spriters-resource.com/pc_computer/mugen/sheet/83414/
- EDIT THESE SPRITES TO FIT THE MOVESET. NOTHING IS SACRED.
Movelist: https://mugen.fandom.com/wiki/Kung_Fu_Man/Elecbyte%27s_fourth_version

After taking a good look at his moveset and trying him out, I actually think that KFM is fine as he is. The main changes would just be adding projectiles to his palm attacks and adding gatlings, plus RCs.



### Custom Moveset Ideas
Normals: 
- crouching(Light, Medium, Heavy, Launcher[Knockdown])
- standing(Light, Medium, Heavy, Launcher[Launches opponent into air for air combo])
    - Launcher uses the animation for Kung Fu Upper. Hits High.
- air(Light, Medium, Heavy, Launcher[I dont know whats special about this yet])
Grabs:
- both standing and air forwards/backwards Launcher
Special Moves:
- Kung Fu Palm (quarter circle forward Light/Medium/Heavy) - releases a projectile, but speed/ strength depends on the strength of button pressed.
- Kung Fu Knee (quarter circle back Light/Medium/Heavy) - Bandit Bringer, Kung Fu Man edition.
- Kung Fu Zankou (dash attack Light/Medium/Heavy) - unlike the top two, I feel like this should be the same strength no matter what. Might change my mind later.
Super Move:
- Triple Kung Fu Palm (double quarter circle forward Launcher) - releases a bunch of projectiles or something, IDK.
(Launcher is like the Launcher from MvC3/Dust from Guilty Gear)
Additionally, the Kung Fu Palm move should throw out a fireball.
If possible, a Roman cancel mechanic that spends 50% meter would also be cool.

* On the ground they have access to 4 buttons
    * Light, Medium, and Heavy, as well as "Launcher" 
        * 5 Light is a move that has three different "states" in it
            * start up lag (The move animation starts up)
                * Getting attacked during this counts as a "counter hit"
            * the actual move hitboxes coming out
                * These do damage on the other entities that it comes in contact with
                * On attack, the entity getting attacked gets put into hitstun
                    *  a period of time after being hit by an attack that a character is unable to act
                    *  the damaged entity will also be knockedback by a set amount
            * end lag (the move animation ends)
                * Getting attacked during this is called a "whiff punish"
                * You can also cancel this lag into a stronger move, like 5 Medium and 5 Heavy.
            * Example
            https://ssb.wiki.gallery/images/b/be/PikachuBAirSSBU.gif
        * 5 Medium is essentially the same thing as Light, except it does a bit move damage and knockback
        * Same thing with Heavy.
        * Special generates a fireball, that damages anybody that comes into contact with it and gets destroyed on it. Also does a bit of hitstun and knockback.


* Being in the air gives you access to different moves, like
    * Air Light, Medium, and Heavy
    * Specials stay the same though
* Additionally, being in the air changes your movement speed and acceleration

There might be other states (such as super moves), that gives the player access to different moves as well.