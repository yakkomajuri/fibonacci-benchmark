# A Tiny Uncientific Benchmark for Nim

## About

![](https://cdn-images-1.medium.com/max/1600/1*7xy8DpkPquQip0pv_ZrZHg.png)

In this experiment I compared speed of execution of a program to print the n-th Fibonacci number.

The goal was to test the speed of [Nim](https://nim-lang.org/) against Python and C, with Go being added later.

## How to Run

**1. Make sure you have the "languages" installed before you try to run these programs.**

* [Download Python](https://www.python.org/downloads/)
* [Download Go](https://golang.org/dl/)
* [Get Nim](https://nim-lang.org/install.html)
* [Download GCC](https://gcc.gnu.org/releases.html) (C Compiler)

**2. Compile the files that need to be compiled.**

Nim:
```
nim c fibonacci.nim
```

Go:
```
go build fibonacci.go
```

C:
```
gcc fibonacci.c
```
