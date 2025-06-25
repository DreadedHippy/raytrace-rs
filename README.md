Simple graphics renderer from scratch in Rust.

# What I've learned.
- Whenever you start a renderer, you need a way to see an image. The most straightforward way is to write it to a file.
- There are many file formats but for the purpose of this book, we will start with a plain text ppm file
- For the rendering of the img format itself:
	1. The pixels are written out in rows.

	2. Every row of pixels is written out left to right.

	3. These rows are written out from top to bottom.

	4. By convention, each of the red/green/blue components are represented internally by real-valued variables that range from 0.0 to 1.0. These must be scaled to integer values between 0 and 255 before we print them out.

	5. Red goes from fully off (black) to fully on (bright red) from left to right, and green goes from fully off at the top (black) to fully on at the bottom (bright green). Adding red and green light together make yellow so we should expect the bottom right corner to be yellow.
- Almost all graphics programs have some class(es) for storing geometric vectors and colors. In many systems these vectors are 4D (3D position plus a homogeneous coordinate for geometry, or RGB plus an alpha transparency component for colors). For our purposes, three coordinates suffice. ..(hence the Vec3 class)
- Converting a floating point value in the range [0, 1] into an int value in the range [0, 255], multiply by 255.999 and cast to int.
	- Why not just 256?
			Because, 255 is the max representation in an 8 bit integer which you want, 256 would overflow
	- Ok then why not just use 255?
			Because, only 1.0 would equate to 255 but with 255.999, values close to 1.0 can also evaluate to 255
-  The one thing that all ray tracers have is a ray class and a computation of what color is seen along a ray. Let’s think of a ray as a function:  
`P(t) = A + tb`  
Here `P` is a 3D position along a line in 3D. `A` is the ray origin and `b` is the ray direction. The ray parameter `t` is a real number (f64 in the code). Plug in a different `t` and `P(t)` moves the point along the ray. Add in negative `t` values and you can go anywhere on the 3D line. For positive `t`, you get only the parts in front of `A``, and this is what is often called a half-line or a ray.
- At its core, a ray tracer sends rays through pixels and computes the color seen in the direction of those rays. The involved steps are:
	1. Calculate the ray from the “eye” through the pixel,
	2. Determine which objects the ray intersects, and
	3. Compute a color for the closest intersection point.
- When first developing a ray tracer, we should do a simple camera for getting the code up and running.
- We'll be rendering images in the 16:9 aspect ratio
- Since we have a given aspect ratio in mind, it's easier to set the image's width and the aspect ratio, and then using this to calculate for its height. This way, we can scale up or down the image by changing the image width, and it won't throw off our desired aspect ratio. We do have to make sure that when we solve for the image height the resulting height is at least 1.
- In addition to setting up the pixel dimensions for the rendered image, we also need to set up a virtual viewport through which to pass our scene rays.
- The viewport is a virtual rectangle in the 3D world that contains the grid of image pixel locations. If pixels are spaced the same distance horizontally as they are vertically, the viewport that bounds them will have the same aspect ratio as the rendered image.
- The distance between two adjacent pixels is called the `pixel spacing`, and square pixels is the standard.
- If you're wondering why we don't just use `aspect_ratio` when computing `viewport_width`, it's because the value set to `aspect_ratio` is the ideal ratio, it may not be the actual ratio between image_width and `image_height`. If `image_height` was allowed to be real valued—rather than just an integer—then it would be fine to use `aspect_ratio`.
- For simplicity we'll start with the camera center at \((0,0,0)\). We'll also have the y-axis go up, the x-axis to the right, and the negative z-axis pointing in the viewing direction. (This is commonly referred to as right-handed coordinates.)
	![alt text](right_handed_coordinates.png)
- I'll use a standard graphics trick to linearly scale `0.0 ≤ a ≤ 1.0`. When `a = 1.0`, I want blue. When `a = 0.0`, I want white. In between, I want a blend. This forms a “linear blend”, or “linear interpolation”. This is commonly referred to as a **lerp** between two values. A lerp is always of the form:  
	`blendedValue = (1-a) • startValue + a • endValue`  
  where `a` goes from zero to one.
- The equation for a sphere of radius \(r\) that is centered at the origin is an important mathematical equation:  
	`x^2 + y^2 + z^2 = r^2`  
  You can also think of this as saying that if a given point `(x,y,z)` is on the surface of the sphere, then `x^2 + y^2 + z^2 = r^2`. If a given point `(x,y,z)` is inside the sphere, then `x^2 + y^2 + z^2 < r^2`, and if a given point `(x,y,z)` is outside the sphere, then `x^2 + y^2 + z^2 > r^2`.  
	If we want to allow the sphere center to be at an arbitrary point	`(C_x, C_y, C_z)`, then the equation becomes a lot less nice:  
	`(C_x - x)^2 + (C_y - y)^2 + (C_z - z)^2 = r^2`
- Since we have conveniently created our `Vec3` class, we can easily realize that given a vector `C` with coordinates`(C_x, C_y, C_z)`, and another, `p` with coords `(x, y, z)`, their `C - p` = `(C_x - x, C_y - y, C_z - z)`, and by using dot product of two vectors:  
`(C-p) • (C-p)` = `(C_x - x)^2 + (C_y - y)^2 + (C_z - z)^2`  
which means that:  
`(C-p) • (C-p) = r^2`  
We can read this as "any point `p` which satisfies the equation above is on the sphere"
	- Because `p` is a function of `t`, `p` = `P(t)`, giving us  
	`(C-P(t)) • (C-P(t)) = r^2`  
	- Replacing `P(t)` with it's full form gives us  
	`(C-(Q + t*d)) • (C - (Q + (t*d))) = r^2`  
	- Expanding terms 1 and 2  
	`(C - Q - t*d) • (C - Q - t*d) = r^2`  
	=> `(-td + (C-Q)) • (-td + (C-Q)) = r^2;  
	- Following the rules of vector algebra, we distribute the product:  
	`t^2d⋅d − 2td⋅(C−Q) + (C−Q)⋅(C−Q) = r2`
	See image below for full explanation:  
	![alt text](image.png)
	- The only vectors that we have are reduced to scalars by dot product. The only unknown is t, and we have a t^2
	, which means that this equation is quadratic. You can solve for a quadratic equation ax2+bx+c=0
	by using the quadratic formula:  
	`(−b ± sqrt(b^2−4ac))/2a`

