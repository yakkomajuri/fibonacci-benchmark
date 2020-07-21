function fibonacci(n::Int)
    n==0 && return 0
    n==1 && return 1
    fibonacci(n-1) + fibonacci(n-2)
    end

print(fibonacci(40))
