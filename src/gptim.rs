# ! [ doc = "General-purpose-timers" ]
# [ doc = r" Register block" ]
# [ repr ( C ) ]
pub struct GpTim {
    # [ doc = "0x00 - control register 1" ]
    pub cr1: Cr1,
    # [ doc = "0x04 - control register 2" ]
    pub cr2: Cr2,
    # [ doc = "0x08 - slave mode control register" ]
    pub smcr: Smcr,
    # [ doc = "0x0c - DMA/Interrupt enable register" ]
    pub dier: Dier,
    # [ doc = "0x10 - status register" ]
    pub sr: Sr,
    # [ doc = "0x14 - event generation register" ]
    pub egr: Egr,
    # [ doc = "0x18 - capture/compare mode register 1 (output mode)" ]
    pub ccmr1_output: Ccmr1Output,
    # [ doc = "0x1c - capture/compare mode register 2 (output mode)" ]
    pub ccmr2_output: Ccmr2Output,
    # [ doc = "0x20 - capture/compare enable register" ]
    pub ccer: Ccer,
    # [ doc = "0x24 - counter" ]
    pub cnt: Cnt,
    # [ doc = "0x28 - prescaler" ]
    pub psc: Psc,
    # [ doc = "0x2c - auto-reload register" ]
    pub arr: Arr,
    _reserved0: [u8; 4usize],
    # [ doc = "0x34 - capture/compare register 1" ]
    pub ccr1: Ccr1,
    # [ doc = "0x38 - capture/compare register 2" ]
    pub ccr2: Ccr2,
    # [ doc = "0x3c - capture/compare register 3" ]
    pub ccr3: Ccr3,
    # [ doc = "0x40 - capture/compare register 4" ]
    pub ccr4: Ccr4,
    _reserved1: [u8; 4usize],
    # [ doc = "0x48 - DMA control register" ]
    pub dcr: Dcr,
    # [ doc = "0x4c - DMA address for full transfer" ]
    pub dmar: Dmar,
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
    # [ doc = "Value of the field CKD" ]
    pub struct CkdR {
        bits: u8,
    }
    impl CkdR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
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
    # [ doc = "Value of the field CMS" ]
    pub struct CmsR {
        bits: u8,
    }
    impl CmsR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field DIR" ]
    pub struct DirR {
        bits: u8,
    }
    impl DirR {
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
    pub struct _CkdW<'a> {
        register: &'a mut W,
    }
    impl<'a> _CkdW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
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
    pub struct _CmsW<'a> {
        register: &'a mut W,
    }
    impl<'a> _CmsW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 5;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _DirW<'a> {
        register: &'a mut W,
    }
    impl<'a> _DirW<'a> {
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
        fn _ckd(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 8:9 - Clock division" ]
        pub fn ckd(&self) -> CkdR {
            CkdR { bits: self._ckd() }
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
        fn _cms(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 5:6 - Center-aligned mode selection" ]
        pub fn cms(&self) -> CmsR {
            CmsR { bits: self._cms() }
        }
        fn _dir(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 4 - Direction" ]
        pub fn dir(&self) -> DirR {
            DirR { bits: self._dir() }
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
        # [ doc = "Bits 8:9 - Clock division" ]
        pub fn ckd(&mut self) -> _CkdW {
            _CkdW { register: self }
        }
        # [ doc = "Bit 7 - Auto-reload preload enable" ]
        pub fn arpe(&mut self) -> _ArpeW {
            _ArpeW { register: self }
        }
        # [ doc = "Bits 5:6 - Center-aligned mode selection" ]
        pub fn cms(&mut self) -> _CmsW {
            _CmsW { register: self }
        }
        # [ doc = "Bit 4 - Direction" ]
        pub fn dir(&mut self) -> _DirW {
            _DirW { register: self }
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
    # [ doc = "Value of the field TI1S" ]
    pub struct Ti1sR {
        bits: u8,
    }
    impl Ti1sR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
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
    # [ doc = "Value of the field CCDS" ]
    pub struct CcdsR {
        bits: u8,
    }
    impl CcdsR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Ti1sW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Ti1sW<'a> {
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
    # [ doc = r" Proxy" ]
    pub struct _CcdsW<'a> {
        register: &'a mut W,
    }
    impl<'a> _CcdsW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 3;
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
        fn _ti1s(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 7 - TI1 selection" ]
        pub fn ti1s(&self) -> Ti1sR {
            Ti1sR { bits: self._ti1s() }
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
        fn _ccds(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 3 - Capture/compare DMA selection" ]
        pub fn ccds(&self) -> CcdsR {
            CcdsR { bits: self._ccds() }
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
        # [ doc = "Bit 7 - TI1 selection" ]
        pub fn ti1s(&mut self) -> _Ti1sW {
            _Ti1sW { register: self }
        }
        # [ doc = "Bits 4:6 - Master mode selection" ]
        pub fn mms(&mut self) -> _MmsW {
            _MmsW { register: self }
        }
        # [ doc = "Bit 3 - Capture/compare DMA selection" ]
        pub fn ccds(&mut self) -> _CcdsW {
            _CcdsW { register: self }
        }
    }
}

# [ doc = "slave mode control register" ]
# [ repr ( C ) ]
pub struct Smcr {
    register: ::volatile_register::RW<u32>,
}

# [ doc = "slave mode control register" ]
pub mod smcr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::Smcr {
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
    # [ doc = "Value of the field ETP" ]
    pub struct EtpR {
        bits: u8,
    }
    impl EtpR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field ECE" ]
    pub struct EceR {
        bits: u8,
    }
    impl EceR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field ETPS" ]
    pub struct EtpsR {
        bits: u8,
    }
    impl EtpsR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field ETF" ]
    pub struct EtfR {
        bits: u8,
    }
    impl EtfR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field MSM" ]
    pub struct MsmR {
        bits: u8,
    }
    impl MsmR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field TS" ]
    pub struct TsR {
        bits: u8,
    }
    impl TsR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field SMS" ]
    pub struct SmsR {
        bits: u8,
    }
    impl SmsR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _EtpW<'a> {
        register: &'a mut W,
    }
    impl<'a> _EtpW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 15;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _EceW<'a> {
        register: &'a mut W,
    }
    impl<'a> _EceW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 14;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _EtpsW<'a> {
        register: &'a mut W,
    }
    impl<'a> _EtpsW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _EtfW<'a> {
        register: &'a mut W,
    }
    impl<'a> _EtfW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _MsmW<'a> {
        register: &'a mut W,
    }
    impl<'a> _MsmW<'a> {
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
    pub struct _TsW<'a> {
        register: &'a mut W,
    }
    impl<'a> _TsW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _SmsW<'a> {
        register: &'a mut W,
    }
    impl<'a> _SmsW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 7;
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
        fn _etp(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 15 - External trigger polarity" ]
        pub fn etp(&self) -> EtpR {
            EtpR { bits: self._etp() }
        }
        fn _ece(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 14 - External clock enable" ]
        pub fn ece(&self) -> EceR {
            EceR { bits: self._ece() }
        }
        fn _etps(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 12:13 - External trigger prescaler" ]
        pub fn etps(&self) -> EtpsR {
            EtpsR { bits: self._etps() }
        }
        fn _etf(&self) -> u8 {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 8:11 - External trigger filter" ]
        pub fn etf(&self) -> EtfR {
            EtfR { bits: self._etf() }
        }
        fn _msm(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 7 - Master/Slave mode" ]
        pub fn msm(&self) -> MsmR {
            MsmR { bits: self._msm() }
        }
        fn _ts(&self) -> u8 {
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 4:6 - Trigger selection" ]
        pub fn ts(&self) -> TsR {
            TsR { bits: self._ts() }
        }
        fn _sms(&self) -> u8 {
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 0:2 - Slave mode selection" ]
        pub fn sms(&self) -> SmsR {
            SmsR { bits: self._sms() }
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
        # [ doc = "Bit 15 - External trigger polarity" ]
        pub fn etp(&mut self) -> _EtpW {
            _EtpW { register: self }
        }
        # [ doc = "Bit 14 - External clock enable" ]
        pub fn ece(&mut self) -> _EceW {
            _EceW { register: self }
        }
        # [ doc = "Bits 12:13 - External trigger prescaler" ]
        pub fn etps(&mut self) -> _EtpsW {
            _EtpsW { register: self }
        }
        # [ doc = "Bits 8:11 - External trigger filter" ]
        pub fn etf(&mut self) -> _EtfW {
            _EtfW { register: self }
        }
        # [ doc = "Bit 7 - Master/Slave mode" ]
        pub fn msm(&mut self) -> _MsmW {
            _MsmW { register: self }
        }
        # [ doc = "Bits 4:6 - Trigger selection" ]
        pub fn ts(&mut self) -> _TsW {
            _TsW { register: self }
        }
        # [ doc = "Bits 0:2 - Slave mode selection" ]
        pub fn sms(&mut self) -> _SmsW {
            _SmsW { register: self }
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
    # [ doc = "Value of the field TDE" ]
    pub struct TdeR {
        bits: u8,
    }
    impl TdeR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field COMDE" ]
    pub struct ComdeR {
        bits: u8,
    }
    impl ComdeR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field CC4DE" ]
    pub struct Cc4deR {
        bits: u8,
    }
    impl Cc4deR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field CC3DE" ]
    pub struct Cc3deR {
        bits: u8,
    }
    impl Cc3deR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field CC2DE" ]
    pub struct Cc2deR {
        bits: u8,
    }
    impl Cc2deR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field CC1DE" ]
    pub struct Cc1deR {
        bits: u8,
    }
    impl Cc1deR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
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
    # [ doc = "Value of the field TIE" ]
    pub struct TieR {
        bits: u8,
    }
    impl TieR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field CC4IE" ]
    pub struct Cc4ieR {
        bits: u8,
    }
    impl Cc4ieR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field CC3IE" ]
    pub struct Cc3ieR {
        bits: u8,
    }
    impl Cc3ieR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field CC2IE" ]
    pub struct Cc2ieR {
        bits: u8,
    }
    impl Cc2ieR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field CC1IE" ]
    pub struct Cc1ieR {
        bits: u8,
    }
    impl Cc1ieR {
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
    pub struct _TdeW<'a> {
        register: &'a mut W,
    }
    impl<'a> _TdeW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 14;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _ComdeW<'a> {
        register: &'a mut W,
    }
    impl<'a> _ComdeW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 13;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Cc4deW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Cc4deW<'a> {
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
    pub struct _Cc3deW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Cc3deW<'a> {
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
    pub struct _Cc2deW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Cc2deW<'a> {
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
    pub struct _Cc1deW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Cc1deW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 9;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
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
    pub struct _TieW<'a> {
        register: &'a mut W,
    }
    impl<'a> _TieW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 6;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Cc4ieW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Cc4ieW<'a> {
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
    pub struct _Cc3ieW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Cc3ieW<'a> {
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
    pub struct _Cc2ieW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Cc2ieW<'a> {
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
    pub struct _Cc1ieW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Cc1ieW<'a> {
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
        fn _tde(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 14 - Trigger DMA request enable" ]
        pub fn tde(&self) -> TdeR {
            TdeR { bits: self._tde() }
        }
        fn _comde(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 13 - Reserved" ]
        pub fn comde(&self) -> ComdeR {
            ComdeR { bits: self._comde() }
        }
        fn _cc4de(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 12 - Capture/Compare 4 DMA request enable" ]
        pub fn cc4de(&self) -> Cc4deR {
            Cc4deR { bits: self._cc4de() }
        }
        fn _cc3de(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 11 - Capture/Compare 3 DMA request enable" ]
        pub fn cc3de(&self) -> Cc3deR {
            Cc3deR { bits: self._cc3de() }
        }
        fn _cc2de(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 10 - Capture/Compare 2 DMA request enable" ]
        pub fn cc2de(&self) -> Cc2deR {
            Cc2deR { bits: self._cc2de() }
        }
        fn _cc1de(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 9 - Capture/Compare 1 DMA request enable" ]
        pub fn cc1de(&self) -> Cc1deR {
            Cc1deR { bits: self._cc1de() }
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
        fn _tie(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 6 - Trigger interrupt enable" ]
        pub fn tie(&self) -> TieR {
            TieR { bits: self._tie() }
        }
        fn _cc4ie(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 4 - Capture/Compare 4 interrupt enable" ]
        pub fn cc4ie(&self) -> Cc4ieR {
            Cc4ieR { bits: self._cc4ie() }
        }
        fn _cc3ie(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 3 - Capture/Compare 3 interrupt enable" ]
        pub fn cc3ie(&self) -> Cc3ieR {
            Cc3ieR { bits: self._cc3ie() }
        }
        fn _cc2ie(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 2 - Capture/Compare 2 interrupt enable" ]
        pub fn cc2ie(&self) -> Cc2ieR {
            Cc2ieR { bits: self._cc2ie() }
        }
        fn _cc1ie(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 1 - Capture/Compare 1 interrupt enable" ]
        pub fn cc1ie(&self) -> Cc1ieR {
            Cc1ieR { bits: self._cc1ie() }
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
        # [ doc = "Bit 14 - Trigger DMA request enable" ]
        pub fn tde(&mut self) -> _TdeW {
            _TdeW { register: self }
        }
        # [ doc = "Bit 13 - Reserved" ]
        pub fn comde(&mut self) -> _ComdeW {
            _ComdeW { register: self }
        }
        # [ doc = "Bit 12 - Capture/Compare 4 DMA request enable" ]
        pub fn cc4de(&mut self) -> _Cc4deW {
            _Cc4deW { register: self }
        }
        # [ doc = "Bit 11 - Capture/Compare 3 DMA request enable" ]
        pub fn cc3de(&mut self) -> _Cc3deW {
            _Cc3deW { register: self }
        }
        # [ doc = "Bit 10 - Capture/Compare 2 DMA request enable" ]
        pub fn cc2de(&mut self) -> _Cc2deW {
            _Cc2deW { register: self }
        }
        # [ doc = "Bit 9 - Capture/Compare 1 DMA request enable" ]
        pub fn cc1de(&mut self) -> _Cc1deW {
            _Cc1deW { register: self }
        }
        # [ doc = "Bit 8 - Update DMA request enable" ]
        pub fn ude(&mut self) -> _UdeW {
            _UdeW { register: self }
        }
        # [ doc = "Bit 6 - Trigger interrupt enable" ]
        pub fn tie(&mut self) -> _TieW {
            _TieW { register: self }
        }
        # [ doc = "Bit 4 - Capture/Compare 4 interrupt enable" ]
        pub fn cc4ie(&mut self) -> _Cc4ieW {
            _Cc4ieW { register: self }
        }
        # [ doc = "Bit 3 - Capture/Compare 3 interrupt enable" ]
        pub fn cc3ie(&mut self) -> _Cc3ieW {
            _Cc3ieW { register: self }
        }
        # [ doc = "Bit 2 - Capture/Compare 2 interrupt enable" ]
        pub fn cc2ie(&mut self) -> _Cc2ieW {
            _Cc2ieW { register: self }
        }
        # [ doc = "Bit 1 - Capture/Compare 1 interrupt enable" ]
        pub fn cc1ie(&mut self) -> _Cc1ieW {
            _Cc1ieW { register: self }
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
    # [ doc = "Value of the field CC4OF" ]
    pub struct Cc4ofR {
        bits: u8,
    }
    impl Cc4ofR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field CC3OF" ]
    pub struct Cc3ofR {
        bits: u8,
    }
    impl Cc3ofR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field CC2OF" ]
    pub struct Cc2ofR {
        bits: u8,
    }
    impl Cc2ofR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field CC1OF" ]
    pub struct Cc1ofR {
        bits: u8,
    }
    impl Cc1ofR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field TIF" ]
    pub struct TifR {
        bits: u8,
    }
    impl TifR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field CC4IF" ]
    pub struct Cc4ifR {
        bits: u8,
    }
    impl Cc4ifR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field CC3IF" ]
    pub struct Cc3ifR {
        bits: u8,
    }
    impl Cc3ifR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field CC2IF" ]
    pub struct Cc2ifR {
        bits: u8,
    }
    impl Cc2ifR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field CC1IF" ]
    pub struct Cc1ifR {
        bits: u8,
    }
    impl Cc1ifR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
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
    pub struct _Cc4ofW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Cc4ofW<'a> {
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
    pub struct _Cc3ofW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Cc3ofW<'a> {
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
    pub struct _Cc2ofW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Cc2ofW<'a> {
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
    pub struct _Cc1ofW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Cc1ofW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 9;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _TifW<'a> {
        register: &'a mut W,
    }
    impl<'a> _TifW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 6;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Cc4ifW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Cc4ifW<'a> {
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
    pub struct _Cc3ifW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Cc3ifW<'a> {
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
    pub struct _Cc2ifW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Cc2ifW<'a> {
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
    pub struct _Cc1ifW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Cc1ifW<'a> {
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
        fn _cc4of(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 12 - Capture/Compare 4 overcapture flag" ]
        pub fn cc4of(&self) -> Cc4ofR {
            Cc4ofR { bits: self._cc4of() }
        }
        fn _cc3of(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 11 - Capture/Compare 3 overcapture flag" ]
        pub fn cc3of(&self) -> Cc3ofR {
            Cc3ofR { bits: self._cc3of() }
        }
        fn _cc2of(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 10 - Capture/compare 2 overcapture flag" ]
        pub fn cc2of(&self) -> Cc2ofR {
            Cc2ofR { bits: self._cc2of() }
        }
        fn _cc1of(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 9 - Capture/Compare 1 overcapture flag" ]
        pub fn cc1of(&self) -> Cc1ofR {
            Cc1ofR { bits: self._cc1of() }
        }
        fn _tif(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 6 - Trigger interrupt flag" ]
        pub fn tif(&self) -> TifR {
            TifR { bits: self._tif() }
        }
        fn _cc4if(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 4 - Capture/Compare 4 interrupt flag" ]
        pub fn cc4if(&self) -> Cc4ifR {
            Cc4ifR { bits: self._cc4if() }
        }
        fn _cc3if(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 3 - Capture/Compare 3 interrupt flag" ]
        pub fn cc3if(&self) -> Cc3ifR {
            Cc3ifR { bits: self._cc3if() }
        }
        fn _cc2if(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 2 - Capture/Compare 2 interrupt flag" ]
        pub fn cc2if(&self) -> Cc2ifR {
            Cc2ifR { bits: self._cc2if() }
        }
        fn _cc1if(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 1 - Capture/compare 1 interrupt flag" ]
        pub fn cc1if(&self) -> Cc1ifR {
            Cc1ifR { bits: self._cc1if() }
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
        # [ doc = "Bit 12 - Capture/Compare 4 overcapture flag" ]
        pub fn cc4of(&mut self) -> _Cc4ofW {
            _Cc4ofW { register: self }
        }
        # [ doc = "Bit 11 - Capture/Compare 3 overcapture flag" ]
        pub fn cc3of(&mut self) -> _Cc3ofW {
            _Cc3ofW { register: self }
        }
        # [ doc = "Bit 10 - Capture/compare 2 overcapture flag" ]
        pub fn cc2of(&mut self) -> _Cc2ofW {
            _Cc2ofW { register: self }
        }
        # [ doc = "Bit 9 - Capture/Compare 1 overcapture flag" ]
        pub fn cc1of(&mut self) -> _Cc1ofW {
            _Cc1ofW { register: self }
        }
        # [ doc = "Bit 6 - Trigger interrupt flag" ]
        pub fn tif(&mut self) -> _TifW {
            _TifW { register: self }
        }
        # [ doc = "Bit 4 - Capture/Compare 4 interrupt flag" ]
        pub fn cc4if(&mut self) -> _Cc4ifW {
            _Cc4ifW { register: self }
        }
        # [ doc = "Bit 3 - Capture/Compare 3 interrupt flag" ]
        pub fn cc3if(&mut self) -> _Cc3ifW {
            _Cc3ifW { register: self }
        }
        # [ doc = "Bit 2 - Capture/Compare 2 interrupt flag" ]
        pub fn cc2if(&mut self) -> _Cc2ifW {
            _Cc2ifW { register: self }
        }
        # [ doc = "Bit 1 - Capture/compare 1 interrupt flag" ]
        pub fn cc1if(&mut self) -> _Cc1ifW {
            _Cc1ifW { register: self }
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
    pub struct _TgW<'a> {
        register: &'a mut W,
    }
    impl<'a> _TgW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 6;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Cc4gW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Cc4gW<'a> {
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
    pub struct _Cc3gW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Cc3gW<'a> {
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
    pub struct _Cc2gW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Cc2gW<'a> {
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
    pub struct _Cc1gW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Cc1gW<'a> {
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
        # [ doc = "Bit 6 - Trigger generation" ]
        pub fn tg(&mut self) -> _TgW {
            _TgW { register: self }
        }
        # [ doc = "Bit 4 - Capture/compare 4 generation" ]
        pub fn cc4g(&mut self) -> _Cc4gW {
            _Cc4gW { register: self }
        }
        # [ doc = "Bit 3 - Capture/compare 3 generation" ]
        pub fn cc3g(&mut self) -> _Cc3gW {
            _Cc3gW { register: self }
        }
        # [ doc = "Bit 2 - Capture/compare 2 generation" ]
        pub fn cc2g(&mut self) -> _Cc2gW {
            _Cc2gW { register: self }
        }
        # [ doc = "Bit 1 - Capture/compare 1 generation" ]
        pub fn cc1g(&mut self) -> _Cc1gW {
            _Cc1gW { register: self }
        }
        # [ doc = "Bit 0 - Update generation" ]
        pub fn ug(&mut self) -> _UgW {
            _UgW { register: self }
        }
    }
}

# [ doc = "capture/compare mode register 1 (output mode)" ]
# [ repr ( C ) ]
pub struct Ccmr1Output {
    register: ::volatile_register::RW<u32>,
}

# [ doc = "capture/compare mode register 1 (output mode)" ]
pub mod ccmr1_output {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::Ccmr1Output {
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
    # [ doc = "Value of the field OC2CE" ]
    pub struct Oc2ceR {
        bits: u8,
    }
    impl Oc2ceR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field OC2M" ]
    pub struct Oc2mR {
        bits: u8,
    }
    impl Oc2mR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field OC2PE" ]
    pub struct Oc2peR {
        bits: u8,
    }
    impl Oc2peR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field OC2FE" ]
    pub struct Oc2feR {
        bits: u8,
    }
    impl Oc2feR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field CC2S" ]
    pub struct Cc2sR {
        bits: u8,
    }
    impl Cc2sR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field OC1CE" ]
    pub struct Oc1ceR {
        bits: u8,
    }
    impl Oc1ceR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field OC1M" ]
    pub struct Oc1mR {
        bits: u8,
    }
    impl Oc1mR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field OC1PE" ]
    pub struct Oc1peR {
        bits: u8,
    }
    impl Oc1peR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field OC1FE" ]
    pub struct Oc1feR {
        bits: u8,
    }
    impl Oc1feR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field CC1S" ]
    pub struct Cc1sR {
        bits: u8,
    }
    impl Cc1sR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Oc2ceW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Oc2ceW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 15;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Oc2mW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Oc2mW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 7;
            const OFFSET: u8 = 12;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Oc2peW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Oc2peW<'a> {
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
    pub struct _Oc2feW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Oc2feW<'a> {
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
    pub struct _Cc2sW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Cc2sW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Oc1ceW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Oc1ceW<'a> {
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
    pub struct _Oc1mW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Oc1mW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Oc1peW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Oc1peW<'a> {
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
    pub struct _Oc1feW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Oc1feW<'a> {
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
    pub struct _Cc1sW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Cc1sW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 3;
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
        fn _oc2ce(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 15 - Output compare 2 clear enable" ]
        pub fn oc2ce(&self) -> Oc2ceR {
            Oc2ceR { bits: self._oc2ce() }
        }
        fn _oc2m(&self) -> u8 {
            const MASK: u8 = 7;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 12:14 - Output compare 2 mode" ]
        pub fn oc2m(&self) -> Oc2mR {
            Oc2mR { bits: self._oc2m() }
        }
        fn _oc2pe(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 11 - Output compare 2 preload enable" ]
        pub fn oc2pe(&self) -> Oc2peR {
            Oc2peR { bits: self._oc2pe() }
        }
        fn _oc2fe(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 10 - Output compare 2 fast enable" ]
        pub fn oc2fe(&self) -> Oc2feR {
            Oc2feR { bits: self._oc2fe() }
        }
        fn _cc2s(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 8:9 - Capture/Compare 2 selection" ]
        pub fn cc2s(&self) -> Cc2sR {
            Cc2sR { bits: self._cc2s() }
        }
        fn _oc1ce(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 7 - Output compare 1 clear enable" ]
        pub fn oc1ce(&self) -> Oc1ceR {
            Oc1ceR { bits: self._oc1ce() }
        }
        fn _oc1m(&self) -> u8 {
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 4:6 - Output compare 1 mode" ]
        pub fn oc1m(&self) -> Oc1mR {
            Oc1mR { bits: self._oc1m() }
        }
        fn _oc1pe(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 3 - Output compare 1 preload enable" ]
        pub fn oc1pe(&self) -> Oc1peR {
            Oc1peR { bits: self._oc1pe() }
        }
        fn _oc1fe(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 2 - Output compare 1 fast enable" ]
        pub fn oc1fe(&self) -> Oc1feR {
            Oc1feR { bits: self._oc1fe() }
        }
        fn _cc1s(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 0:1 - Capture/Compare 1 selection" ]
        pub fn cc1s(&self) -> Cc1sR {
            Cc1sR { bits: self._cc1s() }
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
        # [ doc = "Bit 15 - Output compare 2 clear enable" ]
        pub fn oc2ce(&mut self) -> _Oc2ceW {
            _Oc2ceW { register: self }
        }
        # [ doc = "Bits 12:14 - Output compare 2 mode" ]
        pub fn oc2m(&mut self) -> _Oc2mW {
            _Oc2mW { register: self }
        }
        # [ doc = "Bit 11 - Output compare 2 preload enable" ]
        pub fn oc2pe(&mut self) -> _Oc2peW {
            _Oc2peW { register: self }
        }
        # [ doc = "Bit 10 - Output compare 2 fast enable" ]
        pub fn oc2fe(&mut self) -> _Oc2feW {
            _Oc2feW { register: self }
        }
        # [ doc = "Bits 8:9 - Capture/Compare 2 selection" ]
        pub fn cc2s(&mut self) -> _Cc2sW {
            _Cc2sW { register: self }
        }
        # [ doc = "Bit 7 - Output compare 1 clear enable" ]
        pub fn oc1ce(&mut self) -> _Oc1ceW {
            _Oc1ceW { register: self }
        }
        # [ doc = "Bits 4:6 - Output compare 1 mode" ]
        pub fn oc1m(&mut self) -> _Oc1mW {
            _Oc1mW { register: self }
        }
        # [ doc = "Bit 3 - Output compare 1 preload enable" ]
        pub fn oc1pe(&mut self) -> _Oc1peW {
            _Oc1peW { register: self }
        }
        # [ doc = "Bit 2 - Output compare 1 fast enable" ]
        pub fn oc1fe(&mut self) -> _Oc1feW {
            _Oc1feW { register: self }
        }
        # [ doc = "Bits 0:1 - Capture/Compare 1 selection" ]
        pub fn cc1s(&mut self) -> _Cc1sW {
            _Cc1sW { register: self }
        }
    }
}

# [ doc = "capture/compare mode register 1 (input mode)" ]
# [ repr ( C ) ]
pub struct Ccmr1Input {
    register: ::volatile_register::RW<u32>,
}

# [ doc = "capture/compare mode register 1 (input mode)" ]
pub mod ccmr1_input {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::Ccmr1Input {
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
    # [ doc = "Value of the field IC2F" ]
    pub struct Ic2fR {
        bits: u8,
    }
    impl Ic2fR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field IC2PSC" ]
    pub struct Ic2pscR {
        bits: u8,
    }
    impl Ic2pscR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field CC2S" ]
    pub struct Cc2sR {
        bits: u8,
    }
    impl Cc2sR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field IC1F" ]
    pub struct Ic1fR {
        bits: u8,
    }
    impl Ic1fR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field IC1PSC" ]
    pub struct Ic1pscR {
        bits: u8,
    }
    impl Ic1pscR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field CC1S" ]
    pub struct Cc1sR {
        bits: u8,
    }
    impl Cc1sR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Ic2fW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Ic2fW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Ic2pscW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Ic2pscW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Cc2sW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Cc2sW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Ic1fW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Ic1fW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Ic1pscW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Ic1pscW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Cc1sW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Cc1sW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 3;
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
        fn _ic2f(&self) -> u8 {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 12:15 - Input capture 2 filter" ]
        pub fn ic2f(&self) -> Ic2fR {
            Ic2fR { bits: self._ic2f() }
        }
        fn _ic2psc(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 10:11 - Input capture 2 prescaler" ]
        pub fn ic2psc(&self) -> Ic2pscR {
            Ic2pscR { bits: self._ic2psc() }
        }
        fn _cc2s(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 8:9 - Capture/compare 2 selection" ]
        pub fn cc2s(&self) -> Cc2sR {
            Cc2sR { bits: self._cc2s() }
        }
        fn _ic1f(&self) -> u8 {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 4:7 - Input capture 1 filter" ]
        pub fn ic1f(&self) -> Ic1fR {
            Ic1fR { bits: self._ic1f() }
        }
        fn _ic1psc(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 2:3 - Input capture 1 prescaler" ]
        pub fn ic1psc(&self) -> Ic1pscR {
            Ic1pscR { bits: self._ic1psc() }
        }
        fn _cc1s(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 0:1 - Capture/Compare 1 selection" ]
        pub fn cc1s(&self) -> Cc1sR {
            Cc1sR { bits: self._cc1s() }
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
        # [ doc = "Bits 12:15 - Input capture 2 filter" ]
        pub fn ic2f(&mut self) -> _Ic2fW {
            _Ic2fW { register: self }
        }
        # [ doc = "Bits 10:11 - Input capture 2 prescaler" ]
        pub fn ic2psc(&mut self) -> _Ic2pscW {
            _Ic2pscW { register: self }
        }
        # [ doc = "Bits 8:9 - Capture/compare 2 selection" ]
        pub fn cc2s(&mut self) -> _Cc2sW {
            _Cc2sW { register: self }
        }
        # [ doc = "Bits 4:7 - Input capture 1 filter" ]
        pub fn ic1f(&mut self) -> _Ic1fW {
            _Ic1fW { register: self }
        }
        # [ doc = "Bits 2:3 - Input capture 1 prescaler" ]
        pub fn ic1psc(&mut self) -> _Ic1pscW {
            _Ic1pscW { register: self }
        }
        # [ doc = "Bits 0:1 - Capture/Compare 1 selection" ]
        pub fn cc1s(&mut self) -> _Cc1sW {
            _Cc1sW { register: self }
        }
    }
}

# [ doc = "capture/compare mode register 2 (output mode)" ]
# [ repr ( C ) ]
pub struct Ccmr2Output {
    register: ::volatile_register::RW<u32>,
}

# [ doc = "capture/compare mode register 2 (output mode)" ]
pub mod ccmr2_output {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::Ccmr2Output {
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
    # [ doc = "Value of the field OC4CE" ]
    pub struct Oc4ceR {
        bits: u8,
    }
    impl Oc4ceR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field OC4M" ]
    pub struct Oc4mR {
        bits: u8,
    }
    impl Oc4mR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field OC4PE" ]
    pub struct Oc4peR {
        bits: u8,
    }
    impl Oc4peR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field OC4FE" ]
    pub struct Oc4feR {
        bits: u8,
    }
    impl Oc4feR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field CC4S" ]
    pub struct Cc4sR {
        bits: u8,
    }
    impl Cc4sR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field OC3CE" ]
    pub struct Oc3ceR {
        bits: u8,
    }
    impl Oc3ceR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field OC3M" ]
    pub struct Oc3mR {
        bits: u8,
    }
    impl Oc3mR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field OC3PE" ]
    pub struct Oc3peR {
        bits: u8,
    }
    impl Oc3peR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field OC3FE" ]
    pub struct Oc3feR {
        bits: u8,
    }
    impl Oc3feR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field CC3S" ]
    pub struct Cc3sR {
        bits: u8,
    }
    impl Cc3sR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Oc4ceW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Oc4ceW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 15;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Oc4mW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Oc4mW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 7;
            const OFFSET: u8 = 12;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Oc4peW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Oc4peW<'a> {
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
    pub struct _Oc4feW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Oc4feW<'a> {
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
    pub struct _Cc4sW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Cc4sW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Oc3ceW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Oc3ceW<'a> {
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
    pub struct _Oc3mW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Oc3mW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Oc3peW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Oc3peW<'a> {
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
    pub struct _Oc3feW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Oc3feW<'a> {
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
    pub struct _Cc3sW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Cc3sW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 3;
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
        fn _oc4ce(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 15 - Output compare 4 clear enable" ]
        pub fn oc4ce(&self) -> Oc4ceR {
            Oc4ceR { bits: self._oc4ce() }
        }
        fn _oc4m(&self) -> u8 {
            const MASK: u8 = 7;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 12:14 - Output compare 4 mode" ]
        pub fn oc4m(&self) -> Oc4mR {
            Oc4mR { bits: self._oc4m() }
        }
        fn _oc4pe(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 11 - Output compare 4 preload enable" ]
        pub fn oc4pe(&self) -> Oc4peR {
            Oc4peR { bits: self._oc4pe() }
        }
        fn _oc4fe(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 10 - Output compare 4 fast enable" ]
        pub fn oc4fe(&self) -> Oc4feR {
            Oc4feR { bits: self._oc4fe() }
        }
        fn _cc4s(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 8:9 - Capture/Compare 4 selection" ]
        pub fn cc4s(&self) -> Cc4sR {
            Cc4sR { bits: self._cc4s() }
        }
        fn _oc3ce(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 7 - Output compare 3 clear enable" ]
        pub fn oc3ce(&self) -> Oc3ceR {
            Oc3ceR { bits: self._oc3ce() }
        }
        fn _oc3m(&self) -> u8 {
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 4:6 - Output compare 3 mode" ]
        pub fn oc3m(&self) -> Oc3mR {
            Oc3mR { bits: self._oc3m() }
        }
        fn _oc3pe(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 3 - Output compare 3 preload enable" ]
        pub fn oc3pe(&self) -> Oc3peR {
            Oc3peR { bits: self._oc3pe() }
        }
        fn _oc3fe(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 2 - Output compare 3 fast enable" ]
        pub fn oc3fe(&self) -> Oc3feR {
            Oc3feR { bits: self._oc3fe() }
        }
        fn _cc3s(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 0:1 - Capture/Compare 3 selection" ]
        pub fn cc3s(&self) -> Cc3sR {
            Cc3sR { bits: self._cc3s() }
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
        # [ doc = "Bit 15 - Output compare 4 clear enable" ]
        pub fn oc4ce(&mut self) -> _Oc4ceW {
            _Oc4ceW { register: self }
        }
        # [ doc = "Bits 12:14 - Output compare 4 mode" ]
        pub fn oc4m(&mut self) -> _Oc4mW {
            _Oc4mW { register: self }
        }
        # [ doc = "Bit 11 - Output compare 4 preload enable" ]
        pub fn oc4pe(&mut self) -> _Oc4peW {
            _Oc4peW { register: self }
        }
        # [ doc = "Bit 10 - Output compare 4 fast enable" ]
        pub fn oc4fe(&mut self) -> _Oc4feW {
            _Oc4feW { register: self }
        }
        # [ doc = "Bits 8:9 - Capture/Compare 4 selection" ]
        pub fn cc4s(&mut self) -> _Cc4sW {
            _Cc4sW { register: self }
        }
        # [ doc = "Bit 7 - Output compare 3 clear enable" ]
        pub fn oc3ce(&mut self) -> _Oc3ceW {
            _Oc3ceW { register: self }
        }
        # [ doc = "Bits 4:6 - Output compare 3 mode" ]
        pub fn oc3m(&mut self) -> _Oc3mW {
            _Oc3mW { register: self }
        }
        # [ doc = "Bit 3 - Output compare 3 preload enable" ]
        pub fn oc3pe(&mut self) -> _Oc3peW {
            _Oc3peW { register: self }
        }
        # [ doc = "Bit 2 - Output compare 3 fast enable" ]
        pub fn oc3fe(&mut self) -> _Oc3feW {
            _Oc3feW { register: self }
        }
        # [ doc = "Bits 0:1 - Capture/Compare 3 selection" ]
        pub fn cc3s(&mut self) -> _Cc3sW {
            _Cc3sW { register: self }
        }
    }
}

# [ doc = "capture/compare mode register 2 (input mode)" ]
# [ repr ( C ) ]
pub struct Ccmr2Input {
    register: ::volatile_register::RW<u32>,
}

# [ doc = "capture/compare mode register 2 (input mode)" ]
pub mod ccmr2_input {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::Ccmr2Input {
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
    # [ doc = "Value of the field IC4F" ]
    pub struct Ic4fR {
        bits: u8,
    }
    impl Ic4fR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field IC4PSC" ]
    pub struct Ic4pscR {
        bits: u8,
    }
    impl Ic4pscR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field CC4S" ]
    pub struct Cc4sR {
        bits: u8,
    }
    impl Cc4sR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field IC3F" ]
    pub struct Ic3fR {
        bits: u8,
    }
    impl Ic3fR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field IC3PSC" ]
    pub struct Ic3pscR {
        bits: u8,
    }
    impl Ic3pscR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field CC3S" ]
    pub struct Cc3sR {
        bits: u8,
    }
    impl Cc3sR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Ic4fW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Ic4fW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Ic4pscW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Ic4pscW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Cc4sW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Cc4sW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Ic3fW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Ic3fW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Ic3pscW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Ic3pscW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Cc3sW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Cc3sW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 3;
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
        fn _ic4f(&self) -> u8 {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 12:15 - Input capture 4 filter" ]
        pub fn ic4f(&self) -> Ic4fR {
            Ic4fR { bits: self._ic4f() }
        }
        fn _ic4psc(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 10:11 - Input capture 4 prescaler" ]
        pub fn ic4psc(&self) -> Ic4pscR {
            Ic4pscR { bits: self._ic4psc() }
        }
        fn _cc4s(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 8:9 - Capture/Compare 4 selection" ]
        pub fn cc4s(&self) -> Cc4sR {
            Cc4sR { bits: self._cc4s() }
        }
        fn _ic3f(&self) -> u8 {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 4:7 - Input capture 3 filter" ]
        pub fn ic3f(&self) -> Ic3fR {
            Ic3fR { bits: self._ic3f() }
        }
        fn _ic3psc(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 2:3 - Input capture 3 prescaler" ]
        pub fn ic3psc(&self) -> Ic3pscR {
            Ic3pscR { bits: self._ic3psc() }
        }
        fn _cc3s(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 0:1 - Capture/Compare 3 selection" ]
        pub fn cc3s(&self) -> Cc3sR {
            Cc3sR { bits: self._cc3s() }
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
        # [ doc = "Bits 12:15 - Input capture 4 filter" ]
        pub fn ic4f(&mut self) -> _Ic4fW {
            _Ic4fW { register: self }
        }
        # [ doc = "Bits 10:11 - Input capture 4 prescaler" ]
        pub fn ic4psc(&mut self) -> _Ic4pscW {
            _Ic4pscW { register: self }
        }
        # [ doc = "Bits 8:9 - Capture/Compare 4 selection" ]
        pub fn cc4s(&mut self) -> _Cc4sW {
            _Cc4sW { register: self }
        }
        # [ doc = "Bits 4:7 - Input capture 3 filter" ]
        pub fn ic3f(&mut self) -> _Ic3fW {
            _Ic3fW { register: self }
        }
        # [ doc = "Bits 2:3 - Input capture 3 prescaler" ]
        pub fn ic3psc(&mut self) -> _Ic3pscW {
            _Ic3pscW { register: self }
        }
        # [ doc = "Bits 0:1 - Capture/Compare 3 selection" ]
        pub fn cc3s(&mut self) -> _Cc3sW {
            _Cc3sW { register: self }
        }
    }
}

# [ doc = "capture/compare enable register" ]
# [ repr ( C ) ]
pub struct Ccer {
    register: ::volatile_register::RW<u32>,
}

# [ doc = "capture/compare enable register" ]
pub mod ccer {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::Ccer {
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
    # [ doc = "Value of the field CC4NP" ]
    pub struct Cc4npR {
        bits: u8,
    }
    impl Cc4npR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field CC4P" ]
    pub struct Cc4pR {
        bits: u8,
    }
    impl Cc4pR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field CC4E" ]
    pub struct Cc4eR {
        bits: u8,
    }
    impl Cc4eR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field CC3NP" ]
    pub struct Cc3npR {
        bits: u8,
    }
    impl Cc3npR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field CC3P" ]
    pub struct Cc3pR {
        bits: u8,
    }
    impl Cc3pR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field CC3E" ]
    pub struct Cc3eR {
        bits: u8,
    }
    impl Cc3eR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field CC2NP" ]
    pub struct Cc2npR {
        bits: u8,
    }
    impl Cc2npR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field CC2P" ]
    pub struct Cc2pR {
        bits: u8,
    }
    impl Cc2pR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field CC2E" ]
    pub struct Cc2eR {
        bits: u8,
    }
    impl Cc2eR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field CC1NP" ]
    pub struct Cc1npR {
        bits: u8,
    }
    impl Cc1npR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field CC1P" ]
    pub struct Cc1pR {
        bits: u8,
    }
    impl Cc1pR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field CC1E" ]
    pub struct Cc1eR {
        bits: u8,
    }
    impl Cc1eR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Cc4npW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Cc4npW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 15;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Cc4pW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Cc4pW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 13;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Cc4eW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Cc4eW<'a> {
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
    pub struct _Cc3npW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Cc3npW<'a> {
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
    pub struct _Cc3pW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Cc3pW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 9;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Cc3eW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Cc3eW<'a> {
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
    pub struct _Cc2npW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Cc2npW<'a> {
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
    pub struct _Cc2pW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Cc2pW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 5;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Cc2eW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Cc2eW<'a> {
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
    pub struct _Cc1npW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Cc1npW<'a> {
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
    pub struct _Cc1pW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Cc1pW<'a> {
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
    pub struct _Cc1eW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Cc1eW<'a> {
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
        fn _cc4np(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 15 - Capture/Compare 4 output Polarity" ]
        pub fn cc4np(&self) -> Cc4npR {
            Cc4npR { bits: self._cc4np() }
        }
        fn _cc4p(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 13 - Capture/Compare 3 output Polarity" ]
        pub fn cc4p(&self) -> Cc4pR {
            Cc4pR { bits: self._cc4p() }
        }
        fn _cc4e(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 12 - Capture/Compare 4 output enable" ]
        pub fn cc4e(&self) -> Cc4eR {
            Cc4eR { bits: self._cc4e() }
        }
        fn _cc3np(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 11 - Capture/Compare 3 output Polarity" ]
        pub fn cc3np(&self) -> Cc3npR {
            Cc3npR { bits: self._cc3np() }
        }
        fn _cc3p(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 9 - Capture/Compare 3 output Polarity" ]
        pub fn cc3p(&self) -> Cc3pR {
            Cc3pR { bits: self._cc3p() }
        }
        fn _cc3e(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 8 - Capture/Compare 3 output enable" ]
        pub fn cc3e(&self) -> Cc3eR {
            Cc3eR { bits: self._cc3e() }
        }
        fn _cc2np(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 7 - Capture/Compare 2 output Polarity" ]
        pub fn cc2np(&self) -> Cc2npR {
            Cc2npR { bits: self._cc2np() }
        }
        fn _cc2p(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 5 - Capture/Compare 2 output Polarity" ]
        pub fn cc2p(&self) -> Cc2pR {
            Cc2pR { bits: self._cc2p() }
        }
        fn _cc2e(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 4 - Capture/Compare 2 output enable" ]
        pub fn cc2e(&self) -> Cc2eR {
            Cc2eR { bits: self._cc2e() }
        }
        fn _cc1np(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 3 - Capture/Compare 1 output Polarity" ]
        pub fn cc1np(&self) -> Cc1npR {
            Cc1npR { bits: self._cc1np() }
        }
        fn _cc1p(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 1 - Capture/Compare 1 output Polarity" ]
        pub fn cc1p(&self) -> Cc1pR {
            Cc1pR { bits: self._cc1p() }
        }
        fn _cc1e(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 0 - Capture/Compare 1 output enable" ]
        pub fn cc1e(&self) -> Cc1eR {
            Cc1eR { bits: self._cc1e() }
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
        # [ doc = "Bit 15 - Capture/Compare 4 output Polarity" ]
        pub fn cc4np(&mut self) -> _Cc4npW {
            _Cc4npW { register: self }
        }
        # [ doc = "Bit 13 - Capture/Compare 3 output Polarity" ]
        pub fn cc4p(&mut self) -> _Cc4pW {
            _Cc4pW { register: self }
        }
        # [ doc = "Bit 12 - Capture/Compare 4 output enable" ]
        pub fn cc4e(&mut self) -> _Cc4eW {
            _Cc4eW { register: self }
        }
        # [ doc = "Bit 11 - Capture/Compare 3 output Polarity" ]
        pub fn cc3np(&mut self) -> _Cc3npW {
            _Cc3npW { register: self }
        }
        # [ doc = "Bit 9 - Capture/Compare 3 output Polarity" ]
        pub fn cc3p(&mut self) -> _Cc3pW {
            _Cc3pW { register: self }
        }
        # [ doc = "Bit 8 - Capture/Compare 3 output enable" ]
        pub fn cc3e(&mut self) -> _Cc3eW {
            _Cc3eW { register: self }
        }
        # [ doc = "Bit 7 - Capture/Compare 2 output Polarity" ]
        pub fn cc2np(&mut self) -> _Cc2npW {
            _Cc2npW { register: self }
        }
        # [ doc = "Bit 5 - Capture/Compare 2 output Polarity" ]
        pub fn cc2p(&mut self) -> _Cc2pW {
            _Cc2pW { register: self }
        }
        # [ doc = "Bit 4 - Capture/Compare 2 output enable" ]
        pub fn cc2e(&mut self) -> _Cc2eW {
            _Cc2eW { register: self }
        }
        # [ doc = "Bit 3 - Capture/Compare 1 output Polarity" ]
        pub fn cc1np(&mut self) -> _Cc1npW {
            _Cc1npW { register: self }
        }
        # [ doc = "Bit 1 - Capture/Compare 1 output Polarity" ]
        pub fn cc1p(&mut self) -> _Cc1pW {
            _Cc1pW { register: self }
        }
        # [ doc = "Bit 0 - Capture/Compare 1 output enable" ]
        pub fn cc1e(&mut self) -> _Cc1eW {
            _Cc1eW { register: self }
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
    # [ doc = "Value of the field CNT_H" ]
    pub struct CntHR {
        bits: u16,
    }
    impl CntHR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = "Value of the field CNT_L" ]
    pub struct CntLR {
        bits: u16,
    }
    impl CntLR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _CntHW<'a> {
        register: &'a mut W,
    }
    impl<'a> _CntHW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u16) -> &'a mut W {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 16;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _CntLW<'a> {
        register: &'a mut W,
    }
    impl<'a> _CntLW<'a> {
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
        fn _cnt_h(&self) -> u16 {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        }
        # [ doc = "Bits 16:31 - High counter value (TIM2 only)" ]
        pub fn cnt_h(&self) -> CntHR {
            CntHR { bits: self._cnt_h() }
        }
        fn _cnt_l(&self) -> u16 {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        }
        # [ doc = "Bits 0:15 - Low counter value" ]
        pub fn cnt_l(&self) -> CntLR {
            CntLR { bits: self._cnt_l() }
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
        # [ doc = "Bits 16:31 - High counter value (TIM2 only)" ]
        pub fn cnt_h(&mut self) -> _CntHW {
            _CntHW { register: self }
        }
        # [ doc = "Bits 0:15 - Low counter value" ]
        pub fn cnt_l(&mut self) -> _CntLW {
            _CntLW { register: self }
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
    # [ doc = "Value of the field ARR_H" ]
    pub struct ArrHR {
        bits: u16,
    }
    impl ArrHR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = "Value of the field ARR_L" ]
    pub struct ArrLR {
        bits: u16,
    }
    impl ArrLR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _ArrHW<'a> {
        register: &'a mut W,
    }
    impl<'a> _ArrHW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u16) -> &'a mut W {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 16;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _ArrLW<'a> {
        register: &'a mut W,
    }
    impl<'a> _ArrLW<'a> {
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
        fn _arr_h(&self) -> u16 {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        }
        # [ doc = "Bits 16:31 - High Auto-reload value (TIM2 only)" ]
        pub fn arr_h(&self) -> ArrHR {
            ArrHR { bits: self._arr_h() }
        }
        fn _arr_l(&self) -> u16 {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        }
        # [ doc = "Bits 0:15 - Low Auto-reload value" ]
        pub fn arr_l(&self) -> ArrLR {
            ArrLR { bits: self._arr_l() }
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
        # [ doc = "Bits 16:31 - High Auto-reload value (TIM2 only)" ]
        pub fn arr_h(&mut self) -> _ArrHW {
            _ArrHW { register: self }
        }
        # [ doc = "Bits 0:15 - Low Auto-reload value" ]
        pub fn arr_l(&mut self) -> _ArrLW {
            _ArrLW { register: self }
        }
    }
}

# [ doc = "capture/compare register 1" ]
# [ repr ( C ) ]
pub struct Ccr1 {
    register: ::volatile_register::RW<u32>,
}

# [ doc = "capture/compare register 1" ]
pub mod ccr1 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::Ccr1 {
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
    # [ doc = "Value of the field CCR1_H" ]
    pub struct Ccr1HR {
        bits: u16,
    }
    impl Ccr1HR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = "Value of the field CCR1_L" ]
    pub struct Ccr1LR {
        bits: u16,
    }
    impl Ccr1LR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Ccr1HW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Ccr1HW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u16) -> &'a mut W {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 16;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Ccr1LW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Ccr1LW<'a> {
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
        fn _ccr1_h(&self) -> u16 {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        }
        # [ doc = "Bits 16:31 - High Capture/Compare 1 value (TIM2 only)" ]
        pub fn ccr1_h(&self) -> Ccr1HR {
            Ccr1HR { bits: self._ccr1_h() }
        }
        fn _ccr1_l(&self) -> u16 {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        }
        # [ doc = "Bits 0:15 - Low Capture/Compare 1 value" ]
        pub fn ccr1_l(&self) -> Ccr1LR {
            Ccr1LR { bits: self._ccr1_l() }
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
        # [ doc = "Bits 16:31 - High Capture/Compare 1 value (TIM2 only)" ]
        pub fn ccr1_h(&mut self) -> _Ccr1HW {
            _Ccr1HW { register: self }
        }
        # [ doc = "Bits 0:15 - Low Capture/Compare 1 value" ]
        pub fn ccr1_l(&mut self) -> _Ccr1LW {
            _Ccr1LW { register: self }
        }
    }
}

# [ doc = "capture/compare register 2" ]
# [ repr ( C ) ]
pub struct Ccr2 {
    register: ::volatile_register::RW<u32>,
}

# [ doc = "capture/compare register 2" ]
pub mod ccr2 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::Ccr2 {
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
    # [ doc = "Value of the field CCR2_H" ]
    pub struct Ccr2HR {
        bits: u16,
    }
    impl Ccr2HR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = "Value of the field CCR2_L" ]
    pub struct Ccr2LR {
        bits: u16,
    }
    impl Ccr2LR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Ccr2HW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Ccr2HW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u16) -> &'a mut W {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 16;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Ccr2LW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Ccr2LW<'a> {
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
        fn _ccr2_h(&self) -> u16 {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        }
        # [ doc = "Bits 16:31 - High Capture/Compare 2 value (TIM2 only)" ]
        pub fn ccr2_h(&self) -> Ccr2HR {
            Ccr2HR { bits: self._ccr2_h() }
        }
        fn _ccr2_l(&self) -> u16 {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        }
        # [ doc = "Bits 0:15 - Low Capture/Compare 2 value" ]
        pub fn ccr2_l(&self) -> Ccr2LR {
            Ccr2LR { bits: self._ccr2_l() }
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
        # [ doc = "Bits 16:31 - High Capture/Compare 2 value (TIM2 only)" ]
        pub fn ccr2_h(&mut self) -> _Ccr2HW {
            _Ccr2HW { register: self }
        }
        # [ doc = "Bits 0:15 - Low Capture/Compare 2 value" ]
        pub fn ccr2_l(&mut self) -> _Ccr2LW {
            _Ccr2LW { register: self }
        }
    }
}

# [ doc = "capture/compare register 3" ]
# [ repr ( C ) ]
pub struct Ccr3 {
    register: ::volatile_register::RW<u32>,
}

# [ doc = "capture/compare register 3" ]
pub mod ccr3 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::Ccr3 {
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
    # [ doc = "Value of the field CCR3_H" ]
    pub struct Ccr3HR {
        bits: u16,
    }
    impl Ccr3HR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = "Value of the field CCR3_L" ]
    pub struct Ccr3LR {
        bits: u16,
    }
    impl Ccr3LR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Ccr3HW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Ccr3HW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u16) -> &'a mut W {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 16;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Ccr3LW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Ccr3LW<'a> {
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
        fn _ccr3_h(&self) -> u16 {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        }
        # [ doc = "Bits 16:31 - High Capture/Compare value (TIM2 only)" ]
        pub fn ccr3_h(&self) -> Ccr3HR {
            Ccr3HR { bits: self._ccr3_h() }
        }
        fn _ccr3_l(&self) -> u16 {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        }
        # [ doc = "Bits 0:15 - Low Capture/Compare value" ]
        pub fn ccr3_l(&self) -> Ccr3LR {
            Ccr3LR { bits: self._ccr3_l() }
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
        # [ doc = "Bits 16:31 - High Capture/Compare value (TIM2 only)" ]
        pub fn ccr3_h(&mut self) -> _Ccr3HW {
            _Ccr3HW { register: self }
        }
        # [ doc = "Bits 0:15 - Low Capture/Compare value" ]
        pub fn ccr3_l(&mut self) -> _Ccr3LW {
            _Ccr3LW { register: self }
        }
    }
}

# [ doc = "capture/compare register 4" ]
# [ repr ( C ) ]
pub struct Ccr4 {
    register: ::volatile_register::RW<u32>,
}

# [ doc = "capture/compare register 4" ]
pub mod ccr4 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::Ccr4 {
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
    # [ doc = "Value of the field CCR4_H" ]
    pub struct Ccr4HR {
        bits: u16,
    }
    impl Ccr4HR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = "Value of the field CCR4_L" ]
    pub struct Ccr4LR {
        bits: u16,
    }
    impl Ccr4LR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Ccr4HW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Ccr4HW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u16) -> &'a mut W {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 16;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Ccr4LW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Ccr4LW<'a> {
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
        fn _ccr4_h(&self) -> u16 {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        }
        # [ doc = "Bits 16:31 - High Capture/Compare value (TIM2 only)" ]
        pub fn ccr4_h(&self) -> Ccr4HR {
            Ccr4HR { bits: self._ccr4_h() }
        }
        fn _ccr4_l(&self) -> u16 {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        }
        # [ doc = "Bits 0:15 - Low Capture/Compare value" ]
        pub fn ccr4_l(&self) -> Ccr4LR {
            Ccr4LR { bits: self._ccr4_l() }
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
        # [ doc = "Bits 16:31 - High Capture/Compare value (TIM2 only)" ]
        pub fn ccr4_h(&mut self) -> _Ccr4HW {
            _Ccr4HW { register: self }
        }
        # [ doc = "Bits 0:15 - Low Capture/Compare value" ]
        pub fn ccr4_l(&mut self) -> _Ccr4LW {
            _Ccr4LW { register: self }
        }
    }
}

# [ doc = "DMA control register" ]
# [ repr ( C ) ]
pub struct Dcr {
    register: ::volatile_register::RW<u32>,
}

# [ doc = "DMA control register" ]
pub mod dcr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::Dcr {
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
    # [ doc = "Value of the field DBL" ]
    pub struct DblR {
        bits: u8,
    }
    impl DblR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field DBA" ]
    pub struct DbaR {
        bits: u8,
    }
    impl DbaR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _DblW<'a> {
        register: &'a mut W,
    }
    impl<'a> _DblW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 31;
            const OFFSET: u8 = 8;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _DbaW<'a> {
        register: &'a mut W,
    }
    impl<'a> _DbaW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 31;
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
        fn _dbl(&self) -> u8 {
            const MASK: u8 = 31;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 8:12 - DMA burst length" ]
        pub fn dbl(&self) -> DblR {
            DblR { bits: self._dbl() }
        }
        fn _dba(&self) -> u8 {
            const MASK: u8 = 31;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 0:4 - DMA base address" ]
        pub fn dba(&self) -> DbaR {
            DbaR { bits: self._dba() }
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
        # [ doc = "Bits 8:12 - DMA burst length" ]
        pub fn dbl(&mut self) -> _DblW {
            _DblW { register: self }
        }
        # [ doc = "Bits 0:4 - DMA base address" ]
        pub fn dba(&mut self) -> _DbaW {
            _DbaW { register: self }
        }
    }
}

# [ doc = "DMA address for full transfer" ]
# [ repr ( C ) ]
pub struct Dmar {
    register: ::volatile_register::RW<u32>,
}

# [ doc = "DMA address for full transfer" ]
pub mod dmar {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::Dmar {
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
    # [ doc = "Value of the field DMAR" ]
    pub struct DmarR {
        bits: u16,
    }
    impl DmarR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _DmarW<'a> {
        register: &'a mut W,
    }
    impl<'a> _DmarW<'a> {
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
        fn _dmar(&self) -> u16 {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        }
        # [ doc = "Bits 0:15 - DMA register for burst accesses" ]
        pub fn dmar(&self) -> DmarR {
            DmarR { bits: self._dmar() }
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
        # [ doc = "Bits 0:15 - DMA register for burst accesses" ]
        pub fn dmar(&mut self) -> _DmarW {
            _DmarW { register: self }
        }
    }
}
