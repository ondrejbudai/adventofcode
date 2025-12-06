const std = @import("std");
pub fn main() !void {
    const f = try std.fs.cwd().openFile("input", .{});
    defer f.close();

    var buffer: [100]u8 = undefined;
    var reader = f.reader(&buffer);
    var sum: usize = 0;
    var sum2: usize = 0;

    while (reader.interface.takeDelimiter(',')) |l| {
        if (l == null) {
            break;
        }
        const line = std.mem.trim(u8, l.?, "\n");

        var seq = std.mem.splitSequence(u8, line, "-");
        const startStr = seq.next().?;
        const endStr = seq.next().?;

        const start = try std.fmt.parseInt(u64, startStr, 10);
        const end = try std.fmt.parseInt(u64, endStr, 10);

        var buf: [50]u8 = undefined;
        for (start..end + 1) |i| {
            const number = try std.fmt.bufPrint(&buf, "{}", .{i});

            outer: for (1..number.len / 2 + 1) |len| {
                if (number.len % len != 0) {
                    continue;
                }
                var s: usize = len;
                while (s < number.len) {
                    if (!std.mem.eql(u8, number[s - len .. s], number[s .. s + len])) {
                        continue :outer;
                    }

                    s += len;
                }

                sum2 += i;
                break :outer;
            }

            if (number.len % 2 != 0) {
                continue;
            }

            if (std.mem.eql(u8, number[0 .. number.len / 2], number[number.len / 2 ..])) {
                sum += i;
            }
        }
    } else |err| return err;

    std.debug.print("{}\n", .{sum});
    std.debug.print("{}\n", .{sum2});
}
