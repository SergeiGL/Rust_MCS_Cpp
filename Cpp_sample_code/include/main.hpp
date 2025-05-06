#pragma once
#include <vector>
#include <memory>

extern "C" {
    enum class ExitFlagEnum_C {
        NormalShutdown,
        StopNfExceeded,
        StopNsweepsExceeded
    };

    struct McsResult_C {
        double* xbest;
        double  fbest;
        double* xmin;
        size_t  xmin_size;
        double* fmi;
        size_t  fmi_size;
        size_t  ncall;
        size_t  ncloc;
        ExitFlagEnum_C flag;
    };

    typedef double (*ObjFuncType)(const double* x, size_t n);

    McsResult_C mcs_c(
        ObjFuncType func,
        const double* u,
        const double* v,
        size_t nsweeps,
        size_t nf,
        size_t local,
        double gamma,
        const double* hess,
        size_t smax,
        size_t n
    );

    void free_mcs_result(McsResult_C* result, size_t n);
}