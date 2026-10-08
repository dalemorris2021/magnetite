const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    const c_module = b.createModule(.{
        .target = target,
        .optimize = optimize,
        .link_libc = true,
    });

    c_module.addIncludePath(b.path("src"));

    c_module.addCSourceFiles(.{
        .files = &.{
            "src/main.c",
            "src/lib.c",
        },
        .flags = &.{ "-std=c99", "-g", "-Wall", "-Wextra", "-Wpedantic", "-Werror" },
    });

    const exe = b.addExecutable(.{
        .name = "magnetite",
        .root_module = c_module,
    });

    b.installArtifact(exe);

    const run_cmd = b.addRunArtifact(exe);
    run_cmd.step.dependOn(b.getInstallStep());

    if (b.args) |args| {
        run_cmd.addArgs(args);
    }

    const run_step = b.step("run", "Run the app");
    run_step.dependOn(&run_cmd.step);
}
