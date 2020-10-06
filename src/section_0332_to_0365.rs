//! @* \[24] Getting the next token.
//! The heart of \TeX's input mechanism is the |get_next| procedure, which
//! we shall develop in the next few sections of the program. Perhaps we
//! shouldn't actually call it the ``heart,'' however, because it really acts
//! as \TeX's eyes and mouth, reading the source files and gobbling them up.
//! And it also helps \TeX\ to regurgitate stored token lists that are to be
//! processed again.
//! @^eyes and mouth@>
//!
//! The main duty of |get_next| is to input one token and to set |cur_cmd|
//! and |cur_chr| to that token's command code and modifier. Furthermore, if
//! the input token is a control sequence, the |eqtb| location of that control
//! sequence is stored in |cur_cs|; otherwise |cur_cs| is set to zero.
//!
//! Underlying this simple description is a certain amount of complexity
//! because of all the cases that need to be handled.
//! However, the inner loop of |get_next| is reasonably short and fast.
//!
//! When |get_next| is asked to get the next token of a \.{\\read} line,
//! it sets |cur_cmd=cur_chr=cur_cs=0| in the case that no more tokens
//! appear on that line. (There might not be any tokens at all, if the
//! |end_line_char| has |ignore| as its catcode.)
//!
//! @ The value of |par_loc| is the |eqtb| address of `\.{\\par}'. This quantity
//! is needed because a blank line of input is supposed to be exactly equivalent
//! to the appearance of \.{\\par}; we must set |cur_cs:=par_loc|
//! when detecting a blank line.
//!
//! @<Glob...@>=
//! @!par_loc:pointer; {location of `\.{\\par}' in |eqtb|}
//! @!par_token:halfword; {token representing `\.{\\par}'}
//!
//! @ @<Put each...@>=
//! primitive("par",par_end,256); {cf.\ |scan_file_name|}
//! @!@:par_}{\.{\\par} primitive@>
//! par_loc:=cur_val; par_token:=cs_token_flag+par_loc;
//!
//! @ @<Cases of |print_cmd_chr|...@>=
//! par_end:print_esc("par");
//!
//! @ Before getting into |get_next|, let's consider the subroutine that
//! is called when an `\.{\\outer}' control sequence has been scanned or
//! when the end of a file has been reached. These two cases are distinguished
//! by |cur_cs|, which is zero at the end of a file.
//!
//! @p procedure check_outer_validity;
//! var p:pointer; {points to inserted token list}
//! @!q:pointer; {auxiliary pointer}
//! begin if scanner_status<>normal then
//!   begin deletions_allowed:=false;
//!   @<Back up an outer control sequence so that it can be reread@>;
//!   if scanner_status>skipping then
//!     @<Tell the user what has run away and try to recover@>
//!   else  begin print_err("Incomplete "); print_cmd_chr(if_test,cur_if);
//! @.Incomplete \\if...@>
//!     print("; all text was ignored after line "); print_int(skip_line);
//!     help3("A forbidden control sequence occurred in skipped text.")@/
//!     ("This kind of error happens when you say `\if...' and forget")@/
//!     ("the matching `\fi'. I've inserted a `\fi'; this might work.");
//!     if cur_cs<>0 then cur_cs:=0
//!     else help_line[2]:=@|
//!       "The file ended while I was skipping conditional text.";
//!     cur_tok:=cs_token_flag+frozen_fi; ins_error;
//!     end;
//!   deletions_allowed:=true;
//!   end;
//! end;
//!
//! @ An outer control sequence that occurs in a \.{\\read} will not be reread,
//! since the error recovery for \.{\\read} is not very powerful.
//!
//! @<Back up an outer control sequence so that it can be reread@>=
//! if cur_cs<>0 then
//!   begin if (state=token_list)or(name<1)or(name>17) then
//!     begin p:=get_avail; info(p):=cs_token_flag+cur_cs;
//!     back_list(p); {prepare to read the control sequence again}
//!     end;
//!   cur_cmd:=spacer; cur_chr:=" "; {replace it by a space}
//!   end
//!
//! @ @<Tell the user what has run away...@>=
//! begin runaway; {print a definition, argument, or preamble}
//! if cur_cs=0 then print_err("File ended")
//! @.File ended while scanning...@>
//! else  begin cur_cs:=0; print_err("Forbidden control sequence found");
//! @.Forbidden control sequence...@>
//!   end;
//! print(" while scanning ");
//! @<Print either `\.{definition}' or `\.{use}' or `\.{preamble}' or `\.{text}',
//!   and insert tokens that should lead to recovery@>;
//! print(" of "); sprint_cs(warning_index);
//! help4("I suspect you have forgotten a `}', causing me")@/
//! ("to read past where you wanted me to stop.")@/
//! ("I'll try to recover; but if the error is serious,")@/
//! ("you'd better type `E' or `X' now and fix your file.");@/
//! error;
//! end
//!
//! @ The recovery procedure can't be fully understood without knowing more
//! about the \TeX\ routines that should be aborted, but we can sketch the
//! ideas here:  For a runaway definition we will insert a right brace; for a
//! runaway preamble, we will insert a special \.{\\cr} token and a right
//! brace; and for a runaway argument, we will set |long_state| to
//! |outer_call| and insert \.{\\par}.
//!
//! @<Print either `\.{definition}' or ...@>=
//! p:=get_avail;
//! case scanner_status of
//! defining:begin print("definition"); info(p):=right_brace_token+"}";
//!   end;
//! matching:begin print("use"); info(p):=par_token; long_state:=outer_call;
//!   end;
//! aligning:begin print("preamble"); info(p):=right_brace_token+"}"; q:=p;
//!   p:=get_avail; link(p):=q; info(p):=cs_token_flag+frozen_cr;
//!   align_state:=-1000000;
//!   end;
//! absorbing:begin print("text"); info(p):=right_brace_token+"}";
//!   end;
//! end; {there are no other cases}
//! ins_list(p)
//!
//! @ We need to mention a procedure here that may be called by |get_next|.
//!
//! @p procedure@?firm_up_the_line; forward;
//!
//! @ Now we're ready to take the plunge into |get_next| itself. Parts of
//! this routine are executed more often than any other instructions of \TeX.
//! @^mastication@>@^inner loop@>
//!
//! @d switch=25 {a label in |get_next|}
//! @d start_cs=26 {another}
//!
//! @p procedure get_next; {sets |cur_cmd|, |cur_chr|, |cur_cs| to next token}
//! label restart, {go here to get the next input token}
//!   switch, {go here to eat the next character from a file}
//!   reswitch, {go here to digest it again}
//!   start_cs, {go here to start looking for a control sequence}
//!   found, {go here when a control sequence has been found}
//!   exit; {go here when the next input token has been got}
//! var k:0..buf_size; {an index into |buffer|}
//! @!t:halfword; {a token}
//! @!cat:0..max_char_code; {|cat_code(cur_chr)|, usually}
//! @!c,@!cc:ASCII_code; {constituents of a possible expanded code}
//! @!d:2..3; {number of excess characters in an expanded code}
//! begin restart: cur_cs:=0;
//! if state<>token_list then
//! @<Input from external file, |goto restart| if no input found@>
//! else @<Input from token list, |goto restart| if end of list or
//!   if a parameter needs to be expanded@>;
//! @<If an alignment entry has just ended, take appropriate action@>;
//! exit:end;
//!
//! @ An alignment entry ends when a tab or \.{\\cr} occurs, provided that the
//! current level of braces is the same as the level that was present at the
//! beginning of that alignment entry; i.e., provided that |align_state| has
//! returned to the value it had after the \<u_j> template for that entry.
//! @^inner loop@>
//!
//! @<If an alignment entry has just ended, take appropriate action@>=
//! if cur_cmd<=car_ret then if cur_cmd>=tab_mark then if align_state=0 then
//!   @<Insert the \(v)\<v_j> template and |goto restart|@>
//!
//! @ @<Input from external file, |goto restart| if no input found@>=
//! @^inner loop@>
//! begin switch: if loc<=limit then {current line not yet finished}
//!   begin cur_chr:=buffer[loc]; incr(loc);
//!   reswitch: cur_cmd:=cat_code(cur_chr);
//!   @<Change state if necessary, and |goto switch| if the
//!     current character should be ignored,
//!     or |goto reswitch| if the current character
//!     changes to another@>;
//!   end
//! else  begin state:=new_line;@/
//!   @<Move to next line of file,
//!     or |goto restart| if there is no next line,
//!     or |return| if a \.{\\read} line has finished@>;
//!   check_interrupt;
//!   goto switch;
//!   end;
//! end
//!
//! @ The following 48-way switch accomplishes the scanning quickly, assuming
//! that a decent \PASCAL\ compiler has translated the code. Note that the numeric
//! values for |mid_line|, |skip_blanks|, and |new_line| are spaced
//! apart from each other by |max_char_code+1|, so we can add a character's
//! command code to the state to get a single number that characterizes both.
//!
//! @d any_state_plus(#) == mid_line+#,skip_blanks+#,new_line+#
//!
//! @<Change state if necessary...@>=
//! case state+cur_cmd of
//! @<Cases where character is ignored@>: goto switch;
//! any_state_plus(escape): @<Scan a control sequence
//!   and set |state:=skip_blanks| or |mid_line|@>;
//! any_state_plus(active_char): @<Process an active-character control sequence
//!   and set |state:=mid_line|@>;
//! any_state_plus(sup_mark): @<If this |sup_mark| starts an expanded character
//!   like~\.{\^\^A} or~\.{\^\^df}, then |goto reswitch|,
//!   otherwise set |state:=mid_line|@>;
//! any_state_plus(invalid_char): @<Decry the invalid character and
//!   |goto restart|@>;
//! @t\4@>@<Handle situations involving spaces, braces, changes of state@>@;
//! othercases do_nothing
//! endcases
//!
//! @ @<Cases where character is ignored@>=
//! any_state_plus(ignore),skip_blanks+spacer,new_line+spacer
//!
//! @ We go to |restart| instead of to |switch|, because |state| might equal
//! |token_list| after the error has been dealt with
//! (cf.\ |clear_for_error_prompt|).
//!
//! @<Decry the invalid...@>=
//! begin print_err("Text line contains an invalid character");
//! @.Text line contains...@>
//! help2("A funny symbol that I can't read has just been input.")@/
//! ("Continue, and I'll forget that it ever happened.");@/
//! deletions_allowed:=false; error; deletions_allowed:=true;
//! goto restart;
//! end
//!
//! @ @d add_delims_to(#)==#+math_shift,#+tab_mark,#+mac_param,
//!   #+sub_mark,#+letter,#+other_char
//!
//! @<Handle situations involving spaces, braces, changes of state@>=
//! mid_line+spacer:@<Enter |skip_blanks| state, emit a space@>;
//! mid_line+car_ret:@<Finish line, emit a space@>;
//! skip_blanks+car_ret,any_state_plus(comment):
//!   @<Finish line, |goto switch|@>;
//! new_line+car_ret:@<Finish line, emit a \.{\\par}@>;
//! mid_line+left_brace: incr(align_state);
//! skip_blanks+left_brace,new_line+left_brace: begin
//!   state:=mid_line; incr(align_state);
//!   end;
//! mid_line+right_brace: decr(align_state);
//! skip_blanks+right_brace,new_line+right_brace: begin
//!   state:=mid_line; decr(align_state);
//!   end;
//! add_delims_to(skip_blanks),add_delims_to(new_line): state:=mid_line;
//!
//! @ When a character of type |spacer| gets through, its character code is
//! changed to $\.{"\ "}=@'40$. This means that the ASCII codes for tab and space,
//! and for the space inserted at the end of a line, will
//! be treated alike when macro parameters are being matched. We do this
//! since such characters are indistinguishable on most computer terminal displays.
//!
//! @<Finish line, emit a space@>=
//! begin loc:=limit+1; cur_cmd:=spacer; cur_chr:=" ";
//! end
//!
//! @ The following code is performed only when |cur_cmd=spacer|.
//!
//! @<Enter |skip_blanks| state, emit a space@>=
//! begin state:=skip_blanks; cur_chr:=" ";
//! end
//!
//! @ @<Finish line, |goto switch|@>=
//! begin loc:=limit+1; goto switch;
//! end
//!
//! @ @<Finish line, emit a \.{\\par}@>=
//! begin loc:=limit+1; cur_cs:=par_loc; cur_cmd:=eq_type(cur_cs);
//! cur_chr:=equiv(cur_cs);
//! if cur_cmd>=outer_call then check_outer_validity;
//! end
//!
//! @ Notice that a code like \.{\^\^8} becomes \.x if not followed by a hex digit.
//!
//! @d is_hex(#)==(((#>="0")and(#<="9"))or((#>="a")and(#<="f")))
//! @d hex_to_cur_chr==
//!   if c<="9" then cur_chr:=c-"0" @+else cur_chr:=c-"a"+10;
//!   if cc<="9" then cur_chr:=16*cur_chr+cc-"0"
//!   else cur_chr:=16*cur_chr+cc-"a"+10
//!
//! @<If this |sup_mark| starts an expanded character...@>=
//! begin if cur_chr=buffer[loc] then if loc<limit then
//!   begin c:=buffer[loc+1]; @+if c<@'200 then {yes we have an expanded char}
//!     begin loc:=loc+2;
//!     if is_hex(c) then if loc<=limit then
//!       begin cc:=buffer[loc]; @+if is_hex(cc) then
//!         begin incr(loc); hex_to_cur_chr; goto reswitch;
//!         end;
//!       end;
//!     if c<@'100 then cur_chr:=c+@'100 @+else cur_chr:=c-@'100;
//!     goto reswitch;
//!     end;
//!   end;
//! state:=mid_line;
//! end
//!
//! @ @<Process an active-character...@>=
//! begin cur_cs:=cur_chr+active_base;
//! cur_cmd:=eq_type(cur_cs); cur_chr:=equiv(cur_cs); state:=mid_line;
//! if cur_cmd>=outer_call then check_outer_validity;
//! end
//!
//! @ Control sequence names are scanned only when they appear in some line of
//! a file; once they have been scanned the first time, their |eqtb| location
//! serves as a unique identification, so \TeX\ doesn't need to refer to the
//! original name any more except when it prints the equivalent in symbolic form.
//!
//! The program that scans a control sequence has been written carefully
//! in order to avoid the blowups that might otherwise occur if a malicious
//! user tried something like `\.{\\catcode\'15=0}'. The algorithm might
//! look at |buffer[limit+1]|, but it never looks at |buffer[limit+2]|.
//!
//! If expanded characters like `\.{\^\^A}' or `\.{\^\^df}'
//! appear in or just following
//! a control sequence name, they are converted to single characters in the
//! buffer and the process is repeated, slowly but surely.
//!
//! @<Scan a control...@>=
//! begin if loc>limit then cur_cs:=null_cs {|state| is irrelevant in this case}
//! else  begin start_cs: k:=loc; cur_chr:=buffer[k]; cat:=cat_code(cur_chr);
//!   incr(k);
//!   if cat=letter then state:=skip_blanks
//!   else if cat=spacer then state:=skip_blanks
//!   else state:=mid_line;
//!   if (cat=letter)and(k<=limit) then
//!     @<Scan ahead in the buffer until finding a nonletter;
//!     if an expanded code is encountered, reduce it
//!     and |goto start_cs|; otherwise if a multiletter control
//!     sequence is found, adjust |cur_cs| and |loc|, and
//!     |goto found|@>
//!   else @<If an expanded code is present, reduce it and |goto start_cs|@>;
//!   cur_cs:=single_base+buffer[loc]; incr(loc);
//!   end;
//! found: cur_cmd:=eq_type(cur_cs); cur_chr:=equiv(cur_cs);
//! if cur_cmd>=outer_call then check_outer_validity;
//! end
//!
//! @ Whenever we reach the following piece of code, we will have
//! |cur_chr=buffer[k-1]| and |k<=limit+1| and |cat=cat_code(cur_chr)|. If an
//! expanded code like \.{\^\^A} or \.{\^\^df} appears in |buffer[(k-1)..(k+1)]|
//! or |buffer[(k-1)..(k+2)]|, we
//! will store the corresponding code in |buffer[k-1]| and shift the rest of
//! the buffer left two or three places.
//!
//! @<If an expanded...@>=
//! begin if buffer[k]=cur_chr then @+if cat=sup_mark then @+if k<limit then
//!   begin c:=buffer[k+1]; @+if c<@'200 then {yes, one is indeed present}
//!     begin d:=2;
//!     if is_hex(c) then @+if k+2<=limit then
//!       begin cc:=buffer[k+2]; @+if is_hex(cc) then incr(d);
//!       end;
//!     if d>2 then
//!       begin hex_to_cur_chr; buffer[k-1]:=cur_chr;
//!       end
//!     else if c<@'100 then buffer[k-1]:=c+@'100
//!     else buffer[k-1]:=c-@'100;
//!     limit:=limit-d; first:=first-d;
//!     while k<=limit do
//!       begin buffer[k]:=buffer[k+d]; incr(k);
//!       end;
//!     goto start_cs;
//!     end;
//!   end;
//! end
//!
//! @ @<Scan ahead in the buffer...@>=
//! begin repeat cur_chr:=buffer[k]; cat:=cat_code(cur_chr); incr(k);
//! until (cat<>letter)or(k>limit);
//! @<If an expanded...@>;
//! if cat<>letter then decr(k);
//!   {now |k| points to first nonletter}
//! if k>loc+1 then {multiletter control sequence has been scanned}
//!   begin cur_cs:=id_lookup(loc,k-loc); loc:=k; goto found;
//!   end;
//! end
//!
//! @ Let's consider now what happens when |get_next| is looking at a token list.
//!
//! @<Input from token list, |goto restart| if end of list or
//!   if a parameter needs to be expanded@>=
//! if loc<>null then {list not exhausted}
//! @^inner loop@>
//!   begin t:=info(loc); loc:=link(loc); {move to next}
//!   if t>=cs_token_flag then {a control sequence token}
//!     begin cur_cs:=t-cs_token_flag;
//!     cur_cmd:=eq_type(cur_cs); cur_chr:=equiv(cur_cs);
//!     if cur_cmd>=outer_call then
//!       if cur_cmd=dont_expand then
//!         @<Get the next token, suppressing expansion@>
//!       else check_outer_validity;
//!     end
//!   else  begin cur_cmd:=t div @'400; cur_chr:=t mod @'400;
//!     case cur_cmd of
//!     left_brace: incr(align_state);
//!     right_brace: decr(align_state);
//!     out_param: @<Insert macro parameter and |goto restart|@>;
//!     othercases do_nothing
//!     endcases;
//!     end;
//!   end
//! else  begin {we are done with this token list}
//!   end_token_list; goto restart; {resume previous level}
//!   end
//!
//! @ The present point in the program is reached only when the |expand|
//! routine has inserted a special marker into the input. In this special
//! case, |info(loc)| is known to be a control sequence token, and |link(loc)=null|.
//!
//! @d no_expand_flag=257 {this characterizes a special variant of |relax|}
//!
//! @<Get the next token, suppressing expansion@>=
//! begin cur_cs:=info(loc)-cs_token_flag; loc:=null;@/
//! cur_cmd:=eq_type(cur_cs); cur_chr:=equiv(cur_cs);
//! if cur_cmd>max_command then
//!   begin cur_cmd:=relax; cur_chr:=no_expand_flag;
//!   end;
//! end
//!
//! @ @<Insert macro parameter...@>=
//! begin begin_token_list(param_stack[param_start+cur_chr-1],parameter);
//! goto restart;
//! end
//!
//! @ All of the easy branches of |get_next| have now been taken care of.
//! There is one more branch.
//!
//! @d end_line_char_inactive == (end_line_char<0)or(end_line_char>255)
//!
//! @<Move to next line of file, or |goto restart|...@>=
//! if name>17 then @<Read next line of file into |buffer|, or
//!   |goto restart| if the file has ended@>
//! else  begin if not terminal_input then {\.{\\read} line has ended}
//!     begin cur_cmd:=0; cur_chr:=0; return;
//!     end;
//!   if input_ptr>0 then {text was inserted during error recovery}
//!     begin end_file_reading; goto restart; {resume previous level}
//!     end;
//!   if selector<log_only then open_log_file;
//!   if interaction>nonstop_mode then
//!     begin if end_line_char_inactive then incr(limit);
//!     if limit=start then {previous line was empty}
//!       print_nl("(Please type a command or say `\end')");
//! @.Please type...@>
//!     print_ln; first:=start;
//!     prompt_input("*"); {input on-line into |buffer|}
//! @.*\relax@>
//!     limit:=last;
//!     if end_line_char_inactive then decr(limit)
//!     else  buffer[limit]:=end_line_char;
//!     first:=limit+1;
//!     loc:=start;
//!     end
//!   else fatal_error("*** (job aborted, no legal \end found)");
//! @.job aborted@>
//!     {nonstop mode, which is intended for overnight batch processing,
//!     never waits for on-line input}
//!   end
//!
//! @ The global variable |force_eof| is normally |false|; it is set |true|
//! by an \.{\\endinput} command.
//!
//! @<Glob...@>=
//! @!force_eof:boolean; {should the next \.{\\input} be aborted early?}
//!
//! @ @<Read next line of file into |buffer|, or
//!   |goto restart| if the file has ended@>=
//! begin incr(line); first:=start;
//! if not force_eof then
//!   begin if input_ln(cur_file,true) then {not end of file}
//!     firm_up_the_line {this sets |limit|}
//!   else force_eof:=true;
//!   end;
//! if force_eof then
//!   begin print_char(")"); decr(open_parens);
//!   update_terminal; {show user that file has been read}
//!   force_eof:=false;
//!   end_file_reading; {resume previous level}
//!   check_outer_validity; goto restart;
//!   end;
//! if end_line_char_inactive then decr(limit)
//! else  buffer[limit]:=end_line_char;
//! first:=limit+1; loc:=start; {ready to read}
//! end
//!
//! @ If the user has set the |pausing| parameter to some positive value,
//! and if nonstop mode has not been selected, each line of input is displayed
//! on the terminal and the transcript file, followed by `\.{=>}'.
//! \TeX\ waits for a response. If the response is simply |carriage_return|, the
//! line is accepted as it stands, otherwise the line typed is
//! used instead of the line in the file.
//!
//! @p procedure firm_up_the_line;
//! var k:0..buf_size; {an index into |buffer|}
//! begin limit:=last;
//! if pausing>0 then if interaction>nonstop_mode then
//!   begin wake_up_terminal; print_ln;
//!   if start<limit then for k:=start to limit-1 do print(buffer[k]);
//!   first:=limit; prompt_input("=>"); {wait for user response}
//! @.=>@>
//!   if last>first then
//!     begin for k:=first to last-1 do {move line down in buffer}
//!       buffer[k+start-first]:=buffer[k];
//!     limit:=start+last-first;
//!     end;
//!   end;
//! end;
//!
//! @ Since |get_next| is used so frequently in \TeX, it is convenient
//! to define three related procedures that do a little more:
//!
//! \yskip\hang|get_token| not only sets |cur_cmd| and |cur_chr|, it
//! also sets |cur_tok|, a packed halfword version of the current token.
//!
//! \yskip\hang|get_x_token|, meaning ``get an expanded token,'' is like
//! |get_token|, but if the current token turns out to be a user-defined
//! control sequence (i.e., a macro call), or a conditional,
//! or something like \.{\\topmark} or \.{\\expandafter} or \.{\\csname},
//! it is eliminated from the input by beginning the expansion of the macro
//! or the evaluation of the conditional.
//!
//! \yskip\hang|x_token| is like |get_x_token| except that it assumes that
//! |get_next| has already been called.
//!
//! \yskip\noindent
//! In fact, these three procedures account for almost every use of |get_next|.
//!
//! @ No new control sequences will be defined except during a call of
//! |get_token|, or when \.{\\csname} compresses a token list, because
//! |no_new_control_sequence| is always |true| at other times.
//!
//! @p procedure get_token; {sets |cur_cmd|, |cur_chr|, |cur_tok|}
//! begin no_new_control_sequence:=false; get_next; no_new_control_sequence:=true;
//! @^inner loop@>
//! if cur_cs=0 then cur_tok:=(cur_cmd*@'400)+cur_chr
//! else cur_tok:=cs_token_flag+cur_cs;
//! end;
//!
