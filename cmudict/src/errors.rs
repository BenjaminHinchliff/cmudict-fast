error_chain!{
    links {
        Core(::cmudict_core::Error, ::cmudict_core::ErrorKind);
    }

    foreign_links {
        IoErr(::std::io::Error);
    }
}
