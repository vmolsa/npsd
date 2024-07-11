use std::io::{self, ErrorKind};

use super::{Error, Middleware, Payload, IntoPayload, FromPayload};

/// Test for serializing and deserializing `io::Error` using `npsd`.
///
/// ```
/// use std::io;
/// use npsd::{IntoPayload, FromPayload, Middleware};
///
/// #[test]
/// fn test_io_error_payload() {
///     let original_error = io::Error::new(io::ErrorKind::NotFound, "File not found");
/// 
///     let mut next = Next::default();
///
///     // Serialize the `io::Error` instance into a packet.
///     original_error.into_packet(&mut (), &mut next).unwrap();
///
///     // Deserialize the packet back into an `io::Error` instance.
///     let deserialized_error = io::Error::from_packet(&mut (), &mut next).unwrap();
///
///     // Ensure the deserialized instance matches the original.
///     assert_eq!(deserialized_error.kind(), original_error.kind());
///     assert_eq!(deserialized_error.to_string(), original_error.to_string());
/// }
/// ```

impl<C> IntoPayload<C> for io::Error {
    fn into_payload<M: Middleware>(&self, ctx: &mut C, next: &mut M) -> Result<(), Error> {
        let kind: u8 = match self.kind() {
            ErrorKind::NotFound => 0,
            ErrorKind::PermissionDenied => 1,
            ErrorKind::ConnectionRefused => 2,
            ErrorKind::ConnectionReset => 3,
            #[cfg(feature = "io_error_more")]
            ErrorKind::HostUnreachable => 4,
            #[cfg(feature = "io_error_more")]
            ErrorKind::NetworkUnreachable => 5,
            ErrorKind::ConnectionAborted => 6,
            ErrorKind::NotConnected => 7,
            ErrorKind::AddrInUse => 8,
            ErrorKind::AddrNotAvailable => 9,
            #[cfg(feature = "io_error_more")]
            ErrorKind::NetworkDown => 10,
            ErrorKind::BrokenPipe => 11,
            ErrorKind::AlreadyExists => 12,
            ErrorKind::WouldBlock => 13,
            #[cfg(feature = "io_error_more")]
            ErrorKind::NotADirectory => 14,
            #[cfg(feature = "io_error_more")]
            ErrorKind::IsADirectory => 15,
            #[cfg(feature = "io_error_more")]
            ErrorKind::DirectoryNotEmpty => 16,
            #[cfg(feature = "io_error_more")]
            ErrorKind::ReadOnlyFilesystem => 17,
            #[cfg(feature = "io_error_more")]
            ErrorKind::FilesystemLoop => 18,
            #[cfg(feature = "io_error_more")]
            ErrorKind::StaleNetworkFileHandle => 19,
            ErrorKind::InvalidInput => 20,
            ErrorKind::InvalidData => 21,
            ErrorKind::TimedOut => 22,
            ErrorKind::WriteZero => 23,
            #[cfg(feature = "io_error_more")]
            ErrorKind::StorageFull => 24,
            #[cfg(feature = "io_error_more")]
            ErrorKind::NotSeekable => 25,
            #[cfg(feature = "io_error_more")]
            ErrorKind::FilesystemQuotaExceeded => 26,
            #[cfg(feature = "io_error_more")]
            ErrorKind::FileTooLarge => 27,
            #[cfg(feature = "io_error_more")]
            ErrorKind::ResourceBusy => 28,
            #[cfg(feature = "io_error_more")]
            ErrorKind::ExecutableFileBusy => 29,
            #[cfg(feature = "io_error_more")]
            ErrorKind::Deadlock => 30,
            #[cfg(feature = "io_error_more")]
            ErrorKind::CrossesDevices => 31,
            #[cfg(feature = "io_error_more")]
            ErrorKind::TooManyLinks => 32,
            #[cfg(feature = "io_error_more")]
            ErrorKind::InvalidFilename => 33,
            #[cfg(feature = "io_error_more")]
            ErrorKind::ArgumentListTooLong => 34,
            ErrorKind::Interrupted => 35,
            ErrorKind::Unsupported => 36,
            ErrorKind::UnexpectedEof => 37,
            ErrorKind::OutOfMemory => 38,
            ErrorKind::Other => 39,
            #[cfg(feature = "io_error_uncategorized")]
            ErrorKind::Uncategorized => 40,
            _ => 39, // ErrorKind::Other
        };

        next.into_payload(&kind, ctx)?;
        next.into_payload(&self.to_string(), ctx)
    }
}