## Shading
- First, let’s get ourselves a surface normal so we can shade. This is a vector that is perpendicular to the surface at the point of intersection.
- All normal vectors will be of unit length
- For a sphere, the outward normal is in the direction of the hit point minus the center e.g the vector from the earth's center to you point's straight up
- Without lights, we can visualize the normals with a color map.
- A common trick for visualizing normals (because it’s easy and somewhat intuitive to assume n is a unit length vector — so each component is between −1 and 1) is to map each component to the interval from 0 to 1, and then map (x,y,z) to (red,green,blue)
- We are currently calculating just whether the ray hit the sphere or not, for the normal, we'd need to know the hit point.
- Because we only have one sphere directly in front of the camera, no need to worry about negative values of `t` yet.
- The closest hit point i.e smallest `t` is the point we want.
- The second design decision for normals is whether they should always point out.
- Currently, if the ray intersects the sphere from the outside, the normal points **against** the ray. If the ray intersects the sphere from the inside, the normal (which always points out) points **with** the ray. 
- Alternatively, we can have the normal always point against the ray. If the ray is outside the sphere, the normal will point **outward**, but if the ray is inside the sphere, the normal will point **inward**.
- If we decide to have the normals always point out from the center of the object, then we will need to determine which side the ray is on when we color it. We can figure this out by comparing the ray with the normal. If the ray and the normal face in the same direction, the ray is inside the object, if the ray and the normal face in the opposite direction, then the ray is outside the object. This can be determined by taking the dot product of the two vectors, where if their dot is positive, the ray is inside the sphere.
- We can set things up so that normals always point “outward” from the surface, or always point against the incident ray. This decision is determined by whether you want to determine the side of the surface at the time of geometry intersection or at the time of coloring. In this book we have more material types than we have geometry types, so we'll go for less work and put the determination at geometry time. This is simply a matter of preference.
----
- To shade a cube, the normal of the cube is simply perpendicular to the face hit, so we can check for the axis of the hitpoint and get the normal from that face to use in our HitRecord
## Interval
- Now we'll implement real-valued intervals with a minimum and maximum
- This helps to keep track of ray min and max precisely, essentially cleaner code
## Camera
- Now, we consolidate our camera and scene-render code into a single new class: the `camera` class
- Camera does the following:
	1. Construct and dispatch rays into the world.
	2. Use the results of these rays to construct the rendered image.
## Antialiasing
- The stair-stepping in the generated images is commonly referred to as “aliasing”, or “jaggies”.
- When a real camera takes a picture, there are usually no jaggies along edges, because the edge pixels are a blend of some foreground and some background because unlike our rendered image, a true image has infinite resolution.
- We can get the same effect by averaging a bunch of pixels
- With a single ray through the center of each pixel, we are performing what is commonly called **point sampling**.
- The problem with **point sampling** can be illustrated by rendering a small checkerboard far away. If this checkerboard consists of an 8×8 grid of black and white tiles, but only four rays hit it, then all four rays might intersect only white tiles, or only black, or some odd combination. In the real world, when we perceive a checkerboard far away with our eyes, we perceive it as a gray color, instead of sharp points of black and white.
- Because our eyes are naturally doing what we want our ray tracer to do: integrate the (continuous function of) light falling on a particular (discrete) region of our rendered image.
- We want to sample the light falling around the pixel, and then integrate those samples to approximate the true continuous result. So, how do we integrate the light falling around the pixel?
- We'll adopt the simplest model: sampling the square region centered at the pixel that extends halfway to each of the four neighboring pixels.
- For a single pixel composed of multiple samples, we'll select samples from the area surrounding the pixel and average the resulting light (color) values together.
- To do this, we'll add the full color from each iteration *(read sample)*, and then finish with a single division (by the number of samples) at the end, before writing out the color.
To ensure that the color components *(r, g, b)* of the final result remain within the proper [0,1] bounds, we'll clamp it to [0, 1]



