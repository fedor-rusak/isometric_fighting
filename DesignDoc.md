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

It is really beautiful thing to have clippy and rustfmt as simple tools out of the box. One can suggest more idiomatic ways to write code and use less mut variables which is great. Another one forces specific styleguide which is really useful as I would forget about this at some moment.

& stands for borrowing yet not destroying parameter after method execution. Nice!

Methods as separate impl block sounds nice buuuut I would like to go for simple function when I can.

Gots some wishes about destructuring but that is a super minor issue.

Keeping code at least somewhat readable is hard, yet it give opportunity to add features easily. And rust feels like a nice tool to refactor existing code even if you forgot some details about its logic.

I still have some fun experience with this little project. And while I do some small steps, author and other contributors do different things and I should not forget to update GGEZ package :).

# Things to try

Do some perspective rendering and image movement. Done!

Do some keyboard keys work to allow smooth yet limited movement with end lag for moves. Done!

Need some speed calculations to move according to floor tiles. Currently tile size is 100x60. Is it optimal? UPDATE: who knows? But moving across the lines is niiice.

Some nice pictures would be really sweet. But not urgent. UPDATE! So Got some stupid result. yet I got the feeling I need some modeling software with orthogonal rendering. Because I need parallel lines to be parallel and perspective means that these lines would cross... Still some progress :) UPDATE! So now it is more like abstract figures but why not?! Update! Now with actually different images for 2 different angles :D. My head nearly fell off while I tried to reinvent the idea behind this. Like ellipses and rotation and crossing points waaah! Now avatar cube has no transparent sides!

Not sure abour resizing and rescaling things out here. By default window is 800x600 and this seems to be too small nowadays. I mean like how to draw on bigger screens and keeps rations and feel secure about using non-standard (4:3) resolutions. something like Zoom? Something like camera to operate with?

Some UI? Like health or map part or something? Not exactly action-rpg like but at least location name and fps counter would be sweet!

Sound! It was quite easy to add just one sound for moving avatar! But what about something like background music? So background sound and steps mixing works! Done!

And what about pausing a game?

My daughter keeps moving box away from screen so I had to add space handling to get it back to center! Done!

Performance even with something like 400 renders of one sprite sucks royally. Yet this is solved by a bit longer compilation with optimisations. Done!

So last addition was rather huge so let me explain the idea. I wanted to move cube around and make tile under it change color. Sounds easy right? Well not so fast. First we made a new tile and I got to the point where my simple rendering just was not enough as it was drawing piece in checkers board style. To fix this I had to change many things, but it can be summarized as beginning of projection camera approach. What is camera? Glad you asked.

There is some 3d world, but when rendering a 2d image of it (to show it on screen you know?) we have to transform 3d coords to some 2d coords. This is called projection. And this projection must be done for some theoretical viewer that is almost like camera man. Henth camera approach. In current state camera shows the center part of grid 20x20 which is playground for cube avatar.

Technical details are like this. I have to introduced normal 3d coords (z is always 0 now), and movement have to be described as moving in this 3d coords and not some screen coords dependant on screen size. This is somewhat difficult to describe easier yet this is really improtant to show many objects and introduce things like physics and collision and predictable movement.

Movement had to be redone as previously it was working with axis, and now this is converted to inputs which are later converted to specific combinations of axis values and special modifier for diagonal movement. Because moving left and right at the sametime should have same length as left OR right. So 1 step for straight movement is like sin45 for two axis which is like 0.85. Sounds clunky but believe me it should be like that.

So now if I would want to do something like zooming it would mean changes in some places but it would be in update and render and not in input handling part.

One more thing, as previously I was working with screen pixels, and had to move in 3d worls, I had to give some dimensions to tiles that I had. I decided that it should be like 30 pixeled square. And this is part of movement calculation now. So technically I can change speed or this edge_length to control the movement around the field.

Got some progress here. When introducing camera concept you should be aware how it actually changes rendering part. As you want sort of transformm pixel coordinates according to camera "part" you are supposed to see on screen. And if you make it right then you can have this beautiful illusion when some camera moves around with your character if it was sort of glued to it. It also provides easier navigation as your main character is in the center all the time.

My attempts to make avatar keep direction after all buttons were released almost failed. As it is really hard to release 2 buttons at the same millisecond. But I guess this part will be reworked in future when some delaying and combinations will become necessary.