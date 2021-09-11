//! ` `

// @<Print string |s| as an error message@>=
pub(crate) macro Print_string_s_as_an_error_message($globals:expr, $s:expr) {{
    // begin print_err(""); slow_print(s);
    print_err!($globals, crate::strpool_str!(""));
    slow_print($globals, $s);
    // if err_help<>null then use_err_help:=true
    // else if long_help_seen then help1("(That was another \errmessage.)")
    // else  begin if interaction<error_stop_mode then long_help_seen:=true;
    //   help4("This error message was generated by an \errmessage")@/
    //   ("command, so I can't give any explicit help.")@/
    //   ("Pretend that you're Hercule Poirot: Examine all clues,")@/
    // @^Poirot, Hercule@>
    //   ("and deduce the truth by order and method.");
    //   end;
    // error; use_err_help:=false;
    // end
    use crate::section_0060::slow_print;
    use crate::section_0073::print_err;
}}
