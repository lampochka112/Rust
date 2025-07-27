package main

/*
#cgo linux LDFLAGS: -L${SRCDIR}/../rust_go_math/target/release -lrust_go_math -Wl,-rpath=${SRCDIR}/../rust_go_math/target/release
#cgo windows LDFLAGS: -L${SRCDIR}/../rust_go_math/target/release -lrust_go_math
#cgo darwin LDFLAGS: -L${SRCDIR}/../rust_go_math/target/release -lrust_go_math -Wl,-rpath=${SRCDIR}/../rust_go_math/target/release

#include <stdlib.h>
int add(int a, int b);
double average(double* arr, int len);
*/
import "C"
import (
	"fmt"
	"unsafe"
)

func main() {
	// Пример 1: Сложение чисел
	sum := C.add(10, 20)
	fmt.Printf("add(10, 20) = %d\n", sum)  // 30

	// Пример 2: Среднее значение массива
	arr := []float64{1.5, 2.5, 3.5, 4.5}
	cArr := (*C.double)(unsafe.Pointer(&arr[0]))
	avg := C.average(cArr, C.int(len(arr)))
	fmt.Printf("average = %.2f\n", avg)  // 3.00
}