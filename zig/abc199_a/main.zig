const std = @import("std");

pub fn main() void {
    var stdin = std.io.getStdIn().reader();
    var stdout = std.io.getStdOut().writer();
    var buf: [100]u8 = undefined;

    const line = stdin.readUntilDelimiterOrEof(&buf, '\n') catch return;
    var tokens = std.mem.tokenizeScalar(u8, line.?, ' ');

    const a = std.fmt.parseInt(i32, tokens.next().?, 10) catch return;
    const b = std.fmt.parseInt(i32, tokens.next().?, 10) catch return;
    const c = std.fmt.parseInt(i32, tokens.next().?, 10) catch return;

    if (a * a + b * b < c * c) {
        _ = stdout.print("Yes\n", .{}) catch {};
    } else {
        _ = stdout.print("No\n", .{}) catch {};
    }
}
