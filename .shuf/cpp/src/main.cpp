#include <algorithm>
#include <chrono>
#include <filesystem>
#include <iostream>
#include <random>
#include <string>
#include <vector>

int main(int argc, char** argv) {
    std::vector<std::string> file_vector;
    for (const auto& entry : std::filesystem::directory_iterator(".")) {
        file_vector.push_back(entry.path().string().erase(0,2));
    }

    // get a time-based seed
    unsigned seed = std::chrono::system_clock::now().time_since_epoch().count();
    std::shuffle(std::begin(file_vector), std::end(file_vector), std::default_random_engine(seed));

    if (argc <= 1) {
        std::cout << file_vector[0] << std::endl;
    } else {
        std::string command_string = "mpv " + file_vector[0];
        const char *command = command_string.c_str();
        std::system(command);
    }
}
