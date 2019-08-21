use core::convert::TryInto;
use libm::F64Ext;

/** PLL configuration */
pub struct Params {
    pub clkr: u8,
    pub clkf: u8,
    pub clkod: u8,
    pub bwadj: u8,
}

/* constants for PLL frequency computation */
const VCO_MIN: f64 = 3.5e+08;
const VCO_MAX: f64 = 1.75e+09;
const REF_MIN: f64 = 1.36719e+07;
const REF_MAX: f64 = 1.75e+09;
const NR_MIN: i32 = 1;
const NR_MAX: i32 = 16;
const NF_MIN: i32 = 1;
const NF_MAX: i32 = 64;
const NO_MIN: i32 = 1;
const NO_MAX: i32 = 16;
const NB_MIN: i32 = 1;
const NB_MAX: i32 = 64;
const MAX_VCO: bool = true;
const REF_RNG: bool = true;

/*
 * Calculate PLL registers' value by finding closest matching parameters
 * NOTE: this uses floating point math ... this is horrible for something so critical :-(
 * TODO: implement this without fp ops
 */
pub fn compute_params(freq_in: u32, freq_out: u32) -> Option<Params> {
    let fin: f64 = freq_in.into();
    let fout: f64 = freq_out.into();
    let val: f64 = fout / fin;
    let terr: f64 = 0.5 / ((NF_MAX / 2) as f64);
    // NOTE: removed the handling that the Kendryte SDK has for terr<=0.0, as this is impossible
    // given that NF_MAX is a positive integer constant
    let mut merr: f64 = terr;
    let mut x_nrx: i32 = 0;
    let mut x_no: i32 = 0;

    // Parameters to be exported from the loop
    let mut found: Option<Params> = None;
    for nfi in (val as i32)..NF_MAX {
        let nr: i32 = ((nfi as f64) / val) as i32;
        if nr == 0 {
            continue;
        }
        if REF_RNG && (nr < NR_MIN) {
            continue;
        }
        if fin / (nr as f64) > REF_MAX {
            continue;
        }
        let mut nrx: i32 = nr;
        let mut nf: i32 = nfi;
        let mut nfx: i64 = nfi.into();
        let nval: f64 = (nfx as f64) / (nr as f64);
        if nf == 0 {
            nf = 1;
        }
        let err: f64 = 1.0 - nval / val;

        if (err.abs() < merr * (1.0 + 1e-6)) || (err.abs() < 1e-16) {
            let mut not: i32 = (VCO_MAX / fout).floor() as i32;
            let mut no: i32 = if not > NO_MAX { NO_MAX } else { not };
            while no > NO_MIN {
                if (REF_RNG) && ((nr / no) < NR_MIN) {
                    no -= 1;
                    continue;
                }
                if (nr % no) == 0 {
                    break;
                }
                no -= 1;
            }
            if (nr % no) != 0 {
                continue;
            }
            let mut nor: i32 = (if not > NO_MAX { NO_MAX } else { not }) / no;
            let mut nore: i32 = NF_MAX / nf;
            if nor > nore {
                nor = nore;
            }
            let noe: i32 = (VCO_MIN / fout).ceil() as i32;
            if !MAX_VCO {
                nore = (noe - 1) / no + 1;
                nor = nore;
                not = 0; /* force next if to fail */
            }
            if (((no * nor) < (not >> 1)) || ((no * nor) < noe)) && ((no * nor) < (NF_MAX / nf)) {
                no = NF_MAX / nf;
                if no > NO_MAX {
                    no = NO_MAX;
                }
                if no > not {
                    no = not;
                }
                nfx *= no as i64;
                nf *= no;
                if (no > 1) && !found.is_none() {
                    continue;
                }
            /* wait for larger nf in later iterations */
            } else {
                nrx /= no;
                nfx *= nor as i64;
                nf *= nor;
                no *= nor;
                if no > NO_MAX {
                    continue;
                }
                if (nor > 1) && !found.is_none() {
                    continue;
                }
                /* wait for larger nf in later iterations */
            }

            let mut nb: i32 = nfx as i32;
            if nb < NB_MIN {
                nb = NB_MIN;
            }
            if nb > NB_MAX {
                continue;
            }

            let fvco: f64 = fin / (nrx as f64) * (nfx as f64);
            if fvco < VCO_MIN {
                continue;
            }
            if fvco > VCO_MAX {
                continue;
            }
            if nf < NF_MIN {
                continue;
            }
            if REF_RNG && (fin / (nrx as f64) < REF_MIN) {
                continue;
            }
            if REF_RNG && (nrx > NR_MAX) {
                continue;
            }
            if found.is_some() {
                // check that this reduces error compared to minimum error value, or is an improvement
                // in no or nrx
                if !((err.abs() < merr * (1.0 - 1e-6)) || (MAX_VCO && (no > x_no))) {
                    continue;
                }
                if nrx > x_nrx {
                    continue;
                }
            }

            found = Some(Params {
                clkr: (nrx - 1).try_into().unwrap(),
                clkf: (nfx - 1).try_into().unwrap(),
                clkod: (no - 1).try_into().unwrap(),
                bwadj: (nb - 1).try_into().unwrap(),
            });
            merr = err.abs();
            x_no = no;
            x_nrx = nrx;
        }
    }
    if merr >= terr * (1.0 - 1e-6) {
        None
    } else {
        found
    }
}

// TODO: add tests
