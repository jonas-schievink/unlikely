This does not work.

LLVM does not seem to uplift a call to a `cold` function to corresponding branch metadata. `cold` is only respected in other circumstances. Either way, it seems to get completely lost when the call gets inlined, so it's useless in any case.

Oh well.
