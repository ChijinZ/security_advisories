# Out-of-Range vulnerability in "go/archive/tar"

[official confirmation](https://github.com/golang/go/issues/40196#issuecomment-659544707)

there is an out-of-range vulnerability in "go/archive/tar", which will lead to an abnormal exiting when a user untars a malicious tar file. The version of Golang that we tested is v1.14.4 (released in 2020/06/01), it seems that all the versions suffer from the issue. The malicious tar file is provided as attachment and the attack driver is as below:

```go
package main

import (
    "archive/tar"
    "fmt"
    "io"
    "os"
)

func main() {
    reader, err := os.Open("/path/to/8fe3a773410fcb2cd44b8b933377114cde825f5e")
    if err != nil {
        fmt.Print(err)
    }
    tarReader := tar.NewReader(reader)
    for {
        _, err := tarReader.Next() <--- **panic**
        if err != nil {
            if err != io.EOF {
                continue
            }
            break
        }
    }
    fmt.Println("Done")
}
```

The panic resides in "_, err := tarReader.Next()". I believe it is abnormal for a library to unexpectedly exit the process. When digging into the source of this vulnerability, I found that it lacks a necessary check before accessing the string slice at strconv.go:269.