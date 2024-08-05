#[doc = "Register `SCIPIO8` reader"]
pub type R = crate::R<Scipio8Spec>;
#[doc = "Register `SCIPIO8` writer"]
pub type W = crate::W<Scipio8Spec>;
#[doc = "Field `CLK_PSL` reader - 0:0\\]
CLK pin Pull Select Selects pull type in the output pin SCICLK. 0=Pull-Down is on SCICLK pin. 1=Pull-Up is on SCICLK pin."]
pub type ClkPslR = crate::BitReader;
#[doc = "Field `CLK_PSL` writer - 0:0\\]
CLK pin Pull Select Selects pull type in the output pin SCICLK. 0=Pull-Down is on SCICLK pin. 1=Pull-Up is on SCICLK pin."]
pub type ClkPslW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_PSL` reader - 1:1\\]
RX pin Pull Select Selects pull type in the output pin SCIRX. 0=Pull-Down is on SCIRX pin. 1=Pull-Up is on SCIRX pin."]
pub type RxPslR = crate::BitReader;
#[doc = "Field `RX_PSL` writer - 1:1\\]
RX pin Pull Select Selects pull type in the output pin SCIRX. 0=Pull-Down is on SCIRX pin. 1=Pull-Up is on SCIRX pin."]
pub type RxPslW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_PSL` reader - 2:2\\]
TX pin Pull Select Selects pull type in the output pin SCITX. 0=Pull-Down is on SCITX pin. 1=Pull-Up is on SCITX pin."]
pub type TxPslR = crate::BitReader;
#[doc = "Field `TX_PSL` writer - 2:2\\]
TX pin Pull Select Selects pull type in the output pin SCITX. 0=Pull-Down is on SCITX pin. 1=Pull-Up is on SCITX pin."]
pub type TxPslW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
CLK pin Pull Select Selects pull type in the output pin SCICLK. 0=Pull-Down is on SCICLK pin. 1=Pull-Up is on SCICLK pin."]
    #[inline(always)]
    pub fn clk_psl(&self) -> ClkPslR {
        ClkPslR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
RX pin Pull Select Selects pull type in the output pin SCIRX. 0=Pull-Down is on SCIRX pin. 1=Pull-Up is on SCIRX pin."]
    #[inline(always)]
    pub fn rx_psl(&self) -> RxPslR {
        RxPslR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
TX pin Pull Select Selects pull type in the output pin SCITX. 0=Pull-Down is on SCITX pin. 1=Pull-Up is on SCITX pin."]
    #[inline(always)]
    pub fn tx_psl(&self) -> TxPslR {
        TxPslR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
CLK pin Pull Select Selects pull type in the output pin SCICLK. 0=Pull-Down is on SCICLK pin. 1=Pull-Up is on SCICLK pin."]
    #[inline(always)]
    #[must_use]
    pub fn clk_psl(&mut self) -> ClkPslW<Scipio8Spec> {
        ClkPslW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
RX pin Pull Select Selects pull type in the output pin SCIRX. 0=Pull-Down is on SCIRX pin. 1=Pull-Up is on SCIRX pin."]
    #[inline(always)]
    #[must_use]
    pub fn rx_psl(&mut self) -> RxPslW<Scipio8Spec> {
        RxPslW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
TX pin Pull Select Selects pull type in the output pin SCITX. 0=Pull-Down is on SCITX pin. 1=Pull-Up is on SCITX pin."]
    #[inline(always)]
    #[must_use]
    pub fn tx_psl(&mut self) -> TxPslW<Scipio8Spec> {
        TxPslW::new(self, 2)
    }
}
#[doc = "SCI Pin I/O Control Register 8\n\nYou can [`read`](crate::Reg::read) this register and get [`scipio8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scipio8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scipio8Spec;
impl crate::RegisterSpec for Scipio8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scipio8::R`](R) reader structure"]
impl crate::Readable for Scipio8Spec {}
#[doc = "`write(|w| ..)` method takes [`scipio8::W`](W) writer structure"]
impl crate::Writable for Scipio8Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCIPIO8 to value 0"]
impl crate::Resettable for Scipio8Spec {
    const RESET_VALUE: u32 = 0;
}
