# ! [ doc = "Universal synchronous asynchronous receiver transmitter" ]
# [ doc = r" Register block" ]
# [ repr ( C ) ]
pub struct Usart {
    # [ doc = "0x00 - Control register 1" ]
    pub cr1: Cr1,
    # [ doc = "0x04 - Control register 2" ]
    pub cr2: Cr2,
    # [ doc = "0x08 - Control register 3" ]
    pub cr3: Cr3,
    # [ doc = "0x0c - Baud rate register" ]
    pub brr: Brr,
    # [ doc = "0x10 - Guard time and prescaler register" ]
    pub gtpr: Gtpr,
    # [ doc = "0x14 - Receiver timeout register" ]
    pub rtor: Rtor,
    # [ doc = "0x18 - Request register" ]
    pub rqr: Rqr,
    # [ doc = "0x1c - Interrupt & status register" ]
    pub isr: Isr,
    # [ doc = "0x20 - Interrupt flag clear register" ]
    pub icr: Icr,
    # [ doc = "0x24 - Receive data register" ]
    pub rdr: Rdr,
    # [ doc = "0x28 - Transmit data register" ]
    pub tdr: Tdr,
}

# [ doc = "Control register 1" ]
# [ repr ( C ) ]
pub struct Cr1 {
    register: ::volatile_register::RW<u32>,
}

