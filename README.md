# Constant multiplicaton

This is a Python library containing a number of utilities related to constant (shift-and-add) multiplication.

- The minimum number of additions/subtractions required for an integer coefficient
- The possible structures to realize an optimal constant multiplication by an integer coefficient

It is a reimplementation in Rust with Python bindings, based on the original work in

- O. Gustafsson, A. G. Dempster, K. Johansson, M. D. Macleod, and L. Wanhammar, "Simplified design of constant coefficient multipliers," *Circuits Syst. Signal Process.*, vol. 25, no. 2, pp. 225–251, 2005. <https://doi.org/10.1007/s00034-005-2505-5>

All integers with up to 19 bits is included.

## Citation

To cite the number of additions/subtractions etc, use

``` bibtex
@article{gustafsson2006simplified,
  title={Simplified design of constant coefficient multipliers},
  author={Gustafsson, Oscar and Dempster, Andrew G and Johansson, Kenny and Macleod, Malcolm D and Wanhammar, Lars},
  journal={Circuits, Systems and Signal Processing},
  volume={25},
  number={2},
  pages={225--251},
  year={2006},
  publisher={Springer}
}
```

## Table sizes

A large part of the library consists of table to look up the required adder graphs.
These are stored in a compressed format, meaning that it takes some time to access it.
The number of graphs are also reduced by taking symmetry into account.
There will be more information about how to restore the symmetric cases later.

Currently, the default implementation contain all coefficients with up to 19 bits (largest odd integer 524287).
This leads to an extension of about 31 MB size.
It is fully feasible to run the generator and create a local library with more (or fewer) bits.
More details will be provided later, but for now, these are the sizes of the tables for different number of bits and the approximate times it takes to generate them on a rather high-end i9 processor.

| Bits | Cost, bytes | Graphs, bytes |     Time  |
|------|-------------|---------------|-----------|
| 19   |      98,304 |    30,537,600 |     2.5 s |
| 24   |   3,145,728 | 1,130,431,623 | 12 m 20 s |
| 25   |   6,291,456 | 2,586,518,434 |  17 m 6 s |

The smallest number currently not covered using six adders is 44784461, meaning that all numbers up to 25 bits can currently be generated. However, there is some structures missing, so this should be sorted soon as 44784461 indeed can be implemented using six adders.

Note that these sizes may improve when we identify even more symmetric cases and/or figure out more efficient compression schemes.
But possibly also increase for more than 19 bits, as there are adder structures missing for the six-adder case.
