#[doc = "Register `SCISETINTLVL` reader"]
pub type R = crate::R<ScisetintlvlSpec>;
#[doc = "Register `SCISETINTLVL` writer"]
pub type W = crate::W<ScisetintlvlSpec>;
#[doc = "Field `SET_BRKDT_INT_LVL` reader - 0:0\\]
Clear Break-detect interrupt Level. User and privilege mode (read): 0 = Interrupt level mapped to INT0 line 1 = Interrupt level mapped to INT1 line User and privilege mode (write): 0 = Leaves the corresponding bit unchanged 1 = Clear interrupt level to line INT1"]
pub type SetBrkdtIntLvlR = crate::BitReader;
#[doc = "Field `SET_BRKDT_INT_LVL` writer - 0:0\\]
Clear Break-detect interrupt Level. User and privilege mode (read): 0 = Interrupt level mapped to INT0 line 1 = Interrupt level mapped to INT1 line User and privilege mode (write): 0 = Leaves the corresponding bit unchanged 1 = Clear interrupt level to line INT1"]
pub type SetBrkdtIntLvlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SET_WAKEUP_INT_LVL` reader - 1:1\\]
Clear Wake-up interrupt Level. User and privilege mode (read): 0 = Interrupt level mapped to INT0 line 1 = Interrupt level mapped to INT1 line User and privilege mode (write): 0 = Leaves the corresponding bit unchanged 1 = Clear interrupt level to line INT1"]
pub type SetWakeupIntLvlR = crate::BitReader;
#[doc = "Field `SET_WAKEUP_INT_LVL` writer - 1:1\\]
Clear Wake-up interrupt Level. User and privilege mode (read): 0 = Interrupt level mapped to INT0 line 1 = Interrupt level mapped to INT1 line User and privilege mode (write): 0 = Leaves the corresponding bit unchanged 1 = Clear interrupt level to line INT1"]
pub type SetWakeupIntLvlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED1` reader - 7:2\\]
Reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `RESERVED1` writer - 7:2\\]
Reserved"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `SET_TX_INT_LVL` reader - 8:8\\]
Clear Transmitter interrupt Level. User and privilege mode (read): 0 = Interrupt level mapped to INT0 line 1 = Interrupt level mapped to INT1 line User and privilege mode (write): 0 = Leaves the corresponding bit unchanged 1 = Clear interrupt level to line INT1"]
pub type SetTxIntLvlR = crate::BitReader;
#[doc = "Field `SET_TX_INT_LVL` writer - 8:8\\]
Clear Transmitter interrupt Level. User and privilege mode (read): 0 = Interrupt level mapped to INT0 line 1 = Interrupt level mapped to INT1 line User and privilege mode (write): 0 = Leaves the corresponding bit unchanged 1 = Clear interrupt level to line INT1"]
pub type SetTxIntLvlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SET_RX_INT_LVL` reader - 9:9\\]
Clear Receiver interrupt Level. User and privilege mode (read): 0 = Interrupt level mapped to INT0 line 1 = Interrupt level mapped to INT1 line User and privilege mode (write): 0 = Leaves the corresponding bit unchanged 1 = Clear interrupt level to line INT1"]
pub type SetRxIntLvlR = crate::BitReader;
#[doc = "Field `SET_RX_INT_LVL` writer - 9:9\\]
Clear Receiver interrupt Level. User and privilege mode (read): 0 = Interrupt level mapped to INT0 line 1 = Interrupt level mapped to INT1 line User and privilege mode (write): 0 = Leaves the corresponding bit unchanged 1 = Clear interrupt level to line INT1"]
pub type SetRxIntLvlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED2` reader - 14:10\\]
Reserved"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `RESERVED2` writer - 14:10\\]
Reserved"]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SET_INC_BR_INT_LVL` reader - 15:15\\]
Reserved"]
pub type SetIncBrIntLvlR = crate::BitReader;
#[doc = "Field `SET_INC_BR_INT_LVL` writer - 15:15\\]
Reserved"]
pub type SetIncBrIntLvlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED3` reader - 17:16\\]
Reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `RESERVED3` writer - 17:16\\]
Reserved"]
pub type Reserved3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SET_RX_DMA_ALL_INT_LVL` reader - 18:18\\]
User and privilege mode (read): 0 = RX interrupt request for address frames mapped to INT0 line. 1 = RX interrupt request for address frames mapped to INT1 line. User and privilege mode (write): 0 = Leaves the corresponding bit unchanged 1 = Clear interrupt level to line INT1"]
pub type SetRxDmaAllIntLvlR = crate::BitReader;
#[doc = "Field `SET_RX_DMA_ALL_INT_LVL` writer - 18:18\\]
User and privilege mode (read): 0 = RX interrupt request for address frames mapped to INT0 line. 1 = RX interrupt request for address frames mapped to INT1 line. User and privilege mode (write): 0 = Leaves the corresponding bit unchanged 1 = Clear interrupt level to line INT1"]
pub type SetRxDmaAllIntLvlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED4` reader - 23:19\\]
Reserved"]
pub type Reserved4R = crate::FieldReader;
#[doc = "Field `RESERVED4` writer - 23:19\\]
Reserved"]
pub type Reserved4W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SET_PE_INT_LVL` reader - 24:24\\]
Clear Parity Error Interrupt Level. User and privilege mode (read): 0 = Interrupt level mapped to INT0 line 1 = Interrupt level mapped to INT1 line User and privilege mode (write): 0 = Leaves the corresponding bit unchanged 1 = Clear interrupt level to line INT1"]
pub type SetPeIntLvlR = crate::BitReader;
#[doc = "Field `SET_PE_INT_LVL` writer - 24:24\\]
Clear Parity Error Interrupt Level. User and privilege mode (read): 0 = Interrupt level mapped to INT0 line 1 = Interrupt level mapped to INT1 line User and privilege mode (write): 0 = Leaves the corresponding bit unchanged 1 = Clear interrupt level to line INT1"]
pub type SetPeIntLvlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SET_OE_INT_LVL` reader - 25:25\\]
Clear Overrun-Error Interrupt Level. User and privilege mode (read): 0 = Interrupt level mapped to INT0 line 1 = Interrupt level mapped to INT1 line User and privilege mode (write): 0 = Leaves the corresponding bit unchanged 1 = Clear interrupt level to line INT1"]
pub type SetOeIntLvlR = crate::BitReader;
#[doc = "Field `SET_OE_INT_LVL` writer - 25:25\\]
Clear Overrun-Error Interrupt Level. User and privilege mode (read): 0 = Interrupt level mapped to INT0 line 1 = Interrupt level mapped to INT1 line User and privilege mode (write): 0 = Leaves the corresponding bit unchanged 1 = Clear interrupt level to line INT1"]
pub type SetOeIntLvlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SET_FE_INT_LVL` reader - 26:26\\]
Clear Framing-Error Interrupt Level. User and privilege mode (read): 0 = Interrupt level mapped to INT0 line 1 = Interrupt level mapped to INT1 line User and privilege mode (write): 0 = Leaves the corresponding bit unchanged 1 = Clear interrupt level to line INT1"]
pub type SetFeIntLvlR = crate::BitReader;
#[doc = "Field `SET_FE_INT_LVL` writer - 26:26\\]
Clear Framing-Error Interrupt Level. User and privilege mode (read): 0 = Interrupt level mapped to INT0 line 1 = Interrupt level mapped to INT1 line User and privilege mode (write): 0 = Leaves the corresponding bit unchanged 1 = Clear interrupt level to line INT1"]
pub type SetFeIntLvlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED5` reader - 31:27\\]
Reserved"]
pub type Reserved5R = crate::FieldReader;
#[doc = "Field `RESERVED5` writer - 31:27\\]
Reserved"]
pub type Reserved5W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Clear Break-detect interrupt Level. User and privilege mode (read): 0 = Interrupt level mapped to INT0 line 1 = Interrupt level mapped to INT1 line User and privilege mode (write): 0 = Leaves the corresponding bit unchanged 1 = Clear interrupt level to line INT1"]
    #[inline(always)]
    pub fn set_brkdt_int_lvl(&self) -> SetBrkdtIntLvlR {
        SetBrkdtIntLvlR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Clear Wake-up interrupt Level. User and privilege mode (read): 0 = Interrupt level mapped to INT0 line 1 = Interrupt level mapped to INT1 line User and privilege mode (write): 0 = Leaves the corresponding bit unchanged 1 = Clear interrupt level to line INT1"]
    #[inline(always)]
    pub fn set_wakeup_int_lvl(&self) -> SetWakeupIntLvlR {
        SetWakeupIntLvlR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - 7:2\\]
Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Clear Transmitter interrupt Level. User and privilege mode (read): 0 = Interrupt level mapped to INT0 line 1 = Interrupt level mapped to INT1 line User and privilege mode (write): 0 = Leaves the corresponding bit unchanged 1 = Clear interrupt level to line INT1"]
    #[inline(always)]
    pub fn set_tx_int_lvl(&self) -> SetTxIntLvlR {
        SetTxIntLvlR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Clear Receiver interrupt Level. User and privilege mode (read): 0 = Interrupt level mapped to INT0 line 1 = Interrupt level mapped to INT1 line User and privilege mode (write): 0 = Leaves the corresponding bit unchanged 1 = Clear interrupt level to line INT1"]
    #[inline(always)]
    pub fn set_rx_int_lvl(&self) -> SetRxIntLvlR {
        SetRxIntLvlR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:14 - 14:10\\]
Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bit 15 - 15:15\\]
Reserved"]
    #[inline(always)]
    pub fn set_inc_br_int_lvl(&self) -> SetIncBrIntLvlR {
        SetIncBrIntLvlR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - 18:18\\]
User and privilege mode (read): 0 = RX interrupt request for address frames mapped to INT0 line. 1 = RX interrupt request for address frames mapped to INT1 line. User and privilege mode (write): 0 = Leaves the corresponding bit unchanged 1 = Clear interrupt level to line INT1"]
    #[inline(always)]
    pub fn set_rx_dma_all_int_lvl(&self) -> SetRxDmaAllIntLvlR {
        SetRxDmaAllIntLvlR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:23 - 23:19\\]
Reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Clear Parity Error Interrupt Level. User and privilege mode (read): 0 = Interrupt level mapped to INT0 line 1 = Interrupt level mapped to INT1 line User and privilege mode (write): 0 = Leaves the corresponding bit unchanged 1 = Clear interrupt level to line INT1"]
    #[inline(always)]
    pub fn set_pe_int_lvl(&self) -> SetPeIntLvlR {
        SetPeIntLvlR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Clear Overrun-Error Interrupt Level. User and privilege mode (read): 0 = Interrupt level mapped to INT0 line 1 = Interrupt level mapped to INT1 line User and privilege mode (write): 0 = Leaves the corresponding bit unchanged 1 = Clear interrupt level to line INT1"]
    #[inline(always)]
    pub fn set_oe_int_lvl(&self) -> SetOeIntLvlR {
        SetOeIntLvlR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
Clear Framing-Error Interrupt Level. User and privilege mode (read): 0 = Interrupt level mapped to INT0 line 1 = Interrupt level mapped to INT1 line User and privilege mode (write): 0 = Leaves the corresponding bit unchanged 1 = Clear interrupt level to line INT1"]
    #[inline(always)]
    pub fn set_fe_int_lvl(&self) -> SetFeIntLvlR {
        SetFeIntLvlR::new(((self.bits >> 26) & 1) != 0)
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
Clear Break-detect interrupt Level. User and privilege mode (read): 0 = Interrupt level mapped to INT0 line 1 = Interrupt level mapped to INT1 line User and privilege mode (write): 0 = Leaves the corresponding bit unchanged 1 = Clear interrupt level to line INT1"]
    #[inline(always)]
    #[must_use]
    pub fn set_brkdt_int_lvl(&mut self) -> SetBrkdtIntLvlW<ScisetintlvlSpec> {
        SetBrkdtIntLvlW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Clear Wake-up interrupt Level. User and privilege mode (read): 0 = Interrupt level mapped to INT0 line 1 = Interrupt level mapped to INT1 line User and privilege mode (write): 0 = Leaves the corresponding bit unchanged 1 = Clear interrupt level to line INT1"]
    #[inline(always)]
    #[must_use]
    pub fn set_wakeup_int_lvl(&mut self) -> SetWakeupIntLvlW<ScisetintlvlSpec> {
        SetWakeupIntLvlW::new(self, 1)
    }
    #[doc = "Bits 2:7 - 7:2\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<ScisetintlvlSpec> {
        Reserved1W::new(self, 2)
    }
    #[doc = "Bit 8 - 8:8\\]
Clear Transmitter interrupt Level. User and privilege mode (read): 0 = Interrupt level mapped to INT0 line 1 = Interrupt level mapped to INT1 line User and privilege mode (write): 0 = Leaves the corresponding bit unchanged 1 = Clear interrupt level to line INT1"]
    #[inline(always)]
    #[must_use]
    pub fn set_tx_int_lvl(&mut self) -> SetTxIntLvlW<ScisetintlvlSpec> {
        SetTxIntLvlW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Clear Receiver interrupt Level. User and privilege mode (read): 0 = Interrupt level mapped to INT0 line 1 = Interrupt level mapped to INT1 line User and privilege mode (write): 0 = Leaves the corresponding bit unchanged 1 = Clear interrupt level to line INT1"]
    #[inline(always)]
    #[must_use]
    pub fn set_rx_int_lvl(&mut self) -> SetRxIntLvlW<ScisetintlvlSpec> {
        SetRxIntLvlW::new(self, 9)
    }
    #[doc = "Bits 10:14 - 14:10\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<ScisetintlvlSpec> {
        Reserved2W::new(self, 10)
    }
    #[doc = "Bit 15 - 15:15\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn set_inc_br_int_lvl(&mut self) -> SetIncBrIntLvlW<ScisetintlvlSpec> {
        SetIncBrIntLvlW::new(self, 15)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> Reserved3W<ScisetintlvlSpec> {
        Reserved3W::new(self, 16)
    }
    #[doc = "Bit 18 - 18:18\\]
User and privilege mode (read): 0 = RX interrupt request for address frames mapped to INT0 line. 1 = RX interrupt request for address frames mapped to INT1 line. User and privilege mode (write): 0 = Leaves the corresponding bit unchanged 1 = Clear interrupt level to line INT1"]
    #[inline(always)]
    #[must_use]
    pub fn set_rx_dma_all_int_lvl(&mut self) -> SetRxDmaAllIntLvlW<ScisetintlvlSpec> {
        SetRxDmaAllIntLvlW::new(self, 18)
    }
    #[doc = "Bits 19:23 - 23:19\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> Reserved4W<ScisetintlvlSpec> {
        Reserved4W::new(self, 19)
    }
    #[doc = "Bit 24 - 24:24\\]
Clear Parity Error Interrupt Level. User and privilege mode (read): 0 = Interrupt level mapped to INT0 line 1 = Interrupt level mapped to INT1 line User and privilege mode (write): 0 = Leaves the corresponding bit unchanged 1 = Clear interrupt level to line INT1"]
    #[inline(always)]
    #[must_use]
    pub fn set_pe_int_lvl(&mut self) -> SetPeIntLvlW<ScisetintlvlSpec> {
        SetPeIntLvlW::new(self, 24)
    }
    #[doc = "Bit 25 - 25:25\\]
Clear Overrun-Error Interrupt Level. User and privilege mode (read): 0 = Interrupt level mapped to INT0 line 1 = Interrupt level mapped to INT1 line User and privilege mode (write): 0 = Leaves the corresponding bit unchanged 1 = Clear interrupt level to line INT1"]
    #[inline(always)]
    #[must_use]
    pub fn set_oe_int_lvl(&mut self) -> SetOeIntLvlW<ScisetintlvlSpec> {
        SetOeIntLvlW::new(self, 25)
    }
    #[doc = "Bit 26 - 26:26\\]
Clear Framing-Error Interrupt Level. User and privilege mode (read): 0 = Interrupt level mapped to INT0 line 1 = Interrupt level mapped to INT1 line User and privilege mode (write): 0 = Leaves the corresponding bit unchanged 1 = Clear interrupt level to line INT1"]
    #[inline(always)]
    #[must_use]
    pub fn set_fe_int_lvl(&mut self) -> SetFeIntLvlW<ScisetintlvlSpec> {
        SetFeIntLvlW::new(self, 26)
    }
    #[doc = "Bits 27:31 - 31:27\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved5(&mut self) -> Reserved5W<ScisetintlvlSpec> {
        Reserved5W::new(self, 27)
    }
}
#[doc = "SCI Set Interrupt Level Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scisetintlvl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scisetintlvl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScisetintlvlSpec;
impl crate::RegisterSpec for ScisetintlvlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scisetintlvl::R`](R) reader structure"]
impl crate::Readable for ScisetintlvlSpec {}
#[doc = "`write(|w| ..)` method takes [`scisetintlvl::W`](W) writer structure"]
impl crate::Writable for ScisetintlvlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCISETINTLVL to value 0"]
impl crate::Resettable for ScisetintlvlSpec {
    const RESET_VALUE: u32 = 0;
}
