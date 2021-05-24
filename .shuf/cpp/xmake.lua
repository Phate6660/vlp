set_project(".shuf")
set_config("cxxflags", "-std=c++2a " .. os.getenv("CXXFLAGS"))

target(".shuf")
    set_kind("binary")
    add_files("src/main.cpp")
