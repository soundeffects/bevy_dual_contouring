# bevy_dual_contouring
This is a [Bevy](https://github.com/bevyengine/bevy) library/plugin that implements [Manifold Dual Contouring](http://faculty.cs.tamu.edu/schaefer/research/dualsimp_tvcg.pdf) to create a mesh from regular samples of voxel data (chunks).

Currently the project is a work-in-progress, and unusable.

## Credits
@hmeyer on GitHub created the tesselation library, which this library forks. Credit to him on the initial implementation of the dual contouring algorithm. Code written by them is labeled with a comment.

I wanted to specifically integrate with Bevy, which is not a design goal of the initial project, so I forked it to create a new Bevy plugin.

## License
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option. See both licenses in the `docs/` directory from the root of the project.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
