# Low earth orbit visualizer

Inspiration: https://orbital-watch-pink.vercel.app/

- Add basic phong lighting on the gloe
  - Use the normal map and the specular map textures
- Render each satellite as a dot:
  - Map SGP4 TEME coordinates to ISTR coordinates, then to InstanceData
- Decide whether to use the daymap or the nightmap based off of the simulated time (current user time?)
- Propagate satellites on a seperate thread
- Click on a satellite to show info on it
- Trace satellite trajectories using a curved line
- Basic UI using imgui:
  - List of all the satellites (name, type, description)
  - Filter satellites by type, by producer, etc
- Zoom in on your GPS position and highlight the swarm of satellites that are overhead
- Port to WASM, release project

---

- Low earth orbit: Orbit altitude of <= 2000 km.
- Drag: An opposing force exterted by the Earth's atmosphere on objects orbiting in LEO.
- SGP4: Simplified general pertubations 4
- TLE: Two-line element set. Although it's the de facto standard, it's very outdated (Y2K problems, running out of satellite identifiers, etc), so instead we'll be parsing CSV
- RAAN: The angle between the vernal equinox and the ascending node.
- The eccentricty of an ellipse is the distance between the two foci [-sqrt(a^2 - b^2), -sqrt(a^2 - b^2)], divided by the semi major axis.

- Periapsis: Point in an orbit closest to the center of mass of a system.
- Apoapsis: Point in an orbit furthest from the center of mass of a system.
  - Perigee and apogee are terms used specifically for Earth.

- Orbital node: When an orbit intersects a plane of reference to which it's inclined.
  - Ascending node: When the orbit intersects the plane of reference moving north.
  - Descending node: When the orbit intersects the plane of reference moving south.

---

- [Coordinate reference frames](https://github.com/horizonanalytic/astrora/blob/main/src/coordinates/frames.rs)
- [Reference frames and coordinate systems](https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/Tutorials/pdf/individual_docs/17_frames_and_coordinate_systems.pdf)
- [Cubemaps](https://learnopengl.com/Advanced-OpenGL/Cubemaps)
- [Shader storage buffer objects](https://ktstephano.github.io/rendering/opengl/ssbos)
- [Creating an icosphere mesh in code](http://blog.andreaskahler.com/2009/06/creating-icosphere-mesh-in-code.html)
- [Sphere.cpp](https://gist.github.com/Pikachuxxxx/5c4c490a7d7679824e0e18af42918efc)
- [Planetary Orbits](https://science.nasa.gov/learn/basics-of-space-flight/chapter5-1/)
- [Supplemental GP Element Sets](https://celestrak.org/NORAD/elements/supplemental/index.php?FORMAT=csv)
- [United States Space Command](https://en.wikipedia.org/wiki/United_States_Space_Command)
- [United States Space Surveillance Network](https://en.wikipedia.org/wiki/United_States_Space_Surveillance_Network)
- [Satellite drag](https://www.swpc.noaa.gov/impacts/satellite-drag)
- [SPACETRACK REPORT NO. 3](https://celestrak.org/NORAD/documentation/spacetrk.pdf)
- [Revisiting Spacetrack Report #3: Rev 2](https://celestrak.org/publications/AIAA/2006-6753/AIAA-2006-6753-Rev2.pdf)
- [Two-line element set](https://en.wikipedia.org/wiki/Two-line_element_sete)
- [A New Way to Obtain GP Data (aka TLEs)](https://celestrak.org/NORAD/documentation/gp-data-formats.php)
- [Classical/Keplerian Orbital Elements](https://www.youtube.com/watch?v=AReKBoiph6g)
- [Introduction to spherical coordinates, Multivariable Calculus](https://www.youtube.com/watch?v=8x_UjFUySRg)
- [Solar Textures](https://www.solarsystemscope.com/textures/)
- [Deep star maps 2020](https://svs.gsfc.nasa.gov/4851)
