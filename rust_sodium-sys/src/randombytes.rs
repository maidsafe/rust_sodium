// randombytes.h

extern "C" {
    pub fn randombytes_buf(buf: *mut c_void, size: size_t);
    pub fn randombytes_set_implementation(function_pointers: *mut randombytes_implementation)
                                          -> c_int;
}
