## Boltchat-rs ⚡️
<sup>the bolt rust library</sup>
<sup>Supported Version: V0.2.0</sup><br>
📦 This is the boltchat [crate](https://crates.io/crates/boltchat)<br>
📖 This is the boltchat [documentation](https://docs.rs/boltchat/)

### 🚧 Please note that the library is in extremely early developement. 🚧
It might be broken af, ion kno i aint used it yet. :shrugs:

### inner workings
I'm trying to build the library in 3 layers.

1. the stream layer (lowest)
2. the event layer (middle)
3. the interaction/client layer (highest)

every struct and method has to fit in one of these layers and be built accordingly.
every method and struct should be documented to make clear what everything does.

### TODO
<sup>It is still a lot</sup>

V0.2.2
- [ ] <b>stream layer</b>
- [ ]   multithread the read stream (lmao im never gonna do this)
- [ ]   srv support
- [x]   make crate public
- [x] <b>client layer</b>
- [x]   implement copy trait
- [ ] <b>General</b>
- [x]   fix all todo's
- [ ]   actually learn about error handling instead of unwrapping litteraly everything