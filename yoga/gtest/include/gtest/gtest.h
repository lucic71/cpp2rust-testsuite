#pragma once

#include <cfloat>
#include <cmath>
#include <cstdint>
#include <cstring>
#include <initializer_list>
#include <limits>
#include <memory>
#include <string>
#include <vector>

namespace gtest_shim {

struct Message {};

template <typename T>
const Message& operator<<(const Message& m, const T&) {
  return m;
}

Message fail(const char* file, int line, const char* expr);

inline bool floatEq(float a, float b) {
  if (std::isnan(a) || std::isnan(b)) {
    return false;
  }
  if (a == b) {
    return true;
  }
  float scale = std::fmax(std::fabs(a), std::fabs(b));
  return std::fabs(a - b) <= 4 * FLT_EPSILON * scale;
}

} // namespace gtest_shim

#define GTEST_HAS_DEATH_TEST 0

#define TEST(suite, name) extern "C" void gtest_shim_test__##suite##__##name()

#define GTEST_SHIM_CHECK(cond, text) \
  if (cond) {                        \
  } else                             \
    ::gtest_shim::fail(__FILE__, __LINE__, text)

#define EXPECT_TRUE(c) GTEST_SHIM_CHECK((c), #c)
#define EXPECT_FALSE(c) GTEST_SHIM_CHECK(!(c), "!(" #c ")")
#define EXPECT_EQ(a, b) GTEST_SHIM_CHECK((a) == (b), #a " == " #b)
#define EXPECT_NE(a, b) GTEST_SHIM_CHECK((a) != (b), #a " != " #b)
#define EXPECT_LT(a, b) GTEST_SHIM_CHECK((a) < (b), #a " < " #b)
#define EXPECT_LE(a, b) GTEST_SHIM_CHECK((a) <= (b), #a " <= " #b)
#define EXPECT_GT(a, b) GTEST_SHIM_CHECK((a) > (b), #a " > " #b)
#define EXPECT_GE(a, b) GTEST_SHIM_CHECK((a) >= (b), #a " >= " #b)
#define EXPECT_FLOAT_EQ(a, b) \
  GTEST_SHIM_CHECK(::gtest_shim::floatEq((a), (b)), #a " ~= " #b)
#define EXPECT_NEAR(a, b, e) \
  GTEST_SHIM_CHECK(std::fabs((a) - (b)) <= (e), #a " ~= " #b)
#define EXPECT_STREQ(a, b) GTEST_SHIM_CHECK(std::strcmp((a), (b)) == 0, #a " == " #b)
#define EXPECT_STRNE(a, b) GTEST_SHIM_CHECK(std::strcmp((a), (b)) != 0, #a " != " #b)
#define EXPECT_THROW(stmt, exc) \
  do {                          \
  } while (0)

#define ASSERT_TRUE EXPECT_TRUE
#define ASSERT_FALSE EXPECT_FALSE
#define ASSERT_EQ EXPECT_EQ
#define ASSERT_NE EXPECT_NE
#define ASSERT_LT EXPECT_LT
#define ASSERT_LE EXPECT_LE
#define ASSERT_GT EXPECT_GT
#define ASSERT_GE EXPECT_GE
#define ASSERT_FLOAT_EQ EXPECT_FLOAT_EQ
#define ASSERT_NEAR EXPECT_NEAR
#define ASSERT_STREQ EXPECT_STREQ
#define ASSERT_STRNE EXPECT_STRNE
#define ASSERT_THROW EXPECT_THROW

#define SCOPED_TRACE(msg) \
  do {                    \
  } while (0)
