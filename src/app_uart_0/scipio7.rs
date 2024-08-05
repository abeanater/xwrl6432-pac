#[doc = "Register `SCIPIO7` reader"]
pub type R = crate::R<Scipio7Spec>;
#[doc = "Register `SCIPIO7` writer"]
pub type W = crate::W<Scipio7Spec>;
#[doc = "Field `CLK_PD` reader - 0:0\\]
CLK pin Pull Control Disable Disables pull control capability in the output pin SCICLK. 0=Pull Control on SCICLK pin is enabled. 1=Pull Control on SCICLK pin is disabled."]
pub type ClkPdR = crate::BitReader;
#[doc = "Field `CLK_PD` writer - 0:0\\]
CLK pin Pull Control Disable Disables pull control capability in the output pin SCICLK. 0=Pull Control on SCICLK pin is enabled. 1=Pull Control on SCICLK pin is disabled."]
pub type ClkPdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_PD` reader - 1:1\\]
RX pin Pull Control Disable Disables pull control capability in the output pin SCIRX. 0=Pull Control on SCIRX pin is enabled. 1=Pull Control on SCIRX pin is disabled."]
pub type RxPdR = crate::BitReader;
#[doc = "Field `RX_PD` writer - 1:1\\]
RX pin Pull Control Disable Disables pull control capability in the output pin SCIRX. 0=Pull Control on SCIRX pin is enabled. 1=Pull Control on SCIRX pin is disabled."]
pub type RxPdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_PD` reader - 2:2\\]
TX pin Pull Control Disable Disables pull control capability in the output pin SCITX. 0=Pull Control on SCITX pin is enabled. 1=Pull Control on SCITX pin is disabled."]
pub type TxPdR = crate::BitReader;
#[doc = "Field `TX_PD` writer - 2:2\\]
TX pin Pull Control Disable Disables pull control capability in the output pin SCITX. 0=Pull Control on SCITX pin is enabled. 1=Pull Control on SCITX pin is disabled."]
pub type TxPdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
CLK pin Pull Control Disable Disables pull control capability in the output pin SCICLK. 0=Pull Control on SCICLK pin is enabled. 1=Pull Control on SCICLK pin is disabled."]
    #[inline(always)]
    pub fn clk_pd(&self) -> ClkPdR {
        ClkPdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
RX pin Pull Control Disable Disables pull control capability in the output pin SCIRX. 0=Pull Control on SCIRX pin is enabled. 1=Pull Control on SCIRX pin is disabled."]
    #[inline(always)]
    pub fn rx_pd(&self) -> RxPdR {
        RxPdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
TX pin Pull Control Disable Disables pull control capability in the output pin SCITX. 0=Pull Control on SCITX pin is enabled. 1=Pull Control on SCITX pin is disabled."]
    #[inline(always)]
    pub fn tx_pd(&self) -> TxPdR {
        TxPdR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
CLK pin Pull Control Disable Disables pull control capability in the output pin SCICLK. 0=Pull Control on SCICLK pin is enabled. 1=Pull Control on SCICLK pin is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn clk_pd(&mut self) -> ClkPdW<Scipio7Spec> {
        ClkPdW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
RX pin Pull Control Disable Disables pull control capability in the output pin SCIRX. 0=Pull Control on SCIRX pin is enabled. 1=Pull Control on SCIRX pin is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn rx_pd(&mut self) -> RxPdW<Scipio7Spec> {
        RxPdW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
TX pin Pull Control Disable Disables pull control capability in the output pin SCITX. 0=Pull Control on SCITX pin is enabled. 1=Pull Control on SCITX pin is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn tx_pd(&mut self) -> TxPdW<Scipio7Spec> {
        TxPdW::new(self, 2)
    }
}
#[doc = "SCI Pin I/O Control Register 7\n\nYou can [`read`](crate::Reg::read) this register and get [`scipio7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scipio7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scipio7Spec;
impl crate::RegisterSpec for Scipio7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scipio7::R`](R) reader structure"]
impl crate::Readable for Scipio7Spec {}
#[doc = "`write(|w| ..)` method takes [`scipio7::W`](W) writer structure"]
impl crate::Writable for Scipio7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCIPIO7 to value 0"]
impl crate::Resettable for Scipio7Spec {
    const RESET_VALUE: u32 = 0;
}
