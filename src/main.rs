fn main() {
    let context_lines = 2;
    let needle = "oo";
    let haystack = "Every face, every shop,
bedroom window, public-house, and
dark square is a picture
feverishly turned--in search of what?
It is the same with books.
What do we seek
through millions of pages?";

    // tags holds line numbers where matches occur
    let mut tags: Vec<usize> = Vec::new();

    // ctx contains a vector per match to hold that match’s context lines
    let mut ctx: Vec<Vec<(usize, String)>> = Vec::new();

    // Iterate through the lines, recording line numbers where matches are encountered
    for (i, line) in haystack.lines().enumerate() {
        if line.contains(needle) {
            tags.push(i);

            // Vec::with_capacity(n) reserves space for n items
            let v = Vec::with_capacity(2 * context_lines + 1);

            // No explicit type signature is required here, as it can be inferred via the definition of ctx on line 15
            ctx.push(v);
            println!("{}: {}", i, line);
        }
    }

    // When there are no matches, exit early
    if tags.len() == 0 {
        return;
    }

    // For each tag, at every line, check to see if we are nearby a match.
    // When we are, add that line to the relevant Vec<T> within ctx.
    for (i, line) in haystack.lines().enumerate() {
        for (j, tag) in tags.iter().enumerate() {
            // usize.saturating_sub() is subtraction that returns 0 on integer
            // underflow rather than crashing the program (CPUs don’t like
            //  attempting to send usize below zero)
            let lower_bound = tag.saturating_sub(context_lines);
            let upper_bound = tag + context_lines;

            if (i >= lower_bound) && (i <= upper_bound) {
                // Copy line into a new String and store that locally for each match
                let line_as_string = String::from(line);
                let local_ctx = (i, line_as_string);
                ctx[j].push(local_ctx);
            }
        }
    }

    for local_ctx in ctx.iter() {
        // ref line informs the compiler that we wish to borrow this value, rather than
        // move it. These two terms are explained fully later in later chapters.
        println!("===========================================");
        for &(i, ref line) in local_ctx.iter() {
            let line_num = i + 1;
            println!("{}: {}", line_num, line);
        }
    }
}