impl<'a, C> FromPayload<'a, C> for io::Error {
    fn from_payload<'b, M: Middleware>(ctx: &mut C, next: &'b mut M) -> Result<Self, Error>
        where 'a: 'b,
    {
        let byte: u8 = next.from_payload(ctx)?;
        let msg: String = next.from_payload(ctx)?;

        let kind: ErrorKind = match byte {
            0 => ErrorKind::NotFound,
            1 => ErrorKind::PermissionDenied,
            2 => ErrorKind::ConnectionRefused,
            3 => ErrorKind::ConnectionReset,
            #[cfg(feature = "io_error_more")]
            4 => ErrorKind::HostUnreachable,
            #[cfg(feature = "io_error_more")]
            5 => ErrorKind::NetworkUnreachable,
            6 => ErrorKind::ConnectionAborted,
            7 => ErrorKind::NotConnected,
            8 => ErrorKind::AddrInUse,
            9 => ErrorKind::AddrNotAvailable,
            #[cfg(feature = "io_error_more")]
            10 => ErrorKind::NetworkDown,
            11 => ErrorKind::BrokenPipe,
            12 => ErrorKind::AlreadyExists,
            13 => ErrorKind::WouldBlock,
            #[cfg(feature = "io_error_more")]
            14 => ErrorKind::NotADirectory,
            #[cfg(feature = "io_error_more")]
            15 => ErrorKind::IsADirectory,
            #[cfg(feature = "io_error_more")]
            16 => ErrorKind::DirectoryNotEmpty,
            #[cfg(feature = "io_error_more")]
            17 => ErrorKind::ReadOnlyFilesystem,
            #[cfg(feature = "io_error_more")]
            18 => ErrorKind::FilesystemLoop,
            #[cfg(feature = "io_error_more")]
            19 => ErrorKind::StaleNetworkFileHandle,
            20 => ErrorKind::InvalidInput,
            21 => ErrorKind::InvalidData,
            22 => ErrorKind::TimedOut,
            23 => ErrorKind::WriteZero,
            #[cfg(feature = "io_error_more")]
            24 => ErrorKind::StorageFull,
            #[cfg(feature = "io_error_more")]
            25 => ErrorKind::NotSeekable,
            #[cfg(feature = "io_error_more")]
            26 => ErrorKind::FilesystemQuotaExceeded,
            #[cfg(feature = "io_error_more")]
            27 => ErrorKind::FileTooLarge,
            #[cfg(feature = "io_error_more")]
            28 => ErrorKind::ResourceBusy,
            #[cfg(feature = "io_error_more")]
            29 => ErrorKind::ExecutableFileBusy,
            #[cfg(feature = "io_error_more")]
            30 => ErrorKind::Deadlock,
            #[cfg(feature = "io_error_more")]
            31 => ErrorKind::CrossesDevices,
            #[cfg(feature = "io_error_more")]
            32 => ErrorKind::TooManyLinks,
            #[cfg(feature = "io_error_more")]
            33 => ErrorKind::InvalidFilename,
            #[cfg(feature = "io_error_more")]
            34 => ErrorKind::ArgumentListTooLong,
            35 => ErrorKind::Interrupted,
            36 => ErrorKind::Unsupported,
            37 => ErrorKind::UnexpectedEof,
            38 => ErrorKind::OutOfMemory,
            39 => ErrorKind::Other,
            #[cfg(feature = "io_error_uncategorized")]
            40 => ErrorKind::Uncategorized,
            _ => ErrorKind::Other,
        };

        Ok(io::Error::new(kind, msg))
    }
}

impl<C> Payload<C> for io::Error {}
