const std = @import("std");
const httpz = @import("httpz");

const Global = struct {
    hits: usize = 0,
    l: std.Thread.Mutex = .{},
};

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();

    var global = Global{};
    var server = try httpz.ServerCtx(*Global, *Global).init(allocator, .{}, &global);
    var router = server.router();
    router.get("/increment", increment);
    return server.listen();
}

fn increment(global: *Global, _: *httpz.Request, res: *httpz.Response) !void {
    global.l.lock();
    const hits = global.hits + 1;
    global.hits = hits;
    global.l.unlock();

    // or, more verbosse: httpz.ContentType.TEXT
    res.content_type = .TEXT;
    res.body = try std.fmt.allocPrint(res.arena, "{d} hits", .{hits});
}
