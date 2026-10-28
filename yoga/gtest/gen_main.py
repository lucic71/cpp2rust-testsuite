import re
import subprocess
import sys

cxx, out, include_dirs, sources = sys.argv[1], sys.argv[2], sys.argv[3], sys.argv[4:]

flags = ["-std=c++20", "-E", "-P"] + [f"-I{d}" for d in include_dirs.split(";")]
pattern = re.compile(
    r"\bextern \"C\" void gtest_shim_test__(\w+?)__(\w+)\s*\(\s*\)\s*\{"
)

tests = []
for src in sources:
    text = subprocess.run(
        [cxx, *flags, src], capture_output=True, text=True, check=True
    ).stdout
    tests += pattern.findall(text)

with open(out, "w") as f:
    f.write("#include <cstdio>\n")
    f.write("#include <gtest/gtest.h>\n\n")
    for suite, name in tests:
        f.write(f"TEST({suite}, {name});\n")
    f.write("""
static int g_failures = 0;

namespace gtest_shim {

Message fail(const char* file, int line, const char* expr) {
  const char* base = std::strrchr(file, '/');
  std::printf("  %s:%d: %s\\n", base ? base + 1 : file, line, expr);
  g_failures++;
  return Message();
}

} // namespace gtest_shim

static int run(const char* name, void (*fn)()) {
  int before = g_failures;
  fn();
  int failed = g_failures != before;
  std::printf("[%s] %s\\n", failed ? "FAILED" : "    OK", name);
  return failed;
}

int main() {
  int failed = 0;
""")
    for suite, name in tests:
        f.write(
            f'  failed += run("{suite}.{name}", gtest_shim_test__{suite}__{name});\n'
        )
    f.write(
        """  std::printf("%d tests, %d failed\\n", """
        + str(len(tests))
        + """, failed);
  return failed != 0;
}
"""
    )
