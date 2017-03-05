# ! [ doc = "Debug support" ]
# [ doc = r" Register block" ]
# [ repr ( C ) ]
pub struct Dbgmcu {
    # [ doc = "0x00 - MCU Device ID Code Register" ]
    pub idcode: Idcode,
    # [ doc = "0x04 - Debug MCU Configuration Register" ]
    pub cr: Cr,
    # [ doc = "0x08 - APB Low Freeze Register" ]
    pub apblfz: Apblfz,
    # [ doc = "0x0c - APB High Freeze Register" ]
    pub apbhfz: Apbhfz,
}

# [ doc = "MCU Device ID Code Register" ]
# [ repr ( C ) ]
pub struct Idcode {
    register: ::volatile_register::RO<u32>,
}

# [ doc = "MCU Device ID Code Register" ]
pub mod idcode {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    impl super::Idcode {
        # [ doc = r" Reads the contents of the register" ]
        pub fn read(&self) -> R {
            R { bits: self.register.read() }
        }
    }
    # [ doc = "Value of the field DEV_ID" ]
    pub struct DevIdR {
        bits: u16,
    }
    impl DevIdR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = "Value of the field DIV_ID" ]
    pub struct DivIdR {
        bits: u8,
    }
    impl DivIdR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field REV_ID" ]
    pub struct RevIdR {
        bits: u16,
    }
    impl RevIdR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        fn _dev_id(&self) -> u16 {
            const MASK: u16 = 4095;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        }
        # [ doc = "Bits 0:11 - Device Identifier" ]
        pub fn dev_id(&self) -> DevIdR {
            DevIdR { bits: self._dev_id() }
        }
        fn _div_id(&self) -> u8 {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 12:15 - Division Identifier" ]
        pub fn div_id(&self) -> DivIdR {
            DivIdR { bits: self._div_id() }
        }
        fn _rev_id(&self) -> u16 {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        }
        # [ doc = "Bits 16:31 - Revision Identifier" ]
        pub fn rev_id(&self) -> RevIdR {
            RevIdR { bits: self._rev_id() }
        }
    }
}

# [ doc = "Debug MCU Configuration Register" ]
# [ repr ( C ) ]
pub struct Cr {
    register: ::volatile_register::RW<u32>,
}

# [ doc = "Debug MCU Configuration Register" ]
pub mod cr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::Cr {
        # [ doc = r" Modifies the contents of the register" ]
        pub fn modify<F>(&mut self, f: F)
            where for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W
        {
            let bits = self.register.read();
            let r = R { bits: bits };
            let mut w = W { bits: bits };
            f(&r, &mut w);
            self.register.write(w.bits);
        }
        # [ doc = r" Reads the contents of the register" ]
        pub fn read(&self) -> R {
            R { bits: self.register.read() }
        }
        # [ doc = r" Writes to the register" ]
        pub fn write<F>(&mut self, f: F)
            where F: FnOnce(&mut W) -> &mut W
        {
            let mut w = W::reset_value();
            f(&mut w);
            self.register.write(w.bits);
        }
    }
    # [ doc = "Value of the field DBG_STOP" ]
    pub struct DbgStopR {
        bits: u8,
    }
    impl DbgStopR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field DBG_STANDBY" ]
    pub struct DbgStandbyR {
        bits: u8,
    }
    impl DbgStandbyR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _DbgStopW<'a> {
        register: &'a mut W,
    }
    impl<'a> _DbgStopW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 1;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _DbgStandbyW<'a> {
        register: &'a mut W,
    }
    impl<'a> _DbgStandbyW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 2;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        fn _dbg_stop(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 1 - Debug Stop Mode" ]
        pub fn dbg_stop(&self) -> DbgStopR {
            DbgStopR { bits: self._dbg_stop() }
        }
        fn _dbg_standby(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 2 - Debug Standby Mode" ]
        pub fn dbg_standby(&self) -> DbgStandbyR {
            DbgStandbyR { bits: self._dbg_standby() }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        # [ doc = r" Writes raw `bits` to the register" ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bit 1 - Debug Stop Mode" ]
        pub fn dbg_stop(&mut self) -> _DbgStopW {
            _DbgStopW { register: self }
        }
        # [ doc = "Bit 2 - Debug Standby Mode" ]
        pub fn dbg_standby(&mut self) -> _DbgStandbyW {
            _DbgStandbyW { register: self }
        }
    }
}

