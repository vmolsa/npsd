# Implemented Rust Types in npsd

| Type                    | Sync Methods                      | Async Methods                                    | Metadata (PayloadInfo)   |
|:------------------------|:----------------------------------|:-------------------------------------------------|:-------------------------|
| Next`<'_>`              | Middleware                        | AsyncMiddleware                                  |                          |
| u8                      | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | Yes                      |
| u16                     | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | Yes                      |
| u32                     | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | Yes                      |
| u64                     | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | Yes                      |
| u128                    | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | Yes                      |
| i8                      | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | Yes                      |
| i16                     | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | Yes                      |
| i32                     | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | Yes                      |
| i64                     | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | Yes                      |
| i128                    | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | Yes                      |
| isize                   | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | Yes                      |
| usize                   | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | Yes                      |
| ()                      | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | Yes                      |
| (tuple `1..8`)          | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | Yes                      |
| bool                    | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | Yes                      |
| char                    | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | Yes                      |
| &'a str                 | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | Yes                      |
| &'a mut str             | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | Yes                      |
| String                  | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | Yes                      |
| Option`<T>`             | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | Yes                      |
| Result`<T, E>`          | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | Yes                      |
| *mut T                  | IntoPayload, FromPayload, Payload | `(Disabled for Send + Sync)`                     | Yes                      |
| &'a T                   | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | Yes                      |
| &'a mut T               | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | Yes                      |
| Vec`<T>`                | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | Yes                      |
| Cow<'a, `[T]`>          | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | Yes                      |
| &'a `[T]`               | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | Yes                      |
| &mut `[T]`              | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | Yes                      |
| `[T; N]`                | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | Yes                      |
| Box`<T>`                | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | Yes                      |
| Arc`<T>`                | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | Yes                      |
| Rc`<T>`                 | IntoPayload, FromPayload, Payload | `(Disabled for Send + Sync)`                     | Yes                      |
| UnsafeCell`<T>`         | IntoPayload, FromPayload, Payload | `(Disabled for Send + Sync)`                     | Yes                      |
| Cell`<T>`               | IntoPayload, FromPayload, Payload | `(Disabled for Send + Sync)`                     | Yes                      |
| Ref`<'a, T>`            | IntoPayload, FromPayload, Payload | `(Disabled for Send + Sync)`                     | Yes                      |
| RefCell`<T>`            | IntoPayload, FromPayload, Payload | `(Disabled for Send + Sync)`                     | Yes                      |
| Pin<Box`<T>`>           | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | Yes                      |
| Weak`<T>`               | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | Yes                      |
| VecDeque`<T>`           | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | Yes                      |
| LinkedList`<T>`         | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | Yes                      |
| HashMap`<K, V>`         | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | Yes                      |
| FxHashMap`<K, V>`       | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | Yes                      |
| FxHashSet`<K>`          | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | Yes                      |
| BTreeMap`<K, V>`        | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | Yes                      |
| HashSet`<K>`            | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | Yes                      |
| BTreeSet`<K>`           | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | Yes                      |
| BinaryHeap`<T>`         | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | Yes                      |
| DateTime`<Utc>`         | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | Yes                      |
| DateTime`<Local>`       | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | Yes                      |
| DateTime`<FixedOffset>` | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | Yes                      |
| Ipv4Addr                | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | Yes                      |
| Ipv6Addr                | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | Yes                      |
| IpAddr                  | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | Yes                      |
| SocketAddr              | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | Yes                      |
| Uuid                    | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | Yes                      |
| Duration                | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | Yes                      |
| Instant                 | IntoPayload, `TODO`               | AsyncIntoPayload, `TODO`                         | Yes                      |
| SystemTime              | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | Yes                      |
