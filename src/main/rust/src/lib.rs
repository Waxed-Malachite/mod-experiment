#![feature(extern_types, register_tool)]
#![register_tool(jvm_codegen)]

#[jvm::class("java/nio/Buffer")]
impl Buffer {}

#[jvm::class("java/nio/CharBuffer")]
impl CharBuffer {
    #[allow(non_snake_case)]
    #[jvm::method]
    pub fn toString(&self) -> &'static JavaString {}
}

#[jvm::class("java/nio/ByteBuffer")]
impl ByteBuffer {
    #[jvm::static_method]
    pub fn allocate(capacity: i32) -> &'static ByteBuffer {}

    #[jvm::method]
    pub fn put(&self, b: i8) -> &'static ByteBuffer {}

    #[jvm::method]
    pub fn flip(&self) -> &'static Buffer {}

    #[jvm::method]
    pub fn limit(&self) -> i32 {}

    #[jvm::method]
    pub fn get(&self) -> i8 {}
}

#[jvm::class("java.nio.charset.Charset", rename_all = "camelCase")]
impl Charset {
    #[jvm::method]
    pub fn decode(&self, bb: &ByteBuffer) -> &'static CharBuffer {}

    #[jvm::method]
    pub fn encode(&self, str: &JavaString) -> &'static ByteBuffer {}
}

#[jvm::class("java.nio.charset.StandardCharsets")]
impl StandardCharsets {
    #[allow(non_snake_case)]
    #[jvm::static_field]
    pub fn UTF_8() -> &'static Charset {}
}

#[jvm::class("java.lang.String", rename_all = "camelCase")]
impl JavaString {}

pub fn to_java_string(rust_string: &str) -> &'static JavaString {
    let utf8_charset = StandardCharsets::UTF_8();

    let rust_bytes = rust_string.as_bytes();

    let buffer = ByteBuffer::allocate(rust_bytes.len() as i32);

    for &b in rust_bytes {
        buffer.put(b as i8);
    }

    buffer.flip();

    let char_buffer = utf8_charset.decode(buffer);

    char_buffer.toString()
}

pub fn from_java_string(j_str: &JavaString) -> String {
    let utf8_charset = StandardCharsets::UTF_8();

    let buffer = utf8_charset.encode(j_str);

    let len = buffer.limit() as usize;

    let mut rust_bytes = Vec::with_capacity(len);
    for _ in 0..len {
        rust_bytes.push(buffer.get() as u8);
    }

    String::from_utf8(rust_bytes).expect("Java produced invalid UTF-8")
}

pub fn hello(input: &JavaString) -> &'static JavaString {
    let rust_string = from_java_string(input);
    let greeting = format!("Hello, {}!", rust_string);
    to_java_string(&greeting)
}
