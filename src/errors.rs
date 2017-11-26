error_chain! {
    types {
        Error, ErrorKind, ResultExt;
    }

    foreign_links {
        Usb(::libusb::Error);
        Io(::std::io::Error);
    }

    errors {
    }
}

