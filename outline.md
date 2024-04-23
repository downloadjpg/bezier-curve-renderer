- Draw a line from point a to b

# Milestone:
- Create a Bezier curve that is displayed on the screen. Red squares for control points, blue line for the curve.
- Allow the user to move the control points by dragging them with the mouse.



Thoughts:

Construct the curve out of several line segments, subdivide (de Casteljau's Algorithm)
Allow user to select how far it's subdivided.

Bezier curve object will implement its own render! Maybe!