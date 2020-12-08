


pub fn set() {
    let __tmp = 0x80u8;
    let __x = 0x1u8;
    let __addr = 0x61;

    unsafe{
        llvm_asm!(
        r#"
        cli;
        sts $1, $0;
        sts $1, $2;"#
        : /* no outputs */
        : "d" (__tmp),
          "M" (__addr),
          "d" (__x)
        : "r0"
        );
    }
}
