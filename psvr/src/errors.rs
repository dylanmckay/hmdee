error_chain! {
    types {
        Error, ErrorKind, ResultExt;
    }

    foreign_links {
        Io(::std::io::Error);
    }

    errors {
        Hid(e: ::hidapi::HidError) {
            description("usb hid error")
            display("usb hid error: {}", e)
        }
    }
}

