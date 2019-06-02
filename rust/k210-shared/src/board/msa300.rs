//! Support for MSA300 accelerometer
/* MSA300 code based on 'accelerometer' demo by j.m.voogd@gmail.com */
use crate::soc::i2c::I2C;

/** MSA300 Registers */
enum reg {
    /** Part ID (R) */
    PARTID = 0x01,
    /** X-acceleration LSB (R) */
    ACC_X_LSB = 0x02,
    /** X-acceleration MSB (R) */
    ACC_X_MSB = 0x03,
    /** Y-acceleration LSB (R) */
    ACC_Y_LSB = 0x04,
    /** Y-acceleration MSB (R) */
    ACC_Y_MSB = 0x05,
    /** Z-acceleration LSB (R) */
    ACC_Z_LSB = 0x06,
    /** Z-acceleration MSB (R) */
    ACC_Z_MSB = 0x07,
    /** Resolution/Range (R/W) */
    RES_RANGE = 0x0F,
    /** Output Data Rate (R/W) */
    ODR = 0x10,
    /** Power Mode/Bandwidth (R/W) */
    PWR_MODE_BW = 0x11,
    /** Interrupt Set 0 (R/W) */
    INT_SET_0 = 0x16,
    /** Interrupt Set 1 (R/W) */
    INT_SET_1 = 0x17,
    /** Interrupt Latch (R/W) */
    INT_LATCH = 0x21,
}

/** 0.488mg per lsb */
const MG2G_MULTIPLIER_4_G: f32 = 0.000488;

/** 500Hz Bandwidth, not available in low power mode */
const DATARATE_1000_HZ: u8 = 0x0F;   

/** Gravity constant (Earth surface) */
const GRAVITY: f32 = 9.80665;

pub struct Accelerometer<IF> {
    i2c: IF,
}

/** Read a register of the MSA300 via I2C */
fn read_register<IF: I2C>(i2c: &IF, reg: reg) -> Result<u8, ()> {
    let mut reg_val = [0u8; 2];
    if i2c.recv_data(&[reg as u8], &mut reg_val).is_ok() {
        Ok(reg_val[0])
    } else {
        Err(())
    }
}

/** Set a register of the MSA300 via I2C */
fn set_register<IF: I2C>(i2c: &IF, reg: reg, val: u8) -> Result<(), ()> {
    i2c.send_data(&[reg as u8, val])
}

impl<IF: I2C> Accelerometer<IF> {
    /** Initialize chip */
    pub fn init(i2c: IF) -> Result<Self, ()> {
        let correct_id = 0x13;
        if let Ok(part_id) = read_register(&i2c, reg::PARTID) {
            if part_id != correct_id {
                //writeln!(stdout, "MSA device not found (ID should be {:02x} but is {:02x})", correct_id, part_id).unwrap();
                return Err(());
            }
        } else {
            //writeln!(stdout, "Could not read MSA device ID").unwrap();
            return Err(());
        }

        // set (and check) the power mode to 0x1A: normal power mode + 500Hz bandwidth
        let desired_mode = 0x1A;
        if set_register(&i2c, reg::PWR_MODE_BW, desired_mode).is_err() {
            //writeln!(stdout, "Problem: setting power mode went wrong").unwrap();
            return Err(());
        }

        let pwr_mode = read_register(&i2c, reg::PWR_MODE_BW).unwrap();
        if pwr_mode != desired_mode {   
            //writeln!(stdout, "Power mode should be {:02x} but is {:02x}", desired_mode, pwr_mode).unwrap();
            return Err(());
        }

        // Enable x, y and z + set output data rate to 500 Hz
        set_register(&i2c, reg::ODR, 0x09)?; 
        // resolution 14 bits (MSB=8 bits, LSB=6 bits + 2 zero bits), range +- 4G
        set_register(&i2c, reg::RES_RANGE, 0x01)?;
        // no interrupts
        set_register(&i2c, reg::INT_SET_0, 0x00)?;
        set_register(&i2c, reg::INT_SET_1, 0x00)?;
        // reset all latched interrupts, temporary latched 250ms
        set_register(&i2c, reg::INT_LATCH, 0x81)?;

        Ok(Self {
            i2c: i2c,
        })
    }

    /** Return measurement in m/s^2 for x, y, z */
    pub fn measure(&self) -> Result<(f32, f32, f32), ()> {
        // for x, y, and z: read the LSB (6 bits + 2 zero bits) and the MSB (8 bits)
        // shift the MSB left 8 bits, add the LSB, and multiply this with a calibration constant
        let tx = (read_register(&self.i2c, reg::ACC_X_LSB)? as i32) +
                 ((read_register(&self.i2c, reg::ACC_X_MSB)? as i8) as i32)*256;
        let x = 0.25f32 * MG2G_MULTIPLIER_4_G * GRAVITY * (tx as f32);
        let ty = (read_register(&self.i2c, reg::ACC_Y_LSB)? as i32) +
                 ((read_register(&self.i2c, reg::ACC_Y_MSB)? as i8) as i32)*256;
        let y = 0.25f32 * MG2G_MULTIPLIER_4_G * GRAVITY * (ty as f32);
        // looks like Z has a large bias -  don't know if this is general or just on my board
        let tz = (read_register(&self.i2c, reg::ACC_Z_LSB)? as i32) +
                 ((read_register(&self.i2c, reg::ACC_Z_MSB)? as i8) as i32)*256 + 3386;
        let z = 0.25f32 * MG2G_MULTIPLIER_4_G * GRAVITY * (tz as f32);

        Ok((x, y, z))
    }
}
