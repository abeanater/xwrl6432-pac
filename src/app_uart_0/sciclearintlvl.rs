#[doc = "Register `SCICLEARINTLVL` reader"]
pub type R = crate::R<SciclearintlvlSpec>;
#[doc = "Register `SCICLEARINTLVL` writer"]
pub type W = crate::W<SciclearintlvlSpec>;
#[doc = "Field `CLR_BRKDT_INT_LVL` reader - 0:0\\]
Clear Break-detect interrupt level. User and privilege mode (read): 0 = Interrupt level mapped to INT0 line 1 = Interrupt level mapped to INT1 line User and privilege mode (write): 0 = Leaves the corresponding bit unchanged 1 = Reset interrupt level to line INT0"]
pub type ClrBrkdtIntLvlR = crate::BitReader;
#[doc = "Field `CLR_BRKDT_INT_LVL` writer - 0:0\\]
Clear Break-detect interrupt level. User and privilege mode (read): 0 = Interrupt level mapped to INT0 line 1 = Interrupt level mapped to INT1 line User and privilege mode (write): 0 = Leaves the corresponding bit unchanged 1 = Reset interrupt level to line INT0"]
pub type ClrBrkdtIntLvlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLR_WAKEUP_INT_LVL` reader - 1:1\\]
Clear Wake-up interrupt level. User and privilege mode (read): 0 = Interrupt level mapped to INT0 line 1 = Interrupt level mapped to INT1 line User and privilege mode (write): 0 = Leaves the corresponding bit unchanged 1 = Reset interrupt level to line INT0"]
pub type ClrWakeupIntLvlR = crate::BitReader;
#[doc = "Field `CLR_WAKEUP_INT_LVL` writer - 1:1\\]
Clear Wake-up interrupt level. User and privilege mode (read): 0 = Interrupt level mapped to INT0 line 1 = Interrupt level mapped to INT1 line User and privilege mode (write): 0 = Leaves the corresponding bit unchanged 1 = Reset interrupt level to line INT0"]
pub type ClrWakeupIntLvlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED1` reader - 7:2\\]
Reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `RESERVED1` writer - 7:2\\]
Reserved"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `CLR_TX_INT_LVL` reader - 8:8\\]
Clear Transmitter interrupt level. User and privilege mode (read): 0 = Interrupt level mapped to INT0 line 1 = Interrupt level mapped to INT1 line User and privilege mode (write): 0 = Leaves the corresponding bit unchanged 1 = Reset interrupt level to line INT0"]
pub type ClrTxIntLvlR = crate::BitReader;
#[doc = "Field `CLR_TX_INT_LVL` writer - 8:8\\]
Clear Transmitter interrupt level. User and privilege mode (read): 0 = Interrupt level mapped to INT0 line 1 = Interrupt level mapped to INT1 line User and privilege mode (write): 0 = Leaves the corresponding bit unchanged 1 = Reset interrupt level to line INT0"]
pub type ClrTxIntLvlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLR_RX_INT_LVL` reader - 9:9\\]
Clear Receiver interrupt level. User and privilege mode (read): 0 = Interrupt level mapped to INT0 line 1 = Interrupt level mapped to INT1 line User and privilege mode (write): 0 = Leaves the corresponding bit unchanged 1 = Reset interrupt level to line INT0"]
pub type ClrRxIntLvlR = crate::BitReader;
#[doc = "Field `CLR_RX_INT_LVL` writer - 9:9\\]
Clear Receiver interrupt level. User and privilege mode (read): 0 = Interrupt level mapped to INT0 line 1 = Interrupt level mapped to INT1 line User and privilege mode (write): 0 = Leaves the corresponding bit unchanged 1 = Reset interrupt level to line INT0"]
pub type ClrRxIntLvlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED2` reader - 14:10\\]
Reserved"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `RESERVED2` writer - 14:10\\]
Reserved"]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CLR_INC_BR_INT_LVL` reader - "]
pub type ClrIncBrIntLvlR = crate::BitReader;
#[doc = "Field `CLR_INC_BR_INT_LVL` writer - "]
pub type ClrIncBrIntLvlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED3` reader - 17:16\\]
Reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `RESERVED3` writer - 17:16\\]
Reserved"]
pub type Reserved3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CLR_RX_DMA_ALL_INT_LVL` reader - 18:18\\]
Clear receive DMA ALL interrupt level. User and privilege mode (read): 0 = RX interrupt request for address frames is mapped to INT0 line. 1 = RX interrupt request for address frames is mapped to INT1 line. User and privilege mode (write): 0 = Leaves the corresponding bit unchanged. 1 = Reset interrupt level to line INT0."]
pub type ClrRxDmaAllIntLvlR = crate::BitReader;
#[doc = "Field `CLR_RX_DMA_ALL_INT_LVL` writer - 18:18\\]
Clear receive DMA ALL interrupt level. User and privilege mode (read): 0 = RX interrupt request for address frames is mapped to INT0 line. 1 = RX interrupt request for address frames is mapped to INT1 line. User and privilege mode (write): 0 = Leaves the corresponding bit unchanged. 1 = Reset interrupt level to line INT0."]
pub type ClrRxDmaAllIntLvlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED4` reader - 23:19\\]
Reserved"]
pub type Reserved4R = crate::FieldReader;
#[doc = "Field `RESERVED4` writer - 23:19\\]
Reserved"]
pub type Reserved4W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CLR_PE_INT_LVL` reader - 24:24\\]
Clear Framing-Error Interrupt Level. User and privilege mode (read): 0 = Interrupt level mapped to INT0 line 1 = Interrupt level mapped to INT1 line User and privilege mode (write): 0 = Leaves the corresponding bit unchanged 1 = Reset interrupt level to line INT0"]
pub type ClrPeIntLvlR = crate::BitReader;
#[doc = "Field `CLR_PE_INT_LVL` writer - 24:24\\]
Clear Framing-Error Interrupt Level. User and privilege mode (read): 0 = Interrupt level mapped to INT0 line 1 = Interrupt level mapped to INT1 line User and privilege mode (write): 0 = Leaves the corresponding bit unchanged 1 = Reset interrupt level to line INT0"]
pub type ClrPeIntLvlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLR_OE_INT_LVL` reader - 25:25\\]
Clear Framing-Error Interrupt Level. User and privilege mode (read): 0 = Interrupt level mapped to INT0 line 1 = Interrupt level mapped to INT1 line User and privilege mode (write): 0 = Leaves the corresponding bit unchanged 1 = Reset interrupt level to line INT0"]
pub type ClrOeIntLvlR = crate::BitReader;
#[doc = "Field `CLR_OE_INT_LVL` writer - 25:25\\]
Clear Framing-Error Interrupt Level. User and privilege mode (read): 0 = Interrupt level mapped to INT0 line 1 = Interrupt level mapped to INT1 line User and privilege mode (write): 0 = Leaves the corresponding bit unchanged 1 = Reset interrupt level to line INT0"]
pub type ClrOeIntLvlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLR_FE_INT_LVL` reader - 26:26\\]
Clear Framing-Error Interrupt Level. User and privilege mode (read): 0 = Interrupt level mapped to INT0 line 1 = Interrupt level mapped to INT1 line User and privilege mode (write): 0 = Leaves the corresponding bit unchanged 1 = Reset interrupt level to line INT0"]
pub type ClrFeIntLvlR = crate::BitReader;
#[doc = "Field `CLR_FE_INT_LVL` writer - 26:26\\]
Clear Framing-Error Interrupt Level. User and privilege mode (read): 0 = Interrupt level mapped to INT0 line 1 = Interrupt level mapped to INT1 line User and privilege mode (write): 0 = Leaves the corresponding bit unchanged 1 = Reset interrupt level to line INT0"]
pub type ClrFeIntLvlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED5` reader - 31:27\\]
Reserved"]
pub type Reserved5R = crate::FieldReader;
#[doc = "Field `RESERVED5` writer - 31:27\\]
Reserved"]
pub type Reserved5W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Clear Break-detect interrupt level. User and privilege mode (read): 0 = Interrupt level mapped to INT0 line 1 = Interrupt level mapped to INT1 line User and privilege mode (write): 0 = Leaves the corresponding bit unchanged 1 = Reset interrupt level to line INT0"]
    #[inline(always)]
    pub fn clr_brkdt_int_lvl(&self) -> ClrBrkdtIntLvlR {
        ClrBrkdtIntLvlR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Clear Wake-up interrupt level. User and privilege mode (read): 0 = Interrupt level mapped to INT0 line 1 = Interrupt level mapped to INT1 line User and privilege mode (write): 0 = Leaves the corresponding bit unchanged 1 = Reset interrupt level to line INT0"]
    #[inline(always)]
    pub fn clr_wakeup_int_lvl(&self) -> ClrWakeupIntLvlR {
        ClrWakeupIntLvlR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - 7:2\\]
Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Clear Transmitter interrupt level. User and privilege mode (read): 0 = Interrupt level mapped to INT0 line 1 = Interrupt level mapped to INT1 line User and privilege mode (write): 0 = Leaves the corresponding bit unchanged 1 = Reset interrupt level to line INT0"]
    #[inline(always)]
    pub fn clr_tx_int_lvl(&self) -> ClrTxIntLvlR {
        ClrTxIntLvlR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Clear Receiver interrupt level. User and privilege mode (read): 0 = Interrupt level mapped to INT0 line 1 = Interrupt level mapped to INT1 line User and privilege mode (write): 0 = Leaves the corresponding bit unchanged 1 = Reset interrupt level to line INT0"]
    #[inline(always)]
    pub fn clr_rx_int_lvl(&self) -> ClrRxIntLvlR {
        ClrRxIntLvlR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:14 - 14:10\\]
Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn clr_inc_br_int_lvl(&self) -> ClrIncBrIntLvlR {
        ClrIncBrIntLvlR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - 18:18\\]
Clear receive DMA ALL interrupt level. User and privilege mode (read): 0 = RX interrupt request for address frames is mapped to INT0 line. 1 = RX interrupt request for address frames is mapped to INT1 line. User and privilege mode (write): 0 = Leaves the corresponding bit unchanged. 1 = Reset interrupt level to line INT0."]
    #[inline(always)]
    pub fn clr_rx_dma_all_int_lvl(&self) -> ClrRxDmaAllIntLvlR {
        ClrRxDmaAllIntLvlR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:23 - 23:19\\]
Reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Clear Framing-Error Interrupt Level. User and privilege mode (read): 0 = Interrupt level mapped to INT0 line 1 = Interrupt level mapped to INT1 line User and privilege mode (write): 0 = Leaves the corresponding bit unchanged 1 = Reset interrupt level to line INT0"]
    #[inline(always)]
    pub fn clr_pe_int_lvl(&self) -> ClrPeIntLvlR {
        ClrPeIntLvlR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Clear Framing-Error Interrupt Level. User and privilege mode (read): 0 = Interrupt level mapped to INT0 line 1 = Interrupt level mapped to INT1 line User and privilege mode (write): 0 = Leaves the corresponding bit unchanged 1 = Reset interrupt level to line INT0"]
    #[inline(always)]
    pub fn clr_oe_int_lvl(&self) -> ClrOeIntLvlR {
        ClrOeIntLvlR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
Clear Framing-Error Interrupt Level. User and privilege mode (read): 0 = Interrupt level mapped to INT0 line 1 = Interrupt level mapped to INT1 line User and privilege mode (write): 0 = Leaves the corresponding bit unchanged 1 = Reset interrupt level to line INT0"]
    #[inline(always)]
    pub fn clr_fe_int_lvl(&self) -> ClrFeIntLvlR {
        ClrFeIntLvlR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:31 - 31:27\\]
Reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Clear Break-detect interrupt level. User and privilege mode (read): 0 = Interrupt level mapped to INT0 line 1 = Interrupt level mapped to INT1 line User and privilege mode (write): 0 = Leaves the corresponding bit unchanged 1 = Reset interrupt level to line INT0"]
    #[inline(always)]
    #[must_use]
    pub fn clr_brkdt_int_lvl(&mut self) -> ClrBrkdtIntLvlW<SciclearintlvlSpec> {
        ClrBrkdtIntLvlW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Clear Wake-up interrupt level. User and privilege mode (read): 0 = Interrupt level mapped to INT0 line 1 = Interrupt level mapped to INT1 line User and privilege mode (write): 0 = Leaves the corresponding bit unchanged 1 = Reset interrupt level to line INT0"]
    #[inline(always)]
    #[must_use]
    pub fn clr_wakeup_int_lvl(&mut self) -> ClrWakeupIntLvlW<SciclearintlvlSpec> {
        ClrWakeupIntLvlW::new(self, 1)
    }
    #[doc = "Bits 2:7 - 7:2\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<SciclearintlvlSpec> {
        Reserved1W::new(self, 2)
    }
    #[doc = "Bit 8 - 8:8\\]
Clear Transmitter interrupt level. User and privilege mode (read): 0 = Interrupt level mapped to INT0 line 1 = Interrupt level mapped to INT1 line User and privilege mode (write): 0 = Leaves the corresponding bit unchanged 1 = Reset interrupt level to line INT0"]
    #[inline(always)]
    #[must_use]
    pub fn clr_tx_int_lvl(&mut self) -> ClrTxIntLvlW<SciclearintlvlSpec> {
        ClrTxIntLvlW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Clear Receiver interrupt level. User and privilege mode (read): 0 = Interrupt level mapped to INT0 line 1 = Interrupt level mapped to INT1 line User and privilege mode (write): 0 = Leaves the corresponding bit unchanged 1 = Reset interrupt level to line INT0"]
    #[inline(always)]
    #[must_use]
    pub fn clr_rx_int_lvl(&mut self) -> ClrRxIntLvlW<SciclearintlvlSpec> {
        ClrRxIntLvlW::new(self, 9)
    }
    #[doc = "Bits 10:14 - 14:10\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<SciclearintlvlSpec> {
        Reserved2W::new(self, 10)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn clr_inc_br_int_lvl(&mut self) -> ClrIncBrIntLvlW<SciclearintlvlSpec> {
        ClrIncBrIntLvlW::new(self, 15)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> Reserved3W<SciclearintlvlSpec> {
        Reserved3W::new(self, 16)
    }
    #[doc = "Bit 18 - 18:18\\]
Clear receive DMA ALL interrupt level. User and privilege mode (read): 0 = RX interrupt request for address frames is mapped to INT0 line. 1 = RX interrupt request for address frames is mapped to INT1 line. User and privilege mode (write): 0 = Leaves the corresponding bit unchanged. 1 = Reset interrupt level to line INT0."]
    #[inline(always)]
    #[must_use]
    pub fn clr_rx_dma_all_int_lvl(&mut self) -> ClrRxDmaAllIntLvlW<SciclearintlvlSpec> {
        ClrRxDmaAllIntLvlW::new(self, 18)
    }
    #[doc = "Bits 19:23 - 23:19\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> Reserved4W<SciclearintlvlSpec> {
        Reserved4W::new(self, 19)
    }
    #[doc = "Bit 24 - 24:24\\]
Clear Framing-Error Interrupt Level. User and privilege mode (read): 0 = Interrupt level mapped to INT0 line 1 = Interrupt level mapped to INT1 line User and privilege mode (write): 0 = Leaves the corresponding bit unchanged 1 = Reset interrupt level to line INT0"]
    #[inline(always)]
    #[must_use]
    pub fn clr_pe_int_lvl(&mut self) -> ClrPeIntLvlW<SciclearintlvlSpec> {
        ClrPeIntLvlW::new(self, 24)
    }
    #[doc = "Bit 25 - 25:25\\]
Clear Framing-Error Interrupt Level. User and privilege mode (read): 0 = Interrupt level mapped to INT0 line 1 = Interrupt level mapped to INT1 line User and privilege mode (write): 0 = Leaves the corresponding bit unchanged 1 = Reset interrupt level to line INT0"]
    #[inline(always)]
    #[must_use]
    pub fn clr_oe_int_lvl(&mut self) -> ClrOeIntLvlW<SciclearintlvlSpec> {
        ClrOeIntLvlW::new(self, 25)
    }
    #[doc = "Bit 26 - 26:26\\]
Clear Framing-Error Interrupt Level. User and privilege mode (read): 0 = Interrupt level mapped to INT0 line 1 = Interrupt level mapped to INT1 line User and privilege mode (write): 0 = Leaves the corresponding bit unchanged 1 = Reset interrupt level to line INT0"]
    #[inline(always)]
    #[must_use]
    pub fn clr_fe_int_lvl(&mut self) -> ClrFeIntLvlW<SciclearintlvlSpec> {
        ClrFeIntLvlW::new(self, 26)
    }
    #[doc = "Bits 27:31 - 31:27\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved5(&mut self) -> Reserved5W<SciclearintlvlSpec> {
        Reserved5W::new(self, 27)
    }
}
#[doc = "SCI Clear Interrupt Level Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sciclearintlvl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sciclearintlvl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SciclearintlvlSpec;
impl crate::RegisterSpec for SciclearintlvlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sciclearintlvl::R`](R) reader structure"]
impl crate::Readable for SciclearintlvlSpec {}
#[doc = "`write(|w| ..)` method takes [`sciclearintlvl::W`](W) writer structure"]
impl crate::Writable for SciclearintlvlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCICLEARINTLVL to value 0"]
impl crate::Resettable for SciclearintlvlSpec {
    const RESET_VALUE: u32 = 0;
}