# [ doc = "APB Low Freeze Register" ]
# [ repr ( C ) ]
pub struct Apblfz {
    register: ::volatile_register::RW<u32>,
}

# [ doc = "APB Low Freeze Register" ]
pub mod apblfz {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::Apblfz {
        # [ doc = r" Modifies the contents of the register" ]
        pub fn modify<F>(&mut self, f: F)
            where for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W
        {
            let bits = self.register.read();
            let r = R { bits: bits };
            let mut w = W { bits: bits };
            f(&r, &mut w);
            self.register.write(w.bits);
        }
        # [ doc = r" Reads the contents of the register" ]
        pub fn read(&self) -> R {
            R { bits: self.register.read() }
        }
        # [ doc = r" Writes to the register" ]
        pub fn write<F>(&mut self, f: F)
            where F: FnOnce(&mut W) -> &mut W
        {
            let mut w = W::reset_value();
            f(&mut w);
            self.register.write(w.bits);
        }
    }
    # [ doc = "Value of the field DBG_TIMER2_STOP" ]
    pub struct DbgTimer2StopR {
        bits: u8,
    }
    impl DbgTimer2StopR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field DBG_TIMER3_STOP" ]
    pub struct DbgTimer3StopR {
        bits: u8,
    }
    impl DbgTimer3StopR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field DBG_TIMER6_STOP" ]
    pub struct DbgTimer6StopR {
        bits: u8,
    }
    impl DbgTimer6StopR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field DBG_TIMER14_STOP" ]
    pub struct DbgTimer14StopR {
        bits: u8,
    }
    impl DbgTimer14StopR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field DBG_RTC_STOP" ]
    pub struct DbgRtcStopR {
        bits: u8,
    }
    impl DbgRtcStopR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field DBG_WWDG_STOP" ]
    pub struct DbgWwdgStopR {
        bits: u8,
    }
    impl DbgWwdgStopR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field DBG_IWDG_STOP" ]
    pub struct DbgIwdgStopR {
        bits: u8,
    }
    impl DbgIwdgStopR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field I2C1_SMBUS_TIMEOUT" ]
    pub struct I2c1SmbusTimeoutR {
        bits: u8,
    }
    impl I2c1SmbusTimeoutR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _DbgTimer2StopW<'a> {
        register: &'a mut W,
    }
    impl<'a> _DbgTimer2StopW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _DbgTimer3StopW<'a> {
        register: &'a mut W,
    }
    impl<'a> _DbgTimer3StopW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 1;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _DbgTimer6StopW<'a> {
        register: &'a mut W,
    }
    impl<'a> _DbgTimer6StopW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 4;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _DbgTimer14StopW<'a> {
        register: &'a mut W,
    }
    impl<'a> _DbgTimer14StopW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 8;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _DbgRtcStopW<'a> {
        register: &'a mut W,
    }
    impl<'a> _DbgRtcStopW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 10;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _DbgWwdgStopW<'a> {
        register: &'a mut W,
    }
    impl<'a> _DbgWwdgStopW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 11;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _DbgIwdgStopW<'a> {
        register: &'a mut W,
    }
    impl<'a> _DbgIwdgStopW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 12;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _I2c1SmbusTimeoutW<'a> {
        register: &'a mut W,
    }
    impl<'a> _I2c1SmbusTimeoutW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 21;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        fn _dbg_timer2_stop(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 0 - Debug Timer 2 stopped when Core is halted" ]
        pub fn dbg_timer2_stop(&self) -> DbgTimer2StopR {
            DbgTimer2StopR { bits: self._dbg_timer2_stop() }
        }
        fn _dbg_timer3_stop(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 1 - Debug Timer 3 stopped when Core is halted" ]
        pub fn dbg_timer3_stop(&self) -> DbgTimer3StopR {
            DbgTimer3StopR { bits: self._dbg_timer3_stop() }
        }
        fn _dbg_timer6_stop(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 4 - Debug Timer 6 stopped when Core is halted" ]
        pub fn dbg_timer6_stop(&self) -> DbgTimer6StopR {
            DbgTimer6StopR { bits: self._dbg_timer6_stop() }
        }
        fn _dbg_timer14_stop(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 8 - Debug Timer 14 stopped when Core is halted" ]
        pub fn dbg_timer14_stop(&self) -> DbgTimer14StopR {
            DbgTimer14StopR { bits: self._dbg_timer14_stop() }
        }
        fn _dbg_rtc_stop(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 10 - Debug RTC stopped when Core is halted" ]
        pub fn dbg_rtc_stop(&self) -> DbgRtcStopR {
            DbgRtcStopR { bits: self._dbg_rtc_stop() }
        }
        fn _dbg_wwdg_stop(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 11 - Debug Window Wachdog stopped when Core is halted" ]
        pub fn dbg_wwdg_stop(&self) -> DbgWwdgStopR {
            DbgWwdgStopR { bits: self._dbg_wwdg_stop() }
        }
        fn _dbg_iwdg_stop(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 12 - Debug Independent Wachdog stopped when Core is halted" ]
        pub fn dbg_iwdg_stop(&self) -> DbgIwdgStopR {
            DbgIwdgStopR { bits: self._dbg_iwdg_stop() }
        }
        fn _i2c1_smbus_timeout(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 21 - SMBUS timeout mode stopped when Core is halted" ]
        pub fn i2c1_smbus_timeout(&self) -> I2c1SmbusTimeoutR {
            I2c1SmbusTimeoutR { bits: self._i2c1_smbus_timeout() }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        # [ doc = r" Writes raw `bits` to the register" ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bit 0 - Debug Timer 2 stopped when Core is halted" ]
        pub fn dbg_timer2_stop(&mut self) -> _DbgTimer2StopW {
            _DbgTimer2StopW { register: self }
        }
        # [ doc = "Bit 1 - Debug Timer 3 stopped when Core is halted" ]
        pub fn dbg_timer3_stop(&mut self) -> _DbgTimer3StopW {
            _DbgTimer3StopW { register: self }
        }
        # [ doc = "Bit 4 - Debug Timer 6 stopped when Core is halted" ]
        pub fn dbg_timer6_stop(&mut self) -> _DbgTimer6StopW {
            _DbgTimer6StopW { register: self }
        }
        # [ doc = "Bit 8 - Debug Timer 14 stopped when Core is halted" ]
        pub fn dbg_timer14_stop(&mut self) -> _DbgTimer14StopW {
            _DbgTimer14StopW { register: self }
        }
        # [ doc = "Bit 10 - Debug RTC stopped when Core is halted" ]
        pub fn dbg_rtc_stop(&mut self) -> _DbgRtcStopW {
            _DbgRtcStopW { register: self }
        }
        # [ doc = "Bit 11 - Debug Window Wachdog stopped when Core is halted" ]
        pub fn dbg_wwdg_stop(&mut self) -> _DbgWwdgStopW {
            _DbgWwdgStopW { register: self }
        }
        # [ doc = "Bit 12 - Debug Independent Wachdog stopped when Core is halted" ]
        pub fn dbg_iwdg_stop(&mut self) -> _DbgIwdgStopW {
            _DbgIwdgStopW { register: self }
        }
        # [ doc = "Bit 21 - SMBUS timeout mode stopped when Core is halted" ]
        pub fn i2c1_smbus_timeout(&mut self) -> _I2c1SmbusTimeoutW {
            _I2c1SmbusTimeoutW { register: self }
        }
    }
}

# [ doc = "APB High Freeze Register" ]
# [ repr ( C ) ]
pub struct Apbhfz {
    register: ::volatile_register::RW<u32>,
}

# [ doc = "APB High Freeze Register" ]
pub mod apbhfz {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::Apbhfz {
        # [ doc = r" Modifies the contents of the register" ]
        pub fn modify<F>(&mut self, f: F)
            where for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W
        {
            let bits = self.register.read();
            let r = R { bits: bits };
            let mut w = W { bits: bits };
            f(&r, &mut w);
            self.register.write(w.bits);
        }
        # [ doc = r" Reads the contents of the register" ]
        pub fn read(&self) -> R {
            R { bits: self.register.read() }
        }
        # [ doc = r" Writes to the register" ]
        pub fn write<F>(&mut self, f: F)
            where F: FnOnce(&mut W) -> &mut W
        {
            let mut w = W::reset_value();
            f(&mut w);
            self.register.write(w.bits);
        }
    }
    # [ doc = "Value of the field DBG_TIMER1_STOP" ]
    pub struct DbgTimer1StopR {
        bits: u8,
    }
    impl DbgTimer1StopR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field DBG_TIMER15_STO" ]
    pub struct DbgTimer15StoR {
        bits: u8,
    }
    impl DbgTimer15StoR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field DBG_TIMER16_STO" ]
    pub struct DbgTimer16StoR {
        bits: u8,
    }
    impl DbgTimer16StoR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field DBG_TIMER17_STO" ]
    pub struct DbgTimer17StoR {
        bits: u8,
    }
    impl DbgTimer17StoR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _DbgTimer1StopW<'a> {
        register: &'a mut W,
    }
    impl<'a> _DbgTimer1StopW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 11;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _DbgTimer15StoW<'a> {
        register: &'a mut W,
    }
    impl<'a> _DbgTimer15StoW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 16;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _DbgTimer16StoW<'a> {
        register: &'a mut W,
    }
    impl<'a> _DbgTimer16StoW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 17;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _DbgTimer17StoW<'a> {
        register: &'a mut W,
    }
    impl<'a> _DbgTimer17StoW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 18;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        fn _dbg_timer1_stop(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 11 - Debug Timer 1 stopped when Core is halted" ]
        pub fn dbg_timer1_stop(&self) -> DbgTimer1StopR {
            DbgTimer1StopR { bits: self._dbg_timer1_stop() }
        }
        fn _dbg_timer15_sto(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 16 - Debug Timer 15 stopped when Core is halted" ]
        pub fn dbg_timer15_sto(&self) -> DbgTimer15StoR {
            DbgTimer15StoR { bits: self._dbg_timer15_sto() }
        }
        fn _dbg_timer16_sto(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 17 - Debug Timer 16 stopped when Core is halted" ]
        pub fn dbg_timer16_sto(&self) -> DbgTimer16StoR {
            DbgTimer16StoR { bits: self._dbg_timer16_sto() }
        }
        fn _dbg_timer17_sto(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 18 - Debug Timer 17 stopped when Core is halted" ]
        pub fn dbg_timer17_sto(&self) -> DbgTimer17StoR {
            DbgTimer17StoR { bits: self._dbg_timer17_sto() }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        # [ doc = r" Writes raw `bits` to the register" ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bit 11 - Debug Timer 1 stopped when Core is halted" ]
        pub fn dbg_timer1_stop(&mut self) -> _DbgTimer1StopW {
            _DbgTimer1StopW { register: self }
        }
        # [ doc = "Bit 16 - Debug Timer 15 stopped when Core is halted" ]
        pub fn dbg_timer15_sto(&mut self) -> _DbgTimer15StoW {
            _DbgTimer15StoW { register: self }
        }
        # [ doc = "Bit 17 - Debug Timer 16 stopped when Core is halted" ]
        pub fn dbg_timer16_sto(&mut self) -> _DbgTimer16StoW {
            _DbgTimer16StoW { register: self }
        }
        # [ doc = "Bit 18 - Debug Timer 17 stopped when Core is halted" ]
        pub fn dbg_timer17_sto(&mut self) -> _DbgTimer17StoW {
            _DbgTimer17StoW { register: self }
        }
    }
}
