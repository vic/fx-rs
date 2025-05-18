# Reader/Writer

A `Reader[T]` allows to read values of type `T` from the environment while `Writer[T]` allows to set them.

- Implementation 
   - [read.go](https://github.com/vic/fx.go/blob/main/rw/read.go)
   - [write.go](https://github.com/vic/fx.go/blob/main/rw/write.go)

- Tests [rw_test.go](https://github.com/vic/fx.go/blob/main/rw/rw_test.go)

Read and Write Handlers take an effectful operation that can modify the external world. See `TestReadWrite`.