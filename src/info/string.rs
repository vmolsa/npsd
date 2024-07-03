use super::PayloadInfo;

impl PayloadInfo for char {
    const TYPE: &'static str = "char";
    const SIZE: Option<usize> = Some(core::mem::size_of::<Self>());
}

impl<'a> PayloadInfo for &'a str {
    const TYPE: &'static str = "&str";
}

impl<'a> PayloadInfo for &'a mut str {
    const TYPE: &'static str = "&mut str";
}

impl PayloadInfo for String {
    const TYPE: &'static str = "String";
}
