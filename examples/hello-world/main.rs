mod fmt;
mod testcase;
mod debug_display;
// Line comments
/*
 * Block comment
 */
/// doc comment
fn main() {
    fmt::show_fmt();
    testcase::test_pi_fmt();
    testcase::test_complex_display();
    debug_display::show_boy();
    testcase::show_list();
    testcase::test_hex_fmt();
}


