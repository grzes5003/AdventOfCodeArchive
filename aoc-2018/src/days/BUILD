COPTS = ["/std:c++20"]

cc_library(
  name = "days",
  srcs = glob(["*.cpp"]),
  hdrs = glob(["*.hpp"]),
  deps = [
    "//src/util:util",
  ],
  copts = COPTS,
  visibility = ["//visibility:public"]
)
