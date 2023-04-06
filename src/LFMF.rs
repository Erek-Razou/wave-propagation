#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::ffi::{c_double, c_int};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct Result {
    pub A_btl__db: c_double,
    pub E_dBuVm: c_double,
    pub P_rx__dbm: c_double,

    pub method: c_int,
}

#[link(name = "lfmf", kind = "static")]
extern "C" {
    fn LFMF(
        h_tx__meter: c_double,
        h_rx__meter: c_double,
        f__mhz: c_double,
        P_tx__watt: c_double,
        N_s: c_double,
        d__km: c_double,
        epsilon: c_double,
        sigma: c_double,
        pol: c_int,
        result: *mut Result,
    ) -> c_int;
}

#[derive(Debug, Copy, Clone)]
pub struct LFMF_result {
    pub status: i32,
    pub A_btl__db: f64,
    pub E_dBuVm: f64,
    pub P_rx__dbm: f64,

    pub method: i32,
}

pub fn calc_LFMF(
    h_tx__meter: f64,
    h_rx__meter: f64,
    f__mhz: f64,
    P_tx__watt: f64,
    N_s: f64,
    d__km: f64,
    epsilon: f64,
    sigma: f64,
    pol: i32,
) -> LFMF_result {
    let mut c_result = Result {
        A_btl__db: 0.0,
        E_dBuVm: 0.0,
        P_rx__dbm: 0.0,
        method: 0,
    };
    let status = unsafe {
        LFMF(
            h_tx__meter,
            h_rx__meter,
            f__mhz,
            P_tx__watt,
            N_s,
            d__km,
            epsilon,
            sigma,
            pol,
            &mut c_result,
        )
    };
    LFMF_result {
        status,
        A_btl__db: c_result.A_btl__db,
        E_dBuVm: c_result.E_dBuVm,
        P_rx__dbm: c_result.P_rx__dbm,
        method: c_result.method,
    }
}
