# ! [ doc = "Reset and clock control" ]
# [ doc = r" Register block" ]
# [ repr ( C ) ]
pub struct Rcc {
    # [ doc = "0x00 - Clock control register" ]
    pub cr: Cr,
    # [ doc = "0x04 - Clock configuration register (RCC_CFGR)" ]
    pub cfgr: Cfgr,
    # [ doc = "0x08 - Clock interrupt register (RCC_CIR)" ]
    pub cir: Cir,
    # [ doc = "0x0c - APB2 peripheral reset register (RCC_APB2RSTR)" ]
    pub apb2rstr: Apb2rstr,
    # [ doc = "0x10 - APB1 peripheral reset register (RCC_APB1RSTR)" ]
    pub apb1rstr: Apb1rstr,
    # [ doc = "0x14 - AHB Peripheral Clock enable register (RCC_AHBENR)" ]
    pub ahbenr: Ahbenr,
    # [ doc = "0x18 - APB2 peripheral clock enable register (RCC_APB2ENR)" ]
    pub apb2enr: Apb2enr,
    # [ doc = "0x1c - APB1 peripheral clock enable register (RCC_APB1ENR)" ]
    pub apb1enr: Apb1enr,
    # [ doc = "0x20 - Backup domain control register (RCC_BDCR)" ]
    pub bdcr: Bdcr,
    # [ doc = "0x24 - Control/status register (RCC_CSR)" ]
    pub csr: Csr,
    # [ doc = "0x28 - AHB peripheral reset register" ]
    pub ahbrstr: Ahbrstr,
    # [ doc = "0x2c - Clock configuration register 2" ]
    pub cfgr2: Cfgr2,
    # [ doc = "0x30 - Clock configuration register 3" ]
    pub cfgr3: Cfgr3,
    # [ doc = "0x34 - Clock control register 2" ]
    pub cr2: Cr2,
}

# [ doc = "Clock control register" ]
# [ repr ( C ) ]
pub struct Cr {
    register: ::volatile_register::RW<u32>,
}

