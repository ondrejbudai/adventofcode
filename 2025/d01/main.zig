const std = @import("std");

const Dir = enum { left, right };

pub fn rotate(input: i32, dir: Dir, count: u32) struct { i32, i32 } {
    var ret = input;
    var count_zero: i32 = 0;
    for (0..count) |_| {
        ret = switch (dir) {
            Dir.left => ret - 1,
            Dir.right => ret + 1,
        };

        if (ret < 0) {
            ret += 100;
        } else if (ret >= 100) {
            ret -= 100;
        }

        if (ret == 0) {
            count_zero += 1;
        }
    }

    return .{ ret, count_zero };
}

pub fn main() !void {
    const input = try std.fs.cwd().openFile("input", .{});
    var buffer: [50]u8 = undefined;
    var reader = input.reader(&buffer);

    var dial: i32 = 50;
    var zero_count: i32 = 0;
    var zero_count_all: i32 = 0;

    while (reader.interface.takeDelimiter('\n')) |l| {
        if (l == null) {
            break;
        }
        const line = l.?;

        const dir = switch (line[0]) {
            'L' => Dir.left,
            'R' => Dir.right,
            else => unreachable,
        };

        const count = try std.fmt.parseInt(u32, line[1..], 10);
        dial, const zeros_met = rotate(dial, dir, count);

        zero_count_all += zeros_met;

        if (dial == 0) {
            zero_count += 1;
        }
    } else |err| return err;

    std.debug.print("{}\n", .{zero_count});
    std.debug.print("{}\n", .{zero_count_all});

    defer input.close();
}
