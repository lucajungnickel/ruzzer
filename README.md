# ruzzer
Basic fuzzer implementation in Rust ( Rust + Fuzzer -> Ruzzer)

Start with: 
    cargo run

This starts a mutational grammar fuzzer with a CGI grammar, which will fuzz a broken implemention of a CGI parser written in C. The CGI parser (located in SUTs/ ) is compiled with ASAN in order to make memory bugs easier detectable.