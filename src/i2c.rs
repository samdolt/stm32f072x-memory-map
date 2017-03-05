# ! [ doc = "Inter-integrated circuit" ]
# [ doc = r" Register block" ]
# [ repr ( C ) ]
pub struct I2c {
    # [ doc = "0x00 - Control register 1" ]
    pub cr1: Cr1,
    # [ doc = "0x04 - Control register 2" ]
    pub cr2: Cr2,
    # [ doc = "0x08 - Own address register 1" ]
    pub oar1: Oar1,
    # [ doc = "0x0c - Own address register 2" ]
    pub oar2: Oar2,
    # [ doc = "0x10 - Timing register" ]
    pub timingr: Timingr,
    # [ doc = "0x14 - Status register 1" ]
    pub timeoutr: Timeoutr,
    # [ doc = "0x18 - Interrupt and Status register" ]
    pub isr: Isr,
    # [ doc = "0x1c - Interrupt clear register" ]
    pub icr: Icr,
    # [ doc = "0x20 - PEC register" ]
    pub pecr: Pecr,
    # [ doc = "0x24 - Receive data register" ]
    pub rxdr: Rxdr,
    # [ doc = "0x28 - Transmit data register" ]
    pub txdr: Txdr,
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
    # [ doc = "Value of the field TXIE" ]
    pub struct TxieR {
        bits: u8,
    }
    impl TxieR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field RXIE" ]
    pub struct RxieR {
        bits: u8,
    }
    impl RxieR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field ADDRIE" ]
    pub struct AddrieR {
        bits: u8,
    }
    impl AddrieR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field NACKIE" ]
    pub struct NackieR {
        bits: u8,
    }
    impl NackieR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field STOPIE" ]
    pub struct StopieR {
        bits: u8,
    }
    impl StopieR {
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
    # [ doc = "Value of the field ERRIE" ]
    pub struct ErrieR {
        bits: u8,
    }
    impl ErrieR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field DNF" ]
    pub struct DnfR {
        bits: u8,
    }
    impl DnfR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field ANFOFF" ]
    pub struct AnfoffR {
        bits: u8,
    }
    impl AnfoffR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field TXDMAEN" ]
    pub struct TxdmaenR {
        bits: u8,
    }
    impl TxdmaenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field RXDMAEN" ]
    pub struct RxdmaenR {
        bits: u8,
    }
    impl RxdmaenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field SBC" ]
    pub struct SbcR {
        bits: u8,
    }
    impl SbcR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field NOSTRETCH" ]
    pub struct NostretchR {
        bits: u8,
    }
    impl NostretchR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field WUPEN" ]
    pub struct WupenR {
        bits: u8,
    }
    impl WupenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field GCEN" ]
    pub struct GcenR {
        bits: u8,
    }
    impl GcenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field SMBHEN" ]
    pub struct SmbhenR {
        bits: u8,
    }
    impl SmbhenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field SMBDEN" ]
    pub struct SmbdenR {
        bits: u8,
    }
    impl SmbdenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field ALERTEN" ]
    pub struct AlertenR {
        bits: u8,
    }
    impl AlertenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field PECEN" ]
    pub struct PecenR {
        bits: u8,
    }
    impl PecenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PeW<'a> {
        register: &'a mut W,
    }
    impl<'a> _PeW<'a> {
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
    pub struct _TxieW<'a> {
        register: &'a mut W,
    }
    impl<'a> _TxieW<'a> {
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
    pub struct _RxieW<'a> {
        register: &'a mut W,
    }
    impl<'a> _RxieW<'a> {
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
    pub struct _AddrieW<'a> {
        register: &'a mut W,
    }
    impl<'a> _AddrieW<'a> {
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
    pub struct _NackieW<'a> {
        register: &'a mut W,
    }
    impl<'a> _NackieW<'a> {
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
    pub struct _StopieW<'a> {
        register: &'a mut W,
    }
    impl<'a> _StopieW<'a> {
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
    pub struct _ErrieW<'a> {
        register: &'a mut W,
    }
    impl<'a> _ErrieW<'a> {
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
    pub struct _DnfW<'a> {
        register: &'a mut W,
    }
    impl<'a> _DnfW<'a> {
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
    pub struct _AnfoffW<'a> {
        register: &'a mut W,
    }
    impl<'a> _AnfoffW<'a> {
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
    pub struct _SwrstW<'a> {
        register: &'a mut W,
    }
    impl<'a> _SwrstW<'a> {
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
    pub struct _TxdmaenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _TxdmaenW<'a> {
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
    pub struct _RxdmaenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _RxdmaenW<'a> {
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
    pub struct _SbcW<'a> {
        register: &'a mut W,
    }
    impl<'a> _SbcW<'a> {
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
    pub struct _NostretchW<'a> {
        register: &'a mut W,
    }
    impl<'a> _NostretchW<'a> {
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
    pub struct _WupenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _WupenW<'a> {
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
    pub struct _GcenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _GcenW<'a> {
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
    pub struct _SmbhenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _SmbhenW<'a> {
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
    pub struct _SmbdenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _SmbdenW<'a> {
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
    pub struct _AlertenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _AlertenW<'a> {
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
    pub struct _PecenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _PecenW<'a> {
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
        fn _pe(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 0 - Peripheral enable" ]
        pub fn pe(&self) -> PeR {
            PeR { bits: self._pe() }
        }
        fn _txie(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 1 - TX Interrupt enable" ]
        pub fn txie(&self) -> TxieR {
            TxieR { bits: self._txie() }
        }
        fn _rxie(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 2 - RX Interrupt enable" ]
        pub fn rxie(&self) -> RxieR {
            RxieR { bits: self._rxie() }
        }
        fn _addrie(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 3 - Address match interrupt enable (slave only)" ]
        pub fn addrie(&self) -> AddrieR {
            AddrieR { bits: self._addrie() }
        }
        fn _nackie(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 4 - Not acknowledge received interrupt enable" ]
        pub fn nackie(&self) -> NackieR {
            NackieR { bits: self._nackie() }
        }
        fn _stopie(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 5 - STOP detection Interrupt enable" ]
        pub fn stopie(&self) -> StopieR {
            StopieR { bits: self._stopie() }
        }
        fn _tcie(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 6 - Transfer Complete interrupt enable" ]
        pub fn tcie(&self) -> TcieR {
            TcieR { bits: self._tcie() }
        }
        fn _errie(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 7 - Error interrupts enable" ]
        pub fn errie(&self) -> ErrieR {
            ErrieR { bits: self._errie() }
        }
        fn _dnf(&self) -> u8 {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 8:11 - Digital noise filter" ]
        pub fn dnf(&self) -> DnfR {
            DnfR { bits: self._dnf() }
        }
        fn _anfoff(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 12 - Analog noise filter OFF" ]
        pub fn anfoff(&self) -> AnfoffR {
            AnfoffR { bits: self._anfoff() }
        }
        fn _txdmaen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 14 - DMA transmission requests enable" ]
        pub fn txdmaen(&self) -> TxdmaenR {
            TxdmaenR { bits: self._txdmaen() }
        }
        fn _rxdmaen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 15 - DMA reception requests enable" ]
        pub fn rxdmaen(&self) -> RxdmaenR {
            RxdmaenR { bits: self._rxdmaen() }
        }
        fn _sbc(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 16 - Slave byte control" ]
        pub fn sbc(&self) -> SbcR {
            SbcR { bits: self._sbc() }
        }
        fn _nostretch(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 17 - Clock stretching disable" ]
        pub fn nostretch(&self) -> NostretchR {
            NostretchR { bits: self._nostretch() }
        }
        fn _wupen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 18 - Wakeup from STOP enable" ]
        pub fn wupen(&self) -> WupenR {
            WupenR { bits: self._wupen() }
        }
        fn _gcen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 19 - General call enable" ]
        pub fn gcen(&self) -> GcenR {
            GcenR { bits: self._gcen() }
        }
        fn _smbhen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 20 - SMBus Host address enable" ]
        pub fn smbhen(&self) -> SmbhenR {
            SmbhenR { bits: self._smbhen() }
        }
        fn _smbden(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 21 - SMBus Device Default address enable" ]
        pub fn smbden(&self) -> SmbdenR {
            SmbdenR { bits: self._smbden() }
        }
        fn _alerten(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 22 - SMBUS alert enable" ]
        pub fn alerten(&self) -> AlertenR {
            AlertenR { bits: self._alerten() }
        }
        fn _pecen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 23 - PEC enable" ]
        pub fn pecen(&self) -> PecenR {
            PecenR { bits: self._pecen() }
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
        # [ doc = "Bit 0 - Peripheral enable" ]
        pub fn pe(&mut self) -> _PeW {
            _PeW { register: self }
        }
        # [ doc = "Bit 1 - TX Interrupt enable" ]
        pub fn txie(&mut self) -> _TxieW {
            _TxieW { register: self }
        }
        # [ doc = "Bit 2 - RX Interrupt enable" ]
        pub fn rxie(&mut self) -> _RxieW {
            _RxieW { register: self }
        }
        # [ doc = "Bit 3 - Address match interrupt enable (slave only)" ]
        pub fn addrie(&mut self) -> _AddrieW {
            _AddrieW { register: self }
        }
        # [ doc = "Bit 4 - Not acknowledge received interrupt enable" ]
        pub fn nackie(&mut self) -> _NackieW {
            _NackieW { register: self }
        }
        # [ doc = "Bit 5 - STOP detection Interrupt enable" ]
        pub fn stopie(&mut self) -> _StopieW {
            _StopieW { register: self }
        }
        # [ doc = "Bit 6 - Transfer Complete interrupt enable" ]
        pub fn tcie(&mut self) -> _TcieW {
            _TcieW { register: self }
        }
        # [ doc = "Bit 7 - Error interrupts enable" ]
        pub fn errie(&mut self) -> _ErrieW {
            _ErrieW { register: self }
        }
        # [ doc = "Bits 8:11 - Digital noise filter" ]
        pub fn dnf(&mut self) -> _DnfW {
            _DnfW { register: self }
        }
        # [ doc = "Bit 12 - Analog noise filter OFF" ]
        pub fn anfoff(&mut self) -> _AnfoffW {
            _AnfoffW { register: self }
        }
        # [ doc = "Bit 13 - Software reset" ]
        pub fn swrst(&mut self) -> _SwrstW {
            _SwrstW { register: self }
        }
        # [ doc = "Bit 14 - DMA transmission requests enable" ]
        pub fn txdmaen(&mut self) -> _TxdmaenW {
            _TxdmaenW { register: self }
        }
        # [ doc = "Bit 15 - DMA reception requests enable" ]
        pub fn rxdmaen(&mut self) -> _RxdmaenW {
            _RxdmaenW { register: self }
        }
        # [ doc = "Bit 16 - Slave byte control" ]
        pub fn sbc(&mut self) -> _SbcW {
            _SbcW { register: self }
        }
        # [ doc = "Bit 17 - Clock stretching disable" ]
        pub fn nostretch(&mut self) -> _NostretchW {
            _NostretchW { register: self }
        }
        # [ doc = "Bit 18 - Wakeup from STOP enable" ]
        pub fn wupen(&mut self) -> _WupenW {
            _WupenW { register: self }
        }
        # [ doc = "Bit 19 - General call enable" ]
        pub fn gcen(&mut self) -> _GcenW {
            _GcenW { register: self }
        }
        # [ doc = "Bit 20 - SMBus Host address enable" ]
        pub fn smbhen(&mut self) -> _SmbhenW {
            _SmbhenW { register: self }
        }
        # [ doc = "Bit 21 - SMBus Device Default address enable" ]
        pub fn smbden(&mut self) -> _SmbdenW {
            _SmbdenW { register: self }
        }
        # [ doc = "Bit 22 - SMBUS alert enable" ]
        pub fn alerten(&mut self) -> _AlertenW {
            _AlertenW { register: self }
        }
        # [ doc = "Bit 23 - PEC enable" ]
        pub fn pecen(&mut self) -> _PecenW {
            _PecenW { register: self }
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
    # [ doc = "Value of the field PECBYTE" ]
    pub struct PecbyteR {
        bits: u8,
    }
    impl PecbyteR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field AUTOEND" ]
    pub struct AutoendR {
        bits: u8,
    }
    impl AutoendR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field RELOAD" ]
    pub struct ReloadR {
        bits: u8,
    }
    impl ReloadR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field NBYTES" ]
    pub struct NbytesR {
        bits: u8,
    }
    impl NbytesR {
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
    # [ doc = "Value of the field START" ]
    pub struct StartR {
        bits: u8,
    }
    impl StartR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field HEAD10R" ]
    pub struct Head10rR {
        bits: u8,
    }
    impl Head10rR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field ADD10" ]
    pub struct Add10R {
        bits: u8,
    }
    impl Add10R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field RD_WRN" ]
    pub struct RdWrnR {
        bits: u8,
    }
    impl RdWrnR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field SADD8" ]
    pub struct Sadd8R {
        bits: u8,
    }
    impl Sadd8R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field SADD1" ]
    pub struct Sadd1R {
        bits: u8,
    }
    impl Sadd1R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field SADD0" ]
    pub struct Sadd0R {
        bits: u8,
    }
    impl Sadd0R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PecbyteW<'a> {
        register: &'a mut W,
    }
    impl<'a> _PecbyteW<'a> {
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
    pub struct _AutoendW<'a> {
        register: &'a mut W,
    }
    impl<'a> _AutoendW<'a> {
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
    pub struct _ReloadW<'a> {
        register: &'a mut W,
    }
    impl<'a> _ReloadW<'a> {
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
    pub struct _NbytesW<'a> {
        register: &'a mut W,
    }
    impl<'a> _NbytesW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
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
            const OFFSET: u8 = 15;
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
            const MASK: u8 = 1;
            const OFFSET: u8 = 14;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _StartW<'a> {
        register: &'a mut W,
    }
    impl<'a> _StartW<'a> {
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
    pub struct _Head10rW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Head10rW<'a> {
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
    pub struct _Add10W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Add10W<'a> {
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
    pub struct _RdWrnW<'a> {
        register: &'a mut W,
    }
    impl<'a> _RdWrnW<'a> {
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
    pub struct _Sadd8W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Sadd8W<'a> {
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
    pub struct _Sadd1W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Sadd1W<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 127;
            const OFFSET: u8 = 1;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Sadd0W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Sadd0W<'a> {
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
        fn _pecbyte(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 26 - Packet error checking byte" ]
        pub fn pecbyte(&self) -> PecbyteR {
            PecbyteR { bits: self._pecbyte() }
        }
        fn _autoend(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 25 - Automatic end mode (master mode)" ]
        pub fn autoend(&self) -> AutoendR {
            AutoendR { bits: self._autoend() }
        }
        fn _reload(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 24 - NBYTES reload mode" ]
        pub fn reload(&self) -> ReloadR {
            ReloadR { bits: self._reload() }
        }
        fn _nbytes(&self) -> u8 {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 16:23 - Number of bytes" ]
        pub fn nbytes(&self) -> NbytesR {
            NbytesR { bits: self._nbytes() }
        }
        fn _nack(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 15 - NACK generation (slave mode)" ]
        pub fn nack(&self) -> NackR {
            NackR { bits: self._nack() }
        }
        fn _stop(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 14 - Stop generation (master mode)" ]
        pub fn stop(&self) -> StopR {
            StopR { bits: self._stop() }
        }
        fn _start(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 13 - Start generation" ]
        pub fn start(&self) -> StartR {
            StartR { bits: self._start() }
        }
        fn _head10r(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 12 - 10-bit address header only read direction (master receiver mode)" ]
        pub fn head10r(&self) -> Head10rR {
            Head10rR { bits: self._head10r() }
        }
        fn _add10(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 11 - 10-bit addressing mode (master mode)" ]
        pub fn add10(&self) -> Add10R {
            Add10R { bits: self._add10() }
        }
        fn _rd_wrn(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 10 - Transfer direction (master mode)" ]
        pub fn rd_wrn(&self) -> RdWrnR {
            RdWrnR { bits: self._rd_wrn() }
        }
        fn _sadd8(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 8:9 - Slave address bit 9:8 (master mode)" ]
        pub fn sadd8(&self) -> Sadd8R {
            Sadd8R { bits: self._sadd8() }
        }
        fn _sadd1(&self) -> u8 {
            const MASK: u8 = 127;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 1:7 - Slave address bit 7:1 (master mode)" ]
        pub fn sadd1(&self) -> Sadd1R {
            Sadd1R { bits: self._sadd1() }
        }
        fn _sadd0(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 0 - Slave address bit 0 (master mode)" ]
        pub fn sadd0(&self) -> Sadd0R {
            Sadd0R { bits: self._sadd0() }
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
        # [ doc = "Bit 26 - Packet error checking byte" ]
        pub fn pecbyte(&mut self) -> _PecbyteW {
            _PecbyteW { register: self }
        }
        # [ doc = "Bit 25 - Automatic end mode (master mode)" ]
        pub fn autoend(&mut self) -> _AutoendW {
            _AutoendW { register: self }
        }
        # [ doc = "Bit 24 - NBYTES reload mode" ]
        pub fn reload(&mut self) -> _ReloadW {
            _ReloadW { register: self }
        }
        # [ doc = "Bits 16:23 - Number of bytes" ]
        pub fn nbytes(&mut self) -> _NbytesW {
            _NbytesW { register: self }
        }
        # [ doc = "Bit 15 - NACK generation (slave mode)" ]
        pub fn nack(&mut self) -> _NackW {
            _NackW { register: self }
        }
        # [ doc = "Bit 14 - Stop generation (master mode)" ]
        pub fn stop(&mut self) -> _StopW {
            _StopW { register: self }
        }
        # [ doc = "Bit 13 - Start generation" ]
        pub fn start(&mut self) -> _StartW {
            _StartW { register: self }
        }
        # [ doc = "Bit 12 - 10-bit address header only read direction (master receiver mode)" ]
        pub fn head10r(&mut self) -> _Head10rW {
            _Head10rW { register: self }
        }
        # [ doc = "Bit 11 - 10-bit addressing mode (master mode)" ]
        pub fn add10(&mut self) -> _Add10W {
            _Add10W { register: self }
        }
        # [ doc = "Bit 10 - Transfer direction (master mode)" ]
        pub fn rd_wrn(&mut self) -> _RdWrnW {
            _RdWrnW { register: self }
        }
        # [ doc = "Bits 8:9 - Slave address bit 9:8 (master mode)" ]
        pub fn sadd8(&mut self) -> _Sadd8W {
            _Sadd8W { register: self }
        }
        # [ doc = "Bits 1:7 - Slave address bit 7:1 (master mode)" ]
        pub fn sadd1(&mut self) -> _Sadd1W {
            _Sadd1W { register: self }
        }
        # [ doc = "Bit 0 - Slave address bit 0 (master mode)" ]
        pub fn sadd0(&mut self) -> _Sadd0W {
            _Sadd0W { register: self }
        }
    }
}

# [ doc = "Own address register 1" ]
# [ repr ( C ) ]
pub struct Oar1 {
    register: ::volatile_register::RW<u32>,
}

# [ doc = "Own address register 1" ]
pub mod oar1 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::Oar1 {
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
    # [ doc = "Value of the field OA1_0" ]
    pub struct Oa10R {
        bits: u8,
    }
    impl Oa10R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field OA1_1" ]
    pub struct Oa11R {
        bits: u8,
    }
    impl Oa11R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field OA1_8" ]
    pub struct Oa18R {
        bits: u8,
    }
    impl Oa18R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field OA1MODE" ]
    pub struct Oa1modeR {
        bits: u8,
    }
    impl Oa1modeR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field OA1EN" ]
    pub struct Oa1enR {
        bits: u8,
    }
    impl Oa1enR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Oa10W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Oa10W<'a> {
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
    pub struct _Oa11W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Oa11W<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 127;
            const OFFSET: u8 = 1;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Oa18W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Oa18W<'a> {
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
    pub struct _Oa1modeW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Oa1modeW<'a> {
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
    pub struct _Oa1enW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Oa1enW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 15;
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
        fn _oa1_0(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 0 - Interface address" ]
        pub fn oa1_0(&self) -> Oa10R {
            Oa10R { bits: self._oa1_0() }
        }
        fn _oa1_1(&self) -> u8 {
            const MASK: u8 = 127;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 1:7 - Interface address" ]
        pub fn oa1_1(&self) -> Oa11R {
            Oa11R { bits: self._oa1_1() }
        }
        fn _oa1_8(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 8:9 - Interface address" ]
        pub fn oa1_8(&self) -> Oa18R {
            Oa18R { bits: self._oa1_8() }
        }
        fn _oa1mode(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 10 - Own Address 1 10-bit mode" ]
        pub fn oa1mode(&self) -> Oa1modeR {
            Oa1modeR { bits: self._oa1mode() }
        }
        fn _oa1en(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 15 - Own Address 1 enable" ]
        pub fn oa1en(&self) -> Oa1enR {
            Oa1enR { bits: self._oa1en() }
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
        # [ doc = "Bit 0 - Interface address" ]
        pub fn oa1_0(&mut self) -> _Oa10W {
            _Oa10W { register: self }
        }
        # [ doc = "Bits 1:7 - Interface address" ]
        pub fn oa1_1(&mut self) -> _Oa11W {
            _Oa11W { register: self }
        }
        # [ doc = "Bits 8:9 - Interface address" ]
        pub fn oa1_8(&mut self) -> _Oa18W {
            _Oa18W { register: self }
        }
        # [ doc = "Bit 10 - Own Address 1 10-bit mode" ]
        pub fn oa1mode(&mut self) -> _Oa1modeW {
            _Oa1modeW { register: self }
        }
        # [ doc = "Bit 15 - Own Address 1 enable" ]
        pub fn oa1en(&mut self) -> _Oa1enW {
            _Oa1enW { register: self }
        }
    }
}

# [ doc = "Own address register 2" ]
# [ repr ( C ) ]
pub struct Oar2 {
    register: ::volatile_register::RW<u32>,
}

# [ doc = "Own address register 2" ]
pub mod oar2 {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::Oar2 {
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
    # [ doc = "Value of the field OA2" ]
    pub struct Oa2R {
        bits: u8,
    }
    impl Oa2R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field OA2MSK" ]
    pub struct Oa2mskR {
        bits: u8,
    }
    impl Oa2mskR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field OA2EN" ]
    pub struct Oa2enR {
        bits: u8,
    }
    impl Oa2enR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Oa2W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Oa2W<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 127;
            const OFFSET: u8 = 1;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Oa2mskW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Oa2mskW<'a> {
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
    pub struct _Oa2enW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Oa2enW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 15;
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
        fn _oa2(&self) -> u8 {
            const MASK: u8 = 127;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 1:7 - Interface address" ]
        pub fn oa2(&self) -> Oa2R {
            Oa2R { bits: self._oa2() }
        }
        fn _oa2msk(&self) -> u8 {
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 8:10 - Own Address 2 masks" ]
        pub fn oa2msk(&self) -> Oa2mskR {
            Oa2mskR { bits: self._oa2msk() }
        }
        fn _oa2en(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 15 - Own Address 2 enable" ]
        pub fn oa2en(&self) -> Oa2enR {
            Oa2enR { bits: self._oa2en() }
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
        # [ doc = "Bits 1:7 - Interface address" ]
        pub fn oa2(&mut self) -> _Oa2W {
            _Oa2W { register: self }
        }
        # [ doc = "Bits 8:10 - Own Address 2 masks" ]
        pub fn oa2msk(&mut self) -> _Oa2mskW {
            _Oa2mskW { register: self }
        }
        # [ doc = "Bit 15 - Own Address 2 enable" ]
        pub fn oa2en(&mut self) -> _Oa2enW {
            _Oa2enW { register: self }
        }
    }
}

# [ doc = "Timing register" ]
# [ repr ( C ) ]
pub struct Timingr {
    register: ::volatile_register::RW<u32>,
}

# [ doc = "Timing register" ]
pub mod timingr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::Timingr {
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
    # [ doc = "Value of the field SCLL" ]
    pub struct ScllR {
        bits: u8,
    }
    impl ScllR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field SCLH" ]
    pub struct SclhR {
        bits: u8,
    }
    impl SclhR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field SDADEL" ]
    pub struct SdadelR {
        bits: u8,
    }
    impl SdadelR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field SCLDEL" ]
    pub struct ScldelR {
        bits: u8,
    }
    impl ScldelR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field PRESC" ]
    pub struct PrescR {
        bits: u8,
    }
    impl PrescR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _ScllW<'a> {
        register: &'a mut W,
    }
    impl<'a> _ScllW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _SclhW<'a> {
        register: &'a mut W,
    }
    impl<'a> _SclhW<'a> {
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
    pub struct _SdadelW<'a> {
        register: &'a mut W,
    }
    impl<'a> _SdadelW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _ScldelW<'a> {
        register: &'a mut W,
    }
    impl<'a> _ScldelW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _PrescW<'a> {
        register: &'a mut W,
    }
    impl<'a> _PrescW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 15;
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
        fn _scll(&self) -> u8 {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 0:7 - SCL low period (master mode)" ]
        pub fn scll(&self) -> ScllR {
            ScllR { bits: self._scll() }
        }
        fn _sclh(&self) -> u8 {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 8:15 - SCL high period (master mode)" ]
        pub fn sclh(&self) -> SclhR {
            SclhR { bits: self._sclh() }
        }
        fn _sdadel(&self) -> u8 {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 16:19 - Data hold time" ]
        pub fn sdadel(&self) -> SdadelR {
            SdadelR { bits: self._sdadel() }
        }
        fn _scldel(&self) -> u8 {
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 20:23 - Data setup time" ]
        pub fn scldel(&self) -> ScldelR {
            ScldelR { bits: self._scldel() }
        }
        fn _presc(&self) -> u8 {
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 28:31 - Timing prescaler" ]
        pub fn presc(&self) -> PrescR {
            PrescR { bits: self._presc() }
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
        # [ doc = "Bits 0:7 - SCL low period (master mode)" ]
        pub fn scll(&mut self) -> _ScllW {
            _ScllW { register: self }
        }
        # [ doc = "Bits 8:15 - SCL high period (master mode)" ]
        pub fn sclh(&mut self) -> _SclhW {
            _SclhW { register: self }
        }
        # [ doc = "Bits 16:19 - Data hold time" ]
        pub fn sdadel(&mut self) -> _SdadelW {
            _SdadelW { register: self }
        }
        # [ doc = "Bits 20:23 - Data setup time" ]
        pub fn scldel(&mut self) -> _ScldelW {
            _ScldelW { register: self }
        }
        # [ doc = "Bits 28:31 - Timing prescaler" ]
        pub fn presc(&mut self) -> _PrescW {
            _PrescW { register: self }
        }
    }
}

# [ doc = "Status register 1" ]
# [ repr ( C ) ]
pub struct Timeoutr {
    register: ::volatile_register::RW<u32>,
}

# [ doc = "Status register 1" ]
pub mod timeoutr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::Timeoutr {
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
    # [ doc = "Value of the field TIMEOUTA" ]
    pub struct TimeoutaR {
        bits: u16,
    }
    impl TimeoutaR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = "Value of the field TIDLE" ]
    pub struct TidleR {
        bits: u8,
    }
    impl TidleR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field TIMOUTEN" ]
    pub struct TimoutenR {
        bits: u8,
    }
    impl TimoutenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field TIMEOUTB" ]
    pub struct TimeoutbR {
        bits: u16,
    }
    impl TimeoutbR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = "Value of the field TEXTEN" ]
    pub struct TextenR {
        bits: u8,
    }
    impl TextenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _TimeoutaW<'a> {
        register: &'a mut W,
    }
    impl<'a> _TimeoutaW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u16) -> &'a mut W {
            const MASK: u16 = 4095;
            const OFFSET: u8 = 0;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _TidleW<'a> {
        register: &'a mut W,
    }
    impl<'a> _TidleW<'a> {
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
    pub struct _TimoutenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _TimoutenW<'a> {
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
    pub struct _TimeoutbW<'a> {
        register: &'a mut W,
    }
    impl<'a> _TimeoutbW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u16) -> &'a mut W {
            const MASK: u16 = 4095;
            const OFFSET: u8 = 16;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _TextenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _TextenW<'a> {
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
        fn _timeouta(&self) -> u16 {
            const MASK: u16 = 4095;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        }
        # [ doc = "Bits 0:11 - Bus timeout A" ]
        pub fn timeouta(&self) -> TimeoutaR {
            TimeoutaR { bits: self._timeouta() }
        }
        fn _tidle(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 12 - Idle clock timeout detection" ]
        pub fn tidle(&self) -> TidleR {
            TidleR { bits: self._tidle() }
        }
        fn _timouten(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 15 - Clock timeout enable" ]
        pub fn timouten(&self) -> TimoutenR {
            TimoutenR { bits: self._timouten() }
        }
        fn _timeoutb(&self) -> u16 {
            const MASK: u16 = 4095;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        }
        # [ doc = "Bits 16:27 - Bus timeout B" ]
        pub fn timeoutb(&self) -> TimeoutbR {
            TimeoutbR { bits: self._timeoutb() }
        }
        fn _texten(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 31 - Extended clock timeout enable" ]
        pub fn texten(&self) -> TextenR {
            TextenR { bits: self._texten() }
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
        # [ doc = "Bits 0:11 - Bus timeout A" ]
        pub fn timeouta(&mut self) -> _TimeoutaW {
            _TimeoutaW { register: self }
        }
        # [ doc = "Bit 12 - Idle clock timeout detection" ]
        pub fn tidle(&mut self) -> _TidleW {
            _TidleW { register: self }
        }
        # [ doc = "Bit 15 - Clock timeout enable" ]
        pub fn timouten(&mut self) -> _TimoutenW {
            _TimoutenW { register: self }
        }
        # [ doc = "Bits 16:27 - Bus timeout B" ]
        pub fn timeoutb(&mut self) -> _TimeoutbW {
            _TimeoutbW { register: self }
        }
        # [ doc = "Bit 31 - Extended clock timeout enable" ]
        pub fn texten(&mut self) -> _TextenW {
            _TextenW { register: self }
        }
    }
}

# [ doc = "Interrupt and Status register" ]
# [ repr ( C ) ]
pub struct Isr {
    register: ::volatile_register::RW<u32>,
}

# [ doc = "Interrupt and Status register" ]
pub mod isr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::Isr {
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
    # [ doc = "Value of the field ADDCODE" ]
    pub struct AddcodeR {
        bits: u8,
    }
    impl AddcodeR {
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
    # [ doc = "Value of the field ALERT" ]
    pub struct AlertR {
        bits: u8,
    }
    impl AlertR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field TIMEOUT" ]
    pub struct TimeoutR {
        bits: u8,
    }
    impl TimeoutR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field PECERR" ]
    pub struct PecerrR {
        bits: u8,
    }
    impl PecerrR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field OVR" ]
    pub struct OvrR {
        bits: u8,
    }
    impl OvrR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field ARLO" ]
    pub struct ArloR {
        bits: u8,
    }
    impl ArloR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field BERR" ]
    pub struct BerrR {
        bits: u8,
    }
    impl BerrR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field TCR" ]
    pub struct TcrR {
        bits: u8,
    }
    impl TcrR {
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
    # [ doc = "Value of the field STOPF" ]
    pub struct StopfR {
        bits: u8,
    }
    impl StopfR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field NACKF" ]
    pub struct NackfR {
        bits: u8,
    }
    impl NackfR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field ADDR" ]
    pub struct AddrR {
        bits: u8,
    }
    impl AddrR {
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
    # [ doc = "Value of the field TXIS" ]
    pub struct TxisR {
        bits: u8,
    }
    impl TxisR {
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
    # [ doc = r" Proxy" ]
    pub struct _TxisW<'a> {
        register: &'a mut W,
    }
    impl<'a> _TxisW<'a> {
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
    pub struct _TxeW<'a> {
        register: &'a mut W,
    }
    impl<'a> _TxeW<'a> {
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
        fn _addcode(&self) -> u8 {
            const MASK: u8 = 127;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 17:23 - Address match code (Slave mode)" ]
        pub fn addcode(&self) -> AddcodeR {
            AddcodeR { bits: self._addcode() }
        }
        fn _dir(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 16 - Transfer direction (Slave mode)" ]
        pub fn dir(&self) -> DirR {
            DirR { bits: self._dir() }
        }
        fn _busy(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 15 - Bus busy" ]
        pub fn busy(&self) -> BusyR {
            BusyR { bits: self._busy() }
        }
        fn _alert(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 13 - SMBus alert" ]
        pub fn alert(&self) -> AlertR {
            AlertR { bits: self._alert() }
        }
        fn _timeout(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 12 - Timeout or t_low detection flag" ]
        pub fn timeout(&self) -> TimeoutR {
            TimeoutR { bits: self._timeout() }
        }
        fn _pecerr(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 11 - PEC Error in reception" ]
        pub fn pecerr(&self) -> PecerrR {
            PecerrR { bits: self._pecerr() }
        }
        fn _ovr(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 10 - Overrun/Underrun (slave mode)" ]
        pub fn ovr(&self) -> OvrR {
            OvrR { bits: self._ovr() }
        }
        fn _arlo(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 9 - Arbitration lost" ]
        pub fn arlo(&self) -> ArloR {
            ArloR { bits: self._arlo() }
        }
        fn _berr(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 8 - Bus error" ]
        pub fn berr(&self) -> BerrR {
            BerrR { bits: self._berr() }
        }
        fn _tcr(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 7 - Transfer Complete Reload" ]
        pub fn tcr(&self) -> TcrR {
            TcrR { bits: self._tcr() }
        }
        fn _tc(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 6 - Transfer Complete (master mode)" ]
        pub fn tc(&self) -> TcR {
            TcR { bits: self._tc() }
        }
        fn _stopf(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 5 - Stop detection flag" ]
        pub fn stopf(&self) -> StopfR {
            StopfR { bits: self._stopf() }
        }
        fn _nackf(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 4 - Not acknowledge received flag" ]
        pub fn nackf(&self) -> NackfR {
            NackfR { bits: self._nackf() }
        }
        fn _addr(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 3 - Address matched (slave mode)" ]
        pub fn addr(&self) -> AddrR {
            AddrR { bits: self._addr() }
        }
        fn _rxne(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 2 - Receive data register not empty (receivers)" ]
        pub fn rxne(&self) -> RxneR {
            RxneR { bits: self._rxne() }
        }
        fn _txis(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 1 - Transmit interrupt status (transmitters)" ]
        pub fn txis(&self) -> TxisR {
            TxisR { bits: self._txis() }
        }
        fn _txe(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 0 - Transmit data register empty (transmitters)" ]
        pub fn txe(&self) -> TxeR {
            TxeR { bits: self._txe() }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        pub fn reset_value() -> W {
            W { bits: 1 }
        }
        # [ doc = r" Writes raw `bits` to the register" ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bit 1 - Transmit interrupt status (transmitters)" ]
        pub fn txis(&mut self) -> _TxisW {
            _TxisW { register: self }
        }
        # [ doc = "Bit 0 - Transmit data register empty (transmitters)" ]
        pub fn txe(&mut self) -> _TxeW {
            _TxeW { register: self }
        }
    }
}

# [ doc = "Interrupt clear register" ]
# [ repr ( C ) ]
pub struct Icr {
    register: ::volatile_register::WO<u32>,
}

# [ doc = "Interrupt clear register" ]
pub mod icr {
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::Icr {
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
    pub struct _AlertcfW<'a> {
        register: &'a mut W,
    }
    impl<'a> _AlertcfW<'a> {
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
    pub struct _TimoutcfW<'a> {
        register: &'a mut W,
    }
    impl<'a> _TimoutcfW<'a> {
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
    pub struct _PeccfW<'a> {
        register: &'a mut W,
    }
    impl<'a> _PeccfW<'a> {
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
    pub struct _OvrcfW<'a> {
        register: &'a mut W,
    }
    impl<'a> _OvrcfW<'a> {
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
    pub struct _ArlocfW<'a> {
        register: &'a mut W,
    }
    impl<'a> _ArlocfW<'a> {
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
    pub struct _BerrcfW<'a> {
        register: &'a mut W,
    }
    impl<'a> _BerrcfW<'a> {
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
    pub struct _StopcfW<'a> {
        register: &'a mut W,
    }
    impl<'a> _StopcfW<'a> {
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
    pub struct _NackcfW<'a> {
        register: &'a mut W,
    }
    impl<'a> _NackcfW<'a> {
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
    pub struct _AddrcfW<'a> {
        register: &'a mut W,
    }
    impl<'a> _AddrcfW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 3;
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
        # [ doc = "Bit 13 - Alert flag clear" ]
        pub fn alertcf(&mut self) -> _AlertcfW {
            _AlertcfW { register: self }
        }
        # [ doc = "Bit 12 - Timeout detection flag clear" ]
        pub fn timoutcf(&mut self) -> _TimoutcfW {
            _TimoutcfW { register: self }
        }
        # [ doc = "Bit 11 - PEC Error flag clear" ]
        pub fn peccf(&mut self) -> _PeccfW {
            _PeccfW { register: self }
        }
        # [ doc = "Bit 10 - Overrun/Underrun flag clear" ]
        pub fn ovrcf(&mut self) -> _OvrcfW {
            _OvrcfW { register: self }
        }
        # [ doc = "Bit 9 - Arbitration lost flag clear" ]
        pub fn arlocf(&mut self) -> _ArlocfW {
            _ArlocfW { register: self }
        }
        # [ doc = "Bit 8 - Bus error flag clear" ]
        pub fn berrcf(&mut self) -> _BerrcfW {
            _BerrcfW { register: self }
        }
        # [ doc = "Bit 5 - Stop detection flag clear" ]
        pub fn stopcf(&mut self) -> _StopcfW {
            _StopcfW { register: self }
        }
        # [ doc = "Bit 4 - Not Acknowledge flag clear" ]
        pub fn nackcf(&mut self) -> _NackcfW {
            _NackcfW { register: self }
        }
        # [ doc = "Bit 3 - Address Matched flag clear" ]
        pub fn addrcf(&mut self) -> _AddrcfW {
            _AddrcfW { register: self }
        }
    }
}

# [ doc = "PEC register" ]
# [ repr ( C ) ]
pub struct Pecr {
    register: ::volatile_register::RO<u32>,
}

# [ doc = "PEC register" ]
pub mod pecr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    impl super::Pecr {
        # [ doc = r" Reads the contents of the register" ]
        pub fn read(&self) -> R {
            R { bits: self.register.read() }
        }
    }
    # [ doc = "Value of the field PEC" ]
    pub struct PecR {
        bits: u8,
    }
    impl PecR {
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
        fn _pec(&self) -> u8 {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 0:7 - Packet error checking register" ]
        pub fn pec(&self) -> PecR {
            PecR { bits: self._pec() }
        }
    }
}

# [ doc = "Receive data register" ]
# [ repr ( C ) ]
pub struct Rxdr {
    register: ::volatile_register::RO<u32>,
}

# [ doc = "Receive data register" ]
pub mod rxdr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    impl super::Rxdr {
        # [ doc = r" Reads the contents of the register" ]
        pub fn read(&self) -> R {
            R { bits: self.register.read() }
        }
    }
    # [ doc = "Value of the field RXDATA" ]
    pub struct RxdataR {
        bits: u8,
    }
    impl RxdataR {
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
        fn _rxdata(&self) -> u8 {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 0:7 - 8-bit receive data" ]
        pub fn rxdata(&self) -> RxdataR {
            RxdataR { bits: self._rxdata() }
        }
    }
}

# [ doc = "Transmit data register" ]
# [ repr ( C ) ]
pub struct Txdr {
    register: ::volatile_register::RW<u32>,
}

# [ doc = "Transmit data register" ]
pub mod txdr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::Txdr {
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
    # [ doc = "Value of the field TXDATA" ]
    pub struct TxdataR {
        bits: u8,
    }
    impl TxdataR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _TxdataW<'a> {
        register: &'a mut W,
    }
    impl<'a> _TxdataW<'a> {
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
        fn _txdata(&self) -> u8 {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 0:7 - 8-bit transmit data" ]
        pub fn txdata(&self) -> TxdataR {
            TxdataR { bits: self._txdata() }
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
        # [ doc = "Bits 0:7 - 8-bit transmit data" ]
        pub fn txdata(&mut self) -> _TxdataW {
            _TxdataW { register: self }
        }
    }
}
