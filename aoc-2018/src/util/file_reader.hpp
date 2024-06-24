#include <string>
#include <optional>
#include <vector>

std::optional<std::vector<std::string>> read_lines(std::string filename);

// constexpr const char* trunc_filename(const char* path) {
//     const char* file = path;
//     while (*path) {
//         if (*path++ == '/') {
//             file = path;
//         }
//     }
//     return file;
// }
//
// inline const char* file_name() {
//   return trunc_filename(__FILE__);
// }