# [ doc = "Control register 1" ]
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
    # [ doc = "Value of the field UE" ]
    pub struct UeR {
        bits: u8,
    }
    impl UeR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field UESM" ]
    pub struct UesmR {
        bits: u8,
    }
    impl UesmR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field RE" ]
    pub struct ReR {
        bits: u8,
    }
    impl ReR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field TE" ]
    pub struct TeR {
        bits: u8,
    }
    impl TeR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field IDLEIE" ]
    pub struct IdleieR {
        bits: u8,
    }
    impl IdleieR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field RXNEIE" ]
    pub struct RxneieR {
        bits: u8,
    }
    impl RxneieR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field TCIE" ]
    pub struct TcieR {
        bits: u8,
    }
    impl TcieR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field TXEIE" ]
    pub struct TxeieR {
        bits: u8,
    }
    impl TxeieR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field PEIE" ]
    pub struct PeieR {
        bits: u8,
    }
    impl PeieR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field PS" ]
    pub struct PsR {
        bits: u8,
    }
    impl PsR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field PCE" ]
    pub struct PceR {
        bits: u8,
    }
    impl PceR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field WAKE" ]
    pub struct WakeR {
        bits: u8,
    }
    impl WakeR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field M" ]
    pub struct MR {
        bits: u8,
    }
    impl MR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field MME" ]
    pub struct MmeR {
        bits: u8,
    }
    impl MmeR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field CMIE" ]
    pub struct CmieR {
        bits: u8,
    }
    impl CmieR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field OVER8" ]
    pub struct Over8R {
        bits: u8,
    }
    impl Over8R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field DEDT" ]
    pub struct DedtR {
        bits: u8,
    }
    impl DedtR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field DEAT" ]
    pub struct DeatR {
        bits: u8,
    }
    impl DeatR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field RTOIE" ]
    pub struct RtoieR {
        bits: u8,
    }
    impl RtoieR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field EOBIE" ]
    pub struct EobieR {
        bits: u8,
    }
    impl EobieR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field M1" ]
    pub struct M1R {
        bits: u8,
    }
    impl M1R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _UeW<'a> {
        register: &'a mut W,
    }
    impl<'a> _UeW<'a> {
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
    pub struct _UesmW<'a> {
        register: &'a mut W,
    }
    impl<'a> _UesmW<'a> {
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
    pub struct _ReW<'a> {
        register: &'a mut W,
    }
    impl<'a> _ReW<'a> {
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
    pub struct _TeW<'a> {
        register: &'a mut W,
    }
    impl<'a> _TeW<'a> {
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
    pub struct _IdleieW<'a> {
        register: &'a mut W,
    }
    impl<'a> _IdleieW<'a> {
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
    pub struct _RxneieW<'a> {
        register: &'a mut W,
    }
    impl<'a> _RxneieW<'a> {
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
    pub struct _TcieW<'a> {
        register: &'a mut W,
    }
    impl<'a> _TcieW<'a> {
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
    pub struct _TxeieW<'a> {
        register: &'a mut W,
    }
    impl<'a> _TxeieW<'a> {
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
    pub struct _PeieW<'a> {
        register: &'a mut W,
    }
    impl<'a> _PeieW<'a> {
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
    pub struct _PsW<'a> {
        register: &'a mut W,
    }
    impl<'a> _PsW<'a> {
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
    pub struct _PceW<'a> {
        register: &'a mut W,
    }
    impl<'a> _PceW<'a> {
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
    pub struct _WakeW<'a> {
        register: &'a mut W,
    }
    impl<'a> _WakeW<'a> {
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
    pub struct _MW<'a> {
        register: &'a mut W,
    }
    impl<'a> _MW<'a> {
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
    pub struct _MmeW<'a> {
        register: &'a mut W,
    }
    impl<'a> _MmeW<'a> {
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
    pub struct _CmieW<'a> {
        register: &'a mut W,
    }
    impl<'a> _CmieW<'a> {
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
    pub struct _Over8W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Over8W<'a> {
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
    pub struct _DedtW<'a> {
        register: &'a mut W,
    }
    impl<'a> _DedtW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 31;
            const OFFSET: u8 = 16;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _DeatW<'a> {
        register: &'a mut W,
    }
    impl<'a> _DeatW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 31;
            const OFFSET: u8 = 21;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _RtoieW<'a> {
        register: &'a mut W,
    }
    impl<'a> _RtoieW<'a> {
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
    pub struct _EobieW<'a> {
        register: &'a mut W,
    }
    impl<'a> _EobieW<'a> {
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
    pub struct _M1W<'a> {
        register: &'a mut W,
    }
    impl<'a> _M1W<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 28;
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
        fn _ue(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 0 - USART enable" ]
        pub fn ue(&self) -> UeR {
            UeR { bits: self._ue() }
        }
        fn _uesm(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 1 - USART enable in Stop mode" ]
        pub fn uesm(&self) -> UesmR {
            UesmR { bits: self._uesm() }
        }
        fn _re(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 2 - Receiver enable" ]
        pub fn re(&self) -> ReR {
            ReR { bits: self._re() }
        }
        fn _te(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 3 - Transmitter enable" ]
        pub fn te(&self) -> TeR {
            TeR { bits: self._te() }
        }
        fn _idleie(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 4 - IDLE interrupt enable" ]
        pub fn idleie(&self) -> IdleieR {
            IdleieR { bits: self._idleie() }
        }
        fn _rxneie(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 5 - RXNE interrupt enable" ]
        pub fn rxneie(&self) -> RxneieR {
            RxneieR { bits: self._rxneie() }
        }
        fn _tcie(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 6 - Transmission complete interrupt enable" ]
        pub fn tcie(&self) -> TcieR {
            TcieR { bits: self._tcie() }
        }
        fn _txeie(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 7 - interrupt enable" ]
        pub fn txeie(&self) -> TxeieR {
            TxeieR { bits: self._txeie() }
        }
        fn _peie(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 8 - PE interrupt enable" ]
        pub fn peie(&self) -> PeieR {
            PeieR { bits: self._peie() }
        }
        fn _ps(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 9 - Parity selection" ]
        pub fn ps(&self) -> PsR {
            PsR { bits: self._ps() }
        }
        fn _pce(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 10 - Parity control enable" ]
        pub fn pce(&self) -> PceR {
            PceR { bits: self._pce() }
        }
        fn _wake(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 11 - Receiver wakeup method" ]
        pub fn wake(&self) -> WakeR {
            WakeR { bits: self._wake() }
        }
        fn _m(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 12 - Word length" ]
        pub fn m(&self) -> MR {
            MR { bits: self._m() }
        }
        fn _mme(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 13 - Mute mode enable" ]
        pub fn mme(&self) -> MmeR {
            MmeR { bits: self._mme() }
        }
        fn _cmie(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 14 - Character match interrupt enable" ]
        pub fn cmie(&self) -> CmieR {
            CmieR { bits: self._cmie() }
        }
        fn _over8(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 15 - Oversampling mode" ]
        pub fn over8(&self) -> Over8R {
            Over8R { bits: self._over8() }
        }
        fn _dedt(&self) -> u8 {
            const MASK: u8 = 31;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 16:20 - Driver Enable deassertion time" ]
        pub fn dedt(&self) -> DedtR {
            DedtR { bits: self._dedt() }
        }
        fn _deat(&self) -> u8 {
            const MASK: u8 = 31;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 21:25 - Driver Enable assertion time" ]
        pub fn deat(&self) -> DeatR {
            DeatR { bits: self._deat() }
        }
        fn _rtoie(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 26 - Receiver timeout interrupt enable" ]
        pub fn rtoie(&self) -> RtoieR {
            RtoieR { bits: self._rtoie() }
        }
        fn _eobie(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 27 - End of Block interrupt enable" ]
        pub fn eobie(&self) -> EobieR {
            EobieR { bits: self._eobie() }
        }
        fn _m1(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 28 - Word length" ]
        pub fn m1(&self) -> M1R {
            M1R { bits: self._m1() }
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
        # [ doc = "Bit 0 - USART enable" ]
        pub fn ue(&mut self) -> _UeW {
            _UeW { register: self }
        }
        # [ doc = "Bit 1 - USART enable in Stop mode" ]
        pub fn uesm(&mut self) -> _UesmW {
            _UesmW { register: self }
        }
        # [ doc = "Bit 2 - Receiver enable" ]
        pub fn re(&mut self) -> _ReW {
            _ReW { register: self }
        }
        # [ doc = "Bit 3 - Transmitter enable" ]
        pub fn te(&mut self) -> _TeW {
            _TeW { register: self }
        }
        # [ doc = "Bit 4 - IDLE interrupt enable" ]
        pub fn idleie(&mut self) -> _IdleieW {
            _IdleieW { register: self }
        }
        # [ doc = "Bit 5 - RXNE interrupt enable" ]
        pub fn rxneie(&mut self) -> _RxneieW {
            _RxneieW { register: self }
        }
        # [ doc = "Bit 6 - Transmission complete interrupt enable" ]
        pub fn tcie(&mut self) -> _TcieW {
            _TcieW { register: self }
        }
        # [ doc = "Bit 7 - interrupt enable" ]
        pub fn txeie(&mut self) -> _TxeieW {
            _TxeieW { register: self }
        }
        # [ doc = "Bit 8 - PE interrupt enable" ]
        pub fn peie(&mut self) -> _PeieW {
            _PeieW { register: self }
        }
        # [ doc = "Bit 9 - Parity selection" ]
        pub fn ps(&mut self) -> _PsW {
            _PsW { register: self }
        }
        # [ doc = "Bit 10 - Parity control enable" ]
        pub fn pce(&mut self) -> _PceW {
            _PceW { register: self }
        }
        # [ doc = "Bit 11 - Receiver wakeup method" ]
        pub fn wake(&mut self) -> _WakeW {
            _WakeW { register: self }
        }
        # [ doc = "Bit 12 - Word length" ]
        pub fn m(&mut self) -> _MW {
            _MW { register: self }
        }
        # [ doc = "Bit 13 - Mute mode enable" ]
        pub fn mme(&mut self) -> _MmeW {
            _MmeW { register: self }
        }
        # [ doc = "Bit 14 - Character match interrupt enable" ]
        pub fn cmie(&mut self) -> _CmieW {
            _CmieW { register: self }
        }
        # [ doc = "Bit 15 - Oversampling mode" ]
        pub fn over8(&mut self) -> _Over8W {
            _Over8W { register: self }
        }
        # [ doc = "Bits 16:20 - Driver Enable deassertion time" ]
        pub fn dedt(&mut self) -> _DedtW {
            _DedtW { register: self }
        }
        # [ doc = "Bits 21:25 - Driver Enable assertion time" ]
        pub fn deat(&mut self) -> _DeatW {
            _DeatW { register: self }
        }
        # [ doc = "Bit 26 - Receiver timeout interrupt enable" ]
        pub fn rtoie(&mut self) -> _RtoieW {
            _RtoieW { register: self }
        }
        # [ doc = "Bit 27 - End of Block interrupt enable" ]
        pub fn eobie(&mut self) -> _EobieW {
            _EobieW { register: self }
        }
        # [ doc = "Bit 28 - Word length" ]
        pub fn m1(&mut self) -> _M1W {
            _M1W { register: self }
        }
    }
}

# [ doc = "Control register 2" ]
# [ repr ( C ) ]
pub struct Cr2 {
    register: ::volatile_register::RW<u32>,
}

# [ doc = "Control register 2" ]
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
    # [ doc = "Value of the field ADD4" ]
    pub struct Add4R {
        bits: u8,
    }
    impl Add4R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field ADD0" ]
    pub struct Add0R {
        bits: u8,
    }
    impl Add0R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field RTOEN" ]
    pub struct RtoenR {
        bits: u8,
    }
    impl RtoenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field ABRMOD" ]
    pub struct AbrmodR {
        bits: u8,
    }
    impl AbrmodR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field ABREN" ]
    pub struct AbrenR {
        bits: u8,
    }
    impl AbrenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field MSBFIRST" ]
    pub struct MsbfirstR {
        bits: u8,
    }
    impl MsbfirstR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field DATAINV" ]
    pub struct DatainvR {
        bits: u8,
    }
    impl DatainvR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field TXINV" ]
    pub struct TxinvR {
        bits: u8,
    }
    impl TxinvR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field RXINV" ]
    pub struct RxinvR {
        bits: u8,
    }
    impl RxinvR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field SWAP" ]
    pub struct SwapR {
        bits: u8,
    }
    impl SwapR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field LINEN" ]
    pub struct LinenR {
        bits: u8,
    }
    impl LinenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field STOP" ]
    pub struct StopR {
        bits: u8,
    }
    impl StopR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field CLKEN" ]
    pub struct ClkenR {
        bits: u8,
    }
    impl ClkenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field CPOL" ]
    pub struct CpolR {
        bits: u8,
    }
    impl CpolR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field CPHA" ]
    pub struct CphaR {
        bits: u8,
    }
    impl CphaR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field LBCL" ]
    pub struct LbclR {
        bits: u8,
    }
    impl LbclR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field LBDIE" ]
    pub struct LbdieR {
        bits: u8,
    }
    impl LbdieR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field LBDL" ]
    pub struct LbdlR {
        bits: u8,
    }
    impl LbdlR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field ADDM7" ]
    pub struct Addm7R {
        bits: u8,
    }
    impl Addm7R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Add4W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Add4W<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Add0W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Add0W<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _RtoenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _RtoenW<'a> {
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
    pub struct _AbrmodW<'a> {
        register: &'a mut W,
    }
    impl<'a> _AbrmodW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 21;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _AbrenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _AbrenW<'a> {
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
    pub struct _MsbfirstW<'a> {
        register: &'a mut W,
    }
    impl<'a> _MsbfirstW<'a> {
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
    pub struct _DatainvW<'a> {
        register: &'a mut W,
    }
    impl<'a> _DatainvW<'a> {
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
    pub struct _TxinvW<'a> {
        register: &'a mut W,
    }
    impl<'a> _TxinvW<'a> {
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
    pub struct _RxinvW<'a> {
        register: &'a mut W,
    }
    impl<'a> _RxinvW<'a> {
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
    pub struct _SwapW<'a> {
        register: &'a mut W,
    }
    impl<'a> _SwapW<'a> {
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
    pub struct _LinenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _LinenW<'a> {
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
    pub struct _StopW<'a> {
        register: &'a mut W,
    }
    impl<'a> _StopW<'a> {
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
    pub struct _ClkenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _ClkenW<'a> {
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
    pub struct _CpolW<'a> {
        register: &'a mut W,
    }
    impl<'a> _CpolW<'a> {
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
    pub struct _CphaW<'a> {
        register: &'a mut W,
    }
    impl<'a> _CphaW<'a> {
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
    pub struct _LbclW<'a> {
        register: &'a mut W,
    }
    impl<'a> _LbclW<'a> {
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
    pub struct _LbdieW<'a> {
        register: &'a mut W,
    }
    impl<'a> _LbdieW<'a> {
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
    pub struct _LbdlW<'a> {
        register: &'a mut W,
    }
    impl<'a> _LbdlW<'a> {
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
    pub struct _Addm7W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Addm7W<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
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
        fn _add4(&self) -> u8 {
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 28:31 - Address of the USART node" ]
        pub fn add4(&self) -> Add4R {
            Add4R { bits: self._add4() }
        }
        fn _add0(&self) -> u8 {
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 24:27 - Address of the USART node" ]
        pub fn add0(&self) -> Add0R {
            Add0R { bits: self._add0() }
        }
        fn _rtoen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 23 - Receiver timeout enable" ]
        pub fn rtoen(&self) -> RtoenR {
            RtoenR { bits: self._rtoen() }
        }
        fn _abrmod(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 21:22 - Auto baud rate mode" ]
        pub fn abrmod(&self) -> AbrmodR {
            AbrmodR { bits: self._abrmod() }
        }
        fn _abren(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 20 - Auto baud rate enable" ]
        pub fn abren(&self) -> AbrenR {
            AbrenR { bits: self._abren() }
        }
        fn _msbfirst(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 19 - Most significant bit first" ]
        pub fn msbfirst(&self) -> MsbfirstR {
            MsbfirstR { bits: self._msbfirst() }
        }
        fn _datainv(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 18 - Binary data inversion" ]
        pub fn datainv(&self) -> DatainvR {
            DatainvR { bits: self._datainv() }
        }
        fn _txinv(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 17 - TX pin active level inversion" ]
        pub fn txinv(&self) -> TxinvR {
            TxinvR { bits: self._txinv() }
        }
        fn _rxinv(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 16 - RX pin active level inversion" ]
        pub fn rxinv(&self) -> RxinvR {
            RxinvR { bits: self._rxinv() }
        }
        fn _swap(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 15 - Swap TX/RX pins" ]
        pub fn swap(&self) -> SwapR {
            SwapR { bits: self._swap() }
        }
        fn _linen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 14 - LIN mode enable" ]
        pub fn linen(&self) -> LinenR {
            LinenR { bits: self._linen() }
        }
        fn _stop(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 12:13 - STOP bits" ]
        pub fn stop(&self) -> StopR {
            StopR { bits: self._stop() }
        }
        fn _clken(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 11 - Clock enable" ]
        pub fn clken(&self) -> ClkenR {
            ClkenR { bits: self._clken() }
        }
        fn _cpol(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 10 - Clock polarity" ]
        pub fn cpol(&self) -> CpolR {
            CpolR { bits: self._cpol() }
        }
        fn _cpha(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 9 - Clock phase" ]
        pub fn cpha(&self) -> CphaR {
            CphaR { bits: self._cpha() }
        }
        fn _lbcl(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 8 - Last bit clock pulse" ]
        pub fn lbcl(&self) -> LbclR {
            LbclR { bits: self._lbcl() }
        }
        fn _lbdie(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 6 - LIN break detection interrupt enable" ]
        pub fn lbdie(&self) -> LbdieR {
            LbdieR { bits: self._lbdie() }
        }
        fn _lbdl(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 5 - LIN break detection length" ]
        pub fn lbdl(&self) -> LbdlR {
            LbdlR { bits: self._lbdl() }
        }
        fn _addm7(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 4 - 7-bit Address Detection/4-bit Address Detection" ]
        pub fn addm7(&self) -> Addm7R {
            Addm7R { bits: self._addm7() }
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
        # [ doc = "Bits 28:31 - Address of the USART node" ]
        pub fn add4(&mut self) -> _Add4W {
            _Add4W { register: self }
        }
        # [ doc = "Bits 24:27 - Address of the USART node" ]
        pub fn add0(&mut self) -> _Add0W {
            _Add0W { register: self }
        }
        # [ doc = "Bit 23 - Receiver timeout enable" ]
        pub fn rtoen(&mut self) -> _RtoenW {
            _RtoenW { register: self }
        }
        # [ doc = "Bits 21:22 - Auto baud rate mode" ]
        pub fn abrmod(&mut self) -> _AbrmodW {
            _AbrmodW { register: self }
        }
        # [ doc = "Bit 20 - Auto baud rate enable" ]
        pub fn abren(&mut self) -> _AbrenW {
            _AbrenW { register: self }
        }
        # [ doc = "Bit 19 - Most significant bit first" ]
        pub fn msbfirst(&mut self) -> _MsbfirstW {
            _MsbfirstW { register: self }
        }
        # [ doc = "Bit 18 - Binary data inversion" ]
        pub fn datainv(&mut self) -> _DatainvW {
            _DatainvW { register: self }
        }
        # [ doc = "Bit 17 - TX pin active level inversion" ]
        pub fn txinv(&mut self) -> _TxinvW {
            _TxinvW { register: self }
        }
        # [ doc = "Bit 16 - RX pin active level inversion" ]
        pub fn rxinv(&mut self) -> _RxinvW {
            _RxinvW { register: self }
        }
        # [ doc = "Bit 15 - Swap TX/RX pins" ]
        pub fn swap(&mut self) -> _SwapW {
            _SwapW { register: self }
        }
        # [ doc = "Bit 14 - LIN mode enable" ]
        pub fn linen(&mut self) -> _LinenW {
            _LinenW { register: self }
        }
        # [ doc = "Bits 12:13 - STOP bits" ]
        pub fn stop(&mut self) -> _StopW {
            _StopW { register: self }
        }
        # [ doc = "Bit 11 - Clock enable" ]
        pub fn clken(&mut self) -> _ClkenW {
            _ClkenW { register: self }
        }
        # [ doc = "Bit 10 - Clock polarity" ]
        pub fn cpol(&mut self) -> _CpolW {
            _CpolW { register: self }
        }
        # [ doc = "Bit 9 - Clock phase" ]
        pub fn cpha(&mut self) -> _CphaW {
            _CphaW { register: self }
        }
        # [ doc = "Bit 8 - Last bit clock pulse" ]
        pub fn lbcl(&mut self) -> _LbclW {
            _LbclW { register: self }
        }
        # [ doc = "Bit 6 - LIN break detection interrupt enable" ]
        pub fn lbdie(&mut self) -> _LbdieW {
            _LbdieW { register: self }
        }
        # [ doc = "Bit 5 - LIN break detection length" ]
        pub fn lbdl(&mut self) -> _LbdlW {
            _LbdlW { register: self }
        }
        # [ doc = "Bit 4 - 7-bit Address Detection/4-bit Address Detection" ]
        pub fn addm7(&mut self) -> _Addm7W {
            _Addm7W { register: self }
        }
    }
}

# [ doc = "Control register 3" ]
# [ repr ( C ) ]
pub struct Cr3 {
    register: ::volatile_register::RW<u32>,
}

# [ doc = "Control register 3" ]
pub mod cr3 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::Cr3 {
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
    # [ doc = "Value of the field WUFIE" ]
    pub struct WufieR {
        bits: u8,
    }
    impl WufieR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field WUS" ]
    pub struct WusR {
        bits: u8,
    }
    impl WusR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field SCARCNT" ]
    pub struct ScarcntR {
        bits: u8,
    }
    impl ScarcntR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field DEP" ]
    pub struct DepR {
        bits: u8,
    }
    impl DepR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field DEM" ]
    pub struct DemR {
        bits: u8,
    }
    impl DemR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field DDRE" ]
    pub struct DdreR {
        bits: u8,
    }
    impl DdreR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field OVRDIS" ]
    pub struct OvrdisR {
        bits: u8,
    }
    impl OvrdisR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field ONEBIT" ]
    pub struct OnebitR {
        bits: u8,
    }
    impl OnebitR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field CTSIE" ]
    pub struct CtsieR {
        bits: u8,
    }
    impl CtsieR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field CTSE" ]
    pub struct CtseR {
        bits: u8,
    }
    impl CtseR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field RTSE" ]
    pub struct RtseR {
        bits: u8,
    }
    impl RtseR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field DMAT" ]
    pub struct DmatR {
        bits: u8,
    }
    impl DmatR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field DMAR" ]
    pub struct DmarR {
        bits: u8,
    }
    impl DmarR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field SCEN" ]
    pub struct ScenR {
        bits: u8,
    }
    impl ScenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field NACK" ]
    pub struct NackR {
        bits: u8,
    }
    impl NackR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field HDSEL" ]
    pub struct HdselR {
        bits: u8,
    }
    impl HdselR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field IRLP" ]
    pub struct IrlpR {
        bits: u8,
    }
    impl IrlpR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field IREN" ]
    pub struct IrenR {
        bits: u8,
    }
    impl IrenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field EIE" ]
    pub struct EieR {
        bits: u8,
    }
    impl EieR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _WufieW<'a> {
        register: &'a mut W,
    }
    impl<'a> _WufieW<'a> {
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
    pub struct _WusW<'a> {
        register: &'a mut W,
    }
    impl<'a> _WusW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _ScarcntW<'a> {
        register: &'a mut W,
    }
    impl<'a> _ScarcntW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 7;
            const OFFSET: u8 = 17;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _DepW<'a> {
        register: &'a mut W,
    }
    impl<'a> _DepW<'a> {
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
    pub struct _DemW<'a> {
        register: &'a mut W,
    }
    impl<'a> _DemW<'a> {
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
    pub struct _DdreW<'a> {
        register: &'a mut W,
    }
    impl<'a> _DdreW<'a> {
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
    pub struct _OvrdisW<'a> {
        register: &'a mut W,
    }
    impl<'a> _OvrdisW<'a> {
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
    pub struct _OnebitW<'a> {
        register: &'a mut W,
    }
    impl<'a> _OnebitW<'a> {
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
    pub struct _CtsieW<'a> {
        register: &'a mut W,
    }
    impl<'a> _CtsieW<'a> {
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
    pub struct _CtseW<'a> {
        register: &'a mut W,
    }
    impl<'a> _CtseW<'a> {
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
    pub struct _RtseW<'a> {
        register: &'a mut W,
    }
    impl<'a> _RtseW<'a> {
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
    pub struct _DmatW<'a> {
        register: &'a mut W,
    }
    impl<'a> _DmatW<'a> {
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
    pub struct _DmarW<'a> {
        register: &'a mut W,
    }
    impl<'a> _DmarW<'a> {
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
    pub struct _ScenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _ScenW<'a> {
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
    pub struct _NackW<'a> {
        register: &'a mut W,
    }
    impl<'a> _NackW<'a> {
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
    pub struct _HdselW<'a> {
        register: &'a mut W,
    }
    impl<'a> _HdselW<'a> {
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
    pub struct _IrlpW<'a> {
        register: &'a mut W,
    }
    impl<'a> _IrlpW<'a> {
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
    pub struct _IrenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _IrenW<'a> {
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
    pub struct _EieW<'a> {
        register: &'a mut W,
    }
    impl<'a> _EieW<'a> {
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
        fn _wufie(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 22 - Wakeup from Stop mode interrupt enable" ]
        pub fn wufie(&self) -> WufieR {
            WufieR { bits: self._wufie() }
        }
        fn _wus(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 20:21 - Wakeup from Stop mode interrupt flag selection" ]
        pub fn wus(&self) -> WusR {
            WusR { bits: self._wus() }
        }
        fn _scarcnt(&self) -> u8 {
            const MASK: u8 = 7;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 17:19 - Smartcard auto-retry count" ]
        pub fn scarcnt(&self) -> ScarcntR {
            ScarcntR { bits: self._scarcnt() }
        }
        fn _dep(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 15 - Driver enable polarity selection" ]
        pub fn dep(&self) -> DepR {
            DepR { bits: self._dep() }
        }
        fn _dem(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 14 - Driver enable mode" ]
        pub fn dem(&self) -> DemR {
            DemR { bits: self._dem() }
        }
        fn _ddre(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 13 - DMA Disable on Reception Error" ]
        pub fn ddre(&self) -> DdreR {
            DdreR { bits: self._ddre() }
        }
        fn _ovrdis(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 12 - Overrun Disable" ]
        pub fn ovrdis(&self) -> OvrdisR {
            OvrdisR { bits: self._ovrdis() }
        }
        fn _onebit(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 11 - One sample bit method enable" ]
        pub fn onebit(&self) -> OnebitR {
            OnebitR { bits: self._onebit() }
        }
        fn _ctsie(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 10 - CTS interrupt enable" ]
        pub fn ctsie(&self) -> CtsieR {
            CtsieR { bits: self._ctsie() }
        }
        fn _ctse(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 9 - CTS enable" ]
        pub fn ctse(&self) -> CtseR {
            CtseR { bits: self._ctse() }
        }
        fn _rtse(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 8 - RTS enable" ]
        pub fn rtse(&self) -> RtseR {
            RtseR { bits: self._rtse() }
        }
        fn _dmat(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 7 - DMA enable transmitter" ]
        pub fn dmat(&self) -> DmatR {
            DmatR { bits: self._dmat() }
        }
        fn _dmar(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 6 - DMA enable receiver" ]
        pub fn dmar(&self) -> DmarR {
            DmarR { bits: self._dmar() }
        }
        fn _scen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 5 - Smartcard mode enable" ]
        pub fn scen(&self) -> ScenR {
            ScenR { bits: self._scen() }
        }
        fn _nack(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 4 - Smartcard NACK enable" ]
        pub fn nack(&self) -> NackR {
            NackR { bits: self._nack() }
        }
        fn _hdsel(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 3 - Half-duplex selection" ]
        pub fn hdsel(&self) -> HdselR {
            HdselR { bits: self._hdsel() }
        }
        fn _irlp(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 2 - IrDA low-power" ]
        pub fn irlp(&self) -> IrlpR {
            IrlpR { bits: self._irlp() }
        }
        fn _iren(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 1 - IrDA mode enable" ]
        pub fn iren(&self) -> IrenR {
            IrenR { bits: self._iren() }
        }
        fn _eie(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 0 - Error interrupt enable" ]
        pub fn eie(&self) -> EieR {
            EieR { bits: self._eie() }
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
        # [ doc = "Bit 22 - Wakeup from Stop mode interrupt enable" ]
        pub fn wufie(&mut self) -> _WufieW {
            _WufieW { register: self }
        }
        # [ doc = "Bits 20:21 - Wakeup from Stop mode interrupt flag selection" ]
        pub fn wus(&mut self) -> _WusW {
            _WusW { register: self }
        }
        # [ doc = "Bits 17:19 - Smartcard auto-retry count" ]
        pub fn scarcnt(&mut self) -> _ScarcntW {
            _ScarcntW { register: self }
        }
        # [ doc = "Bit 15 - Driver enable polarity selection" ]
        pub fn dep(&mut self) -> _DepW {
            _DepW { register: self }
        }
        # [ doc = "Bit 14 - Driver enable mode" ]
        pub fn dem(&mut self) -> _DemW {
            _DemW { register: self }
        }
        # [ doc = "Bit 13 - DMA Disable on Reception Error" ]
        pub fn ddre(&mut self) -> _DdreW {
            _DdreW { register: self }
        }
        # [ doc = "Bit 12 - Overrun Disable" ]
        pub fn ovrdis(&mut self) -> _OvrdisW {
            _OvrdisW { register: self }
        }
        # [ doc = "Bit 11 - One sample bit method enable" ]
        pub fn onebit(&mut self) -> _OnebitW {
            _OnebitW { register: self }
        }
        # [ doc = "Bit 10 - CTS interrupt enable" ]
        pub fn ctsie(&mut self) -> _CtsieW {
            _CtsieW { register: self }
        }
        # [ doc = "Bit 9 - CTS enable" ]
        pub fn ctse(&mut self) -> _CtseW {
            _CtseW { register: self }
        }
        # [ doc = "Bit 8 - RTS enable" ]
        pub fn rtse(&mut self) -> _RtseW {
            _RtseW { register: self }
        }
        # [ doc = "Bit 7 - DMA enable transmitter" ]
        pub fn dmat(&mut self) -> _DmatW {
            _DmatW { register: self }
        }
        # [ doc = "Bit 6 - DMA enable receiver" ]
        pub fn dmar(&mut self) -> _DmarW {
            _DmarW { register: self }
        }
        # [ doc = "Bit 5 - Smartcard mode enable" ]
        pub fn scen(&mut self) -> _ScenW {
            _ScenW { register: self }
        }
        # [ doc = "Bit 4 - Smartcard NACK enable" ]
        pub fn nack(&mut self) -> _NackW {
            _NackW { register: self }
        }
        # [ doc = "Bit 3 - Half-duplex selection" ]
        pub fn hdsel(&mut self) -> _HdselW {
            _HdselW { register: self }
        }
        # [ doc = "Bit 2 - IrDA low-power" ]
        pub fn irlp(&mut self) -> _IrlpW {
            _IrlpW { register: self }
        }
        # [ doc = "Bit 1 - IrDA mode enable" ]
        pub fn iren(&mut self) -> _IrenW {
            _IrenW { register: self }
        }
        # [ doc = "Bit 0 - Error interrupt enable" ]
        pub fn eie(&mut self) -> _EieW {
            _EieW { register: self }
        }
    }
}

# [ doc = "Baud rate register" ]
# [ repr ( C ) ]
pub struct Brr {
    register: ::volatile_register::RW<u32>,
}

# [ doc = "Baud rate register" ]
pub mod brr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::Brr {
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
    # [ doc = "Value of the field DIV_Mantissa" ]
    pub struct DivMantissaR {
        bits: u16,
    }
    impl DivMantissaR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = "Value of the field DIV_Fraction" ]
    pub struct DivFractionR {
        bits: u8,
    }
    impl DivFractionR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _DivMantissaW<'a> {
        register: &'a mut W,
    }
    impl<'a> _DivMantissaW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u16) -> &'a mut W {
            const MASK: u16 = 4095;
            const OFFSET: u8 = 4;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _DivFractionW<'a> {
        register: &'a mut W,
    }
    impl<'a> _DivFractionW<'a> {
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
        fn _div_mantissa(&self) -> u16 {
            const MASK: u16 = 4095;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        }
        # [ doc = "Bits 4:15 - mantissa of USARTDIV" ]
        pub fn div_mantissa(&self) -> DivMantissaR {
            DivMantissaR { bits: self._div_mantissa() }
        }
        fn _div_fraction(&self) -> u8 {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 0:3 - fraction of USARTDIV" ]
        pub fn div_fraction(&self) -> DivFractionR {
            DivFractionR { bits: self._div_fraction() }
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
        # [ doc = "Bits 4:15 - mantissa of USARTDIV" ]
        pub fn div_mantissa(&mut self) -> _DivMantissaW {
            _DivMantissaW { register: self }
        }
        # [ doc = "Bits 0:3 - fraction of USARTDIV" ]
        pub fn div_fraction(&mut self) -> _DivFractionW {
            _DivFractionW { register: self }
        }
    }
}

# [ doc = "Guard time and prescaler register" ]
# [ repr ( C ) ]
pub struct Gtpr {
    register: ::volatile_register::RW<u32>,
}

# [ doc = "Guard time and prescaler register" ]
pub mod gtpr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::Gtpr {
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
    # [ doc = "Value of the field GT" ]
    pub struct GtR {
        bits: u8,
    }
    impl GtR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field PSC" ]
    pub struct PscR {
        bits: u8,
    }
    impl PscR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _GtW<'a> {
        register: &'a mut W,
    }
    impl<'a> _GtW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PscW<'a> {
        register: &'a mut W,
    }
    impl<'a> _PscW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 255;
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
        fn _gt(&self) -> u8 {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 8:15 - Guard time value" ]
        pub fn gt(&self) -> GtR {
            GtR { bits: self._gt() }
        }
        fn _psc(&self) -> u8 {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 0:7 - Prescaler value" ]
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
        # [ doc = "Bits 8:15 - Guard time value" ]
        pub fn gt(&mut self) -> _GtW {
            _GtW { register: self }
        }
        # [ doc = "Bits 0:7 - Prescaler value" ]
        pub fn psc(&mut self) -> _PscW {
            _PscW { register: self }
        }
    }
}

# [ doc = "Receiver timeout register" ]
# [ repr ( C ) ]
pub struct Rtor {
    register: ::volatile_register::RW<u32>,
}

# [ doc = "Receiver timeout register" ]
pub mod rtor {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::Rtor {
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
    # [ doc = "Value of the field BLEN" ]
    pub struct BlenR {
        bits: u8,
    }
    impl BlenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field RTO" ]
    pub struct RtoR {
        bits: u32,
    }
    impl RtoR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _BlenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _BlenW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _RtoW<'a> {
        register: &'a mut W,
    }
    impl<'a> _RtoW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u32) -> &'a mut W {
            const MASK: u32 = 16777215;
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
        fn _blen(&self) -> u8 {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 24:31 - Block Length" ]
        pub fn blen(&self) -> BlenR {
            BlenR { bits: self._blen() }
        }
        fn _rto(&self) -> u32 {
            const MASK: u32 = 16777215;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        }
        # [ doc = "Bits 0:23 - Receiver timeout value" ]
        pub fn rto(&self) -> RtoR {
            RtoR { bits: self._rto() }
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
        # [ doc = "Bits 24:31 - Block Length" ]
        pub fn blen(&mut self) -> _BlenW {
            _BlenW { register: self }
        }
        # [ doc = "Bits 0:23 - Receiver timeout value" ]
        pub fn rto(&mut self) -> _RtoW {
            _RtoW { register: self }
        }
    }
}

# [ doc = "Request register" ]
# [ repr ( C ) ]
pub struct Rqr {
    register: ::volatile_register::RW<u32>,
}

# [ doc = "Request register" ]
pub mod rqr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::Rqr {
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
    # [ doc = "Value of the field TXFRQ" ]
    pub struct TxfrqR {
        bits: u8,
    }
    impl TxfrqR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field RXFRQ" ]
    pub struct RxfrqR {
        bits: u8,
    }
    impl RxfrqR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field MMRQ" ]
    pub struct MmrqR {
        bits: u8,
    }
    impl MmrqR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field SBKRQ" ]
    pub struct SbkrqR {
        bits: u8,
    }
    impl SbkrqR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field ABRRQ" ]
    pub struct AbrrqR {
        bits: u8,
    }
    impl AbrrqR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _TxfrqW<'a> {
        register: &'a mut W,
    }
    impl<'a> _TxfrqW<'a> {
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
    pub struct _RxfrqW<'a> {
        register: &'a mut W,
    }
    impl<'a> _RxfrqW<'a> {
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
    pub struct _MmrqW<'a> {
        register: &'a mut W,
    }
    impl<'a> _MmrqW<'a> {
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
    pub struct _SbkrqW<'a> {
        register: &'a mut W,
    }
    impl<'a> _SbkrqW<'a> {
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
    pub struct _AbrrqW<'a> {
        register: &'a mut W,
    }
    impl<'a> _AbrrqW<'a> {
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
        fn _txfrq(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 4 - Transmit data flush request" ]
        pub fn txfrq(&self) -> TxfrqR {
            TxfrqR { bits: self._txfrq() }
        }
        fn _rxfrq(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 3 - Receive data flush request" ]
        pub fn rxfrq(&self) -> RxfrqR {
            RxfrqR { bits: self._rxfrq() }
        }
        fn _mmrq(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 2 - Mute mode request" ]
        pub fn mmrq(&self) -> MmrqR {
            MmrqR { bits: self._mmrq() }
        }
        fn _sbkrq(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 1 - Send break request" ]
        pub fn sbkrq(&self) -> SbkrqR {
            SbkrqR { bits: self._sbkrq() }
        }
        fn _abrrq(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 0 - Auto baud rate request" ]
        pub fn abrrq(&self) -> AbrrqR {
            AbrrqR { bits: self._abrrq() }
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
        # [ doc = "Bit 4 - Transmit data flush request" ]
        pub fn txfrq(&mut self) -> _TxfrqW {
            _TxfrqW { register: self }
        }
        # [ doc = "Bit 3 - Receive data flush request" ]
        pub fn rxfrq(&mut self) -> _RxfrqW {
            _RxfrqW { register: self }
        }
        # [ doc = "Bit 2 - Mute mode request" ]
        pub fn mmrq(&mut self) -> _MmrqW {
            _MmrqW { register: self }
        }
        # [ doc = "Bit 1 - Send break request" ]
        pub fn sbkrq(&mut self) -> _SbkrqW {
            _SbkrqW { register: self }
        }
        # [ doc = "Bit 0 - Auto baud rate request" ]
        pub fn abrrq(&mut self) -> _AbrrqW {
            _AbrrqW { register: self }
        }
    }
}

# [ doc = "Interrupt & status register" ]
# [ repr ( C ) ]
pub struct Isr {
    register: ::volatile_register::RO<u32>,
}

# [ doc = "Interrupt & status register" ]
pub mod isr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    impl super::Isr {
        # [ doc = r" Reads the contents of the register" ]
        pub fn read(&self) -> R {
            R { bits: self.register.read() }
        }
    }
    # [ doc = "Value of the field REACK" ]
    pub struct ReackR {
        bits: u8,
    }
    impl ReackR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field TEACK" ]
    pub struct TeackR {
        bits: u8,
    }
    impl TeackR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field WUF" ]
    pub struct WufR {
        bits: u8,
    }
    impl WufR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field RWU" ]
    pub struct RwuR {
        bits: u8,
    }
    impl RwuR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field SBKF" ]
    pub struct SbkfR {
        bits: u8,
    }
    impl SbkfR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field CMF" ]
    pub struct CmfR {
        bits: u8,
    }
    impl CmfR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field BUSY" ]
    pub struct BusyR {
        bits: u8,
    }
    impl BusyR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field ABRF" ]
    pub struct AbrfR {
        bits: u8,
    }
    impl AbrfR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field ABRE" ]
    pub struct AbreR {
        bits: u8,
    }
    impl AbreR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field EOBF" ]
    pub struct EobfR {
        bits: u8,
    }
    impl EobfR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field RTOF" ]
    pub struct RtofR {
        bits: u8,
    }
    impl RtofR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field CTS" ]
    pub struct CtsR {
        bits: u8,
    }
    impl CtsR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field CTSIF" ]
    pub struct CtsifR {
        bits: u8,
    }
    impl CtsifR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field LBDF" ]
    pub struct LbdfR {
        bits: u8,
    }
    impl LbdfR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field TXE" ]
    pub struct TxeR {
        bits: u8,
    }
    impl TxeR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field TC" ]
    pub struct TcR {
        bits: u8,
    }
    impl TcR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field RXNE" ]
    pub struct RxneR {
        bits: u8,
    }
    impl RxneR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field IDLE" ]
    pub struct IdleR {
        bits: u8,
    }
    impl IdleR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field ORE" ]
    pub struct OreR {
        bits: u8,
    }
    impl OreR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field NF" ]
    pub struct NfR {
        bits: u8,
    }
    impl NfR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field FE" ]
    pub struct FeR {
        bits: u8,
    }
    impl FeR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field PE" ]
    pub struct PeR {
        bits: u8,
    }
    impl PeR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        fn _reack(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 22 - Receive enable acknowledge flag" ]
        pub fn reack(&self) -> ReackR {
            ReackR { bits: self._reack() }
        }
        fn _teack(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 21 - Transmit enable acknowledge flag" ]
        pub fn teack(&self) -> TeackR {
            TeackR { bits: self._teack() }
        }
        fn _wuf(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 20 - Wakeup from Stop mode flag" ]
        pub fn wuf(&self) -> WufR {
            WufR { bits: self._wuf() }
        }
        fn _rwu(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 19 - Receiver wakeup from Mute mode" ]
        pub fn rwu(&self) -> RwuR {
            RwuR { bits: self._rwu() }
        }
        fn _sbkf(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 18 - Send break flag" ]
        pub fn sbkf(&self) -> SbkfR {
            SbkfR { bits: self._sbkf() }
        }
        fn _cmf(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 17 - character match flag" ]
        pub fn cmf(&self) -> CmfR {
            CmfR { bits: self._cmf() }
        }
        fn _busy(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 16 - Busy flag" ]
        pub fn busy(&self) -> BusyR {
            BusyR { bits: self._busy() }
        }
        fn _abrf(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 15 - Auto baud rate flag" ]
        pub fn abrf(&self) -> AbrfR {
            AbrfR { bits: self._abrf() }
        }
        fn _abre(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 14 - Auto baud rate error" ]
        pub fn abre(&self) -> AbreR {
            AbreR { bits: self._abre() }
        }
        fn _eobf(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 12 - End of block flag" ]
        pub fn eobf(&self) -> EobfR {
            EobfR { bits: self._eobf() }
        }
        fn _rtof(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 11 - Receiver timeout" ]
        pub fn rtof(&self) -> RtofR {
            RtofR { bits: self._rtof() }
        }
        fn _cts(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 10 - CTS flag" ]
        pub fn cts(&self) -> CtsR {
            CtsR { bits: self._cts() }
        }
        fn _ctsif(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 9 - CTS interrupt flag" ]
        pub fn ctsif(&self) -> CtsifR {
            CtsifR { bits: self._ctsif() }
        }
        fn _lbdf(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 8 - LIN break detection flag" ]
        pub fn lbdf(&self) -> LbdfR {
            LbdfR { bits: self._lbdf() }
        }
        fn _txe(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 7 - Transmit data register empty" ]
        pub fn txe(&self) -> TxeR {
            TxeR { bits: self._txe() }
        }
        fn _tc(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 6 - Transmission complete" ]
        pub fn tc(&self) -> TcR {
            TcR { bits: self._tc() }
        }
        fn _rxne(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 5 - Read data register not empty" ]
        pub fn rxne(&self) -> RxneR {
            RxneR { bits: self._rxne() }
        }
        fn _idle(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 4 - Idle line detected" ]
        pub fn idle(&self) -> IdleR {
            IdleR { bits: self._idle() }
        }
        fn _ore(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 3 - Overrun error" ]
        pub fn ore(&self) -> OreR {
            OreR { bits: self._ore() }
        }
        fn _nf(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 2 - Noise detected flag" ]
        pub fn nf(&self) -> NfR {
            NfR { bits: self._nf() }
        }
        fn _fe(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 1 - Framing error" ]
        pub fn fe(&self) -> FeR {
            FeR { bits: self._fe() }
        }
        fn _pe(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 0 - Parity error" ]
        pub fn pe(&self) -> PeR {
            PeR { bits: self._pe() }
        }
    }
}

# [ doc = "Interrupt flag clear register" ]
# [ repr ( C ) ]
pub struct Icr {
    register: ::volatile_register::RW<u32>,
}

# [ doc = "Interrupt flag clear register" ]
pub mod icr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::Icr {
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
    # [ doc = "Value of the field WUCF" ]
    pub struct WucfR {
        bits: u8,
    }
    impl WucfR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field CMCF" ]
    pub struct CmcfR {
        bits: u8,
    }
    impl CmcfR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field EOBCF" ]
    pub struct EobcfR {
        bits: u8,
    }
    impl EobcfR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field RTOCF" ]
    pub struct RtocfR {
        bits: u8,
    }
    impl RtocfR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field CTSCF" ]
    pub struct CtscfR {
        bits: u8,
    }
    impl CtscfR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field LBDCF" ]
    pub struct LbdcfR {
        bits: u8,
    }
    impl LbdcfR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field TCCF" ]
    pub struct TccfR {
        bits: u8,
    }
    impl TccfR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field IDLECF" ]
    pub struct IdlecfR {
        bits: u8,
    }
    impl IdlecfR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field ORECF" ]
    pub struct OrecfR {
        bits: u8,
    }
    impl OrecfR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field NCF" ]
    pub struct NcfR {
        bits: u8,
    }
    impl NcfR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field FECF" ]
    pub struct FecfR {
        bits: u8,
    }
    impl FecfR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field PECF" ]
    pub struct PecfR {
        bits: u8,
    }
    impl PecfR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _WucfW<'a> {
        register: &'a mut W,
    }
    impl<'a> _WucfW<'a> {
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
    pub struct _CmcfW<'a> {
        register: &'a mut W,
    }
    impl<'a> _CmcfW<'a> {
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
    pub struct _EobcfW<'a> {
        register: &'a mut W,
    }
    impl<'a> _EobcfW<'a> {
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
    pub struct _RtocfW<'a> {
        register: &'a mut W,
    }
    impl<'a> _RtocfW<'a> {
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
    pub struct _CtscfW<'a> {
        register: &'a mut W,
    }
    impl<'a> _CtscfW<'a> {
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
    pub struct _LbdcfW<'a> {
        register: &'a mut W,
    }
    impl<'a> _LbdcfW<'a> {
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
    pub struct _TccfW<'a> {
        register: &'a mut W,
    }
    impl<'a> _TccfW<'a> {
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
    pub struct _IdlecfW<'a> {
        register: &'a mut W,
    }
    impl<'a> _IdlecfW<'a> {
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
    pub struct _OrecfW<'a> {
        register: &'a mut W,
    }
    impl<'a> _OrecfW<'a> {
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
    pub struct _NcfW<'a> {
        register: &'a mut W,
    }
    impl<'a> _NcfW<'a> {
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
    pub struct _FecfW<'a> {
        register: &'a mut W,
    }
    impl<'a> _FecfW<'a> {
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
    pub struct _PecfW<'a> {
        register: &'a mut W,
    }
    impl<'a> _PecfW<'a> {
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
        fn _wucf(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 20 - Wakeup from Stop mode clear flag" ]
        pub fn wucf(&self) -> WucfR {
            WucfR { bits: self._wucf() }
        }
        fn _cmcf(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 17 - Character match clear flag" ]
        pub fn cmcf(&self) -> CmcfR {
            CmcfR { bits: self._cmcf() }
        }
        fn _eobcf(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 12 - End of timeout clear flag" ]
        pub fn eobcf(&self) -> EobcfR {
            EobcfR { bits: self._eobcf() }
        }
        fn _rtocf(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 11 - Receiver timeout clear flag" ]
        pub fn rtocf(&self) -> RtocfR {
            RtocfR { bits: self._rtocf() }
        }
        fn _ctscf(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 9 - CTS clear flag" ]
        pub fn ctscf(&self) -> CtscfR {
            CtscfR { bits: self._ctscf() }
        }
        fn _lbdcf(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 8 - LIN break detection clear flag" ]
        pub fn lbdcf(&self) -> LbdcfR {
            LbdcfR { bits: self._lbdcf() }
        }
        fn _tccf(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 6 - Transmission complete clear flag" ]
        pub fn tccf(&self) -> TccfR {
            TccfR { bits: self._tccf() }
        }
        fn _idlecf(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 4 - Idle line detected clear flag" ]
        pub fn idlecf(&self) -> IdlecfR {
            IdlecfR { bits: self._idlecf() }
        }
        fn _orecf(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 3 - Overrun error clear flag" ]
        pub fn orecf(&self) -> OrecfR {
            OrecfR { bits: self._orecf() }
        }
        fn _ncf(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 2 - Noise detected clear flag" ]
        pub fn ncf(&self) -> NcfR {
            NcfR { bits: self._ncf() }
        }
        fn _fecf(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 1 - Framing error clear flag" ]
        pub fn fecf(&self) -> FecfR {
            FecfR { bits: self._fecf() }
        }
        fn _pecf(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 0 - Parity error clear flag" ]
        pub fn pecf(&self) -> PecfR {
            PecfR { bits: self._pecf() }
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
        # [ doc = "Bit 20 - Wakeup from Stop mode clear flag" ]
        pub fn wucf(&mut self) -> _WucfW {
            _WucfW { register: self }
        }
        # [ doc = "Bit 17 - Character match clear flag" ]
        pub fn cmcf(&mut self) -> _CmcfW {
            _CmcfW { register: self }
        }
        # [ doc = "Bit 12 - End of timeout clear flag" ]
        pub fn eobcf(&mut self) -> _EobcfW {
            _EobcfW { register: self }
        }
        # [ doc = "Bit 11 - Receiver timeout clear flag" ]
        pub fn rtocf(&mut self) -> _RtocfW {
            _RtocfW { register: self }
        }
        # [ doc = "Bit 9 - CTS clear flag" ]
        pub fn ctscf(&mut self) -> _CtscfW {
            _CtscfW { register: self }
        }
        # [ doc = "Bit 8 - LIN break detection clear flag" ]
        pub fn lbdcf(&mut self) -> _LbdcfW {
            _LbdcfW { register: self }
        }
        # [ doc = "Bit 6 - Transmission complete clear flag" ]
        pub fn tccf(&mut self) -> _TccfW {
            _TccfW { register: self }
        }
        # [ doc = "Bit 4 - Idle line detected clear flag" ]
        pub fn idlecf(&mut self) -> _IdlecfW {
            _IdlecfW { register: self }
        }
        # [ doc = "Bit 3 - Overrun error clear flag" ]
        pub fn orecf(&mut self) -> _OrecfW {
            _OrecfW { register: self }
        }
        # [ doc = "Bit 2 - Noise detected clear flag" ]
        pub fn ncf(&mut self) -> _NcfW {
            _NcfW { register: self }
        }
        # [ doc = "Bit 1 - Framing error clear flag" ]
        pub fn fecf(&mut self) -> _FecfW {
            _FecfW { register: self }
        }
        # [ doc = "Bit 0 - Parity error clear flag" ]
        pub fn pecf(&mut self) -> _PecfW {
            _PecfW { register: self }
        }
    }
}

# [ doc = "Receive data register" ]
# [ repr ( C ) ]
pub struct Rdr {
    register: ::volatile_register::RO<u32>,
}

# [ doc = "Receive data register" ]
pub mod rdr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    impl super::Rdr {
        # [ doc = r" Reads the contents of the register" ]
        pub fn read(&self) -> R {
            R { bits: self.register.read() }
        }
    }
    # [ doc = "Value of the field RDR" ]
    pub struct RdrR {
        bits: u16,
    }
    impl RdrR {
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
        fn _rdr(&self) -> u16 {
            const MASK: u16 = 511;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        }
        # [ doc = "Bits 0:8 - Receive data value" ]
        pub fn rdr(&self) -> RdrR {
            RdrR { bits: self._rdr() }
        }
    }
}

# [ doc = "Transmit data register" ]
# [ repr ( C ) ]
pub struct Tdr {
    register: ::volatile_register::RW<u32>,
}

# [ doc = "Transmit data register" ]
pub mod tdr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::Tdr {
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
    # [ doc = "Value of the field TDR" ]
    pub struct TdrR {
        bits: u16,
    }
    impl TdrR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _TdrW<'a> {
        register: &'a mut W,
    }
    impl<'a> _TdrW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u16) -> &'a mut W {
            const MASK: u16 = 511;
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
        fn _tdr(&self) -> u16 {
            const MASK: u16 = 511;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        }
        # [ doc = "Bits 0:8 - Transmit data value" ]
        pub fn tdr(&self) -> TdrR {
            TdrR { bits: self._tdr() }
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
        # [ doc = "Bits 0:8 - Transmit data value" ]
        pub fn tdr(&mut self) -> _TdrW {
            _TdrW { register: self }
        }
    }
}
