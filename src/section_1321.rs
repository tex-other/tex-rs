//! ` `
// @<Undump the font information@>=
macro_rules! Undump_the_font_information {
    ($globals:expr, $lbl_bad_fmt:lifetime) => {{
        // undump_size(7)(font_mem_size)('font mem size')(fmem_ptr);
        // for k:=0 to fmem_ptr-1 do undump_wd(font_info[k]);
        // undump_size(font_base)(font_max)('font max')(font_ptr);
        // for k:=null_font to font_ptr do
        //   @<Undump the array info for internal font number |k|@>
        panic!("Undump font info");
    }}
}
