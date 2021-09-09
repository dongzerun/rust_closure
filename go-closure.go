package main

import "fmt"

func test(f func()) {
    f()
    f()
}

func main() {
    a:=1
    fn := func() {
        a++
        fmt.Printf("a is %d\n", a)
    }
    test(fn)
}
