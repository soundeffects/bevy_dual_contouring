# bevy_dual_contouring
This is a [Bevy](https://github.com/bevyengine/bevy) library/plugin that implements [Manifold Dual Contouring](http://faculty.cs.tamu.edu/schaefer/research/dualsimp_tvcg.pdf) to create a mesh from regular samples of voxel data (chunks).

Currently the project is a work-in-progress, and unusable.

## Credits
[@hmeyer](https://github.com/hmeyer) created the [tesselation](https://github.com/hmeyer/tessellation) library, which this library forks. Credit to them on the initial implementation of the dual contouring algorithm. This plugin is functionally much the same as the original library, but also aims to provide a smooth integration and developer experience with Bevy users.

## License
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option. See both licenses in the `docs/` directory from the root of the project.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
