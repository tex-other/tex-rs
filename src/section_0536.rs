//! ` `

// @<Print the banner...@>=
macro_rules! Print_the_banner_line__including_the_date_and_time {
    ($globals:expr) => {{
        // begin wlog(banner);
        wlog(make_globals_log_view!($globals), banner);
        // slow_print(format_ident); print("  ");
        slow_print($globals, $globals.format_ident.get() as _);
        print($globals, strpool_str!("  ").get() as _);
        // print_int(day); print_char(" ");
        print_int($globals, day!($globals));
        print_char(make_globals_io_string_log_view!($globals), ASCII_code_literal!(b' '));
        // months:='JANFEBMARAPRMAYJUNJULAUGSEPOCTNOVDEC';
        let months = b" JANFEBMARAPRMAYJUNJULAUGSEPOCTNOVDEC";
        /// NOTE: pascal string stores its length at the first byte
        const _ : () = ();
        // for k:=3*month-2 to 3*month do wlog(months[k]);
        for k in 3 * month!($globals) - 2 ..= 3 * month!($globals) {
            wlog(make_globals_log_view!($globals), months[k as usize] as char);
        }
        // print_char(" "); print_int(year); print_char(" ");
        print_char(make_globals_io_string_log_view!($globals), ASCII_code_literal!(b' '));
        print_int($globals, year!($globals));
        print_char(make_globals_io_string_log_view!($globals), ASCII_code_literal!(b' '));
        // print_two(time div 60); print_char(":"); print_two(time mod 60);
        print_two($globals, time!($globals) / 60);
        print_char(make_globals_io_string_log_view!($globals), ASCII_code_literal!(b':'));
        print_two($globals, time!($globals) % 60);
        // end
        use crate::section_0002::banner;
        use crate::section_0004::TeXGlobalsLogView;
        use crate::section_0056::wlog;
        use crate::section_0058::print_char;
        use crate::section_0060::slow_print;
        use crate::section_0065::print_int;
        use crate::section_0066::print_two;
    }}
}
