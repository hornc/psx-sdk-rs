//! BIOS kernel functions
// This file was automatically generated by build.rs

global_asm!(include_str!("trampoline.s"));

extern "C" {
    /// [BIOS Function A(00h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn file_open(filename: *const u8, accessmode: u32) -> u8;
    /// [BIOS Function A(06h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn exit(exitcode: i32) -> !;
    /// [BIOS Function A(13h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn save_state(buf: *mut u8);
    /// [BIOS Function A(2Fh)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn rand() -> i16;
    /// [BIOS Function A(30h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn srand(seed: u32);
    /// [BIOS Function A(33h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn malloc(size: usize) -> *mut u8;
    /// [BIOS Function A(34h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn free(buf: *mut u8);
    /// [BIOS Function A(37h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn calloc(sizex: usize, sizey: usize) -> *const u8;
    /// [BIOS Function A(38h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn realloc(old_buf: *const u8, new_size: usize);
    /// [BIOS Function A(39h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn init_heap(addr: usize, size: usize);
    /// [BIOS Function A(3Ah)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn system_error_exit(exitcode: i32) -> !;
    /// [BIOS Function A(3Fh)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn printf(msg: *const u8, ...);
    /// [BIOS Function A(41h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn load_exe_header(filename: *const u8, headerbuf: *mut u8);
    /// [BIOS Function A(42h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn load_exe_file(filename: *const u8, headerbuf: *mut u8);
    /// [BIOS Function A(43h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn do_execute(headerbuf: *mut u8, param1: u32, param2: u32);
    /// [BIOS Function A(44h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn flush_cache();
    /// [BIOS Function A(47h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn gpu_send_dma(xdst: u16, ydst: u16, xsiz: u16, ysize: u16, src: u32);
    /// [BIOS Function A(48h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn gpu_gp1_command_word(cmd: u32);
    /// [BIOS Function A(49h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn gpu_command_word(cmd: u32);
    /// [BIOS Function A(4Ah)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn gpu_command_word_params(src: *const u32, num: usize);
    /// [BIOS Function A(4Dh)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn gpu_get_status() -> u32;
    /// [BIOS Function A(51h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn load_and_execute(filename: *const u8, stackbase: u32, stackoffset: u32);
    /// [BIOS Function A(72h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn cd_remove();
    /// [BIOS Function A(A0h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn warm_boot() -> !;
    /// [BIOS Function B(03h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn get_timer(t: u32);
    /// [BIOS Function B(04h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn enable_timer_irq(t: u32);
    /// [BIOS Function B(05h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn disable_timer_irq(t: u32);
    /// [BIOS Function B(06h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn restart_timer(t: u32);
    /// [BIOS Function B(12h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn init_pad(buf1: *mut u8, siz1: usize, buf2: *mut u8, siz2: usize);
    /// [BIOS Function B(13h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn start_pad();
    /// [BIOS Function B(14h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn stop_pad();
    /// [BIOS Function B(5Bh)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn change_clear_pad(int: u32);
    /// [BIOS Function C(0Ah)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn change_clear_rcnt(t: u32, flag: bool) -> bool;
    /// [BIOS Function C(13h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn flush_std_in_out_put();
    /// [BIOS Function SYS(01h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn enter_critical_section() -> bool;
    /// [BIOS Function SYS(02h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
    pub fn exit_critical_section();
}
