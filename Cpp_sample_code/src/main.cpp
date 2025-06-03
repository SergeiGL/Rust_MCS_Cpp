#include <vector>
#include <memory>
#include "../include/main.hpp"
#include <iostream>


extern "C" double simple_func_c(const double* x, size_t n) {
    // build a std::vector<double> view over the C array
    std::vector<double> vx(x, x + n);
    return -123456.0;
}

int main() {
    // Problem dimension
    const size_t n = 6;
    const size_t smax = 20;

    // Lower bounds vector
    std::vector<double> u(n, 0.0);
    // Upper bounds vector
    std::vector<double> v(n, 10.0);

    // Algorithm parameters
    size_t nsweeps = 100;
    size_t nf = 1000;
    size_t local = 20;
    double gamma = 0.5;

    // Identity matrix for Hessian (or provide your own)
    std::vector<double> hess(n * n, 1.0);

    // Call the MCS algorithm
    std::cout << "Starting MCS optimization..." << std::endl;
    McsResult_C result = mcs_c(
        simple_func_c,
        u.data(),
        v.data(),
        nsweeps, 
        nf, 
        local, 
        gamma, 
        hess.data(),
        smax,
        n
    );


    std::cout << "fbest: " << result.fbest << std::endl;

    free_mcs_result(&result, n);

    return 0;
}