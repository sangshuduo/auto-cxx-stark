#pragma once
#include <iostream>
#include <string>
#include "BairWitness.hpp"

inline void hello_world() {
    std::cout << "Hello World!" << std::endl;
}

inline std::unique_ptr<libstark::BairWitness> new_bair_witness() {
    return NULL;
//    return std::unique_ptr<libstark::BairWitness>(new libstark::BairWitness());
}
