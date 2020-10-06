//! @* \[32] Shipping pages out.
//! After considering \TeX's eyes and stomach, we come now to the bowels.
//! @^bowels@>
//!
//! The |ship_out| procedure is given a pointer to a box; its mission is
//! to describe that box in \.{DVI} form, outputting a ``page'' to |dvi_file|.
//! The \.{DVI} coordinates $(h,v)=(0,0)$ should correspond to the upper left
//! corner of the box being shipped.
//!
//! Since boxes can be inside of boxes inside of boxes, the main work of
//! |ship_out| is done by two mutually recursive routines, |hlist_out|
//! and |vlist_out|, which traverse the hlists and vlists inside of horizontal
//! and vertical boxes.
//!
//! As individual pages are being processed, we need to accumulate
//! information about the entire set of pages, since such statistics must be
//! reported in the postamble. The global variables |total_pages|, |max_v|,
//! |max_h|, |max_push|, and |last_bop| are used to record this information.
//!
//! The variable |doing_leaders| is |true| while leaders are being output.
//! The variable |dead_cycles| contains the number of times an output routine
//! has been initiated since the last |ship_out|.
//!
//! A few additional global variables are also defined here for use in
//! |vlist_out| and |hlist_out|. They could have been local variables, but
//! that would waste stack space when boxes are deeply nested, since the
//! values of these variables are not needed during recursive calls.
//! @^recursion@>
//!
//! @<Glob...@>=
//! @!total_pages:integer; {the number of pages that have been shipped out}
//! @!max_v:scaled; {maximum height-plus-depth of pages shipped so far}
//! @!max_h:scaled; {maximum width of pages shipped so far}
//! @!max_push:integer; {deepest nesting of |push| commands encountered so far}
//! @!last_bop:integer; {location of previous |bop| in the \.{DVI} output}
//! @!dead_cycles:integer; {recent outputs that didn't ship anything out}
//! @!doing_leaders:boolean; {are we inside a leader box?}
//! @#
//! @!c,@!f:quarterword; {character and font in current |char_node|}
//! @!rule_ht,@!rule_dp,@!rule_wd:scaled; {size of current rule being output}
//! @!g:pointer; {current glue specification}
//! @!lq,@!lr:integer; {quantities used in calculations for leaders}
//!
//! @ @<Set init...@>=
//! total_pages:=0; max_v:=0; max_h:=0; max_push:=0; last_bop:=-1;
//! doing_leaders:=false; dead_cycles:=0; cur_s:=-1;
//!
//! @ The \.{DVI} bytes are output to a buffer instead of being written directly
//! to the output file. This makes it possible to reduce the overhead of
//! subroutine calls, thereby measurably speeding up the computation, since
//! output of \.{DVI} bytes is part of \TeX's inner loop. And it has another
//! advantage as well, since we can change instructions in the buffer in order to
//! make the output more compact. For example, a `|down2|' command can be
//! changed to a `|y2|', thereby making a subsequent `|y0|' command possible,
//! saving two bytes.
//!
//! The output buffer is divided into two parts of equal size; the bytes found
//! in |dvi_buf[0..half_buf-1]| constitute the first half, and those in
//! |dvi_buf[half_buf..dvi_buf_size-1]| constitute the second. The global
//! variable |dvi_ptr| points to the position that will receive the next
//! output byte. When |dvi_ptr| reaches |dvi_limit|, which is always equal
//! to one of the two values |half_buf| or |dvi_buf_size|, the half buffer that
//! is about to be invaded next is sent to the output and |dvi_limit| is
//! changed to its other value. Thus, there is always at least a half buffer's
//! worth of information present, except at the very beginning of the job.
//!
//! Bytes of the \.{DVI} file are numbered sequentially starting with 0;
//! the next byte to be generated will be number |dvi_offset+dvi_ptr|.
//! A byte is present in the buffer only if its number is |>=dvi_gone|.
//!
//! @<Types...@>=
//! @!dvi_index=0..dvi_buf_size; {an index into the output buffer}
//!
//! @ Some systems may find it more efficient to make |dvi_buf| a |packed|
//! array, since output of four bytes at once may be facilitated.
//! @^system dependencies@>
//!
//! @<Glob...@>=
//! @!dvi_buf:array[dvi_index] of eight_bits; {buffer for \.{DVI} output}
//! @!half_buf:dvi_index; {half of |dvi_buf_size|}
//! @!dvi_limit:dvi_index; {end of the current half buffer}
//! @!dvi_ptr:dvi_index; {the next available buffer address}
//! @!dvi_offset:integer; {|dvi_buf_size| times the number of times the
//!   output buffer has been fully emptied}
//! @!dvi_gone:integer; {the number of bytes already output to |dvi_file|}
//!
//! @ Initially the buffer is all in one piece; we will output half of it only
//! after it first fills up.
//!
//! @<Set init...@>=
//! half_buf:=dvi_buf_size div 2; dvi_limit:=dvi_buf_size; dvi_ptr:=0;
//! dvi_offset:=0; dvi_gone:=0;
//!
//! @ The actual output of |dvi_buf[a..b]| to |dvi_file| is performed by calling
//! |write_dvi(a,b)|. For best results, this procedure should be optimized to
//! run as fast as possible on each particular system, since it is part of
//! \TeX's inner loop. It is safe to assume that |a| and |b+1| will both be
//! multiples of 4 when |write_dvi(a,b)| is called; therefore it is possible on
//! many machines to use efficient methods to pack four bytes per word and to
//! output an array of words with one system call.
//! @^system dependencies@>
//! @^inner loop@>
//! @^defecation@>
//!
//! @p procedure write_dvi(@!a,@!b:dvi_index);
//! var k:dvi_index;
//! begin for k:=a to b do write(dvi_file,dvi_buf[k]);
//! end;
//!
//! @ To put a byte in the buffer without paying the cost of invoking a procedure
//! each time, we use the macro |dvi_out|.
//!
//! @d dvi_out(#)==@+begin dvi_buf[dvi_ptr]:=#; incr(dvi_ptr);
//!   if dvi_ptr=dvi_limit then dvi_swap;
//!   end
//!
//! @p procedure dvi_swap; {outputs half of the buffer}
//! begin if dvi_limit=dvi_buf_size then
//!   begin write_dvi(0,half_buf-1); dvi_limit:=half_buf;
//!   dvi_offset:=dvi_offset+dvi_buf_size; dvi_ptr:=0;
//!   end
//! else  begin write_dvi(half_buf,dvi_buf_size-1); dvi_limit:=dvi_buf_size;
//!   end;
//! dvi_gone:=dvi_gone+half_buf;
//! end;
//!
//! @ Here is how we clean out the buffer when \TeX\ is all through; |dvi_ptr|
//! will be a multiple of~4.
//!
//! @<Empty the last bytes out of |dvi_buf|@>=
//! if dvi_limit=half_buf then write_dvi(half_buf,dvi_buf_size-1);
//! if dvi_ptr>0 then write_dvi(0,dvi_ptr-1)
