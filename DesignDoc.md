# Main inspiration

This idea came to me in form of animations from some of games I enjoyed.

1) Super Smash Bros (2d fighting) Cloud/CapFalcon down smash. If performed right opponent will be hit once then will be "grabbed" behind and hit once again.

2) Diablo2/PathOfExile movement which happens on a flat surface yet camera is set in certain perspective to allow comfortable movement and actions in 8 directions.

3) RocketLeague as a feel for game where you interact with whole level and other players to achieve your goal or just marvel on bizzare clashes and crashes.

Of course I could mention something like Ninja Turtles fighing or Kunio Kun or Double Dragon or Streets of Rage. But these game actually so invested in multi-lane uni-dimensional actions/movement that it hurts variety in things you could do.

# Random thoughts

Just actual "fun" brawler in isometric perspective. Where you could kick things, grab opponents, do crazy things like "death dance" from D2 or some combo style hit-sequences.

Dodge and block as useful actions to defend yourself. Projectiles, explosives, traps, pits.

Actually things that I don't want to see are some corridors or narrow labyrinths. Yet this is ok as long as it is fun.

# Random technical thoughts

So by default alpha blending is on and this really simplifies things for starter!

Keyboard works a bit clunky due to check for sticky presses (I guess???). But it is for testing and NOT for final results.

Should I think about dynamic lighting? It requires some deep thoughts to make them sort-of-like-real. Like fires and torches and fireballs and reflections? Naaah screw it! I am not making another Avengers movie %) .

But some form of shadows is a must. as it would create desired illusion of volume and depth!

# Things to try

Do some perspective rendering and image movement. Done!

Do some keyboard keys work to allow smooth yet limited movement with end lag for moves. Done!

Need some speed calculations to move according to floor tiles. Currently tile size is 100x60. Is it optimal? UPDATE: who knows? But moving across the lines is niiice.

Some nice pictures would be really sweet. But not urgent. UPDATE! So Got some stupid result. yet I got the feeling I need some modeling software with orthogonal rendering. Because I need parallel lines to be parallel and perspective means that these lines would cross... Still some progress :)

Not sure abour resizing and rescaling things out here. By default window is 800x600 and this seems to be too small nowadays. I mean like how to draw on bigger screens and keeps rations and feel secure about using non-standard (4:3) resolutions. something like Zoom? Something like camera to operate with?

Some UI? Like health or map part or something? Not exactly action-rpg like but at least location name and fps counter would be sweet!