# [ doc = "Clock control register" ]
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
    # [ doc = "Value of the field HSION" ]
    pub struct HsionR {
        bits: u8,
    }
    impl HsionR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field HSIRDY" ]
    pub struct HsirdyR {
        bits: u8,
    }
    impl HsirdyR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field HSITRIM" ]
    pub struct HsitrimR {
        bits: u8,
    }
    impl HsitrimR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field HSICAL" ]
    pub struct HsicalR {
        bits: u8,
    }
    impl HsicalR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field HSEON" ]
    pub struct HseonR {
        bits: u8,
    }
    impl HseonR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field HSERDY" ]
    pub struct HserdyR {
        bits: u8,
    }
    impl HserdyR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field HSEBYP" ]
    pub struct HsebypR {
        bits: u8,
    }
    impl HsebypR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field CSSON" ]
    pub struct CssonR {
        bits: u8,
    }
    impl CssonR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field PLLON" ]
    pub struct PllonR {
        bits: u8,
    }
    impl PllonR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field PLLRDY" ]
    pub struct PllrdyR {
        bits: u8,
    }
    impl PllrdyR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _HsionW<'a> {
        register: &'a mut W,
    }
    impl<'a> _HsionW<'a> {
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
    pub struct _HsitrimW<'a> {
        register: &'a mut W,
    }
    impl<'a> _HsitrimW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 31;
            const OFFSET: u8 = 3;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _HseonW<'a> {
        register: &'a mut W,
    }
    impl<'a> _HseonW<'a> {
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
    pub struct _HsebypW<'a> {
        register: &'a mut W,
    }
    impl<'a> _HsebypW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 18;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _CssonW<'a> {
        register: &'a mut W,
    }
    impl<'a> _CssonW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 19;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PllonW<'a> {
        register: &'a mut W,
    }
    impl<'a> _PllonW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 24;
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
        fn _hsion(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 0 - Internal High Speed clock enable" ]
        pub fn hsion(&self) -> HsionR {
            HsionR { bits: self._hsion() }
        }
        fn _hsirdy(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 1 - Internal High Speed clock ready flag" ]
        pub fn hsirdy(&self) -> HsirdyR {
            HsirdyR { bits: self._hsirdy() }
        }
        fn _hsitrim(&self) -> u8 {
            const MASK: u8 = 31;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 3:7 - Internal High Speed clock trimming" ]
        pub fn hsitrim(&self) -> HsitrimR {
            HsitrimR { bits: self._hsitrim() }
        }
        fn _hsical(&self) -> u8 {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 8:15 - Internal High Speed clock Calibration" ]
        pub fn hsical(&self) -> HsicalR {
            HsicalR { bits: self._hsical() }
        }
        fn _hseon(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 16 - External High Speed clock enable" ]
        pub fn hseon(&self) -> HseonR {
            HseonR { bits: self._hseon() }
        }
        fn _hserdy(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 17 - External High Speed clock ready flag" ]
        pub fn hserdy(&self) -> HserdyR {
            HserdyR { bits: self._hserdy() }
        }
        fn _hsebyp(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 18 - External High Speed clock Bypass" ]
        pub fn hsebyp(&self) -> HsebypR {
            HsebypR { bits: self._hsebyp() }
        }
        fn _csson(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 19 - Clock Security System enable" ]
        pub fn csson(&self) -> CssonR {
            CssonR { bits: self._csson() }
        }
        fn _pllon(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 24 - PLL enable" ]
        pub fn pllon(&self) -> PllonR {
            PllonR { bits: self._pllon() }
        }
        fn _pllrdy(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 25 - PLL clock ready flag" ]
        pub fn pllrdy(&self) -> PllrdyR {
            PllrdyR { bits: self._pllrdy() }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        pub fn reset_value() -> W {
            W { bits: 131 }
        }
        # [ doc = r" Writes raw `bits` to the register" ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bit 0 - Internal High Speed clock enable" ]
        pub fn hsion(&mut self) -> _HsionW {
            _HsionW { register: self }
        }
        # [ doc = "Bits 3:7 - Internal High Speed clock trimming" ]
        pub fn hsitrim(&mut self) -> _HsitrimW {
            _HsitrimW { register: self }
        }
        # [ doc = "Bit 16 - External High Speed clock enable" ]
        pub fn hseon(&mut self) -> _HseonW {
            _HseonW { register: self }
        }
        # [ doc = "Bit 18 - External High Speed clock Bypass" ]
        pub fn hsebyp(&mut self) -> _HsebypW {
            _HsebypW { register: self }
        }
        # [ doc = "Bit 19 - Clock Security System enable" ]
        pub fn csson(&mut self) -> _CssonW {
            _CssonW { register: self }
        }
        # [ doc = "Bit 24 - PLL enable" ]
        pub fn pllon(&mut self) -> _PllonW {
            _PllonW { register: self }
        }
    }
}

# [ doc = "Clock configuration register (RCC_CFGR)" ]
# [ repr ( C ) ]
pub struct Cfgr {
    register: ::volatile_register::RW<u32>,
}

# [ doc = "Clock configuration register (RCC_CFGR)" ]
pub mod cfgr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::Cfgr {
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
    # [ doc = "Value of the field SW" ]
    pub struct SwR {
        bits: u8,
    }
    impl SwR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field SWS" ]
    pub struct SwsR {
        bits: u8,
    }
    impl SwsR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field HPRE" ]
    pub struct HpreR {
        bits: u8,
    }
    impl HpreR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field PPRE" ]
    pub struct PpreR {
        bits: u8,
    }
    impl PpreR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field ADCPRE" ]
    pub struct AdcpreR {
        bits: u8,
    }
    impl AdcpreR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field PLLSRC" ]
    pub struct PllsrcR {
        bits: u8,
    }
    impl PllsrcR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field PLLXTPRE" ]
    pub struct PllxtpreR {
        bits: u8,
    }
    impl PllxtpreR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field PLLMUL" ]
    pub struct PllmulR {
        bits: u8,
    }
    impl PllmulR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field MCO" ]
    pub struct McoR {
        bits: u8,
    }
    impl McoR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field MCOPRE" ]
    pub struct McopreR {
        bits: u8,
    }
    impl McopreR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field PLLNODIV" ]
    pub struct PllnodivR {
        bits: u8,
    }
    impl PllnodivR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _SwW<'a> {
        register: &'a mut W,
    }
    impl<'a> _SwW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _HpreW<'a> {
        register: &'a mut W,
    }
    impl<'a> _HpreW<'a> {
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
    pub struct _PpreW<'a> {
        register: &'a mut W,
    }
    impl<'a> _PpreW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _AdcpreW<'a> {
        register: &'a mut W,
    }
    impl<'a> _AdcpreW<'a> {
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
    pub struct _PllsrcW<'a> {
        register: &'a mut W,
    }
    impl<'a> _PllsrcW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 15;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PllxtpreW<'a> {
        register: &'a mut W,
    }
    impl<'a> _PllxtpreW<'a> {
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
    pub struct _PllmulW<'a> {
        register: &'a mut W,
    }
    impl<'a> _PllmulW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 15;
            const OFFSET: u8 = 18;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _McoW<'a> {
        register: &'a mut W,
    }
    impl<'a> _McoW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 7;
            const OFFSET: u8 = 24;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _McopreW<'a> {
        register: &'a mut W,
    }
    impl<'a> _McopreW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 7;
            const OFFSET: u8 = 28;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PllnodivW<'a> {
        register: &'a mut W,
    }
    impl<'a> _PllnodivW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 31;
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
        fn _sw(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 0:1 - System clock Switch" ]
        pub fn sw(&self) -> SwR {
            SwR { bits: self._sw() }
        }
        fn _sws(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 2:3 - System Clock Switch Status" ]
        pub fn sws(&self) -> SwsR {
            SwsR { bits: self._sws() }
        }
        fn _hpre(&self) -> u8 {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 4:7 - AHB prescaler" ]
        pub fn hpre(&self) -> HpreR {
            HpreR { bits: self._hpre() }
        }
        fn _ppre(&self) -> u8 {
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 8:10 - APB Low speed prescaler (APB1)" ]
        pub fn ppre(&self) -> PpreR {
            PpreR { bits: self._ppre() }
        }
        fn _adcpre(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 14 - ADC prescaler" ]
        pub fn adcpre(&self) -> AdcpreR {
            AdcpreR { bits: self._adcpre() }
        }
        fn _pllsrc(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 15:16 - PLL input clock source" ]
        pub fn pllsrc(&self) -> PllsrcR {
            PllsrcR { bits: self._pllsrc() }
        }
        fn _pllxtpre(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 17 - HSE divider for PLL entry" ]
        pub fn pllxtpre(&self) -> PllxtpreR {
            PllxtpreR { bits: self._pllxtpre() }
        }
        fn _pllmul(&self) -> u8 {
            const MASK: u8 = 15;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 18:21 - PLL Multiplication Factor" ]
        pub fn pllmul(&self) -> PllmulR {
            PllmulR { bits: self._pllmul() }
        }
        fn _mco(&self) -> u8 {
            const MASK: u8 = 7;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 24:26 - Microcontroller clock output" ]
        pub fn mco(&self) -> McoR {
            McoR { bits: self._mco() }
        }
        fn _mcopre(&self) -> u8 {
            const MASK: u8 = 7;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 28:30 - Microcontroller Clock Output Prescaler" ]
        pub fn mcopre(&self) -> McopreR {
            McopreR { bits: self._mcopre() }
        }
        fn _pllnodiv(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 31 - PLL clock not divided for MCO" ]
        pub fn pllnodiv(&self) -> PllnodivR {
            PllnodivR { bits: self._pllnodiv() }
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
        # [ doc = "Bits 0:1 - System clock Switch" ]
        pub fn sw(&mut self) -> _SwW {
            _SwW { register: self }
        }
        # [ doc = "Bits 4:7 - AHB prescaler" ]
        pub fn hpre(&mut self) -> _HpreW {
            _HpreW { register: self }
        }
        # [ doc = "Bits 8:10 - APB Low speed prescaler (APB1)" ]
        pub fn ppre(&mut self) -> _PpreW {
            _PpreW { register: self }
        }
        # [ doc = "Bit 14 - ADC prescaler" ]
        pub fn adcpre(&mut self) -> _AdcpreW {
            _AdcpreW { register: self }
        }
        # [ doc = "Bits 15:16 - PLL input clock source" ]
        pub fn pllsrc(&mut self) -> _PllsrcW {
            _PllsrcW { register: self }
        }
        # [ doc = "Bit 17 - HSE divider for PLL entry" ]
        pub fn pllxtpre(&mut self) -> _PllxtpreW {
            _PllxtpreW { register: self }
        }
        # [ doc = "Bits 18:21 - PLL Multiplication Factor" ]
        pub fn pllmul(&mut self) -> _PllmulW {
            _PllmulW { register: self }
        }
        # [ doc = "Bits 24:26 - Microcontroller clock output" ]
        pub fn mco(&mut self) -> _McoW {
            _McoW { register: self }
        }
        # [ doc = "Bits 28:30 - Microcontroller Clock Output Prescaler" ]
        pub fn mcopre(&mut self) -> _McopreW {
            _McopreW { register: self }
        }
        # [ doc = "Bit 31 - PLL clock not divided for MCO" ]
        pub fn pllnodiv(&mut self) -> _PllnodivW {
            _PllnodivW { register: self }
        }
    }
}

# [ doc = "Clock interrupt register (RCC_CIR)" ]
# [ repr ( C ) ]
pub struct Cir {
    register: ::volatile_register::RW<u32>,
}

# [ doc = "Clock interrupt register (RCC_CIR)" ]
pub mod cir {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::Cir {
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
    # [ doc = "Value of the field LSIRDYF" ]
    pub struct LsirdyfR {
        bits: u8,
    }
    impl LsirdyfR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field LSERDYF" ]
    pub struct LserdyfR {
        bits: u8,
    }
    impl LserdyfR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field HSIRDYF" ]
    pub struct HsirdyfR {
        bits: u8,
    }
    impl HsirdyfR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field HSERDYF" ]
    pub struct HserdyfR {
        bits: u8,
    }
    impl HserdyfR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field PLLRDYF" ]
    pub struct PllrdyfR {
        bits: u8,
    }
    impl PllrdyfR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field HSI14RDYF" ]
    pub struct Hsi14rdyfR {
        bits: u8,
    }
    impl Hsi14rdyfR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field HSI48RDYF" ]
    pub struct Hsi48rdyfR {
        bits: u8,
    }
    impl Hsi48rdyfR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field CSSF" ]
    pub struct CssfR {
        bits: u8,
    }
    impl CssfR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field LSIRDYIE" ]
    pub struct LsirdyieR {
        bits: u8,
    }
    impl LsirdyieR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field LSERDYIE" ]
    pub struct LserdyieR {
        bits: u8,
    }
    impl LserdyieR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field HSIRDYIE" ]
    pub struct HsirdyieR {
        bits: u8,
    }
    impl HsirdyieR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field HSERDYIE" ]
    pub struct HserdyieR {
        bits: u8,
    }
    impl HserdyieR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field PLLRDYIE" ]
    pub struct PllrdyieR {
        bits: u8,
    }
    impl PllrdyieR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field HSI14RDYE" ]
    pub struct Hsi14rdyeR {
        bits: u8,
    }
    impl Hsi14rdyeR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field HSI48RDYIE" ]
    pub struct Hsi48rdyieR {
        bits: u8,
    }
    impl Hsi48rdyieR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _LsirdyieW<'a> {
        register: &'a mut W,
    }
    impl<'a> _LsirdyieW<'a> {
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
    pub struct _LserdyieW<'a> {
        register: &'a mut W,
    }
    impl<'a> _LserdyieW<'a> {
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
    pub struct _HsirdyieW<'a> {
        register: &'a mut W,
    }
    impl<'a> _HsirdyieW<'a> {
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
    pub struct _HserdyieW<'a> {
        register: &'a mut W,
    }
    impl<'a> _HserdyieW<'a> {
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
    pub struct _PllrdyieW<'a> {
        register: &'a mut W,
    }
    impl<'a> _PllrdyieW<'a> {
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
    pub struct _Hsi14rdyeW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Hsi14rdyeW<'a> {
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
    pub struct _Hsi48rdyieW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Hsi48rdyieW<'a> {
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
    pub struct _LsirdycW<'a> {
        register: &'a mut W,
    }
    impl<'a> _LsirdycW<'a> {
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
    pub struct _LserdycW<'a> {
        register: &'a mut W,
    }
    impl<'a> _LserdycW<'a> {
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
    pub struct _HsirdycW<'a> {
        register: &'a mut W,
    }
    impl<'a> _HsirdycW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 18;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _HserdycW<'a> {
        register: &'a mut W,
    }
    impl<'a> _HserdycW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 19;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PllrdycW<'a> {
        register: &'a mut W,
    }
    impl<'a> _PllrdycW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 20;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Hsi14rdycW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Hsi14rdycW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 21;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Hsi48rdycW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Hsi48rdycW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 22;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _CsscW<'a> {
        register: &'a mut W,
    }
    impl<'a> _CsscW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 23;
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
        fn _lsirdyf(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 0 - LSI Ready Interrupt flag" ]
        pub fn lsirdyf(&self) -> LsirdyfR {
            LsirdyfR { bits: self._lsirdyf() }
        }
        fn _lserdyf(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 1 - LSE Ready Interrupt flag" ]
        pub fn lserdyf(&self) -> LserdyfR {
            LserdyfR { bits: self._lserdyf() }
        }
        fn _hsirdyf(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 2 - HSI Ready Interrupt flag" ]
        pub fn hsirdyf(&self) -> HsirdyfR {
            HsirdyfR { bits: self._hsirdyf() }
        }
        fn _hserdyf(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 3 - HSE Ready Interrupt flag" ]
        pub fn hserdyf(&self) -> HserdyfR {
            HserdyfR { bits: self._hserdyf() }
        }
        fn _pllrdyf(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 4 - PLL Ready Interrupt flag" ]
        pub fn pllrdyf(&self) -> PllrdyfR {
            PllrdyfR { bits: self._pllrdyf() }
        }
        fn _hsi14rdyf(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 5 - HSI14 ready interrupt flag" ]
        pub fn hsi14rdyf(&self) -> Hsi14rdyfR {
            Hsi14rdyfR { bits: self._hsi14rdyf() }
        }
        fn _hsi48rdyf(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 6 - HSI48 ready interrupt flag" ]
        pub fn hsi48rdyf(&self) -> Hsi48rdyfR {
            Hsi48rdyfR { bits: self._hsi48rdyf() }
        }
        fn _cssf(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 7 - Clock Security System Interrupt flag" ]
        pub fn cssf(&self) -> CssfR {
            CssfR { bits: self._cssf() }
        }
        fn _lsirdyie(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 8 - LSI Ready Interrupt Enable" ]
        pub fn lsirdyie(&self) -> LsirdyieR {
            LsirdyieR { bits: self._lsirdyie() }
        }
        fn _lserdyie(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 9 - LSE Ready Interrupt Enable" ]
        pub fn lserdyie(&self) -> LserdyieR {
            LserdyieR { bits: self._lserdyie() }
        }
        fn _hsirdyie(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 10 - HSI Ready Interrupt Enable" ]
        pub fn hsirdyie(&self) -> HsirdyieR {
            HsirdyieR { bits: self._hsirdyie() }
        }
        fn _hserdyie(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 11 - HSE Ready Interrupt Enable" ]
        pub fn hserdyie(&self) -> HserdyieR {
            HserdyieR { bits: self._hserdyie() }
        }
        fn _pllrdyie(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 12 - PLL Ready Interrupt Enable" ]
        pub fn pllrdyie(&self) -> PllrdyieR {
            PllrdyieR { bits: self._pllrdyie() }
        }
        fn _hsi14rdye(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 13 - HSI14 ready interrupt enable" ]
        pub fn hsi14rdye(&self) -> Hsi14rdyeR {
            Hsi14rdyeR { bits: self._hsi14rdye() }
        }
        fn _hsi48rdyie(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 14 - HSI48 ready interrupt enable" ]
        pub fn hsi48rdyie(&self) -> Hsi48rdyieR {
            Hsi48rdyieR { bits: self._hsi48rdyie() }
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
        # [ doc = "Bit 8 - LSI Ready Interrupt Enable" ]
        pub fn lsirdyie(&mut self) -> _LsirdyieW {
            _LsirdyieW { register: self }
        }
        # [ doc = "Bit 9 - LSE Ready Interrupt Enable" ]
        pub fn lserdyie(&mut self) -> _LserdyieW {
            _LserdyieW { register: self }
        }
        # [ doc = "Bit 10 - HSI Ready Interrupt Enable" ]
        pub fn hsirdyie(&mut self) -> _HsirdyieW {
            _HsirdyieW { register: self }
        }
        # [ doc = "Bit 11 - HSE Ready Interrupt Enable" ]
        pub fn hserdyie(&mut self) -> _HserdyieW {
            _HserdyieW { register: self }
        }
        # [ doc = "Bit 12 - PLL Ready Interrupt Enable" ]
        pub fn pllrdyie(&mut self) -> _PllrdyieW {
            _PllrdyieW { register: self }
        }
        # [ doc = "Bit 13 - HSI14 ready interrupt enable" ]
        pub fn hsi14rdye(&mut self) -> _Hsi14rdyeW {
            _Hsi14rdyeW { register: self }
        }
        # [ doc = "Bit 14 - HSI48 ready interrupt enable" ]
        pub fn hsi48rdyie(&mut self) -> _Hsi48rdyieW {
            _Hsi48rdyieW { register: self }
        }
        # [ doc = "Bit 16 - LSI Ready Interrupt Clear" ]
        pub fn lsirdyc(&mut self) -> _LsirdycW {
            _LsirdycW { register: self }
        }
        # [ doc = "Bit 17 - LSE Ready Interrupt Clear" ]
        pub fn lserdyc(&mut self) -> _LserdycW {
            _LserdycW { register: self }
        }
        # [ doc = "Bit 18 - HSI Ready Interrupt Clear" ]
        pub fn hsirdyc(&mut self) -> _HsirdycW {
            _HsirdycW { register: self }
        }
        # [ doc = "Bit 19 - HSE Ready Interrupt Clear" ]
        pub fn hserdyc(&mut self) -> _HserdycW {
            _HserdycW { register: self }
        }
        # [ doc = "Bit 20 - PLL Ready Interrupt Clear" ]
        pub fn pllrdyc(&mut self) -> _PllrdycW {
            _PllrdycW { register: self }
        }
        # [ doc = "Bit 21 - HSI 14 MHz Ready Interrupt Clear" ]
        pub fn hsi14rdyc(&mut self) -> _Hsi14rdycW {
            _Hsi14rdycW { register: self }
        }
        # [ doc = "Bit 22 - HSI48 Ready Interrupt Clear" ]
        pub fn hsi48rdyc(&mut self) -> _Hsi48rdycW {
            _Hsi48rdycW { register: self }
        }
        # [ doc = "Bit 23 - Clock security system interrupt clear" ]
        pub fn cssc(&mut self) -> _CsscW {
            _CsscW { register: self }
        }
    }
}

# [ doc = "APB2 peripheral reset register (RCC_APB2RSTR)" ]
# [ repr ( C ) ]
pub struct Apb2rstr {
    register: ::volatile_register::RW<u32>,
}

# [ doc = "APB2 peripheral reset register (RCC_APB2RSTR)" ]
pub mod apb2rstr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::Apb2rstr {
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
    # [ doc = "Value of the field SYSCFGRST" ]
    pub struct SyscfgrstR {
        bits: u8,
    }
    impl SyscfgrstR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field ADCRST" ]
    pub struct AdcrstR {
        bits: u8,
    }
    impl AdcrstR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field TIM1RST" ]
    pub struct Tim1rstR {
        bits: u8,
    }
    impl Tim1rstR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field SPI1RST" ]
    pub struct Spi1rstR {
        bits: u8,
    }
    impl Spi1rstR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field USART1RST" ]
    pub struct Usart1rstR {
        bits: u8,
    }
    impl Usart1rstR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field TIM15RST" ]
    pub struct Tim15rstR {
        bits: u8,
    }
    impl Tim15rstR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field TIM16RST" ]
    pub struct Tim16rstR {
        bits: u8,
    }
    impl Tim16rstR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field TIM17RST" ]
    pub struct Tim17rstR {
        bits: u8,
    }
    impl Tim17rstR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field DBGMCURST" ]
    pub struct DbgmcurstR {
        bits: u8,
    }
    impl DbgmcurstR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _SyscfgrstW<'a> {
        register: &'a mut W,
    }
    impl<'a> _SyscfgrstW<'a> {
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
    pub struct _AdcrstW<'a> {
        register: &'a mut W,
    }
    impl<'a> _AdcrstW<'a> {
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
    pub struct _Tim1rstW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Tim1rstW<'a> {
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
    pub struct _Spi1rstW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Spi1rstW<'a> {
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
    pub struct _Usart1rstW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Usart1rstW<'a> {
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
    pub struct _Tim15rstW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Tim15rstW<'a> {
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
    pub struct _Tim16rstW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Tim16rstW<'a> {
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
    pub struct _Tim17rstW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Tim17rstW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 18;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _DbgmcurstW<'a> {
        register: &'a mut W,
    }
    impl<'a> _DbgmcurstW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 22;
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
        fn _syscfgrst(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 0 - SYSCFG and COMP reset" ]
        pub fn syscfgrst(&self) -> SyscfgrstR {
            SyscfgrstR { bits: self._syscfgrst() }
        }
        fn _adcrst(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 9 - ADC interface reset" ]
        pub fn adcrst(&self) -> AdcrstR {
            AdcrstR { bits: self._adcrst() }
        }
        fn _tim1rst(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 11 - TIM1 timer reset" ]
        pub fn tim1rst(&self) -> Tim1rstR {
            Tim1rstR { bits: self._tim1rst() }
        }
        fn _spi1rst(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 12 - SPI 1 reset" ]
        pub fn spi1rst(&self) -> Spi1rstR {
            Spi1rstR { bits: self._spi1rst() }
        }
        fn _usart1rst(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 14 - USART1 reset" ]
        pub fn usart1rst(&self) -> Usart1rstR {
            Usart1rstR { bits: self._usart1rst() }
        }
        fn _tim15rst(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 16 - TIM15 timer reset" ]
        pub fn tim15rst(&self) -> Tim15rstR {
            Tim15rstR { bits: self._tim15rst() }
        }
        fn _tim16rst(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 17 - TIM16 timer reset" ]
        pub fn tim16rst(&self) -> Tim16rstR {
            Tim16rstR { bits: self._tim16rst() }
        }
        fn _tim17rst(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 18 - TIM17 timer reset" ]
        pub fn tim17rst(&self) -> Tim17rstR {
            Tim17rstR { bits: self._tim17rst() }
        }
        fn _dbgmcurst(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 22 - Debug MCU reset" ]
        pub fn dbgmcurst(&self) -> DbgmcurstR {
            DbgmcurstR { bits: self._dbgmcurst() }
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
        # [ doc = "Bit 0 - SYSCFG and COMP reset" ]
        pub fn syscfgrst(&mut self) -> _SyscfgrstW {
            _SyscfgrstW { register: self }
        }
        # [ doc = "Bit 9 - ADC interface reset" ]
        pub fn adcrst(&mut self) -> _AdcrstW {
            _AdcrstW { register: self }
        }
        # [ doc = "Bit 11 - TIM1 timer reset" ]
        pub fn tim1rst(&mut self) -> _Tim1rstW {
            _Tim1rstW { register: self }
        }
        # [ doc = "Bit 12 - SPI 1 reset" ]
        pub fn spi1rst(&mut self) -> _Spi1rstW {
            _Spi1rstW { register: self }
        }
        # [ doc = "Bit 14 - USART1 reset" ]
        pub fn usart1rst(&mut self) -> _Usart1rstW {
            _Usart1rstW { register: self }
        }
        # [ doc = "Bit 16 - TIM15 timer reset" ]
        pub fn tim15rst(&mut self) -> _Tim15rstW {
            _Tim15rstW { register: self }
        }
        # [ doc = "Bit 17 - TIM16 timer reset" ]
        pub fn tim16rst(&mut self) -> _Tim16rstW {
            _Tim16rstW { register: self }
        }
        # [ doc = "Bit 18 - TIM17 timer reset" ]
        pub fn tim17rst(&mut self) -> _Tim17rstW {
            _Tim17rstW { register: self }
        }
        # [ doc = "Bit 22 - Debug MCU reset" ]
        pub fn dbgmcurst(&mut self) -> _DbgmcurstW {
            _DbgmcurstW { register: self }
        }
    }
}

# [ doc = "APB1 peripheral reset register (RCC_APB1RSTR)" ]
# [ repr ( C ) ]
pub struct Apb1rstr {
    register: ::volatile_register::RW<u32>,
}

# [ doc = "APB1 peripheral reset register (RCC_APB1RSTR)" ]
pub mod apb1rstr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::Apb1rstr {
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
    # [ doc = "Value of the field TIM2RST" ]
    pub struct Tim2rstR {
        bits: u8,
    }
    impl Tim2rstR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field TIM3RST" ]
    pub struct Tim3rstR {
        bits: u8,
    }
    impl Tim3rstR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field TIM6RST" ]
    pub struct Tim6rstR {
        bits: u8,
    }
    impl Tim6rstR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field TIM7RST" ]
    pub struct Tim7rstR {
        bits: u8,
    }
    impl Tim7rstR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field TIM14RST" ]
    pub struct Tim14rstR {
        bits: u8,
    }
    impl Tim14rstR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field WWDGRST" ]
    pub struct WwdgrstR {
        bits: u8,
    }
    impl WwdgrstR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field SPI2RST" ]
    pub struct Spi2rstR {
        bits: u8,
    }
    impl Spi2rstR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field USART2RST" ]
    pub struct Usart2rstR {
        bits: u8,
    }
    impl Usart2rstR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field USART3RST" ]
    pub struct Usart3rstR {
        bits: u8,
    }
    impl Usart3rstR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field USART4RST" ]
    pub struct Usart4rstR {
        bits: u8,
    }
    impl Usart4rstR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field I2C1RST" ]
    pub struct I2c1rstR {
        bits: u8,
    }
    impl I2c1rstR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field I2C2RST" ]
    pub struct I2c2rstR {
        bits: u8,
    }
    impl I2c2rstR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field USBRST" ]
    pub struct UsbrstR {
        bits: u8,
    }
    impl UsbrstR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field CANRST" ]
    pub struct CanrstR {
        bits: u8,
    }
    impl CanrstR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field CRSRST" ]
    pub struct CrsrstR {
        bits: u8,
    }
    impl CrsrstR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field PWRRST" ]
    pub struct PwrrstR {
        bits: u8,
    }
    impl PwrrstR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field DACRST" ]
    pub struct DacrstR {
        bits: u8,
    }
    impl DacrstR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field CECRST" ]
    pub struct CecrstR {
        bits: u8,
    }
    impl CecrstR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Tim2rstW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Tim2rstW<'a> {
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
    pub struct _Tim3rstW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Tim3rstW<'a> {
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
    pub struct _Tim6rstW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Tim6rstW<'a> {
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
    pub struct _Tim7rstW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Tim7rstW<'a> {
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
    pub struct _Tim14rstW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Tim14rstW<'a> {
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
    pub struct _WwdgrstW<'a> {
        register: &'a mut W,
    }
    impl<'a> _WwdgrstW<'a> {
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
    pub struct _Spi2rstW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Spi2rstW<'a> {
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
    pub struct _Usart2rstW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Usart2rstW<'a> {
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
    pub struct _Usart3rstW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Usart3rstW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 18;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Usart4rstW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Usart4rstW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 19;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _I2c1rstW<'a> {
        register: &'a mut W,
    }
    impl<'a> _I2c1rstW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 21;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _I2c2rstW<'a> {
        register: &'a mut W,
    }
    impl<'a> _I2c2rstW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 22;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _UsbrstW<'a> {
        register: &'a mut W,
    }
    impl<'a> _UsbrstW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 23;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _CanrstW<'a> {
        register: &'a mut W,
    }
    impl<'a> _CanrstW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 25;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _CrsrstW<'a> {
        register: &'a mut W,
    }
    impl<'a> _CrsrstW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 27;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PwrrstW<'a> {
        register: &'a mut W,
    }
    impl<'a> _PwrrstW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 28;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _DacrstW<'a> {
        register: &'a mut W,
    }
    impl<'a> _DacrstW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 29;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _CecrstW<'a> {
        register: &'a mut W,
    }
    impl<'a> _CecrstW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 30;
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
        fn _tim2rst(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 0 - Timer 2 reset" ]
        pub fn tim2rst(&self) -> Tim2rstR {
            Tim2rstR { bits: self._tim2rst() }
        }
        fn _tim3rst(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 1 - Timer 3 reset" ]
        pub fn tim3rst(&self) -> Tim3rstR {
            Tim3rstR { bits: self._tim3rst() }
        }
        fn _tim6rst(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 4 - Timer 6 reset" ]
        pub fn tim6rst(&self) -> Tim6rstR {
            Tim6rstR { bits: self._tim6rst() }
        }
        fn _tim7rst(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 5 - TIM7 timer reset" ]
        pub fn tim7rst(&self) -> Tim7rstR {
            Tim7rstR { bits: self._tim7rst() }
        }
        fn _tim14rst(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 8 - Timer 14 reset" ]
        pub fn tim14rst(&self) -> Tim14rstR {
            Tim14rstR { bits: self._tim14rst() }
        }
        fn _wwdgrst(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 11 - Window watchdog reset" ]
        pub fn wwdgrst(&self) -> WwdgrstR {
            WwdgrstR { bits: self._wwdgrst() }
        }
        fn _spi2rst(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 14 - SPI2 reset" ]
        pub fn spi2rst(&self) -> Spi2rstR {
            Spi2rstR { bits: self._spi2rst() }
        }
        fn _usart2rst(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 17 - USART 2 reset" ]
        pub fn usart2rst(&self) -> Usart2rstR {
            Usart2rstR { bits: self._usart2rst() }
        }
        fn _usart3rst(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 18 - USART3 reset" ]
        pub fn usart3rst(&self) -> Usart3rstR {
            Usart3rstR { bits: self._usart3rst() }
        }
        fn _usart4rst(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 19 - USART4 reset" ]
        pub fn usart4rst(&self) -> Usart4rstR {
            Usart4rstR { bits: self._usart4rst() }
        }
        fn _i2c1rst(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 21 - I2C1 reset" ]
        pub fn i2c1rst(&self) -> I2c1rstR {
            I2c1rstR { bits: self._i2c1rst() }
        }
        fn _i2c2rst(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 22 - I2C2 reset" ]
        pub fn i2c2rst(&self) -> I2c2rstR {
            I2c2rstR { bits: self._i2c2rst() }
        }
        fn _usbrst(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 23 - USB interface reset" ]
        pub fn usbrst(&self) -> UsbrstR {
            UsbrstR { bits: self._usbrst() }
        }
        fn _canrst(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 25 - CAN interface reset" ]
        pub fn canrst(&self) -> CanrstR {
            CanrstR { bits: self._canrst() }
        }
        fn _crsrst(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 27 - Clock Recovery System interface reset" ]
        pub fn crsrst(&self) -> CrsrstR {
            CrsrstR { bits: self._crsrst() }
        }
        fn _pwrrst(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 28 - Power interface reset" ]
        pub fn pwrrst(&self) -> PwrrstR {
            PwrrstR { bits: self._pwrrst() }
        }
        fn _dacrst(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 29 - DAC interface reset" ]
        pub fn dacrst(&self) -> DacrstR {
            DacrstR { bits: self._dacrst() }
        }
        fn _cecrst(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 30 - HDMI CEC reset" ]
        pub fn cecrst(&self) -> CecrstR {
            CecrstR { bits: self._cecrst() }
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
        # [ doc = "Bit 0 - Timer 2 reset" ]
        pub fn tim2rst(&mut self) -> _Tim2rstW {
            _Tim2rstW { register: self }
        }
        # [ doc = "Bit 1 - Timer 3 reset" ]
        pub fn tim3rst(&mut self) -> _Tim3rstW {
            _Tim3rstW { register: self }
        }
        # [ doc = "Bit 4 - Timer 6 reset" ]
        pub fn tim6rst(&mut self) -> _Tim6rstW {
            _Tim6rstW { register: self }
        }
        # [ doc = "Bit 5 - TIM7 timer reset" ]
        pub fn tim7rst(&mut self) -> _Tim7rstW {
            _Tim7rstW { register: self }
        }
        # [ doc = "Bit 8 - Timer 14 reset" ]
        pub fn tim14rst(&mut self) -> _Tim14rstW {
            _Tim14rstW { register: self }
        }
        # [ doc = "Bit 11 - Window watchdog reset" ]
        pub fn wwdgrst(&mut self) -> _WwdgrstW {
            _WwdgrstW { register: self }
        }
        # [ doc = "Bit 14 - SPI2 reset" ]
        pub fn spi2rst(&mut self) -> _Spi2rstW {
            _Spi2rstW { register: self }
        }
        # [ doc = "Bit 17 - USART 2 reset" ]
        pub fn usart2rst(&mut self) -> _Usart2rstW {
            _Usart2rstW { register: self }
        }
        # [ doc = "Bit 18 - USART3 reset" ]
        pub fn usart3rst(&mut self) -> _Usart3rstW {
            _Usart3rstW { register: self }
        }
        # [ doc = "Bit 19 - USART4 reset" ]
        pub fn usart4rst(&mut self) -> _Usart4rstW {
            _Usart4rstW { register: self }
        }
        # [ doc = "Bit 21 - I2C1 reset" ]
        pub fn i2c1rst(&mut self) -> _I2c1rstW {
            _I2c1rstW { register: self }
        }
        # [ doc = "Bit 22 - I2C2 reset" ]
        pub fn i2c2rst(&mut self) -> _I2c2rstW {
            _I2c2rstW { register: self }
        }
        # [ doc = "Bit 23 - USB interface reset" ]
        pub fn usbrst(&mut self) -> _UsbrstW {
            _UsbrstW { register: self }
        }
        # [ doc = "Bit 25 - CAN interface reset" ]
        pub fn canrst(&mut self) -> _CanrstW {
            _CanrstW { register: self }
        }
        # [ doc = "Bit 27 - Clock Recovery System interface reset" ]
        pub fn crsrst(&mut self) -> _CrsrstW {
            _CrsrstW { register: self }
        }
        # [ doc = "Bit 28 - Power interface reset" ]
        pub fn pwrrst(&mut self) -> _PwrrstW {
            _PwrrstW { register: self }
        }
        # [ doc = "Bit 29 - DAC interface reset" ]
        pub fn dacrst(&mut self) -> _DacrstW {
            _DacrstW { register: self }
        }
        # [ doc = "Bit 30 - HDMI CEC reset" ]
        pub fn cecrst(&mut self) -> _CecrstW {
            _CecrstW { register: self }
        }
    }
}

# [ doc = "AHB Peripheral Clock enable register (RCC_AHBENR)" ]
# [ repr ( C ) ]
pub struct Ahbenr {
    register: ::volatile_register::RW<u32>,
}

# [ doc = "AHB Peripheral Clock enable register (RCC_AHBENR)" ]
pub mod ahbenr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::Ahbenr {
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
    # [ doc = "Value of the field DMAEN" ]
    pub struct DmaenR {
        bits: u8,
    }
    impl DmaenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field SRAMEN" ]
    pub struct SramenR {
        bits: u8,
    }
    impl SramenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field FLITFEN" ]
    pub struct FlitfenR {
        bits: u8,
    }
    impl FlitfenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field CRCEN" ]
    pub struct CrcenR {
        bits: u8,
    }
    impl CrcenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field IOPAEN" ]
    pub struct IopaenR {
        bits: u8,
    }
    impl IopaenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field IOPBEN" ]
    pub struct IopbenR {
        bits: u8,
    }
    impl IopbenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field IOPCEN" ]
    pub struct IopcenR {
        bits: u8,
    }
    impl IopcenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field IOPDEN" ]
    pub struct IopdenR {
        bits: u8,
    }
    impl IopdenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field IOPFEN" ]
    pub struct IopfenR {
        bits: u8,
    }
    impl IopfenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field TSCEN" ]
    pub struct TscenR {
        bits: u8,
    }
    impl TscenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _DmaenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _DmaenW<'a> {
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
    pub struct _SramenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _SramenW<'a> {
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
    pub struct _FlitfenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _FlitfenW<'a> {
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
    pub struct _CrcenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _CrcenW<'a> {
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
    pub struct _IopaenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _IopaenW<'a> {
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
    pub struct _IopbenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _IopbenW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 18;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _IopcenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _IopcenW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 19;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _IopdenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _IopdenW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 20;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _IopfenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _IopfenW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 22;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _TscenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _TscenW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 24;
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
        fn _dmaen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 0 - DMA1 clock enable" ]
        pub fn dmaen(&self) -> DmaenR {
            DmaenR { bits: self._dmaen() }
        }
        fn _sramen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 2 - SRAM interface clock enable" ]
        pub fn sramen(&self) -> SramenR {
            SramenR { bits: self._sramen() }
        }
        fn _flitfen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 4 - FLITF clock enable" ]
        pub fn flitfen(&self) -> FlitfenR {
            FlitfenR { bits: self._flitfen() }
        }
        fn _crcen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 6 - CRC clock enable" ]
        pub fn crcen(&self) -> CrcenR {
            CrcenR { bits: self._crcen() }
        }
        fn _iopaen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 17 - I/O port A clock enable" ]
        pub fn iopaen(&self) -> IopaenR {
            IopaenR { bits: self._iopaen() }
        }
        fn _iopben(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 18 - I/O port B clock enable" ]
        pub fn iopben(&self) -> IopbenR {
            IopbenR { bits: self._iopben() }
        }
        fn _iopcen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 19 - I/O port C clock enable" ]
        pub fn iopcen(&self) -> IopcenR {
            IopcenR { bits: self._iopcen() }
        }
        fn _iopden(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 20 - I/O port D clock enable" ]
        pub fn iopden(&self) -> IopdenR {
            IopdenR { bits: self._iopden() }
        }
        fn _iopfen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 22 - I/O port F clock enable" ]
        pub fn iopfen(&self) -> IopfenR {
            IopfenR { bits: self._iopfen() }
        }
        fn _tscen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 24 - Touch sensing controller clock enable" ]
        pub fn tscen(&self) -> TscenR {
            TscenR { bits: self._tscen() }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        pub fn reset_value() -> W {
            W { bits: 20 }
        }
        # [ doc = r" Writes raw `bits` to the register" ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bit 0 - DMA1 clock enable" ]
        pub fn dmaen(&mut self) -> _DmaenW {
            _DmaenW { register: self }
        }
        # [ doc = "Bit 2 - SRAM interface clock enable" ]
        pub fn sramen(&mut self) -> _SramenW {
            _SramenW { register: self }
        }
        # [ doc = "Bit 4 - FLITF clock enable" ]
        pub fn flitfen(&mut self) -> _FlitfenW {
            _FlitfenW { register: self }
        }
        # [ doc = "Bit 6 - CRC clock enable" ]
        pub fn crcen(&mut self) -> _CrcenW {
            _CrcenW { register: self }
        }
        # [ doc = "Bit 17 - I/O port A clock enable" ]
        pub fn iopaen(&mut self) -> _IopaenW {
            _IopaenW { register: self }
        }
        # [ doc = "Bit 18 - I/O port B clock enable" ]
        pub fn iopben(&mut self) -> _IopbenW {
            _IopbenW { register: self }
        }
        # [ doc = "Bit 19 - I/O port C clock enable" ]
        pub fn iopcen(&mut self) -> _IopcenW {
            _IopcenW { register: self }
        }
        # [ doc = "Bit 20 - I/O port D clock enable" ]
        pub fn iopden(&mut self) -> _IopdenW {
            _IopdenW { register: self }
        }
        # [ doc = "Bit 22 - I/O port F clock enable" ]
        pub fn iopfen(&mut self) -> _IopfenW {
            _IopfenW { register: self }
        }
        # [ doc = "Bit 24 - Touch sensing controller clock enable" ]
        pub fn tscen(&mut self) -> _TscenW {
            _TscenW { register: self }
        }
    }
}

# [ doc = "APB2 peripheral clock enable register (RCC_APB2ENR)" ]
# [ repr ( C ) ]
pub struct Apb2enr {
    register: ::volatile_register::RW<u32>,
}

# [ doc = "APB2 peripheral clock enable register (RCC_APB2ENR)" ]
pub mod apb2enr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::Apb2enr {
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
    # [ doc = "Value of the field SYSCFGEN" ]
    pub struct SyscfgenR {
        bits: u8,
    }
    impl SyscfgenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field ADCEN" ]
    pub struct AdcenR {
        bits: u8,
    }
    impl AdcenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field TIM1EN" ]
    pub struct Tim1enR {
        bits: u8,
    }
    impl Tim1enR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field SPI1EN" ]
    pub struct Spi1enR {
        bits: u8,
    }
    impl Spi1enR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field USART1EN" ]
    pub struct Usart1enR {
        bits: u8,
    }
    impl Usart1enR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field TIM15EN" ]
    pub struct Tim15enR {
        bits: u8,
    }
    impl Tim15enR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field TIM16EN" ]
    pub struct Tim16enR {
        bits: u8,
    }
    impl Tim16enR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field TIM17EN" ]
    pub struct Tim17enR {
        bits: u8,
    }
    impl Tim17enR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field DBGMCUEN" ]
    pub struct DbgmcuenR {
        bits: u8,
    }
    impl DbgmcuenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _SyscfgenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _SyscfgenW<'a> {
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
    pub struct _AdcenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _AdcenW<'a> {
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
    pub struct _Tim1enW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Tim1enW<'a> {
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
    pub struct _Spi1enW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Spi1enW<'a> {
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
    pub struct _Usart1enW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Usart1enW<'a> {
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
    pub struct _Tim15enW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Tim15enW<'a> {
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
    pub struct _Tim16enW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Tim16enW<'a> {
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
    pub struct _Tim17enW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Tim17enW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 18;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _DbgmcuenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _DbgmcuenW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 22;
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
        fn _syscfgen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 0 - SYSCFG clock enable" ]
        pub fn syscfgen(&self) -> SyscfgenR {
            SyscfgenR { bits: self._syscfgen() }
        }
        fn _adcen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 9 - ADC 1 interface clock enable" ]
        pub fn adcen(&self) -> AdcenR {
            AdcenR { bits: self._adcen() }
        }
        fn _tim1en(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 11 - TIM1 Timer clock enable" ]
        pub fn tim1en(&self) -> Tim1enR {
            Tim1enR { bits: self._tim1en() }
        }
        fn _spi1en(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 12 - SPI 1 clock enable" ]
        pub fn spi1en(&self) -> Spi1enR {
            Spi1enR { bits: self._spi1en() }
        }
        fn _usart1en(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 14 - USART1 clock enable" ]
        pub fn usart1en(&self) -> Usart1enR {
            Usart1enR { bits: self._usart1en() }
        }
        fn _tim15en(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 16 - TIM15 timer clock enable" ]
        pub fn tim15en(&self) -> Tim15enR {
            Tim15enR { bits: self._tim15en() }
        }
        fn _tim16en(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 17 - TIM16 timer clock enable" ]
        pub fn tim16en(&self) -> Tim16enR {
            Tim16enR { bits: self._tim16en() }
        }
        fn _tim17en(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 18 - TIM17 timer clock enable" ]
        pub fn tim17en(&self) -> Tim17enR {
            Tim17enR { bits: self._tim17en() }
        }
        fn _dbgmcuen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 22 - MCU debug module clock enable" ]
        pub fn dbgmcuen(&self) -> DbgmcuenR {
            DbgmcuenR { bits: self._dbgmcuen() }
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
        # [ doc = "Bit 0 - SYSCFG clock enable" ]
        pub fn syscfgen(&mut self) -> _SyscfgenW {
            _SyscfgenW { register: self }
        }
        # [ doc = "Bit 9 - ADC 1 interface clock enable" ]
        pub fn adcen(&mut self) -> _AdcenW {
            _AdcenW { register: self }
        }
        # [ doc = "Bit 11 - TIM1 Timer clock enable" ]
        pub fn tim1en(&mut self) -> _Tim1enW {
            _Tim1enW { register: self }
        }
        # [ doc = "Bit 12 - SPI 1 clock enable" ]
        pub fn spi1en(&mut self) -> _Spi1enW {
            _Spi1enW { register: self }
        }
        # [ doc = "Bit 14 - USART1 clock enable" ]
        pub fn usart1en(&mut self) -> _Usart1enW {
            _Usart1enW { register: self }
        }
        # [ doc = "Bit 16 - TIM15 timer clock enable" ]
        pub fn tim15en(&mut self) -> _Tim15enW {
            _Tim15enW { register: self }
        }
        # [ doc = "Bit 17 - TIM16 timer clock enable" ]
        pub fn tim16en(&mut self) -> _Tim16enW {
            _Tim16enW { register: self }
        }
        # [ doc = "Bit 18 - TIM17 timer clock enable" ]
        pub fn tim17en(&mut self) -> _Tim17enW {
            _Tim17enW { register: self }
        }
        # [ doc = "Bit 22 - MCU debug module clock enable" ]
        pub fn dbgmcuen(&mut self) -> _DbgmcuenW {
            _DbgmcuenW { register: self }
        }
    }
}

# [ doc = "APB1 peripheral clock enable register (RCC_APB1ENR)" ]
# [ repr ( C ) ]
pub struct Apb1enr {
    register: ::volatile_register::RW<u32>,
}

# [ doc = "APB1 peripheral clock enable register (RCC_APB1ENR)" ]
pub mod apb1enr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::Apb1enr {
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
    # [ doc = "Value of the field TIM2EN" ]
    pub struct Tim2enR {
        bits: u8,
    }
    impl Tim2enR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field TIM3EN" ]
    pub struct Tim3enR {
        bits: u8,
    }
    impl Tim3enR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field TIM6EN" ]
    pub struct Tim6enR {
        bits: u8,
    }
    impl Tim6enR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field TIM7EN" ]
    pub struct Tim7enR {
        bits: u8,
    }
    impl Tim7enR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field TIM14EN" ]
    pub struct Tim14enR {
        bits: u8,
    }
    impl Tim14enR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field WWDGEN" ]
    pub struct WwdgenR {
        bits: u8,
    }
    impl WwdgenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field SPI2EN" ]
    pub struct Spi2enR {
        bits: u8,
    }
    impl Spi2enR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field USART2EN" ]
    pub struct Usart2enR {
        bits: u8,
    }
    impl Usart2enR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field USART3EN" ]
    pub struct Usart3enR {
        bits: u8,
    }
    impl Usart3enR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field USART4EN" ]
    pub struct Usart4enR {
        bits: u8,
    }
    impl Usart4enR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field I2C1EN" ]
    pub struct I2c1enR {
        bits: u8,
    }
    impl I2c1enR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field I2C2EN" ]
    pub struct I2c2enR {
        bits: u8,
    }
    impl I2c2enR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field USBRST" ]
    pub struct UsbrstR {
        bits: u8,
    }
    impl UsbrstR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field CANEN" ]
    pub struct CanenR {
        bits: u8,
    }
    impl CanenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field CRSEN" ]
    pub struct CrsenR {
        bits: u8,
    }
    impl CrsenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field PWREN" ]
    pub struct PwrenR {
        bits: u8,
    }
    impl PwrenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field DACEN" ]
    pub struct DacenR {
        bits: u8,
    }
    impl DacenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field CECEN" ]
    pub struct CecenR {
        bits: u8,
    }
    impl CecenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Tim2enW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Tim2enW<'a> {
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
    pub struct _Tim3enW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Tim3enW<'a> {
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
    pub struct _Tim6enW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Tim6enW<'a> {
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
    pub struct _Tim7enW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Tim7enW<'a> {
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
    pub struct _Tim14enW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Tim14enW<'a> {
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
    pub struct _WwdgenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _WwdgenW<'a> {
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
    pub struct _Spi2enW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Spi2enW<'a> {
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
    pub struct _Usart2enW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Usart2enW<'a> {
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
    pub struct _Usart3enW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Usart3enW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 18;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Usart4enW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Usart4enW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 19;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _I2c1enW<'a> {
        register: &'a mut W,
    }
    impl<'a> _I2c1enW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 21;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _I2c2enW<'a> {
        register: &'a mut W,
    }
    impl<'a> _I2c2enW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 22;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _UsbrstW<'a> {
        register: &'a mut W,
    }
    impl<'a> _UsbrstW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 23;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _CanenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _CanenW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 25;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _CrsenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _CrsenW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 27;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PwrenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _PwrenW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 28;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _DacenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _DacenW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 29;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _CecenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _CecenW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 30;
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
        fn _tim2en(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 0 - Timer 2 clock enable" ]
        pub fn tim2en(&self) -> Tim2enR {
            Tim2enR { bits: self._tim2en() }
        }
        fn _tim3en(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 1 - Timer 3 clock enable" ]
        pub fn tim3en(&self) -> Tim3enR {
            Tim3enR { bits: self._tim3en() }
        }
        fn _tim6en(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 4 - Timer 6 clock enable" ]
        pub fn tim6en(&self) -> Tim6enR {
            Tim6enR { bits: self._tim6en() }
        }
        fn _tim7en(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 5 - TIM7 timer clock enable" ]
        pub fn tim7en(&self) -> Tim7enR {
            Tim7enR { bits: self._tim7en() }
        }
        fn _tim14en(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 8 - Timer 14 clock enable" ]
        pub fn tim14en(&self) -> Tim14enR {
            Tim14enR { bits: self._tim14en() }
        }
        fn _wwdgen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 11 - Window watchdog clock enable" ]
        pub fn wwdgen(&self) -> WwdgenR {
            WwdgenR { bits: self._wwdgen() }
        }
        fn _spi2en(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 14 - SPI 2 clock enable" ]
        pub fn spi2en(&self) -> Spi2enR {
            Spi2enR { bits: self._spi2en() }
        }
        fn _usart2en(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 17 - USART 2 clock enable" ]
        pub fn usart2en(&self) -> Usart2enR {
            Usart2enR { bits: self._usart2en() }
        }
        fn _usart3en(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 18 - USART3 clock enable" ]
        pub fn usart3en(&self) -> Usart3enR {
            Usart3enR { bits: self._usart3en() }
        }
        fn _usart4en(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 19 - USART4 clock enable" ]
        pub fn usart4en(&self) -> Usart4enR {
            Usart4enR { bits: self._usart4en() }
        }
        fn _i2c1en(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 21 - I2C 1 clock enable" ]
        pub fn i2c1en(&self) -> I2c1enR {
            I2c1enR { bits: self._i2c1en() }
        }
        fn _i2c2en(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 22 - I2C 2 clock enable" ]
        pub fn i2c2en(&self) -> I2c2enR {
            I2c2enR { bits: self._i2c2en() }
        }
        fn _usbrst(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 23 - USB interface clock enable" ]
        pub fn usbrst(&self) -> UsbrstR {
            UsbrstR { bits: self._usbrst() }
        }
        fn _canen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 25 - CAN interface clock enable" ]
        pub fn canen(&self) -> CanenR {
            CanenR { bits: self._canen() }
        }
        fn _crsen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 27 - Clock Recovery System interface clock enable" ]
        pub fn crsen(&self) -> CrsenR {
            CrsenR { bits: self._crsen() }
        }
        fn _pwren(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 28 - Power interface clock enable" ]
        pub fn pwren(&self) -> PwrenR {
            PwrenR { bits: self._pwren() }
        }
        fn _dacen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 29 - DAC interface clock enable" ]
        pub fn dacen(&self) -> DacenR {
            DacenR { bits: self._dacen() }
        }
        fn _cecen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 30 - HDMI CEC interface clock enable" ]
        pub fn cecen(&self) -> CecenR {
            CecenR { bits: self._cecen() }
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
        # [ doc = "Bit 0 - Timer 2 clock enable" ]
        pub fn tim2en(&mut self) -> _Tim2enW {
            _Tim2enW { register: self }
        }
        # [ doc = "Bit 1 - Timer 3 clock enable" ]
        pub fn tim3en(&mut self) -> _Tim3enW {
            _Tim3enW { register: self }
        }
        # [ doc = "Bit 4 - Timer 6 clock enable" ]
        pub fn tim6en(&mut self) -> _Tim6enW {
            _Tim6enW { register: self }
        }
        # [ doc = "Bit 5 - TIM7 timer clock enable" ]
        pub fn tim7en(&mut self) -> _Tim7enW {
            _Tim7enW { register: self }
        }
        # [ doc = "Bit 8 - Timer 14 clock enable" ]
        pub fn tim14en(&mut self) -> _Tim14enW {
            _Tim14enW { register: self }
        }
        # [ doc = "Bit 11 - Window watchdog clock enable" ]
        pub fn wwdgen(&mut self) -> _WwdgenW {
            _WwdgenW { register: self }
        }
        # [ doc = "Bit 14 - SPI 2 clock enable" ]
        pub fn spi2en(&mut self) -> _Spi2enW {
            _Spi2enW { register: self }
        }
        # [ doc = "Bit 17 - USART 2 clock enable" ]
        pub fn usart2en(&mut self) -> _Usart2enW {
            _Usart2enW { register: self }
        }
        # [ doc = "Bit 18 - USART3 clock enable" ]
        pub fn usart3en(&mut self) -> _Usart3enW {
            _Usart3enW { register: self }
        }
        # [ doc = "Bit 19 - USART4 clock enable" ]
        pub fn usart4en(&mut self) -> _Usart4enW {
            _Usart4enW { register: self }
        }
        # [ doc = "Bit 21 - I2C 1 clock enable" ]
        pub fn i2c1en(&mut self) -> _I2c1enW {
            _I2c1enW { register: self }
        }
        # [ doc = "Bit 22 - I2C 2 clock enable" ]
        pub fn i2c2en(&mut self) -> _I2c2enW {
            _I2c2enW { register: self }
        }
        # [ doc = "Bit 23 - USB interface clock enable" ]
        pub fn usbrst(&mut self) -> _UsbrstW {
            _UsbrstW { register: self }
        }
        # [ doc = "Bit 25 - CAN interface clock enable" ]
        pub fn canen(&mut self) -> _CanenW {
            _CanenW { register: self }
        }
        # [ doc = "Bit 27 - Clock Recovery System interface clock enable" ]
        pub fn crsen(&mut self) -> _CrsenW {
            _CrsenW { register: self }
        }
        # [ doc = "Bit 28 - Power interface clock enable" ]
        pub fn pwren(&mut self) -> _PwrenW {
            _PwrenW { register: self }
        }
        # [ doc = "Bit 29 - DAC interface clock enable" ]
        pub fn dacen(&mut self) -> _DacenW {
            _DacenW { register: self }
        }
        # [ doc = "Bit 30 - HDMI CEC interface clock enable" ]
        pub fn cecen(&mut self) -> _CecenW {
            _CecenW { register: self }
        }
    }
}

# [ doc = "Backup domain control register (RCC_BDCR)" ]
# [ repr ( C ) ]
pub struct Bdcr {
    register: ::volatile_register::RW<u32>,
}

# [ doc = "Backup domain control register (RCC_BDCR)" ]
pub mod bdcr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::Bdcr {
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
    # [ doc = "Value of the field LSEON" ]
    pub struct LseonR {
        bits: u8,
    }
    impl LseonR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field LSERDY" ]
    pub struct LserdyR {
        bits: u8,
    }
    impl LserdyR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field LSEBYP" ]
    pub struct LsebypR {
        bits: u8,
    }
    impl LsebypR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field LSEDRV" ]
    pub struct LsedrvR {
        bits: u8,
    }
    impl LsedrvR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field RTCSEL" ]
    pub struct RtcselR {
        bits: u8,
    }
    impl RtcselR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field RTCEN" ]
    pub struct RtcenR {
        bits: u8,
    }
    impl RtcenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field BDRST" ]
    pub struct BdrstR {
        bits: u8,
    }
    impl BdrstR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _LseonW<'a> {
        register: &'a mut W,
    }
    impl<'a> _LseonW<'a> {
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
    pub struct _LsebypW<'a> {
        register: &'a mut W,
    }
    impl<'a> _LsebypW<'a> {
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
    pub struct _LsedrvW<'a> {
        register: &'a mut W,
    }
    impl<'a> _LsedrvW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 3;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _RtcselW<'a> {
        register: &'a mut W,
    }
    impl<'a> _RtcselW<'a> {
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
    pub struct _RtcenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _RtcenW<'a> {
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
    pub struct _BdrstW<'a> {
        register: &'a mut W,
    }
    impl<'a> _BdrstW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 16;
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
        fn _lseon(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 0 - External Low Speed oscillator enable" ]
        pub fn lseon(&self) -> LseonR {
            LseonR { bits: self._lseon() }
        }
        fn _lserdy(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 1 - External Low Speed oscillator ready" ]
        pub fn lserdy(&self) -> LserdyR {
            LserdyR { bits: self._lserdy() }
        }
        fn _lsebyp(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 2 - External Low Speed oscillator bypass" ]
        pub fn lsebyp(&self) -> LsebypR {
            LsebypR { bits: self._lsebyp() }
        }
        fn _lsedrv(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 3:4 - LSE oscillator drive capability" ]
        pub fn lsedrv(&self) -> LsedrvR {
            LsedrvR { bits: self._lsedrv() }
        }
        fn _rtcsel(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 8:9 - RTC clock source selection" ]
        pub fn rtcsel(&self) -> RtcselR {
            RtcselR { bits: self._rtcsel() }
        }
        fn _rtcen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 15 - RTC clock enable" ]
        pub fn rtcen(&self) -> RtcenR {
            RtcenR { bits: self._rtcen() }
        }
        fn _bdrst(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 16 - Backup domain software reset" ]
        pub fn bdrst(&self) -> BdrstR {
            BdrstR { bits: self._bdrst() }
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
        # [ doc = "Bit 0 - External Low Speed oscillator enable" ]
        pub fn lseon(&mut self) -> _LseonW {
            _LseonW { register: self }
        }
        # [ doc = "Bit 2 - External Low Speed oscillator bypass" ]
        pub fn lsebyp(&mut self) -> _LsebypW {
            _LsebypW { register: self }
        }
        # [ doc = "Bits 3:4 - LSE oscillator drive capability" ]
        pub fn lsedrv(&mut self) -> _LsedrvW {
            _LsedrvW { register: self }
        }
        # [ doc = "Bits 8:9 - RTC clock source selection" ]
        pub fn rtcsel(&mut self) -> _RtcselW {
            _RtcselW { register: self }
        }
        # [ doc = "Bit 15 - RTC clock enable" ]
        pub fn rtcen(&mut self) -> _RtcenW {
            _RtcenW { register: self }
        }
        # [ doc = "Bit 16 - Backup domain software reset" ]
        pub fn bdrst(&mut self) -> _BdrstW {
            _BdrstW { register: self }
        }
    }
}

# [ doc = "Control/status register (RCC_CSR)" ]
# [ repr ( C ) ]
pub struct Csr {
    register: ::volatile_register::RW<u32>,
}

# [ doc = "Control/status register (RCC_CSR)" ]
pub mod csr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::Csr {
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
    # [ doc = "Value of the field LSION" ]
    pub struct LsionR {
        bits: u8,
    }
    impl LsionR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field LSIRDY" ]
    pub struct LsirdyR {
        bits: u8,
    }
    impl LsirdyR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field RMVF" ]
    pub struct RmvfR {
        bits: u8,
    }
    impl RmvfR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field OBLRSTF" ]
    pub struct OblrstfR {
        bits: u8,
    }
    impl OblrstfR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field PINRSTF" ]
    pub struct PinrstfR {
        bits: u8,
    }
    impl PinrstfR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field PORRSTF" ]
    pub struct PorrstfR {
        bits: u8,
    }
    impl PorrstfR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field SFTRSTF" ]
    pub struct SftrstfR {
        bits: u8,
    }
    impl SftrstfR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field IWDGRSTF" ]
    pub struct IwdgrstfR {
        bits: u8,
    }
    impl IwdgrstfR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field WWDGRSTF" ]
    pub struct WwdgrstfR {
        bits: u8,
    }
    impl WwdgrstfR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field LPWRRSTF" ]
    pub struct LpwrrstfR {
        bits: u8,
    }
    impl LpwrrstfR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _LsionW<'a> {
        register: &'a mut W,
    }
    impl<'a> _LsionW<'a> {
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
    pub struct _RmvfW<'a> {
        register: &'a mut W,
    }
    impl<'a> _RmvfW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 24;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _OblrstfW<'a> {
        register: &'a mut W,
    }
    impl<'a> _OblrstfW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 25;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PinrstfW<'a> {
        register: &'a mut W,
    }
    impl<'a> _PinrstfW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 26;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PorrstfW<'a> {
        register: &'a mut W,
    }
    impl<'a> _PorrstfW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 27;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _SftrstfW<'a> {
        register: &'a mut W,
    }
    impl<'a> _SftrstfW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 28;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _IwdgrstfW<'a> {
        register: &'a mut W,
    }
    impl<'a> _IwdgrstfW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 29;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _WwdgrstfW<'a> {
        register: &'a mut W,
    }
    impl<'a> _WwdgrstfW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 30;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _LpwrrstfW<'a> {
        register: &'a mut W,
    }
    impl<'a> _LpwrrstfW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 31;
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
        fn _lsion(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 0 - Internal low speed oscillator enable" ]
        pub fn lsion(&self) -> LsionR {
            LsionR { bits: self._lsion() }
        }
        fn _lsirdy(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 1 - Internal low speed oscillator ready" ]
        pub fn lsirdy(&self) -> LsirdyR {
            LsirdyR { bits: self._lsirdy() }
        }
        fn _rmvf(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 24 - Remove reset flag" ]
        pub fn rmvf(&self) -> RmvfR {
            RmvfR { bits: self._rmvf() }
        }
        fn _oblrstf(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 25 - Option byte loader reset flag" ]
        pub fn oblrstf(&self) -> OblrstfR {
            OblrstfR { bits: self._oblrstf() }
        }
        fn _pinrstf(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 26 - PIN reset flag" ]
        pub fn pinrstf(&self) -> PinrstfR {
            PinrstfR { bits: self._pinrstf() }
        }
        fn _porrstf(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 27 - POR/PDR reset flag" ]
        pub fn porrstf(&self) -> PorrstfR {
            PorrstfR { bits: self._porrstf() }
        }
        fn _sftrstf(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 28 - Software reset flag" ]
        pub fn sftrstf(&self) -> SftrstfR {
            SftrstfR { bits: self._sftrstf() }
        }
        fn _iwdgrstf(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 29 - Independent watchdog reset flag" ]
        pub fn iwdgrstf(&self) -> IwdgrstfR {
            IwdgrstfR { bits: self._iwdgrstf() }
        }
        fn _wwdgrstf(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 30 - Window watchdog reset flag" ]
        pub fn wwdgrstf(&self) -> WwdgrstfR {
            WwdgrstfR { bits: self._wwdgrstf() }
        }
        fn _lpwrrstf(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 31 - Low-power reset flag" ]
        pub fn lpwrrstf(&self) -> LpwrrstfR {
            LpwrrstfR { bits: self._lpwrrstf() }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        pub fn reset_value() -> W {
            W { bits: 201326592 }
        }
        # [ doc = r" Writes raw `bits` to the register" ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bit 0 - Internal low speed oscillator enable" ]
        pub fn lsion(&mut self) -> _LsionW {
            _LsionW { register: self }
        }
        # [ doc = "Bit 24 - Remove reset flag" ]
        pub fn rmvf(&mut self) -> _RmvfW {
            _RmvfW { register: self }
        }
        # [ doc = "Bit 25 - Option byte loader reset flag" ]
        pub fn oblrstf(&mut self) -> _OblrstfW {
            _OblrstfW { register: self }
        }
        # [ doc = "Bit 26 - PIN reset flag" ]
        pub fn pinrstf(&mut self) -> _PinrstfW {
            _PinrstfW { register: self }
        }
        # [ doc = "Bit 27 - POR/PDR reset flag" ]
        pub fn porrstf(&mut self) -> _PorrstfW {
            _PorrstfW { register: self }
        }
        # [ doc = "Bit 28 - Software reset flag" ]
        pub fn sftrstf(&mut self) -> _SftrstfW {
            _SftrstfW { register: self }
        }
        # [ doc = "Bit 29 - Independent watchdog reset flag" ]
        pub fn iwdgrstf(&mut self) -> _IwdgrstfW {
            _IwdgrstfW { register: self }
        }
        # [ doc = "Bit 30 - Window watchdog reset flag" ]
        pub fn wwdgrstf(&mut self) -> _WwdgrstfW {
            _WwdgrstfW { register: self }
        }
        # [ doc = "Bit 31 - Low-power reset flag" ]
        pub fn lpwrrstf(&mut self) -> _LpwrrstfW {
            _LpwrrstfW { register: self }
        }
    }
}

# [ doc = "AHB peripheral reset register" ]
# [ repr ( C ) ]
pub struct Ahbrstr {
    register: ::volatile_register::RW<u32>,
}

# [ doc = "AHB peripheral reset register" ]
pub mod ahbrstr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::Ahbrstr {
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
    # [ doc = "Value of the field IOPARST" ]
    pub struct IoparstR {
        bits: u8,
    }
    impl IoparstR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field IOPBRST" ]
    pub struct IopbrstR {
        bits: u8,
    }
    impl IopbrstR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field IOPCRST" ]
    pub struct IopcrstR {
        bits: u8,
    }
    impl IopcrstR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field IOPDRST" ]
    pub struct IopdrstR {
        bits: u8,
    }
    impl IopdrstR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field IOPFRST" ]
    pub struct IopfrstR {
        bits: u8,
    }
    impl IopfrstR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field TSCRST" ]
    pub struct TscrstR {
        bits: u8,
    }
    impl TscrstR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _IoparstW<'a> {
        register: &'a mut W,
    }
    impl<'a> _IoparstW<'a> {
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
    pub struct _IopbrstW<'a> {
        register: &'a mut W,
    }
    impl<'a> _IopbrstW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 18;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _IopcrstW<'a> {
        register: &'a mut W,
    }
    impl<'a> _IopcrstW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 19;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _IopdrstW<'a> {
        register: &'a mut W,
    }
    impl<'a> _IopdrstW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 20;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _IopfrstW<'a> {
        register: &'a mut W,
    }
    impl<'a> _IopfrstW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 22;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _TscrstW<'a> {
        register: &'a mut W,
    }
    impl<'a> _TscrstW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 24;
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
        fn _ioparst(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 17 - I/O port A reset" ]
        pub fn ioparst(&self) -> IoparstR {
            IoparstR { bits: self._ioparst() }
        }
        fn _iopbrst(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 18 - I/O port B reset" ]
        pub fn iopbrst(&self) -> IopbrstR {
            IopbrstR { bits: self._iopbrst() }
        }
        fn _iopcrst(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 19 - I/O port C reset" ]
        pub fn iopcrst(&self) -> IopcrstR {
            IopcrstR { bits: self._iopcrst() }
        }
        fn _iopdrst(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 20 - I/O port D reset" ]
        pub fn iopdrst(&self) -> IopdrstR {
            IopdrstR { bits: self._iopdrst() }
        }
        fn _iopfrst(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 22 - I/O port F reset" ]
        pub fn iopfrst(&self) -> IopfrstR {
            IopfrstR { bits: self._iopfrst() }
        }
        fn _tscrst(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 24 - Touch sensing controller reset" ]
        pub fn tscrst(&self) -> TscrstR {
            TscrstR { bits: self._tscrst() }
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
        # [ doc = "Bit 17 - I/O port A reset" ]
        pub fn ioparst(&mut self) -> _IoparstW {
            _IoparstW { register: self }
        }
        # [ doc = "Bit 18 - I/O port B reset" ]
        pub fn iopbrst(&mut self) -> _IopbrstW {
            _IopbrstW { register: self }
        }
        # [ doc = "Bit 19 - I/O port C reset" ]
        pub fn iopcrst(&mut self) -> _IopcrstW {
            _IopcrstW { register: self }
        }
        # [ doc = "Bit 20 - I/O port D reset" ]
        pub fn iopdrst(&mut self) -> _IopdrstW {
            _IopdrstW { register: self }
        }
        # [ doc = "Bit 22 - I/O port F reset" ]
        pub fn iopfrst(&mut self) -> _IopfrstW {
            _IopfrstW { register: self }
        }
        # [ doc = "Bit 24 - Touch sensing controller reset" ]
        pub fn tscrst(&mut self) -> _TscrstW {
            _TscrstW { register: self }
        }
    }
}

# [ doc = "Clock configuration register 2" ]
# [ repr ( C ) ]
pub struct Cfgr2 {
    register: ::volatile_register::RW<u32>,
}

# [ doc = "Clock configuration register 2" ]
pub mod cfgr2 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::Cfgr2 {
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
    # [ doc = "Value of the field PREDIV" ]
    pub struct PredivR {
        bits: u8,
    }
    impl PredivR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PredivW<'a> {
        register: &'a mut W,
    }
    impl<'a> _PredivW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 15;
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
        fn _prediv(&self) -> u8 {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 0:3 - PREDIV division factor" ]
        pub fn prediv(&self) -> PredivR {
            PredivR { bits: self._prediv() }
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
        # [ doc = "Bits 0:3 - PREDIV division factor" ]
        pub fn prediv(&mut self) -> _PredivW {
            _PredivW { register: self }
        }
    }
}

# [ doc = "Clock configuration register 3" ]
# [ repr ( C ) ]
pub struct Cfgr3 {
    register: ::volatile_register::RW<u32>,
}

# [ doc = "Clock configuration register 3" ]
pub mod cfgr3 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::Cfgr3 {
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
    # [ doc = "Value of the field USART1SW" ]
    pub struct Usart1swR {
        bits: u8,
    }
    impl Usart1swR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field I2C1SW" ]
    pub struct I2c1swR {
        bits: u8,
    }
    impl I2c1swR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field CECSW" ]
    pub struct CecswR {
        bits: u8,
    }
    impl CecswR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field USBSW" ]
    pub struct UsbswR {
        bits: u8,
    }
    impl UsbswR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field ADCSW" ]
    pub struct AdcswR {
        bits: u8,
    }
    impl AdcswR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field USART2SW" ]
    pub struct Usart2swR {
        bits: u8,
    }
    impl Usart2swR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Usart1swW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Usart1swW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _I2c1swW<'a> {
        register: &'a mut W,
    }
    impl<'a> _I2c1swW<'a> {
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
    pub struct _CecswW<'a> {
        register: &'a mut W,
    }
    impl<'a> _CecswW<'a> {
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
    pub struct _UsbswW<'a> {
        register: &'a mut W,
    }
    impl<'a> _UsbswW<'a> {
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
    pub struct _AdcswW<'a> {
        register: &'a mut W,
    }
    impl<'a> _AdcswW<'a> {
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
    pub struct _Usart2swW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Usart2swW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
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
        fn _usart1sw(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 0:1 - USART1 clock source selection" ]
        pub fn usart1sw(&self) -> Usart1swR {
            Usart1swR { bits: self._usart1sw() }
        }
        fn _i2c1sw(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 4 - I2C1 clock source selection" ]
        pub fn i2c1sw(&self) -> I2c1swR {
            I2c1swR { bits: self._i2c1sw() }
        }
        fn _cecsw(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 6 - HDMI CEC clock source selection" ]
        pub fn cecsw(&self) -> CecswR {
            CecswR { bits: self._cecsw() }
        }
        fn _usbsw(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 7 - USB clock source selection" ]
        pub fn usbsw(&self) -> UsbswR {
            UsbswR { bits: self._usbsw() }
        }
        fn _adcsw(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 8 - ADC clock source selection" ]
        pub fn adcsw(&self) -> AdcswR {
            AdcswR { bits: self._adcsw() }
        }
        fn _usart2sw(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 16:17 - USART2 clock source selection" ]
        pub fn usart2sw(&self) -> Usart2swR {
            Usart2swR { bits: self._usart2sw() }
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
        # [ doc = "Bits 0:1 - USART1 clock source selection" ]
        pub fn usart1sw(&mut self) -> _Usart1swW {
            _Usart1swW { register: self }
        }
        # [ doc = "Bit 4 - I2C1 clock source selection" ]
        pub fn i2c1sw(&mut self) -> _I2c1swW {
            _I2c1swW { register: self }
        }
        # [ doc = "Bit 6 - HDMI CEC clock source selection" ]
        pub fn cecsw(&mut self) -> _CecswW {
            _CecswW { register: self }
        }
        # [ doc = "Bit 7 - USB clock source selection" ]
        pub fn usbsw(&mut self) -> _UsbswW {
            _UsbswW { register: self }
        }
        # [ doc = "Bit 8 - ADC clock source selection" ]
        pub fn adcsw(&mut self) -> _AdcswW {
            _AdcswW { register: self }
        }
        # [ doc = "Bits 16:17 - USART2 clock source selection" ]
        pub fn usart2sw(&mut self) -> _Usart2swW {
            _Usart2swW { register: self }
        }
    }
}

# [ doc = "Clock control register 2" ]
# [ repr ( C ) ]
pub struct Cr2 {
    register: ::volatile_register::RW<u32>,
}

# [ doc = "Clock control register 2" ]
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
    # [ doc = "Value of the field HSI14ON" ]
    pub struct Hsi14onR {
        bits: u8,
    }
    impl Hsi14onR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field HSI14RDY" ]
    pub struct Hsi14rdyR {
        bits: u8,
    }
    impl Hsi14rdyR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field HSI14DIS" ]
    pub struct Hsi14disR {
        bits: u8,
    }
    impl Hsi14disR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field HSI14TRIM" ]
    pub struct Hsi14trimR {
        bits: u8,
    }
    impl Hsi14trimR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field HSI14CAL" ]
    pub struct Hsi14calR {
        bits: u8,
    }
    impl Hsi14calR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field HSI48ON" ]
    pub struct Hsi48onR {
        bits: u8,
    }
    impl Hsi48onR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field HSI48RDY" ]
    pub struct Hsi48rdyR {
        bits: u8,
    }
    impl Hsi48rdyR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field HSI48CAL" ]
    pub struct Hsi48calR {
        bits: u8,
    }
    impl Hsi48calR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Hsi14onW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Hsi14onW<'a> {
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
    pub struct _Hsi14disW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Hsi14disW<'a> {
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
    pub struct _Hsi14trimW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Hsi14trimW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 31;
            const OFFSET: u8 = 3;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Hsi48onW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Hsi48onW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 16;
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
        fn _hsi14on(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 0 - HSI14 clock enable" ]
        pub fn hsi14on(&self) -> Hsi14onR {
            Hsi14onR { bits: self._hsi14on() }
        }
        fn _hsi14rdy(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 1 - HR14 clock ready flag" ]
        pub fn hsi14rdy(&self) -> Hsi14rdyR {
            Hsi14rdyR { bits: self._hsi14rdy() }
        }
        fn _hsi14dis(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 2 - HSI14 clock request from ADC disable" ]
        pub fn hsi14dis(&self) -> Hsi14disR {
            Hsi14disR { bits: self._hsi14dis() }
        }
        fn _hsi14trim(&self) -> u8 {
            const MASK: u8 = 31;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 3:7 - HSI14 clock trimming" ]
        pub fn hsi14trim(&self) -> Hsi14trimR {
            Hsi14trimR { bits: self._hsi14trim() }
        }
        fn _hsi14cal(&self) -> u8 {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 8:15 - HSI14 clock calibration" ]
        pub fn hsi14cal(&self) -> Hsi14calR {
            Hsi14calR { bits: self._hsi14cal() }
        }
        fn _hsi48on(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 16 - HSI48 clock enable" ]
        pub fn hsi48on(&self) -> Hsi48onR {
            Hsi48onR { bits: self._hsi48on() }
        }
        fn _hsi48rdy(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 17 - HSI48 clock ready flag" ]
        pub fn hsi48rdy(&self) -> Hsi48rdyR {
            Hsi48rdyR { bits: self._hsi48rdy() }
        }
        fn _hsi48cal(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 24 - HSI48 factory clock calibration" ]
        pub fn hsi48cal(&self) -> Hsi48calR {
            Hsi48calR { bits: self._hsi48cal() }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        pub fn reset_value() -> W {
            W { bits: 128 }
        }
        # [ doc = r" Writes raw `bits` to the register" ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bit 0 - HSI14 clock enable" ]
        pub fn hsi14on(&mut self) -> _Hsi14onW {
            _Hsi14onW { register: self }
        }
        # [ doc = "Bit 2 - HSI14 clock request from ADC disable" ]
        pub fn hsi14dis(&mut self) -> _Hsi14disW {
            _Hsi14disW { register: self }
        }
        # [ doc = "Bits 3:7 - HSI14 clock trimming" ]
        pub fn hsi14trim(&mut self) -> _Hsi14trimW {
            _Hsi14trimW { register: self }
        }
        # [ doc = "Bit 16 - HSI48 clock enable" ]
        pub fn hsi48on(&mut self) -> _Hsi48onW {
            _Hsi48onW { register: self }
        }
    }
}
