<a name="v0.2.2"></a>
### v0.2.2 (2016-09-06)


#### Bug Fixes

*   fix division by zero ([d6b2978b](https://github.com/kbknapp/cargo-count/commit/d6b2978b5135ba587e1eb6bc9005e4dcd3d45ec0))
*   Batch results by language, not file extension ([1828538a](https://github.com/kbknapp/cargo-count/commit/1828538a9b30a8aec9fb059127d90721b4f9c29d), closes [#19](https://github.com/kbknapp/cargo-count/issues/19))

#### Features

* **languages:**  Add ASM, Shell, D and Nim ([350790c7](https://github.com/kbknapp/cargo-count/commit/350790c7f1e352fb21d7c274abcd30c10bb73a52))

#### Improvements

*   updates clap and uses new features ([4251fda2](https://github.com/kbknapp/cargo-count/commit/4251fda2473e6eda3630c777d0a674d7ee13449d))



<a name="v0.2.1"></a>
### v0.2.1 (2015-12-24)


#### Improvements

*   adds C, h++, cc, cxx, cp, and htm extensions ([ea4f540a](https://github.com/kbknapp/cargo-count/commit/ea4f540ac356dc946c42aaebe683f8c6d70fd362), closes [#19](https://github.com/kbknapp/cargo-count/issues/19))


<a name="v0.2.0"></a>
## v0.2.0 (2015-12-24)


#### Improvements

*   Ignore files in accordance with .gitignore ([a0c30706](https://github.com/kbknapp/cargo-count/commit/a0c307061413972b973f148802abe06e80a01099), Closes [#8](https://github.com/kbknapp/cargo-count/issues/8), [#9](https://github.com/kbknapp/cargo-count/issues/9))

#### Bug Fixes

*   fixes building on windows due to upstream dep ([3333f252](https://github.com/kbknapp/cargo-count/commit/3333f252f4c7e5e1324d5a178b9f020823283bc7))



<a name="v0.1.4"></a>
### v0.1.4 (2015-11-14)


#### Bug Fixes

*   fixes building on windows due to upstream dep ([3333f252](https://github.com/kbknapp/cargo-count/commit/3333f252f4c7e5e1324d5a178b9f020823283bc7))



<a name="v0.1.3"></a>
### v0.1.3 (2015-11-04)


#### Documentation

*   adds cargo install instructions ([467dd945](https://github.com/kbknapp/cargo-count/commit/467dd9456e6b605e1cbf48e033db9053bcfe1735))

#### Features

*   uses clippy to lint dev builds ([e02b9d9b](https://github.com/kbknapp/cargo-count/commit/e02b9d9b7381385721466677f6c80bf340aae9ae))

#### Improvements

*   better comparison of f64 values thanks to clippy ([6c3a1362](https://github.com/kbknapp/cargo-count/commit/6c3a13625fc93038dc6ab799dc023f03d2a4bfe9))

#### Bug Fixes

*   fixes features declarations ([de98dde6](https://github.com/kbknapp/cargo-count/commit/de98dde6e4d207f88130b9668c4517adf719dac7))



<a name="v0.1.2"></a>
### v0.1.2 (2015-08-25)


#### Bug Fixes

* **Symlinks:**  adds ability to optionally follow symlinks while counting ([d265980e](https://github.com/kbknapp/cargo-count/commit/d265980e8e06101c07dd3265dd2d66d834b09c58), closes [#6](https://github.com/kbknapp/cargo-count/issues/6), [#7](https://github.com/kbknapp/cargo-count/issues/7))



<a name="v0.1.1"></a>
### v0.1.1 (2015-08-24)


#### Bug Fixes

*   fixes unsafe code count bug in C and C++ files ([1c1e01d6](https://github.com/kbknapp/cargo-count/commit/1c1e01d67c0f5ad717b3842295c5fb597db65656), closes [#1](https://github.com/kbknapp/cargo-count/issues/1))
*   fixes single line block comment bug ([d896412b](https://github.com/kbknapp/cargo-count/commit/d896412bf81da6271c762ab5168d40e27e8eb988), closes [#2](https://github.com/kbknapp/cargo-count/issues/2))
* **Unsafe Counter:**  fixes a bug in the unsafe counter for Rust giving incorrect numbers ([317d2fc9](https://github.com/kbknapp/cargo-count/commit/317d2fc9964d131dbdc28fa93a6e29230143cb94), closes [#5](https://github.com/kbknapp/cargo-count/issues/5))



<a name="v0.1.0"></a>
## v0.1.0 (2015-08-21)


#### Documentation

*   adds a changelog using clog ([34fdc52b](https://github.com/kbknapp/cargo-count/commit/34fdc52b8dac02b5668a0cd9daca57ae3dd9de17))
*   updates the readme ([d302d65d](https://github.com/kbknapp/cargo-count/commit/d302d65da7614c609120011858cee0cc4e32bcc3))

#### Features

*   initial implementation ([b6e968fb](https://github.com/kbknapp/cargo-count/commit/b6e968fb2c1ff0bc5af6b21a11f83099c6fe6e68))

