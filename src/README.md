# Implemented Rust Types in npsd

| Type                    | Safe | Trait constraint   | Sync Methods                      | Async Methods                                    | Metadata (PayloadInfo)  |
|:------------------------|:-----|:-------------------|:----------------------------------|:-------------------------------------------------|:------------------------|
| Next`<'_>`              | ✅   |                    | Middleware                        | AsyncMiddleware                                  |                         |
| u8                      | ✅   |                    | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | ✅                      |
| u16                     | ✅   |                    | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | ✅                      |
| u32                     | ✅   |                    | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | ✅                      |
| u64                     | ✅   |                    | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | ✅                      |
| u128                    | ✅   |                    | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | ✅                      |
| i8                      | ✅   |                    | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | ✅                      |
| i16                     | ✅   |                    | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | ✅                      |
| i32                     | ✅   |                    | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | ✅                      |
| i64                     | ✅   |                    | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | ✅                      |
| i128                    | ✅   |                    | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | ✅                      |
| isize                   | ✅   |                    | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | ✅                      |
| usize                   | ✅   |                    | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | ✅                      |
| ()                      | ✅   |                    | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | ✅                      |
| (tuple `1..8`)          | ✅   |                    | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | ✅                      |
| bool                    | ✅   |                    | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | ✅                      |
| char                    | ✅   |                    | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | ✅                      |
| &'a str                 | ❌   |                    | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | ✅                      |
| &'a mut str             | ❌   |                    | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | ✅                      |
| String                  | ✅   |                    | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | ✅                      |
| Option`<T>`             | ✅   |                    | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | ✅                      |
| Result`<T, E>`          | ✅   |                    | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | ✅                      |
| *mut T                  | ❌   |                    | IntoPayload, FromPayload, Payload | `(Disabled for Send + Sync)`                     | ✅                      |
| &'a T                   | ❌   |`Borrow<T>`         | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | ✅                      |
| &'a mut T               | ❌   |`BorrowMut<T>`      | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | ✅                      |
| Vec`<T>`                | ✅   |`Clone`             | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | ✅                      |
| Cow<'a, `[T]`>          | ✅   |`Clone`             | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | ✅                      |
| &'a `[T]`               | ❌   |                    | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | ✅                      |
| &mut `[T]`              | ❌   |`Clone`             | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | ✅                      |
| `[T; N]`                | ✅   |`Default`, `Copy`   | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | ✅                      |
| Box`<T>`                | ✅   |`ToOwned`, `Clone`  | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | ✅                      |
| Arc`<T>`                | ✅   |`ToOwned`, `Clone`  | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | ✅                      |
| Rc`<T>`                 | ✅   |`ToOwned`, `Clone`  | IntoPayload, FromPayload, Payload | `(Disabled for Send + Sync)`                     | ✅                      |
| UnsafeCell`<T>`         | ✅   |`Copy`              | IntoPayload, FromPayload, Payload | `(Disabled for Send + Sync)`                     | ✅                      |
| Cell`<T>`               | ✅   |`Copy`              | IntoPayload, FromPayload, Payload | `(Disabled for Send + Sync)`                     | ✅                      |
| Ref`<'a, T>`            | ❌   |                    | IntoPayload, FromPayload, Payload | `(Disabled for Send + Sync)`                     | ✅                      |
| RefCell`<T>`            | ✅   |                    | IntoPayload, FromPayload, Payload | `(Disabled for Send + Sync)`                     | ✅                      |
| Pin<Box`<T>`>           | ✅   |                    | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | ✅                      |
| Weak`<T>`               | ✅   |`Clone`             | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | ✅                      |
| VecDeque`<T>`           | ✅   |                    | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | ✅                      |
| LinkedList`<T>`         | ✅   |                    | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | ✅                      |
| HashMap`<K, V>`         | ✅   |`Hash`, `Eq`        | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | ✅                      |
| FxHashMap`<K, V>`       | ✅   |`Hash`, `Eq`        | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | ✅                      |
| FxHashSet`<K>`          | ✅   |`Hash`, `Eq`        | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | ✅                      |
| BTreeMap`<K, V>`        | ✅   |`Hash`, `Eq`, `Ord` | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | ✅                      |
| HashSet`<K>`            | ✅   |`Hash`, `Eq`        | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | ✅                      |
| BTreeSet`<K>`           | ✅   |`Hash`, `Eq`, `Ord` | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | ✅                      |
| BinaryHeap`<T>`         | ✅   |`Ord`               | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | ✅                      |
| DateTime`<Utc>`         | ✅   |                    | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | ✅                      |
| DateTime`<Local>`       | ✅   |                    | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | ✅                      |
| DateTime`<FixedOffset>` | ✅   |                    | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | ✅                      |
| Ipv4Addr                | ✅   |                    | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | ✅                      |
| Ipv6Addr                | ✅   |                    | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | ✅                      |
| IpAddr                  | ✅   |                    | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | ✅                      |
| SocketAddr              | ✅   |                    | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | ✅                      |
| Uuid                    | ✅   |                    | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | ✅                      |
| Duration                | ✅   |                    | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | ✅                      |
| Instant                 | ✅   |                    | IntoPayload, `TODO`               | AsyncIntoPayload, `TODO`                         | ✅                      |
| SystemTime              | ✅   |                    | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | ✅                      |
| io::`Error`             | ✅   |                    | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | ✅                      |
| Range`<T>`              | ✅   |                    | IntoPayload, FromPayload, Payload | AsyncIntoPayload, AsyncFromPayload, AsyncPayload | ✅                      |

