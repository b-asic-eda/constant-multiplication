# Constant multiplicaton

This is a Python library containing a number of utilities related to constant (shift-and-add) multiplication.

- The minimum number of additions/subtractions required for an integer coefficient
- The possible structures to realize an optimal constant multiplication by an integer coefficient

It is a reimplementation in Rust with Python bindings,based on the original work in:

- O. Gustafsson, A. G. Dempster, K. Johansson, M. D. Macleod, and L. Wanhammar, "Simplified design of constant coefficient multipliers," *Circuits Syst. Signal Process.*, vol. 25, no. 2, pp. 225–251, 2005. <https://doi.org/10.1007/s00034-005-2505-5>

All integers with up to 23 bits is included.

## Citation

To cite the number of additions/subtractions etc, use:

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
