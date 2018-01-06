error_chain!{
    foreign_links {
        IoErr(::std::io::Error);
    }
}
