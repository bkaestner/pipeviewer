# Pipeviewer Rust Demo application

This is an application modelled after the popular [`pv`][pv] application. It was started as a Rust exercise after watching the [Udemy Course: *Hands-On Systems Programming with Rust*][udemy], where the goal was to achieve a minimal `pv` variant.

Similar to the course's result, this project only supports a very small subset of `pv` and should not be used in production.
However, instead of following the course material and the course content with minute detail, I have started from scratch after some days have passed.

# Goals of this project

- [ ] Implement a simple pipeviewer that reports the current status via `stderr` and either takes input from `stdin` or a file and writes to `stdout`

 [pv]: https://sourceforge.net/projects/pipeviewer/ "pipeviewer project" 
 [udemy]: https://www.udemy.com/course/hands-on-systems-programming-with-rust/
