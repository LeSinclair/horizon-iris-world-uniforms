# Applies the iris_world bundle by hand (as horizon-mixins would) to a Horizon checkout, for testing.
# Usage: python3 dev/simulate.py <horizon checkout at the pinned commit> iris_world
import sys, pathlib, re
src = pathlib.Path(sys.argv[1]); b = pathlib.Path(sys.argv[2])
def r(p): return (src/p).read_text()
def w(p, t): (src/p).write_text(t)
def helper(name): return b.joinpath(name).read_text()
lib = "crates/horizon-shaders/src/lib.rs"; w(lib, r(lib) + "\npub mod iris_world {\n" + helper("state.rs") + "}\n")
cu = "crates/horizon-shaders/src/custom.rs"; t = r(cu)
# RETURN on parse: the tail `out` (first function ending in `        out\n    }` after `pub fn parse`)
i = t.index("pub fn parse(")
j = t.index("        out\n    }", i)
t = t[:j] + "        { " + helper("add_uniforms.rs") + " add_uniforms(out) }\n    }" + t[j+len("        out\n    }"):]
i = t.index("pub fn eval(&mut self")
j = t.index("{\n", i) + 2
t = t[:j] + "        { " + helper("frame_values.rs") + " frame_values(&mut *self, builtin); }\n" + t[j:]
t += "\npub(crate) mod iris_world_items {\n" + helper("items.rs") + "}\n"
w(cu, t)
end = "crates/horizon-client/src/game/end.rs"; w(end, r(end) + "\npub(crate) mod iris_world_flash {\n" + helper("flash.rs") + "}\n")
sp = "crates/horizon-client/src/game/shader_packs.rs"; t = r(sp)
i = t.index("pub(super) fn shader_pack_inputs(")
j = t.index("{\n", i) + 2
t = t[:j] + "        { " + helper("game_values.rs") + " game_values(&*self, partial); }\n" + t[j:]
w(sp, t)
