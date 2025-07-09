# Reader/Writer

A `Reader<T>` allows reading values of type `T` from the environment, while `Writer<T>` allows setting them.

Read and Write handlers take an effectful operation that can modify the external world. See the fx-rs codebase and tests for usage examples.
