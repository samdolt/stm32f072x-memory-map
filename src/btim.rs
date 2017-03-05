# ! [ doc = "Basic-timers" ]
# [ doc = r" Register block" ]
# [ repr ( C ) ]
pub struct BTim {
    # [ doc = "0x00 - control register 1" ]
    pub cr1: Cr1,
    # [ doc = "0x04 - control register 2" ]
    pub cr2: Cr2,
    _reserved0: [u8; 4usize],
    # [ doc = "0x0c - DMA/Interrupt enable register" ]
    pub dier: Dier,
    # [ doc = "0x10 - status register" ]
    pub sr: Sr,
    # [ doc = "0x14 - event generation register" ]
    pub egr: Egr,
    _reserved1: [u8; 12usize],
    # [ doc = "0x24 - counter" ]
    pub cnt: Cnt,
    # [ doc = "0x28 - prescaler" ]
    pub psc: Psc,
    # [ doc = "0x2c - auto-reload register" ]
    pub arr: Arr,
}

# [ doc = "control register 1" ]
# [ repr ( C ) ]
pub struct Cr1 {
    register: ::volatile_register::RW<u32>,
}

# [ doc = "control register 1" ]
pub mod cr1 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::Cr1 {
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
    # [ doc = "Value of the field ARPE" ]
    pub struct ArpeR {
        bits: u8,
    }
    impl ArpeR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field OPM" ]
    pub struct OpmR {
        bits: u8,
    }
    impl OpmR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field URS" ]
    pub struct UrsR {
        bits: u8,
    }
    impl UrsR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field UDIS" ]
    pub struct UdisR {
        bits: u8,
    }
    impl UdisR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field CEN" ]
    pub struct CenR {
        bits: u8,
    }
    impl CenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _ArpeW<'a> {
        register: &'a mut W,
    }
    impl<'a> _ArpeW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 7;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _OpmW<'a> {
        register: &'a mut W,
    }
    impl<'a> _OpmW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 3;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _UrsW<'a> {
        register: &'a mut W,
    }
    impl<'a> _UrsW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 2;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _UdisW<'a> {
        register: &'a mut W,
    }
    impl<'a> _UdisW<'a> {
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
    pub struct _CenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _CenW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
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
        fn _arpe(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 7 - Auto-reload preload enable" ]
        pub fn arpe(&self) -> ArpeR {
            ArpeR { bits: self._arpe() }
        }
        fn _opm(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 3 - One-pulse mode" ]
        pub fn opm(&self) -> OpmR {
            OpmR { bits: self._opm() }
        }
        fn _urs(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 2 - Update request source" ]
        pub fn urs(&self) -> UrsR {
            UrsR { bits: self._urs() }
        }
        fn _udis(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 1 - Update disable" ]
        pub fn udis(&self) -> UdisR {
            UdisR { bits: self._udis() }
        }
        fn _cen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 0 - Counter enable" ]
        pub fn cen(&self) -> CenR {
            CenR { bits: self._cen() }
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
        # [ doc = "Bit 7 - Auto-reload preload enable" ]
        pub fn arpe(&mut self) -> _ArpeW {
            _ArpeW { register: self }
        }
        # [ doc = "Bit 3 - One-pulse mode" ]
        pub fn opm(&mut self) -> _OpmW {
            _OpmW { register: self }
        }
        # [ doc = "Bit 2 - Update request source" ]
        pub fn urs(&mut self) -> _UrsW {
            _UrsW { register: self }
        }
        # [ doc = "Bit 1 - Update disable" ]
        pub fn udis(&mut self) -> _UdisW {
            _UdisW { register: self }
        }
        # [ doc = "Bit 0 - Counter enable" ]
        pub fn cen(&mut self) -> _CenW {
            _CenW { register: self }
        }
    }
}

# [ doc = "control register 2" ]
# [ repr ( C ) ]
pub struct Cr2 {
    register: ::volatile_register::RW<u32>,
}

# [ doc = "control register 2" ]
pub mod cr2 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::Cr2 {
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
    # [ doc = "Value of the field MMS" ]
    pub struct MmsR {
        bits: u8,
    }
    impl MmsR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _MmsW<'a> {
        register: &'a mut W,
    }
    impl<'a> _MmsW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
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
        fn _mms(&self) -> u8 {
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 4:6 - Master mode selection" ]
        pub fn mms(&self) -> MmsR {
            MmsR { bits: self._mms() }
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
        # [ doc = "Bits 4:6 - Master mode selection" ]
        pub fn mms(&mut self) -> _MmsW {
            _MmsW { register: self }
        }
    }
}

# [ doc = "DMA/Interrupt enable register" ]
# [ repr ( C ) ]
pub struct Dier {
    register: ::volatile_register::RW<u32>,
}

# [ doc = "DMA/Interrupt enable register" ]
pub mod dier {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::Dier {
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
    # [ doc = "Value of the field UDE" ]
    pub struct UdeR {
        bits: u8,
    }
    impl UdeR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field UIE" ]
    pub struct UieR {
        bits: u8,
    }
    impl UieR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _UdeW<'a> {
        register: &'a mut W,
    }
    impl<'a> _UdeW<'a> {
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
    pub struct _UieW<'a> {
        register: &'a mut W,
    }
    impl<'a> _UieW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
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
        fn _ude(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 8 - Update DMA request enable" ]
        pub fn ude(&self) -> UdeR {
            UdeR { bits: self._ude() }
        }
        fn _uie(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 0 - Update interrupt enable" ]
        pub fn uie(&self) -> UieR {
            UieR { bits: self._uie() }
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
        # [ doc = "Bit 8 - Update DMA request enable" ]
        pub fn ude(&mut self) -> _UdeW {
            _UdeW { register: self }
        }
        # [ doc = "Bit 0 - Update interrupt enable" ]
        pub fn uie(&mut self) -> _UieW {
            _UieW { register: self }
        }
    }
}

# [ doc = "status register" ]
# [ repr ( C ) ]
pub struct Sr {
    register: ::volatile_register::RW<u32>,
}

# [ doc = "status register" ]
pub mod sr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::Sr {
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
    # [ doc = "Value of the field UIF" ]
    pub struct UifR {
        bits: u8,
    }
    impl UifR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _UifW<'a> {
        register: &'a mut W,
    }
    impl<'a> _UifW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
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
        fn _uif(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 0 - Update interrupt flag" ]
        pub fn uif(&self) -> UifR {
            UifR { bits: self._uif() }
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
        # [ doc = "Bit 0 - Update interrupt flag" ]
        pub fn uif(&mut self) -> _UifW {
            _UifW { register: self }
        }
    }
}

# [ doc = "event generation register" ]
# [ repr ( C ) ]
pub struct Egr {
    register: ::volatile_register::WO<u32>,
}

# [ doc = "event generation register" ]
pub mod egr {
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::Egr {
        # [ doc = r" Writes to the register" ]
        pub fn write<F>(&mut self, f: F)
            where F: FnOnce(&mut W) -> &mut W
        {
            let mut w = W::reset_value();
            f(&mut w);
            self.register.write(w.bits);
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _UgW<'a> {
        register: &'a mut W,
    }
    impl<'a> _UgW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
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
        # [ doc = "Bit 0 - Update generation" ]
        pub fn ug(&mut self) -> _UgW {
            _UgW { register: self }
        }
    }
}

# [ doc = "counter" ]
# [ repr ( C ) ]
pub struct Cnt {
    register: ::volatile_register::RW<u32>,
}

# [ doc = "counter" ]
pub mod cnt {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::Cnt {
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
    # [ doc = "Value of the field CNT" ]
    pub struct CntR {
        bits: u16,
    }
    impl CntR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _CntW<'a> {
        register: &'a mut W,
    }
    impl<'a> _CntW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u16) -> &'a mut W {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
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
        fn _cnt(&self) -> u16 {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        }
        # [ doc = "Bits 0:15 - Low counter value" ]
        pub fn cnt(&self) -> CntR {
            CntR { bits: self._cnt() }
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
        # [ doc = "Bits 0:15 - Low counter value" ]
        pub fn cnt(&mut self) -> _CntW {
            _CntW { register: self }
        }
    }
}

# [ doc = "prescaler" ]
# [ repr ( C ) ]
pub struct Psc {
    register: ::volatile_register::RW<u32>,
}

# [ doc = "prescaler" ]
pub mod psc {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::Psc {
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
    # [ doc = "Value of the field PSC" ]
    pub struct PscR {
        bits: u16,
    }
    impl PscR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PscW<'a> {
        register: &'a mut W,
    }
    impl<'a> _PscW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u16) -> &'a mut W {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
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
        fn _psc(&self) -> u16 {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        }
        # [ doc = "Bits 0:15 - Prescaler value" ]
        pub fn psc(&self) -> PscR {
            PscR { bits: self._psc() }
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
        # [ doc = "Bits 0:15 - Prescaler value" ]
        pub fn psc(&mut self) -> _PscW {
            _PscW { register: self }
        }
    }
}

# [ doc = "auto-reload register" ]
# [ repr ( C ) ]
pub struct Arr {
    register: ::volatile_register::RW<u32>,
}

# [ doc = "auto-reload register" ]
pub mod arr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::Arr {
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
    # [ doc = "Value of the field ARR" ]
    pub struct ArrR {
        bits: u16,
    }
    impl ArrR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _ArrW<'a> {
        register: &'a mut W,
    }
    impl<'a> _ArrW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u16) -> &'a mut W {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
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
        fn _arr(&self) -> u16 {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        }
        # [ doc = "Bits 0:15 - Low Auto-reload value" ]
        pub fn arr(&self) -> ArrR {
            ArrR { bits: self._arr() }
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
        # [ doc = "Bits 0:15 - Low Auto-reload value" ]
        pub fn arr(&mut self) -> _ArrW {
            _ArrW { register: self }
        }
    }
}
