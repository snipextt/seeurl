pub fn print_help() {
    // print help for cli
    println!("Usage: seeurl [options...] <url>");
    println!("Options:");
    println!("  -h, --help\t\tPrint this help message");
    println!("  -v, --version\t\tPrint version");
    println!("  -V, --verbose\t\tEnable verbose mode");
    println!("  -H, --headers\t\tSpecify request headers");
    println!("  -o, --output\t\tSpecify output file");
    println!("  -t, --timeout\t\tSpecify request timeout");
    println!("  -m, --method\t\tSpecify request method");
    println!("  -b, --body\t\tSpecify request body");
    println!("  -d, --download\tDownload response");
}
