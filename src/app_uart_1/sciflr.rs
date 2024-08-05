#[doc = "Register `SCIFLR` reader"]
pub type R = crate::R<SciflrSpec>;
#[doc = "Register `SCIFLR` writer"]
pub type W = crate::W<SciflrSpec>;
#[doc = "Field `BRKDT` reader - 0:0\\]
SCI break-detect flag. This bit is set when the SCI detects a break condition on the SCIRX pin."]
pub type BrkdtR = crate::BitReader;
#[doc = "Field `BRKDT` writer - 0:0\\]
SCI break-detect flag. This bit is set when the SCI detects a break condition on the SCIRX pin."]
pub type BrkdtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAKEUP` reader - 1:1\\]
Wake-up flag. This bit is set by the SCI when receiver or transmitter activity has taken the module out of power-down mode."]
pub type WakeupR = crate::BitReader;
#[doc = "Field `WAKEUP` writer - 1:1\\]
Wake-up flag. This bit is set by the SCI when receiver or transmitter activity has taken the module out of power-down mode."]
pub type WakeupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDLE` reader - 2:2\\]
SCI receiver in idle state. While this bit is set, the SCI looks for an idle period to resynchronize itself with the bit stream."]
pub type IdleR = crate::BitReader;
#[doc = "Field `IDLE` writer - 2:2\\]
SCI receiver in idle state. While this bit is set, the SCI looks for an idle period to resynchronize itself with the bit stream."]
pub type IdleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Bus_busy_flag` reader - 3:3\\]
This bit indicates whether the receiver is in the process of receiving a frame."]
pub type BusBusyFlagR = crate::BitReader;
#[doc = "Field `Bus_busy_flag` writer - 3:3\\]
This bit indicates whether the receiver is in the process of receiving a frame."]
pub type BusBusyFlagW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED1` reader - 7:4\\]
Reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `RESERVED1` writer - 7:4\\]
Reserved"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TXRDY` reader - 8:8\\]
Transmitter buffer register ready flag. When set, this bit indicates that the transmit buffer register (SCITD) is ready to receive another character."]
pub type TxrdyR = crate::BitReader;
#[doc = "Field `TXRDY` writer - 8:8\\]
Transmitter buffer register ready flag. When set, this bit indicates that the transmit buffer register (SCITD) is ready to receive another character."]
pub type TxrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXRDY` reader - 9:9\\]
SCI receiver ready flag. The receiver sets this bit to indicate that the SCIRD contains new data and is ready to be read by the CPU or DMA."]
pub type RxrdyR = crate::BitReader;
#[doc = "Field `RXRDY` writer - 9:9\\]
SCI receiver ready flag. The receiver sets this bit to indicate that the SCIRD contains new data and is ready to be read by the CPU or DMA."]
pub type RxrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXWAKE` reader - 10:10\\]
SCI transmitter wake-up method select. The TXWAKE bit controls whether the data in SCITD should be sent as an address or data frame using multiprocessor communication format"]
pub type TxwakeR = crate::BitReader;
#[doc = "Field `TXWAKE` writer - 10:10\\]
SCI transmitter wake-up method select. The TXWAKE bit controls whether the data in SCITD should be sent as an address or data frame using multiprocessor communication format"]
pub type TxwakeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_EMPTY` reader - 11:11\\]
Transmitter empty flag. The value of this flag indicates the contents of the transmitterΓÇÖs buffer register (SCITD) and shift register (SCITXSHF)"]
pub type TxEmptyR = crate::BitReader;
#[doc = "Field `TX_EMPTY` writer - 11:11\\]
Transmitter empty flag. The value of this flag indicates the contents of the transmitterΓÇÖs buffer register (SCITD) and shift register (SCITXSHF)"]
pub type TxEmptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXWAKE` reader - 12:12\\]
Receiver wake-up detect flag. The SCI sets this bit to indicate that the data currently in SCIRD is an address"]
pub type RxwakeR = crate::BitReader;
#[doc = "Field `RXWAKE` writer - 12:12\\]
Receiver wake-up detect flag. The SCI sets this bit to indicate that the data currently in SCIRD is an address"]
pub type RxwakeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED2` reader - 23:13\\]
Reserved"]
pub type Reserved2R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED2` writer - 23:13\\]
Reserved"]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `PE` reader - 24:24\\]
SCI parity error flag. This bit is set when a parity error is detected in the received data"]
pub type PeR = crate::BitReader;
#[doc = "Field `PE` writer - 24:24\\]
SCI parity error flag. This bit is set when a parity error is detected in the received data"]
pub type PeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OE` reader - 25:25\\]
SCI overrun error flag This bit is set when the transfer of data from SCIRXSHF to SCIRD overwrites unread data already in SCIRD"]
pub type OeR = crate::BitReader;
#[doc = "Field `OE` writer - 25:25\\]
SCI overrun error flag This bit is set when the transfer of data from SCIRXSHF to SCIRD overwrites unread data already in SCIRD"]
pub type OeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FE` reader - 26:26\\]
SCI framing error flag Read: 0=No framing error detected 1=Framing error detected Write: 0=No effect 1=Clears this bit to 0"]
pub type FeR = crate::BitReader;
#[doc = "Field `FE` writer - 26:26\\]
SCI framing error flag Read: 0=No framing error detected 1=Framing error detected Write: 0=No effect 1=Clears this bit to 0"]
pub type FeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED3` reader - 31:27\\]
Reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `RESERVED3` writer - 31:27\\]
Reserved"]
pub type Reserved3W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
SCI break-detect flag. This bit is set when the SCI detects a break condition on the SCIRX pin."]
    #[inline(always)]
    pub fn brkdt(&self) -> BrkdtR {
        BrkdtR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Wake-up flag. This bit is set by the SCI when receiver or transmitter activity has taken the module out of power-down mode."]
    #[inline(always)]
    pub fn wakeup(&self) -> WakeupR {
        WakeupR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
SCI receiver in idle state. While this bit is set, the SCI looks for an idle period to resynchronize itself with the bit stream."]
    #[inline(always)]
    pub fn idle(&self) -> IdleR {
        IdleR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
This bit indicates whether the receiver is in the process of receiving a frame."]
    #[inline(always)]
    pub fn bus_busy_flag(&self) -> BusBusyFlagR {
        BusBusyFlagR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Transmitter buffer register ready flag. When set, this bit indicates that the transmit buffer register (SCITD) is ready to receive another character."]
    #[inline(always)]
    pub fn txrdy(&self) -> TxrdyR {
        TxrdyR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
SCI receiver ready flag. The receiver sets this bit to indicate that the SCIRD contains new data and is ready to be read by the CPU or DMA."]
    #[inline(always)]
    pub fn rxrdy(&self) -> RxrdyR {
        RxrdyR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
SCI transmitter wake-up method select. The TXWAKE bit controls whether the data in SCITD should be sent as an address or data frame using multiprocessor communication format"]
    #[inline(always)]
    pub fn txwake(&self) -> TxwakeR {
        TxwakeR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Transmitter empty flag. The value of this flag indicates the contents of the transmitterΓÇÖs buffer register (SCITD) and shift register (SCITXSHF)"]
    #[inline(always)]
    pub fn tx_empty(&self) -> TxEmptyR {
        TxEmptyR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Receiver wake-up detect flag. The SCI sets this bit to indicate that the data currently in SCIRD is an address"]
    #[inline(always)]
    pub fn rxwake(&self) -> RxwakeR {
        RxwakeR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:23 - 23:13\\]
Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 13) & 0x07ff) as u16)
    }
    #[doc = "Bit 24 - 24:24\\]
SCI parity error flag. This bit is set when a parity error is detected in the received data"]
    #[inline(always)]
    pub fn pe(&self) -> PeR {
        PeR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
SCI overrun error flag This bit is set when the transfer of data from SCIRXSHF to SCIRD overwrites unread data already in SCIRD"]
    #[inline(always)]
    pub fn oe(&self) -> OeR {
        OeR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
SCI framing error flag Read: 0=No framing error detected 1=Framing error detected Write: 0=No effect 1=Clears this bit to 0"]
    #[inline(always)]
    pub fn fe(&self) -> FeR {
        FeR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:31 - 31:27\\]
Reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
SCI break-detect flag. This bit is set when the SCI detects a break condition on the SCIRX pin."]
    #[inline(always)]
    #[must_use]
    pub fn brkdt(&mut self) -> BrkdtW<SciflrSpec> {
        BrkdtW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Wake-up flag. This bit is set by the SCI when receiver or transmitter activity has taken the module out of power-down mode."]
    #[inline(always)]
    #[must_use]
    pub fn wakeup(&mut self) -> WakeupW<SciflrSpec> {
        WakeupW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
SCI receiver in idle state. While this bit is set, the SCI looks for an idle period to resynchronize itself with the bit stream."]
    #[inline(always)]
    #[must_use]
    pub fn idle(&mut self) -> IdleW<SciflrSpec> {
        IdleW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
This bit indicates whether the receiver is in the process of receiving a frame."]
    #[inline(always)]
    #[must_use]
    pub fn bus_busy_flag(&mut self) -> BusBusyFlagW<SciflrSpec> {
        BusBusyFlagW::new(self, 3)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<SciflrSpec> {
        Reserved1W::new(self, 4)
    }
    #[doc = "Bit 8 - 8:8\\]
Transmitter buffer register ready flag. When set, this bit indicates that the transmit buffer register (SCITD) is ready to receive another character."]
    #[inline(always)]
    #[must_use]
    pub fn txrdy(&mut self) -> TxrdyW<SciflrSpec> {
        TxrdyW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
SCI receiver ready flag. The receiver sets this bit to indicate that the SCIRD contains new data and is ready to be read by the CPU or DMA."]
    #[inline(always)]
    #[must_use]
    pub fn rxrdy(&mut self) -> RxrdyW<SciflrSpec> {
        RxrdyW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
SCI transmitter wake-up method select. The TXWAKE bit controls whether the data in SCITD should be sent as an address or data frame using multiprocessor communication format"]
    #[inline(always)]
    #[must_use]
    pub fn txwake(&mut self) -> TxwakeW<SciflrSpec> {
        TxwakeW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Transmitter empty flag. The value of this flag indicates the contents of the transmitterΓÇÖs buffer register (SCITD) and shift register (SCITXSHF)"]
    #[inline(always)]
    #[must_use]
    pub fn tx_empty(&mut self) -> TxEmptyW<SciflrSpec> {
        TxEmptyW::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
Receiver wake-up detect flag. The SCI sets this bit to indicate that the data currently in SCIRD is an address"]
    #[inline(always)]
    #[must_use]
    pub fn rxwake(&mut self) -> RxwakeW<SciflrSpec> {
        RxwakeW::new(self, 12)
    }
    #[doc = "Bits 13:23 - 23:13\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<SciflrSpec> {
        Reserved2W::new(self, 13)
    }
    #[doc = "Bit 24 - 24:24\\]
SCI parity error flag. This bit is set when a parity error is detected in the received data"]
    #[inline(always)]
    #[must_use]
    pub fn pe(&mut self) -> PeW<SciflrSpec> {
        PeW::new(self, 24)
    }
    #[doc = "Bit 25 - 25:25\\]
SCI overrun error flag This bit is set when the transfer of data from SCIRXSHF to SCIRD overwrites unread data already in SCIRD"]
    #[inline(always)]
    #[must_use]
    pub fn oe(&mut self) -> OeW<SciflrSpec> {
        OeW::new(self, 25)
    }
    #[doc = "Bit 26 - 26:26\\]
SCI framing error flag Read: 0=No framing error detected 1=Framing error detected Write: 0=No effect 1=Clears this bit to 0"]
    #[inline(always)]
    #[must_use]
    pub fn fe(&mut self) -> FeW<SciflrSpec> {
        FeW::new(self, 26)
    }
    #[doc = "Bits 27:31 - 31:27\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> Reserved3W<SciflrSpec> {
        Reserved3W::new(self, 27)
    }
}
#[doc = "SCI Flags Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sciflr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sciflr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SciflrSpec;
impl crate::RegisterSpec for SciflrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sciflr::R`](R) reader structure"]
impl crate::Readable for SciflrSpec {}
#[doc = "`write(|w| ..)` method takes [`sciflr::W`](W) writer structure"]
impl crate::Writable for SciflrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCIFLR to value 0"]
impl crate::Resettable for SciflrSpec {
    const RESET_VALUE: u32 = 0;
}
