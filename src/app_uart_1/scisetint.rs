#[doc = "Register `SCISETINT` reader"]
pub type R = crate::R<ScisetintSpec>;
#[doc = "Register `SCISETINT` writer"]
pub type W = crate::W<ScisetintSpec>;
#[doc = "Field `SET_BRKDT_INT` reader - 0:0\\]
Set Break-detect interrupt. Setting this bit enables the SCI to generate an error interrupt if a break condition is detected on the SCIRX pin. User and privilege mode (read): 0 = Interrupt is disabled 1 = Interrupt is enabled User and privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable interrupt"]
pub type SetBrkdtIntR = crate::BitReader;
#[doc = "Field `SET_BRKDT_INT` writer - 0:0\\]
Set Break-detect interrupt. Setting this bit enables the SCI to generate an error interrupt if a break condition is detected on the SCIRX pin. User and privilege mode (read): 0 = Interrupt is disabled 1 = Interrupt is enabled User and privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable interrupt"]
pub type SetBrkdtIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SET_WAKEUP_INT` reader - 1:1\\]
Set Wake-up interrupt User and privilege mode (read): 0 = Interrupt is disabled 1 = Interrupt is enabled User and privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable interrupt"]
pub type SetWakeupIntR = crate::BitReader;
#[doc = "Field `SET_WAKEUP_INT` writer - 1:1\\]
Set Wake-up interrupt User and privilege mode (read): 0 = Interrupt is disabled 1 = Interrupt is enabled User and privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable interrupt"]
pub type SetWakeupIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED1` reader - 7:2\\]
Reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `RESERVED1` writer - 7:2\\]
Reserved"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `SET_TX_INT` reader - 8:8\\]
Set Transmitter interrupt. Setting this bit enables the SCI to generate a transmit interrupt as data is being transferred from SCITD to SCITXSHF and the TXRDY bit is being set. User and privilege mode (read): 0 = Interrupt is disabled 1 = Interrupt is enabled User and privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable interrupt"]
pub type SetTxIntR = crate::BitReader;
#[doc = "Field `SET_TX_INT` writer - 8:8\\]
Set Transmitter interrupt. Setting this bit enables the SCI to generate a transmit interrupt as data is being transferred from SCITD to SCITXSHF and the TXRDY bit is being set. User and privilege mode (read): 0 = Interrupt is disabled 1 = Interrupt is enabled User and privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable interrupt"]
pub type SetTxIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SET_RX_INT` reader - 9:9\\]
Receiver interrupt enable:Setting this bit enables the SCI to generate a receive interrupt after a frame has been completely received and the data is being transferred from SCIRXSHF to SCIRD. User and privilege mode (read): 0 = Interrupt is disabled 1 = Interrupt is enabled User and privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable interrupt"]
pub type SetRxIntR = crate::BitReader;
#[doc = "Field `SET_RX_INT` writer - 9:9\\]
Receiver interrupt enable:Setting this bit enables the SCI to generate a receive interrupt after a frame has been completely received and the data is being transferred from SCIRXSHF to SCIRD. User and privilege mode (read): 0 = Interrupt is disabled 1 = Interrupt is enabled User and privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable interrupt"]
pub type SetRxIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED2` reader - 15:10\\]
Reserved"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `RESERVED2` writer - 15:10\\]
Reserved"]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `SET_TX_DMA` reader - 16:16\\]
To select DMA requests for the transmitter, this bit must be set. If it is cleared, interrupt requests are generated depending on SET TX INT bit (SCISETINT.8) User and privilege mode (read): 0 = TX interrupt request selected 1 = TX DMA request selected User and privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable interrupt"]
pub type SetTxDmaR = crate::BitReader;
#[doc = "Field `SET_TX_DMA` writer - 16:16\\]
To select DMA requests for the transmitter, this bit must be set. If it is cleared, interrupt requests are generated depending on SET TX INT bit (SCISETINT.8) User and privilege mode (read): 0 = TX interrupt request selected 1 = TX DMA request selected User and privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable interrupt"]
pub type SetTxDmaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SET_RX_DMA` reader - 17:17\\]
To select receiver DMA requests, this bit must be set. If it is cleared, interrupt requests are generated depending on bit SCISETINT.9 User and privilege mode (read): 0 = DMA request is disabled 1 = DMA request is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable DMA request"]
pub type SetRxDmaR = crate::BitReader;
#[doc = "Field `SET_RX_DMA` writer - 17:17\\]
To select receiver DMA requests, this bit must be set. If it is cleared, interrupt requests are generated depending on bit SCISETINT.9 User and privilege mode (read): 0 = DMA request is disabled 1 = DMA request is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable DMA request"]
pub type SetRxDmaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SET_RX_DMA_ALL` reader - 18:18\\]
Determines if a separate interrupt is generated for the address frames sent in multiprocessor communications User and privilege mode (read): 0 = DMA request is disabled for address frames (RX interrupt request is enabled for address frames) 1 = DMA request is enabled for address and data frames User and privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable DMA request for address and data frames"]
pub type SetRxDmaAllR = crate::BitReader;
#[doc = "Field `SET_RX_DMA_ALL` writer - 18:18\\]
Determines if a separate interrupt is generated for the address frames sent in multiprocessor communications User and privilege mode (read): 0 = DMA request is disabled for address frames (RX interrupt request is enabled for address frames) 1 = DMA request is enabled for address and data frames User and privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable DMA request for address and data frames"]
pub type SetRxDmaAllW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED3` reader - 23:19\\]
Reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `RESERVED3` writer - 23:19\\]
Reserved"]
pub type Reserved3W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SET_PE_INT` reader - 24:24\\]
Set Parity Interrupt User and privilege mode (read): 0 = Interrupt is disabled 1 = Interrupt is enabled User and privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable interrupt"]
pub type SetPeIntR = crate::BitReader;
#[doc = "Field `SET_PE_INT` writer - 24:24\\]
Set Parity Interrupt User and privilege mode (read): 0 = Interrupt is disabled 1 = Interrupt is enabled User and privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable interrupt"]
pub type SetPeIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SET_OE_INT` reader - 25:25\\]
Set Overrun-Error Interrupt User and privilege mode (read): 0 = Interrupt is disabled 1 = Interrupt is enabled User and privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable interrupt"]
pub type SetOeIntR = crate::BitReader;
#[doc = "Field `SET_OE_INT` writer - 25:25\\]
Set Overrun-Error Interrupt User and privilege mode (read): 0 = Interrupt is disabled 1 = Interrupt is enabled User and privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable interrupt"]
pub type SetOeIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SET_FE_INT` reader - 26:26\\]
Set Framing-Error Interrupt User and privilege mode (read): 0 = Interrupt is disabled 1 = Interrupt is enabled User and privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable interrupt"]
pub type SetFeIntR = crate::BitReader;
#[doc = "Field `SET_FE_INT` writer - 26:26\\]
Set Framing-Error Interrupt User and privilege mode (read): 0 = Interrupt is disabled 1 = Interrupt is enabled User and privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable interrupt"]
pub type SetFeIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED4` reader - 31:27\\]
Reserved"]
pub type Reserved4R = crate::FieldReader;
#[doc = "Field `RESERVED4` writer - 31:27\\]
Reserved"]
pub type Reserved4W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Set Break-detect interrupt. Setting this bit enables the SCI to generate an error interrupt if a break condition is detected on the SCIRX pin. User and privilege mode (read): 0 = Interrupt is disabled 1 = Interrupt is enabled User and privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable interrupt"]
    #[inline(always)]
    pub fn set_brkdt_int(&self) -> SetBrkdtIntR {
        SetBrkdtIntR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Set Wake-up interrupt User and privilege mode (read): 0 = Interrupt is disabled 1 = Interrupt is enabled User and privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable interrupt"]
    #[inline(always)]
    pub fn set_wakeup_int(&self) -> SetWakeupIntR {
        SetWakeupIntR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - 7:2\\]
Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Set Transmitter interrupt. Setting this bit enables the SCI to generate a transmit interrupt as data is being transferred from SCITD to SCITXSHF and the TXRDY bit is being set. User and privilege mode (read): 0 = Interrupt is disabled 1 = Interrupt is enabled User and privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable interrupt"]
    #[inline(always)]
    pub fn set_tx_int(&self) -> SetTxIntR {
        SetTxIntR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Receiver interrupt enable:Setting this bit enables the SCI to generate a receive interrupt after a frame has been completely received and the data is being transferred from SCIRXSHF to SCIRD. User and privilege mode (read): 0 = Interrupt is disabled 1 = Interrupt is enabled User and privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable interrupt"]
    #[inline(always)]
    pub fn set_rx_int(&self) -> SetRxIntR {
        SetRxIntR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - 15:10\\]
Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
To select DMA requests for the transmitter, this bit must be set. If it is cleared, interrupt requests are generated depending on SET TX INT bit (SCISETINT.8) User and privilege mode (read): 0 = TX interrupt request selected 1 = TX DMA request selected User and privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable interrupt"]
    #[inline(always)]
    pub fn set_tx_dma(&self) -> SetTxDmaR {
        SetTxDmaR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
To select receiver DMA requests, this bit must be set. If it is cleared, interrupt requests are generated depending on bit SCISETINT.9 User and privilege mode (read): 0 = DMA request is disabled 1 = DMA request is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable DMA request"]
    #[inline(always)]
    pub fn set_rx_dma(&self) -> SetRxDmaR {
        SetRxDmaR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Determines if a separate interrupt is generated for the address frames sent in multiprocessor communications User and privilege mode (read): 0 = DMA request is disabled for address frames (RX interrupt request is enabled for address frames) 1 = DMA request is enabled for address and data frames User and privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable DMA request for address and data frames"]
    #[inline(always)]
    pub fn set_rx_dma_all(&self) -> SetRxDmaAllR {
        SetRxDmaAllR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:23 - 23:19\\]
Reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Set Parity Interrupt User and privilege mode (read): 0 = Interrupt is disabled 1 = Interrupt is enabled User and privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable interrupt"]
    #[inline(always)]
    pub fn set_pe_int(&self) -> SetPeIntR {
        SetPeIntR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Set Overrun-Error Interrupt User and privilege mode (read): 0 = Interrupt is disabled 1 = Interrupt is enabled User and privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable interrupt"]
    #[inline(always)]
    pub fn set_oe_int(&self) -> SetOeIntR {
        SetOeIntR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
Set Framing-Error Interrupt User and privilege mode (read): 0 = Interrupt is disabled 1 = Interrupt is enabled User and privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable interrupt"]
    #[inline(always)]
    pub fn set_fe_int(&self) -> SetFeIntR {
        SetFeIntR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:31 - 31:27\\]
Reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Set Break-detect interrupt. Setting this bit enables the SCI to generate an error interrupt if a break condition is detected on the SCIRX pin. User and privilege mode (read): 0 = Interrupt is disabled 1 = Interrupt is enabled User and privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn set_brkdt_int(&mut self) -> SetBrkdtIntW<ScisetintSpec> {
        SetBrkdtIntW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Set Wake-up interrupt User and privilege mode (read): 0 = Interrupt is disabled 1 = Interrupt is enabled User and privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn set_wakeup_int(&mut self) -> SetWakeupIntW<ScisetintSpec> {
        SetWakeupIntW::new(self, 1)
    }
    #[doc = "Bits 2:7 - 7:2\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<ScisetintSpec> {
        Reserved1W::new(self, 2)
    }
    #[doc = "Bit 8 - 8:8\\]
Set Transmitter interrupt. Setting this bit enables the SCI to generate a transmit interrupt as data is being transferred from SCITD to SCITXSHF and the TXRDY bit is being set. User and privilege mode (read): 0 = Interrupt is disabled 1 = Interrupt is enabled User and privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn set_tx_int(&mut self) -> SetTxIntW<ScisetintSpec> {
        SetTxIntW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Receiver interrupt enable:Setting this bit enables the SCI to generate a receive interrupt after a frame has been completely received and the data is being transferred from SCIRXSHF to SCIRD. User and privilege mode (read): 0 = Interrupt is disabled 1 = Interrupt is enabled User and privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn set_rx_int(&mut self) -> SetRxIntW<ScisetintSpec> {
        SetRxIntW::new(self, 9)
    }
    #[doc = "Bits 10:15 - 15:10\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<ScisetintSpec> {
        Reserved2W::new(self, 10)
    }
    #[doc = "Bit 16 - 16:16\\]
To select DMA requests for the transmitter, this bit must be set. If it is cleared, interrupt requests are generated depending on SET TX INT bit (SCISETINT.8) User and privilege mode (read): 0 = TX interrupt request selected 1 = TX DMA request selected User and privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn set_tx_dma(&mut self) -> SetTxDmaW<ScisetintSpec> {
        SetTxDmaW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
To select receiver DMA requests, this bit must be set. If it is cleared, interrupt requests are generated depending on bit SCISETINT.9 User and privilege mode (read): 0 = DMA request is disabled 1 = DMA request is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable DMA request"]
    #[inline(always)]
    #[must_use]
    pub fn set_rx_dma(&mut self) -> SetRxDmaW<ScisetintSpec> {
        SetRxDmaW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Determines if a separate interrupt is generated for the address frames sent in multiprocessor communications User and privilege mode (read): 0 = DMA request is disabled for address frames (RX interrupt request is enabled for address frames) 1 = DMA request is enabled for address and data frames User and privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable DMA request for address and data frames"]
    #[inline(always)]
    #[must_use]
    pub fn set_rx_dma_all(&mut self) -> SetRxDmaAllW<ScisetintSpec> {
        SetRxDmaAllW::new(self, 18)
    }
    #[doc = "Bits 19:23 - 23:19\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> Reserved3W<ScisetintSpec> {
        Reserved3W::new(self, 19)
    }
    #[doc = "Bit 24 - 24:24\\]
Set Parity Interrupt User and privilege mode (read): 0 = Interrupt is disabled 1 = Interrupt is enabled User and privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn set_pe_int(&mut self) -> SetPeIntW<ScisetintSpec> {
        SetPeIntW::new(self, 24)
    }
    #[doc = "Bit 25 - 25:25\\]
Set Overrun-Error Interrupt User and privilege mode (read): 0 = Interrupt is disabled 1 = Interrupt is enabled User and privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn set_oe_int(&mut self) -> SetOeIntW<ScisetintSpec> {
        SetOeIntW::new(self, 25)
    }
    #[doc = "Bit 26 - 26:26\\]
Set Framing-Error Interrupt User and privilege mode (read): 0 = Interrupt is disabled 1 = Interrupt is enabled User and privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn set_fe_int(&mut self) -> SetFeIntW<ScisetintSpec> {
        SetFeIntW::new(self, 26)
    }
    #[doc = "Bits 27:31 - 31:27\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> Reserved4W<ScisetintSpec> {
        Reserved4W::new(self, 27)
    }
}
#[doc = "SCI Set Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scisetint::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scisetint::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScisetintSpec;
impl crate::RegisterSpec for ScisetintSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scisetint::R`](R) reader structure"]
impl crate::Readable for ScisetintSpec {}
#[doc = "`write(|w| ..)` method takes [`scisetint::W`](W) writer structure"]
impl crate::Writable for ScisetintSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCISETINT to value 0"]
impl crate::Resettable for ScisetintSpec {
    const RESET_VALUE: u32 = 0;
}
