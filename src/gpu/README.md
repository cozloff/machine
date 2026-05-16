This folder is for setting up cuda
- GPU 1: RTX 3060
- GPU 2: RTX 2000 Ada Generation

`gpu_test.cu` prints the available CUDA device capabilities.

`rho_guesser.cu` contains a simple high-throughput reciprocal-space initial
charge-density guesser. It mirrors the plane-wave DFT idea behind
`atomic_rho_g`: each CUDA thread owns one G-vector and accumulates the
superposition of atomic form factors into `rho(G)` without atomics.

Test github account
