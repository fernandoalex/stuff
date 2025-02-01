const std = @import("std");

pub fn main() !void {
    // stdout is for the actual output of your application, for example if you
    // are implementing gzip, then only the compressed bytes should be sent to
    // stdout, not any debugging messages.
    const stdout_file = std.io.getStdOut().writer();
    var bw = std.io.bufferedWriter(stdout_file);
    const stdout = bw.writer();

    try stdout.print("Fizz buzz.\n", .{});

    var count: u8 = 1;
    while (count <= 100) : (count += 1) {
        try stdout.print("{}\n", .{count});
    }

    try bw.flush(); // don't forget to flush!
}
