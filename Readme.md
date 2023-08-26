
# Spaceglider - Singularity

This will be a simple game, inspired by Starglider from 1987. One of
the first first-person starship shooters, written for Atari ST, Amiga and
IBM PC.

You may be able to relive the experience [here](
https://www.myabandonware.com/game/starglider-95/play-95).

The original game revolves around a story, about freeing an planet
from the Egrons, stopping them to conquer another part of the galaxy. This
game has a different background/motivation. (Just for the sake of
originality.)

While the story, technology and look differs, the game idea will be
similar. Basically the pilot must shoot at enemy vessels, which are
shooting back. He must have a look on his energy level, shields and weapon
configurations in order to achieve a dedicated objective, which is not
overcomplicated made up.

While the energy level can be recharged by "Induction Pads" which must be
flight over in short distance, the physical structural integrity can be
only restored at the home base. This implies a certain risk of being
discovered and destroyed by the enemy.

The induction pads are also used by the enemy and connected with the energy
sources by a grid of microwave lasers. You should avoid to hit any of these
lasers beams, as they will vaporize your vessels within milliseconds.

The numbers and positions of the different enemy vessels are computer
generated in certain ranges, which make sure, that you can get used to the
game at the beginning. More enemies and/or more enemies with more firepower
will be added later through out the game.

There will be just final fight against the last stand of the enemy AI.

It is assumed, that the pilot is integrated into the machine. He get a
360° fisheye view from all sensor data. The center focus is on the area,
reachable with the main weapons.

Please be aware, that this implies not that this is the direction the
vessel is heading to. As we are basically operating in space, the current
direction of view can easily differ from the direction of the gun. An
acceleration will lead to an adaption of the vectors, as the four engines
are firing in exactly the opposite direction of the guns.

These guns are two femtosecond gigawatt lasers. With the integration of the
pilot into the vessel, the eyes and ears of the pilot were adapted, so that
he can "see" the beams and "hear" a sound, any time he fires, hits or is
been hitten by enemy fire.

Spacecrafts operating deep space, are not painted somehow colorful. Color
would just add dead weight and gain nothing in terms of survival factors.
(Nearer to a sun, it is of value to reflect the sun light to avoid
additional need for cooling.) Given, the availability and durability of the
material, spacecrafts are made from different carbon composites and
therefore pitch black.

# Story

The AI won the war. Sooner or later it seemed to be inevitable, that the
machines outsmart the humans. Humans tends to get lazier over time. They
stop to think and just pursue a single purpose. Which is basically wealth
and status, connected with wealth. The welcoming of others are of no
importance for them. Easy game for an AI, making sure, that this is no
problem.

Humans were enslaved as energy efficient workers to assemble even more
computing capacity for them. Mining resources, fixing stuff etc were
machines are good in enhancing the power needed to achieve a task, but have
often not enough data to be successful. And the best, they were no thread
to the AI, as they were made into believing, that this is generating most
wealth and status for them.

A small underground group were able to assemble some vessels, which are
now ready to pick up the fight and cut off the flow of resources, the AI
needs, to enhance further.

In a first step, the mankind has to conquer a moon in the outer system
back. This is the objective of the complete mission.

The guy, how was the source of the brain inside the pod, was used to be a
small child, when he was burned, almost to ashes. He was already human
enough, to feel the attraction to other humans and flexible enough, to be
adapted to the geometry and physics of space flight. He is a member of a
small group of pod pilots, all educated in a facility, similar to the
one described in 'Endors Game'.

# Technically

The application is written in the rust language, using the Glium OpenGL
library.

The mathematics for the screen should be calculated basically on the
graphics card using GLSL-code.

For the input we'll use SDL2, which comes with an decent support for game
devices, which might be useful in the development of the application.

# Building

The building depends on no external resources but rust libraries and the
cargo build system. The release build will be optimized for size.

```sh
cargo build --release
```

# Task List

* Switch to Glium
* Outline only
* Write the Outline
* Read from Blender
* Create the list of enemy vessels
