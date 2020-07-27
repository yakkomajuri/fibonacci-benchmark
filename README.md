# A Tiny Uncientific Benchmark for Nim

## About

![](https://cdn-images-1.medium.com/max/1600/1*7xy8DpkPquQip0pv_ZrZHg.png)

### Originally published in the article [A Python Substitute? I Tried Out the Best Programming Language You’ve Never Heard Of](https://medium.com/better-programming/a-python-substitute-i-tried-out-the-best-programming-language-youve-never-heard-of-9e29cd1893c0?source=friends_link&sk=61b12cfd6456f992013ba61e710efc72).

<hr>

In this experiment I compared speed of execution of a program to print the n-th Fibonacci number.

The goal was to test the speed of [Nim](https://nim-lang.org/) against Python and C, with Go being added later.

## How to Run

**1. Make sure you have the "languages" installed before you try to run these programs.**

* [Download Python](https://www.python.org/downloads/)
* [Download Go](https://golang.org/dl/)
* [Get Nim](https://nim-lang.org/install.html)
* [Download GCC](https://gcc.gnu.org/releases.html) (C Compiler)

**2. Compile the files that need to be compiled.**

Nim (Original Method - Used in article):
```
nim c fibonacci.nim
```

Nim (Suggested Method - Optimized version, [suggested to me after the article](#important-note):
```
nim c -d:release fibonacci.nim
```

Go:
```
go build fibonacci.go -o fib
```

C:
```
gcc fibonacci.c
```

**3. Run and time the programs**

To time the programs I used the `time` command in Bash. The first run of each program after system startup should be discarded, and we're interested in the "real" time.

Nim:
```
time ./fibonacci
```

Go:
```
time ./fib
```

C:
```
time ./a.out
```

Python:
```
time python3 fibonacci.py
```

## Important Note

After writing the article, it was pointed out to me that by simply passing optimization flags to the Nim compiler I could improve the speed of execution by over 10x. This happens because, as stated in the Official Docs: “By default the Nim compiler generates a large amount of runtime checks aiming for your debugging pleasure. With `-d:release` some checks are turned off and optimizations are turned on.” 

Besides `-d:release`, there are also other flags that can optimize the runtime even further. Here's a table provided by [sschwarzer](https://github.com/sschwarzer) denoting the different options accepted by the compiler and their impact on runtime.

| Compilation |	Runtime in s |	Remarks |
| :---------: | :-----------:| :-------:|
|nim c fibonacci.nim	|3.3	|without any runtime optimization, but faster compilation
|nim c -d:release fibonacci.nim	|0.56	|with optimization
|nim c -d:release -d:danger fibonacci.nim|	0.29|	without runtime checks
|nim c -d:release --gc:arc fibonacci.nim	|0.31	|new memory management approach
|nim c -d:release -d:danger --gc:arc fibonacci.nim	|0.30	| all combined

