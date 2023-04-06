#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::ffi::{c_double, c_int};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct c_Result {
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
        result: *mut c_Result,
    ) -> c_int;
}

#[derive(Debug, Copy, Clone)]
pub struct LFMF_Parameters {
    pub h_tx__meter: f64,
    pub h_rx__meter: f64,
    pub f__mhz: f64,
    pub P_tx__watt: f64,
    pub N_s: f64,
    pub d__km: f64,
    pub epsilon: f64,
    pub sigma: f64,
    pub pol: i32,
}

#[derive(Debug, Copy, Clone)]
pub struct LFMF_Result {
    pub A_btl__db: f64,
    pub E_dBuVm: f64,
    pub P_rx__dbm: f64,

    pub method: i32,
}

#[derive(Debug, Clone)]
pub struct LFMF_Error {
    pub status: i32,
    pub message: String,
}

pub fn calc_LFMF(parameters: LFMF_Parameters) -> Result<LFMF_Result, LFMF_Error> {
    let mut c_result = c_Result {
        A_btl__db: 0.0,
        E_dBuVm: 0.0,
        P_rx__dbm: 0.0,
        method: 0,
    };
    let status = unsafe {
        LFMF(
            parameters.h_tx__meter,
            parameters.h_rx__meter,
            parameters.f__mhz,
            parameters.P_tx__watt,
            parameters.N_s,
            parameters.d__km,
            parameters.epsilon,
            parameters.sigma,
            parameters.pol,
            &mut c_result,
        )
    };
    match status {
        0 => Ok(LFMF_Result {
            A_btl__db: c_result.A_btl__db,
            E_dBuVm: c_result.E_dBuVm,
            P_rx__dbm: c_result.P_rx__dbm,
            method: c_result.method,
        }),
        1000 => Err(LFMF_Error {
            status,
            message: format!(
                "TX terminal height is {} which is out of the range 0 <= h_tx__meter <= 50.",
                { parameters.h_tx__meter }
            ),
        }),
        1001 => Err(LFMF_Error {
            status,
            message: format!(
                "RX terminal height is {} which is out of the range 0 <= h_rx__meter <= 50.",
                { parameters.h_rx__meter }
            ),
        }),
        1002 => Err(LFMF_Error {
            status,
            message: format!(
                "Frequency is {} which is out of the range 0.01 <= f__mhz <= 30.",
                { parameters.f__mhz }
            ),
        }),
        1003 => Err(LFMF_Error {
            status,
            message: format!(
                "Transmit power is {} which is out of the range 0 < P_tx__watt.",
                { parameters.P_tx__watt }
            ),
        }),
        1004 => Err(LFMF_Error {
            status,
            message: format!(
                "Surface refractivity is {} which is out of the range 250 <= N_s <= 400.",
                { parameters.N_s }
            ),
        }),
        1005 => Err(LFMF_Error {
            status,
            message: format!(
                "Path distance is {} which is out of the range d__km <= 10000.",
                { parameters.d__km }
            ),
        }),
        1006 => Err(LFMF_Error {
            status,
            message: format!(
                "Epsilon (relative permittivity) is {} which is out of the range 1 <= epsilon.",
                { parameters.epsilon }
            ),
        }),
        1007 => Err(LFMF_Error {
            status,
            message: format!(
                "Sigma (conductivity) is {} which is out of the range 0 < sigma.",
                { parameters.sigma }
            ),
        }),
        1008 => Err(LFMF_Error {
            status,
            message: format!(
                "Polarization is {} which is invalid as it must be either 0 or 1.",
                { parameters.pol }
            ),
        }),
        _ => unreachable!("Unknown lfmf error status: {}", status),
    }
}
