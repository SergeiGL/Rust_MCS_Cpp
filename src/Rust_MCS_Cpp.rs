use Rust_MCS::{mcs, ExitFlagEnum}; 
use nalgebra::{SMatrix, SVector}; 


// C-compatible enum for ExitFlag
#[repr(C)] 
pub enum ExitFlagEnum_C { 
    NormalShutdown, 
    StopNfExceeded, 
    StopNsweepsExceeded, 
} 
 
// C-compatible struct for MCS results
#[repr(C)] 
pub struct McsResult_C { 
    xbest: *mut f64, 
    fbest: f64, 
    xmin: *mut f64, 
    xmin_size: usize, 
    fmi: *mut f64, 
    fmi_size: usize, 
    ncall: usize, 
    ncloc: usize, 
    flag: ExitFlagEnum_C, 
}

// Type for the C++ callback function
type ObjFuncType = unsafe extern "C" fn(*const f64, usize) -> f64;

// Global callback storage - this is thread-safe thanks to the mutex
static mut CALLBACK: Option<ObjFuncType> = None;

// Function pointer trampoline (not a closure)
fn obj_func_trampoline<const N: usize>(x: &SVector<f64, N>) -> f64 {
    let func = unsafe {
        CALLBACK.expect("Callback not set") // Expect is safe here because mcs_c sets CALLBACK before calling mcs
    };
    unsafe { func(x.as_ptr(), N) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mcs_c( 
    c_func: ObjFuncType, // C++ callback function
    u_ptr: *const f64,
    v_ptr: *const f64,
    nsweeps: usize,
    nf: usize,
    local: usize,
    gamma: f64,
    hess_ptr: *const f64,
    smax: usize,
    n: usize
) -> McsResult_C {
    
    println!("Hi from Rust!");

    // Set the global callback
    unsafe {
        CALLBACK = Some(c_func);
    }


    // Call the appropriate helper based on dimensions
    match (smax, n) { 
        (20, 6) => { mcs_c_helper::<20, 6>(u_ptr, v_ptr, nsweeps, nf, local, gamma, hess_ptr) }
        (21, 6) => { mcs_c_helper::<21, 6>(u_ptr, v_ptr, nsweeps, nf, local, gamma, hess_ptr) }
        (22, 6) => { mcs_c_helper::<22, 6>(u_ptr, v_ptr, nsweeps, nf, local, gamma, hess_ptr) }
        (20, 8) => { mcs_c_helper::<20, 8>(u_ptr, v_ptr, nsweeps, nf, local, gamma, hess_ptr) }
        (25, 8) => { mcs_c_helper::<25, 8>(u_ptr, v_ptr, nsweeps, nf, local, gamma, hess_ptr) }
        _ => panic!("Unsupported SMAX and N combination: ({}, {})", smax, n)
    } 
} 

unsafe fn mcs_c_helper<const SMAX: usize, const N: usize>(
    u_ptr: *const f64,
    v_ptr: *const f64,
    nsweeps: usize,
    nf: usize,
    local: usize,
    gamma: f64,
    hess_ptr: *const f64
) -> McsResult_C 
{
    use std::slice;

    // Convert raw pointers to Rust types
    let u = SVector::<f64, N>::from_row_slice(slice::from_raw_parts(u_ptr, N)); 
    let v = SVector::<f64, N>::from_row_slice(slice::from_raw_parts(v_ptr, N)); 
    let hess = SMatrix::<f64, N, N>::from_row_slice(slice::from_raw_parts(hess_ptr, N * N)); 

    // Call the original mcs function with our trampoline
    let (xbest, fbest, xmin, fmi, ncall, ncloc, flag) = 
        mcs::<SMAX, N>(
            obj_func_trampoline::<N>,
            &u,
            &v,
            nsweeps,
            nf,
            local,
            gamma,
            &hess
        ).unwrap();
 
    // Allocate and copy results 
    let xbest_vec = xbest.as_slice().to_vec(); 
    let xbest_ptr = Box::into_raw(xbest_vec.into_boxed_slice()) as *mut f64; 
 
    let mut xmin_flat = Vec::with_capacity(xmin.len() * N); 
    for x in &xmin { 
        xmin_flat.extend_from_slice(x.as_slice()); 
    } 
    let xmin_ptr = Box::into_raw(xmin_flat.into_boxed_slice()) as *mut f64; 
 
    let fmi_ptr = Box::into_raw(fmi.clone().into_boxed_slice()) as *mut f64; 
 
    let flag_c = match flag { 
        ExitFlagEnum::NormalShutdown => ExitFlagEnum_C::NormalShutdown, 
        ExitFlagEnum::StopNfExceeded => ExitFlagEnum_C::StopNfExceeded, 
        ExitFlagEnum::StopNsweepsExceeded => ExitFlagEnum_C::StopNsweepsExceeded, 
    }; 
 
    McsResult_C { 
        xbest: xbest_ptr, 
        fbest, 
        xmin: xmin_ptr, 
        xmin_size: xmin.len(), 
        fmi: fmi_ptr, 
        fmi_size: fmi.len(), 
        ncall, 
        ncloc, 
        flag: flag_c, 
    } 
} 

#[unsafe(no_mangle)]
pub unsafe extern "C" fn free_mcs_result(result: *mut McsResult_C, n: usize) { 
    match n { 
        6 => free_mcs_result_helper::<6>(result), 
        8 => free_mcs_result_helper::<8>(result),
        10 => free_mcs_result_helper::<10>(result),
        // Add more dimensions as needed
        _ => panic!("Unsupported dimension: {}", n)
    } 
} 

unsafe fn free_mcs_result_helper<const N: usize>(result: *mut McsResult_C) { 
    use std::slice; 
 
    if !result.is_null() { 
        let result = &mut *result; 
 
        // Convert back to Box and drop 
        if !result.xbest.is_null() { 
            let _ = Box::from_raw(slice::from_raw_parts_mut(result.xbest, N)); 
        } 
        if !result.xmin.is_null() { 
            let _ = Box::from_raw(slice::from_raw_parts_mut(result.xmin, result.xmin_size * N)); 
        } 
        if !result.fmi.is_null() { 
            let _ = Box::from_raw(slice::from_raw_parts_mut(result.fmi, result.fmi_size)); 
        } 
    } 
}

// To properly deallocate the entire result structure
#[unsafe(no_mangle)]
pub unsafe extern "C" fn destroy_mcs_result(result: *mut McsResult_C, n: usize) {
    if !result.is_null() {
        // First free the contained pointers
        free_mcs_result(result, n);
        
        // Then free the struct itself
        let _ = Box::from_raw(result);
    }
}