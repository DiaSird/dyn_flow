# Dynamic-System Flow in Rust

Initial version: 2024/05/06
Revised: 2024/mm/dd

---

## Contents

- [Dynamic-System Flow in Rust](#dynamic-system-flow-in-rust)
  - [Contents](#contents)
  - [What is the Dynamic System?](#what-is-the-dynamic-system)
  - [Example](#example)
  - [Used version](#used-version)
  - [License](#license)

## What is the Dynamic System?

$\frac{d}{dt} x(t) = v(t)$

## Example

- Equation of Motion

  - Newton's low: $m\bm{a} = \bm{F}$

  - Navier-Stokes equation:
    $\frac{d\bm{v}}{dt} = \frac{1}{\rho}\nabla \cdot {\bm{\sigma}}$,
    where $\bm{\sigma}$ is fluid stress given as
    $\bm{\sigma} = p\bm{I} + 2\mu\bm{D}$.

- Ricci Flow: $\frac{\partial}{\partial t} g_{ij} = - 2 Rc_{ij}$
  ($g_{ij}$: metric tensor components and $Rc_{ij}$: Ricci curvature)

  In case of the Einstein manifold $(M, g_0)$, we can write the metric $g_{ij}$ as
  $g_{ij} = f(t) g_0$, where $Rc_{ij} = c g_0$
  $(g(0) = g_0, c = const.)$.

  Therefore, $\frac{\partial}{\partial t} f(t) = - 2 c \Leftrightarrow f(t) = 1 - 2ct$ and $g_{ij} = (1 - 2ct) g_0$.

<!-- - Using Gnuplot and Plotters
  \*for instance, below: -->

<!-- <p align="center">
  <img src="rst\2d_laplace.png", width="80%">
  <img src="rst\2d_laplace2.png", width="80%">
</p> -->

## Used version

- rustc 1.76.0
- rustup 1.26.0

## License

- MIT license
