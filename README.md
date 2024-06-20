# Space Adventure
## My experiments with Rust & Bevy
### Intro
I involved myself in learning Rust via Bevy, and in the process of learning I started making this game (or not game). At least  i playing in making game .;)  
I don’t think this should be taken as a basis, but if my code helps someone, I’ll be glad.  
In addition, I will gratefully accept any advice and directions.

### I express my gratitude to the authors of all the crates that I used.

![screenshot](img/image.png)
## How to build
cargo build --release

## About game
Everything is simple and usual.  :)  
Space (overlooking something green), you (in spaceship clumsily made in Blender), asteroids (advancing inexorably on all hands), square surface at Vec3::ZERO (as placeholder an as yet unbuilt space base), several gas stations (How can we live without them, everyone eats fuel :) ) and that's all for now. 

### Gameplay
At the moment you can chase asteroids and destroy them.  (you have unguided missiles (temporary unlimited ) and laser that consumes fluel)
Hint: Click on asteroid makes it the target for spaceship (at least  it helps in targeting in 3rd person view).  
Hint: Arrow at the top right corner points to target.  
Hint: Dist XZ and Dist Y on top - distance to target in according planes.  
Hint: Strange looking spheres near Vec3::ZERO  is a gas stations  (when you fluel level less 20%  your ship target will be closest station ).  

### Controls
#### View
1 - Third Person  
2 - Back  
3 - Top  
4 - Left  
5 - Right  
6 - Orbit Camera  

#### Movement
W - Forward  
S - Back  
A - Turn Left  
D - Turn Right  
ArrowUp - Up  
ArrowDown - Down  
ArrowLeft - Turn Left (slow)  
ArrowRight - Turn Right (slow)  
B - Break  

#### Weapon
ControlLeft - Launch a missile  
ControlRight - Laser Shot  

To be continued...
