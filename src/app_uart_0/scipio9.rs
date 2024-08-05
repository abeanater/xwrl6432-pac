#[doc = "Register `SCIPIO9` reader"]
pub type R = crate::R<Scipio9Spec>;
#[doc = "Register `SCIPIO9` writer"]
pub type W = crate::W<Scipio9Spec>;
#[doc = "Field `CLK_SL` reader - 0:0\\]
This bit controls the slew rate for the SCICLK pin. 0=The normal output buffer is used for SCICLK pin 1=The output buffer with slew control is used for SCICLK pin"]
pub type ClkSlR = crate::BitReader;
#[doc = "Field `CLK_SL` writer - 0:0\\]
This bit controls the slew rate for the SCICLK pin. 0=The normal output buffer is used for SCICLK pin 1=The output buffer with slew control is used for SCICLK pin"]
pub type ClkSlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_SL` reader - 1:1\\]
This bit controls the slew rate for the SCIRX pin. 0=The normal output buffer is used for SCIRX pin 1=The output buffer with slew control is used for SCIRX pin"]
pub type RxSlR = crate::BitReader;
#[doc = "Field `RX_SL` writer - 1:1\\]
This bit controls the slew rate for the SCIRX pin. 0=The normal output buffer is used for SCIRX pin 1=The output buffer with slew control is used for SCIRX pin"]
pub type RxSlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_SL` reader - 2:2\\]
This bit controls the slew rate for the SCITX pin. 0=The normal output buffer is used for SCITX pin 1=The output buffer with slew control is used for SCITX pin."]
pub type TxSlR = crate::BitReader;
#[doc = "Field `TX_SL` writer - 2:2\\]
This bit controls the slew rate for the SCITX pin. 0=The normal output buffer is used for SCITX pin 1=The output buffer with slew control is used for SCITX pin."]
pub type TxSlW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
This bit controls the slew rate for the SCICLK pin. 0=The normal output buffer is used for SCICLK pin 1=The output buffer with slew control is used for SCICLK pin"]
    #[inline(always)]
    pub fn clk_sl(&self) -> ClkSlR {
        ClkSlR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
This bit controls the slew rate for the SCIRX pin. 0=The normal output buffer is used for SCIRX pin 1=The output buffer with slew control is used for SCIRX pin"]
    #[inline(always)]
    pub fn rx_sl(&self) -> RxSlR {
        RxSlR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
This bit controls the slew rate for the SCITX pin. 0=The normal output buffer is used for SCITX pin 1=The output buffer with slew control is used for SCITX pin."]
    #[inline(always)]
    pub fn tx_sl(&self) -> TxSlR {
        TxSlR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
This bit controls the slew rate for the SCICLK pin. 0=The normal output buffer is used for SCICLK pin 1=The output buffer with slew control is used for SCICLK pin"]
    #[inline(always)]
    #[must_use]
    pub fn clk_sl(&mut self) -> ClkSlW<Scipio9Spec> {
        ClkSlW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
This bit controls the slew rate for the SCIRX pin. 0=The normal output buffer is used for SCIRX pin 1=The output buffer with slew control is used for SCIRX pin"]
    #[inline(always)]
    #[must_use]
    pub fn rx_sl(&mut self) -> RxSlW<Scipio9Spec> {
        RxSlW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
This bit controls the slew rate for the SCITX pin. 0=The normal output buffer is used for SCITX pin 1=The output buffer with slew control is used for SCITX pin."]
    #[inline(always)]
    #[must_use]
    pub fn tx_sl(&mut self) -> TxSlW<Scipio9Spec> {
        TxSlW::new(self, 2)
    }
}
#[doc = "SCI Pin I/O Control Register 9\n\nYou can [`read`](crate::Reg::read) this register and get [`scipio9::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scipio9::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scipio9Spec;
impl crate::RegisterSpec for Scipio9Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scipio9::R`](R) reader structure"]
impl crate::Readable for Scipio9Spec {}
#[doc = "`write(|w| ..)` method takes [`scipio9::W`](W) writer structure"]
impl crate::Writable for Scipio9Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCIPIO9 to value 0"]
impl crate::Resettable for Scipio9Spec {
    const RESET_VALUE: u32 = 0;
}
