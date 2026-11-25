# N-Body Simulations

This is a programming learning project.\
AI is used as a search engine. The code is my own.\
The goal is to code various n-body simulation techniques in C and Rust.\
This is not a performance comparison. The code won't necessarily be equivalent in both languages and I use GCC for C while Rust backend is LLVM.

The simulations are done using newtonian gravitation applied to the Solar System.\
I currently use **N = 27**: the Sun, the planets and various satellites.\
Masses and radii are from Wikipedia.\
Positions and velocities are from NASA Horizons data for 26-09-2026 at 00:00:00.0000 TDB using the Solar System Barycenter.


## Simple Euler method

(see Euler/)

$$
\begin{aligned}
\mathbf{v}_i(t_{n+1}) &= \mathbf{v}_i(t_n) + \mathbf{a}_i(t_n)\,\Delta t \\
\mathbf{r}_i(t_{n+1}) &= \mathbf{r}_i(t_n) + \mathbf{v}_i(t_n)\,\Delta t
\end{aligned}
$$

Single-threaded.

## Velocity Verlet method

**TODO**
