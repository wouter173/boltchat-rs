## Bolt.rs
<sup>the bolt rust library</sup>
<sup>Supported Version: V0.1.0</sup>

### 🚧 Please note that the library is in extremely early developement. 🚧
It litteraly does not work yet, I am still working on it.
I mean it straitup does not support the latest version of bolt.

### inner workings
I'm trying to build the library in 3 layers.

1. the stream layer (lowest)
2. the event layer (middle)
3. the interaction/client layer (highest)

every struct and method has to fit in one of these layers and be built accordingly.
every method and struct should be documented to make clear what everything does.

### TODO
<sup>It is a lot</sup>

V0.1.0
- [x] <b>stream layer</b>
- [x]   stream manager
- [ ]   multithread the read stream
- [ ]   srv support
- [x] <b>event layer</b>
- [x]   err event
- [x]   join event
- [x]   leave event
- [x]   msg event
- [x] <b>client layer</b>
- [x]	client struct