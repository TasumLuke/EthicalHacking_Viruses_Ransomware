#include <fstream>
#include <string>

int main() {
  std::ifstream source_file(__FILE__);
  std::ofstream output_file(__FILE__, std::ios::trunc);
  std::string line;

  while (std::getline(source_file, line)) {
    output_file << line << std::endl;
  }

  return 0;
}
