# RwBuffer
> Read/Write Buffer

The `rwbuffer` is intended to help encoding and decoding data into and from binary buffers. This can be useful for implementing custom binary network protocols, or custom binary serialization / deserialization in general. 

The [example](./examples/example.rs) shows how `rwbuffer` can be used to encode an xyz position into a simple (length, type, content, checksum) binary protocol and decode it again on the other side.
