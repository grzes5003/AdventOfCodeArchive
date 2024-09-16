COPTS = ["/std:c++20"]

cc_test(
  name = "days_test",
  size = "small",
  srcs = glob(["*_test.cpp"]),
  copts = COPTS,
  deps = [
    "//src/days:days",
    "@googletest//:gtest",
    "@googletest//:gtest_main",
  ]
)
