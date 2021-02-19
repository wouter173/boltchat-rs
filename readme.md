## Bolt.rs
<sup>the bolt rust library</sup>

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

- [ ] <b>stream layer</b>
- [ ] stream manager
- [ ] multithread the read stream
- [ ] srv support
- [ ] <b>event layer</b>
- [ ] err event
- [ ] join event
- [x] leave event
- [ ] msg event
- [ ] event struct derive (ONLY FROM 0.2.0)
- [ ] <b>client layer</b>
- [ ]	client struct