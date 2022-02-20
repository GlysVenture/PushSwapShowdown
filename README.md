# PushSwapShowdown
A dual visualizer for the push_swap project from 42school.

To build from source you need rust installed. Then do:
```
cargo build
```
or if you have the need for SPEED
```
cargo build --release
```

Then to use you need 2 push_swap binaries:
```
./p_swap_showdown ./push_swap1 ./push_swap2 [lower bound] [higher bound]
```

An already compiled Linux binary is available as p_swap_showdown_release0.1\
I will try to provide a MacOS binary when possible.\

Gui is primitive, one button to start/pause (blue) and one to step (pink)\
![alt text](https://github.com/GlysVenture/PushSwapShowdown/blob/master/vis.png)
