# Fizzerb

A reverb that actually simulates sound bouncing around in a space, using 2D path tracing and the
principles of physically-based rendering.

## Usage

Fizzerb is still in its infancy, but if you want to try out how it sounds, you can do so by using
the following command:
```
$ cargo run --release
```
Hitting the **Render** button will result in a .wav sample being output to the current working
directory, which corresponds to the impulse response (IR) of the simulated space. You can then
convolve the IR with any sound you'd like, although don't be surprised if the results are a bit…
underwhelming at the moment.

