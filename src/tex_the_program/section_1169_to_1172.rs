//! @ The routine that inserts a |style_node| holds no surprises.
//!
//! @<Put each...@>=
//! primitive("displaystyle",math_style,display_style);
//! @!@:display_style_}{\.{\\displaystyle} primitive@>
//! primitive("textstyle",math_style,text_style);
//! @!@:text_style_}{\.{\\textstyle} primitive@>
//! primitive("scriptstyle",math_style,script_style);
//! @!@:script_style_}{\.{\\scriptstyle} primitive@>
//! primitive("scriptscriptstyle",math_style,script_script_style);
//! @!@:script_script_style_}{\.{\\scriptscriptstyle} primitive@>
//!
//! @ @<Cases of |print_cmd_chr|...@>=
//! math_style: print_style(chr_code);
//!
//! @ @<Cases of |main_control| that build...@>=
//! mmode+math_style: tail_append(new_style(cur_chr));
//! mmode+non_script: begin tail_append(new_glue(zero_glue));
//!   subtype(tail):=cond_math_glue;
//!   end;
//! mmode+math_choice: append_choices;
//!
//! @ The routine that scans the four mlists of a \.{\\mathchoice} is very
//! much like the routine that builds discretionary nodes.
//!
//! @<Declare act...@>=
//! procedure append_choices;
//! begin tail_append(new_choice); incr(save_ptr); saved(-1):=0;
//! push_math(math_choice_group); scan_left_brace;
//! end;
//!
