#[doc = "Register `SCICLEARINT` reader"]
pub type R = crate::R<SciclearintSpec>;
#[doc = "Register `SCICLEARINT` writer"]
pub type W = crate::W<SciclearintSpec>;
#[doc = "Field `CLR_BRKDT_INT` reader - 0:0\\]
Clear Break-detect interrupt. This bit disables the Break-detect interrupt when set. User and privilege mode (read): 0 = Interrupt is disabled 1 = Interrupt is enabled User and privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable interrupt"]
pub type ClrBrkdtIntR = crate::BitReader;
#[doc = "Field `CLR_BRKDT_INT` writer - 0:0\\]
Clear Break-detect interrupt. This bit disables the Break-detect interrupt when set. User and privilege mode (read): 0 = Interrupt is disabled 1 = Interrupt is enabled User and privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable interrupt"]
pub type ClrBrkdtIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLR_WAKEUP_INT` reader - 1:1\\]
Clear Wake-up interrupt. This bit disables the wakeup interrupt when set. User and privilege mode (read): 0 = Interrupt is disabled 1 = Interrupt is enabled User and privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable interrupt"]
pub type ClrWakeupIntR = crate::BitReader;
#[doc = "Field `CLR_WAKEUP_INT` writer - 1:1\\]
Clear Wake-up interrupt. This bit disables the wakeup interrupt when set. User and privilege mode (read): 0 = Interrupt is disabled 1 = Interrupt is enabled User and privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable interrupt"]
pub type ClrWakeupIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED1` reader - 7:2\\]
Reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `RESERVED1` writer - 7:2\\]
Reserved"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `CLR_TX_INT` reader - 8:8\\]
Clear Transmitter interrupt. This bit disables the transmitter interrupt when set. User and privilege mode (read): 0 = Interrupt is disabled 1 = Interrupt is enabled User and privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable interrupt"]
pub type ClrTxIntR = crate::BitReader;
#[doc = "Field `CLR_TX_INT` writer - 8:8\\]
Clear Transmitter interrupt. This bit disables the transmitter interrupt when set. User and privilege mode (read): 0 = Interrupt is disabled 1 = Interrupt is enabled User and privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable interrupt"]
pub type ClrTxIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLR_RX_INT` reader - 9:9\\]
Clear Receiver interrupt. This bit disables the receiver interrupt when set. User and privilege mode (read): 0 = Interrupt is disabled 1 = Interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable interrupt"]
pub type ClrRxIntR = crate::BitReader;
#[doc = "Field `CLR_RX_INT` writer - 9:9\\]
Clear Receiver interrupt. This bit disables the receiver interrupt when set. User and privilege mode (read): 0 = Interrupt is disabled 1 = Interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable interrupt"]
pub type ClrRxIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED2` reader - 15:10\\]
Reserved"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `RESERVED2` writer - 15:10\\]
Reserved"]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `CLR_TX_DMA` reader - 16:16\\]
Clear TX DMA request. This bit disables the transmit DMA request when set. User and privilege mode (read): 0 = DMA request is disabled 1 = DMA request is enabled User and privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable DMA request"]
pub type ClrTxDmaR = crate::BitReader;
#[doc = "Field `CLR_TX_DMA` writer - 16:16\\]
Clear TX DMA request. This bit disables the transmit DMA request when set. User and privilege mode (read): 0 = DMA request is disabled 1 = DMA request is enabled User and privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable DMA request"]
pub type ClrTxDmaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLR_RX_DMA` reader - 17:17\\]
Clear RX DMA request. This bit disalbes the receive DMA request when set. User and privilege mode (read): 0 = DMA request is disabled 1 = DMA request is enabled User and privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable DMA request"]
pub type ClrRxDmaR = crate::BitReader;
#[doc = "Field `CLR_RX_DMA` writer - 17:17\\]
Clear RX DMA request. This bit disalbes the receive DMA request when set. User and privilege mode (read): 0 = DMA request is disabled 1 = DMA request is enabled User and privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable DMA request"]
pub type ClrRxDmaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLR_RX_DMA_ALL` reader - 18:18\\]
User and privilege mode (read): 0 = DMA request is disabled for address frames (RX interrupt request is enabled for address frames). DMA request is enabled for data frames. 1 = DMA request is enabled for address and data frames User and privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable DMA request for address frames"]
pub type ClrRxDmaAllR = crate::BitReader;
#[doc = "Field `CLR_RX_DMA_ALL` writer - 18:18\\]
User and privilege mode (read): 0 = DMA request is disabled for address frames (RX interrupt request is enabled for address frames). DMA request is enabled for data frames. 1 = DMA request is enabled for address and data frames User and privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable DMA request for address frames"]
pub type ClrRxDmaAllW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED3` reader - 23:19\\]
Reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `RESERVED3` writer - 23:19\\]
Reserved"]
pub type Reserved3W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CLR_PE_INT` reader - 24:24\\]
Clear Parity Interrupt. Setting this bit disables the SCI Parity error interrupt. User and privilege mode (read): 0 = Interrupt is disabled 1 = Interrupt is enabled User and privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable interrupt"]
pub type ClrPeIntR = crate::BitReader;
#[doc = "Field `CLR_PE_INT` writer - 24:24\\]
Clear Parity Interrupt. Setting this bit disables the SCI Parity error interrupt. User and privilege mode (read): 0 = Interrupt is disabled 1 = Interrupt is enabled User and privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable interrupt"]
pub type ClrPeIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLR_OE_INT` reader - 25:25\\]
Clear Overrun-Error Interrupt. This bit disables the SCI overrun interrupt when set. User and privilege mode (read): 0 = Interrupt is disabled 1 = Interrupt is enabled User and privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable interrupt"]
pub type ClrOeIntR = crate::BitReader;
#[doc = "Field `CLR_OE_INT` writer - 25:25\\]
Clear Overrun-Error Interrupt. This bit disables the SCI overrun interrupt when set. User and privilege mode (read): 0 = Interrupt is disabled 1 = Interrupt is enabled User and privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable interrupt"]
pub type ClrOeIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLR_FE_INT` reader - 26:26\\]
Clear Framing-Error Interrupt: Setting this bit disables the SCI module to generate an interrupt when there is a Framing error. User and privilege mode (read): 0 = Interrupt is disabled 1 = Interrupt is enabled User and privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable interrupt"]
pub type ClrFeIntR = crate::BitReader;
#[doc = "Field `CLR_FE_INT` writer - 26:26\\]
Clear Framing-Error Interrupt: Setting this bit disables the SCI module to generate an interrupt when there is a Framing error. User and privilege mode (read): 0 = Interrupt is disabled 1 = Interrupt is enabled User and privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable interrupt"]
pub type ClrFeIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED4` reader - 31:27\\]
Reserved"]
pub type Reserved4R = crate::FieldReader;
#[doc = "Field `RESERVED4` writer - 31:27\\]
Reserved"]
pub type Reserved4W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Clear Break-detect interrupt. This bit disables the Break-detect interrupt when set. User and privilege mode (read): 0 = Interrupt is disabled 1 = Interrupt is enabled User and privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable interrupt"]
    #[inline(always)]
    pub fn clr_brkdt_int(&self) -> ClrBrkdtIntR {
        ClrBrkdtIntR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Clear Wake-up interrupt. This bit disables the wakeup interrupt when set. User and privilege mode (read): 0 = Interrupt is disabled 1 = Interrupt is enabled User and privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable interrupt"]
    #[inline(always)]
    pub fn clr_wakeup_int(&self) -> ClrWakeupIntR {
        ClrWakeupIntR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - 7:2\\]
Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Clear Transmitter interrupt. This bit disables the transmitter interrupt when set. User and privilege mode (read): 0 = Interrupt is disabled 1 = Interrupt is enabled User and privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable interrupt"]
    #[inline(always)]
    pub fn clr_tx_int(&self) -> ClrTxIntR {
        ClrTxIntR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Clear Receiver interrupt. This bit disables the receiver interrupt when set. User and privilege mode (read): 0 = Interrupt is disabled 1 = Interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable interrupt"]
    #[inline(always)]
    pub fn clr_rx_int(&self) -> ClrRxIntR {
        ClrRxIntR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - 15:10\\]
Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Clear TX DMA request. This bit disables the transmit DMA request when set. User and privilege mode (read): 0 = DMA request is disabled 1 = DMA request is enabled User and privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable DMA request"]
    #[inline(always)]
    pub fn clr_tx_dma(&self) -> ClrTxDmaR {
        ClrTxDmaR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Clear RX DMA request. This bit disalbes the receive DMA request when set. User and privilege mode (read): 0 = DMA request is disabled 1 = DMA request is enabled User and privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable DMA request"]
    #[inline(always)]
    pub fn clr_rx_dma(&self) -> ClrRxDmaR {
        ClrRxDmaR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
User and privilege mode (read): 0 = DMA request is disabled for address frames (RX interrupt request is enabled for address frames). DMA request is enabled for data frames. 1 = DMA request is enabled for address and data frames User and privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable DMA request for address frames"]
    #[inline(always)]
    pub fn clr_rx_dma_all(&self) -> ClrRxDmaAllR {
        ClrRxDmaAllR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:23 - 23:19\\]
Reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Clear Parity Interrupt. Setting this bit disables the SCI Parity error interrupt. User and privilege mode (read): 0 = Interrupt is disabled 1 = Interrupt is enabled User and privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable interrupt"]
    #[inline(always)]
    pub fn clr_pe_int(&self) -> ClrPeIntR {
        ClrPeIntR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Clear Overrun-Error Interrupt. This bit disables the SCI overrun interrupt when set. User and privilege mode (read): 0 = Interrupt is disabled 1 = Interrupt is enabled User and privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable interrupt"]
    #[inline(always)]
    pub fn clr_oe_int(&self) -> ClrOeIntR {
        ClrOeIntR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
Clear Framing-Error Interrupt: Setting this bit disables the SCI module to generate an interrupt when there is a Framing error. User and privilege mode (read): 0 = Interrupt is disabled 1 = Interrupt is enabled User and privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable interrupt"]
    #[inline(always)]
    pub fn clr_fe_int(&self) -> ClrFeIntR {
        ClrFeIntR::new(((self.bits >> 26) & 1) != 0)
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
Clear Break-detect interrupt. This bit disables the Break-detect interrupt when set. User and privilege mode (read): 0 = Interrupt is disabled 1 = Interrupt is enabled User and privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn clr_brkdt_int(&mut self) -> ClrBrkdtIntW<SciclearintSpec> {
        ClrBrkdtIntW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Clear Wake-up interrupt. This bit disables the wakeup interrupt when set. User and privilege mode (read): 0 = Interrupt is disabled 1 = Interrupt is enabled User and privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn clr_wakeup_int(&mut self) -> ClrWakeupIntW<SciclearintSpec> {
        ClrWakeupIntW::new(self, 1)
    }
    #[doc = "Bits 2:7 - 7:2\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<SciclearintSpec> {
        Reserved1W::new(self, 2)
    }
    #[doc = "Bit 8 - 8:8\\]
Clear Transmitter interrupt. This bit disables the transmitter interrupt when set. User and privilege mode (read): 0 = Interrupt is disabled 1 = Interrupt is enabled User and privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn clr_tx_int(&mut self) -> ClrTxIntW<SciclearintSpec> {
        ClrTxIntW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Clear Receiver interrupt. This bit disables the receiver interrupt when set. User and privilege mode (read): 0 = Interrupt is disabled 1 = Interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn clr_rx_int(&mut self) -> ClrRxIntW<SciclearintSpec> {
        ClrRxIntW::new(self, 9)
    }
    #[doc = "Bits 10:15 - 15:10\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<SciclearintSpec> {
        Reserved2W::new(self, 10)
    }
    #[doc = "Bit 16 - 16:16\\]
Clear TX DMA request. This bit disables the transmit DMA request when set. User and privilege mode (read): 0 = DMA request is disabled 1 = DMA request is enabled User and privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable DMA request"]
    #[inline(always)]
    #[must_use]
    pub fn clr_tx_dma(&mut self) -> ClrTxDmaW<SciclearintSpec> {
        ClrTxDmaW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Clear RX DMA request. This bit disalbes the receive DMA request when set. User and privilege mode (read): 0 = DMA request is disabled 1 = DMA request is enabled User and privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable DMA request"]
    #[inline(always)]
    #[must_use]
    pub fn clr_rx_dma(&mut self) -> ClrRxDmaW<SciclearintSpec> {
        ClrRxDmaW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
User and privilege mode (read): 0 = DMA request is disabled for address frames (RX interrupt request is enabled for address frames). DMA request is enabled for data frames. 1 = DMA request is enabled for address and data frames User and privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable DMA request for address frames"]
    #[inline(always)]
    #[must_use]
    pub fn clr_rx_dma_all(&mut self) -> ClrRxDmaAllW<SciclearintSpec> {
        ClrRxDmaAllW::new(self, 18)
    }
    #[doc = "Bits 19:23 - 23:19\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> Reserved3W<SciclearintSpec> {
        Reserved3W::new(self, 19)
    }
    #[doc = "Bit 24 - 24:24\\]
Clear Parity Interrupt. Setting this bit disables the SCI Parity error interrupt. User and privilege mode (read): 0 = Interrupt is disabled 1 = Interrupt is enabled User and privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn clr_pe_int(&mut self) -> ClrPeIntW<SciclearintSpec> {
        ClrPeIntW::new(self, 24)
    }
    #[doc = "Bit 25 - 25:25\\]
Clear Overrun-Error Interrupt. This bit disables the SCI overrun interrupt when set. User and privilege mode (read): 0 = Interrupt is disabled 1 = Interrupt is enabled User and privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn clr_oe_int(&mut self) -> ClrOeIntW<SciclearintSpec> {
        ClrOeIntW::new(self, 25)
    }
    #[doc = "Bit 26 - 26:26\\]
Clear Framing-Error Interrupt: Setting this bit disables the SCI module to generate an interrupt when there is a Framing error. User and privilege mode (read): 0 = Interrupt is disabled 1 = Interrupt is enabled User and privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn clr_fe_int(&mut self) -> ClrFeIntW<SciclearintSpec> {
        ClrFeIntW::new(self, 26)
    }
    #[doc = "Bits 27:31 - 31:27\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> Reserved4W<SciclearintSpec> {
        Reserved4W::new(self, 27)
    }
}
#[doc = "SCI Clear Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sciclearint::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sciclearint::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SciclearintSpec;
impl crate::RegisterSpec for SciclearintSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sciclearint::R`](R) reader structure"]
impl crate::Readable for SciclearintSpec {}
#[doc = "`write(|w| ..)` method takes [`sciclearint::W`](W) writer structure"]
impl crate::Writable for SciclearintSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCICLEARINT to value 0"]
impl crate::Resettable for SciclearintSpec {
    const RESET_VALUE: u32 = 0;
}
