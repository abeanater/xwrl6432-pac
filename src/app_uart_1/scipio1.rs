#[doc = "Register `SCIPIO1` reader"]
pub type R = crate::R<Scipio1Spec>;
#[doc = "Register `SCIPIO1` writer"]
pub type W = crate::W<Scipio1Spec>;
#[doc = "Field `CLK_DIR` reader - 0:0\\]
Clock data direction. Determines the data direction on the SCICLK pin. The direction is defined differently depending upon the value of the CLK FUNC bit 0=SCICLK is a general-purpose input pin. 1=SCICLK is a general-purpose output pin"]
pub type ClkDirR = crate::BitReader;
#[doc = "Field `CLK_DIR` writer - 0:0\\]
Clock data direction. Determines the data direction on the SCICLK pin. The direction is defined differently depending upon the value of the CLK FUNC bit 0=SCICLK is a general-purpose input pin. 1=SCICLK is a general-purpose output pin"]
pub type ClkDirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_DIR` reader - 1:1\\]
Determines the data direction on the SCIRX pin if it is configured with general-purpose I/O functionality (RX FUNC = 0). See Table 12 for bit values. 0=SCIRX is a general-purpose input pin. 1=SCIRX is a general-purpose output pin"]
pub type RxDirR = crate::BitReader;
#[doc = "Field `RX_DIR` writer - 1:1\\]
Determines the data direction on the SCIRX pin if it is configured with general-purpose I/O functionality (RX FUNC = 0). See Table 12 for bit values. 0=SCIRX is a general-purpose input pin. 1=SCIRX is a general-purpose output pin"]
pub type RxDirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_DIR` reader - 2:2\\]
Determines the data direction on the SCITX pin if it is configured with general-purpose I/O functionality (TX FUNC = 0). See Table 11 for bit values. 0=SCITX is a general-purpose input pin. 1=SCITX is a general-purpose output pin"]
pub type TxDirR = crate::BitReader;
#[doc = "Field `TX_DIR` writer - 2:2\\]
Determines the data direction on the SCITX pin if it is configured with general-purpose I/O functionality (TX FUNC = 0). See Table 11 for bit values. 0=SCITX is a general-purpose input pin. 1=SCITX is a general-purpose output pin"]
pub type TxDirW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Clock data direction. Determines the data direction on the SCICLK pin. The direction is defined differently depending upon the value of the CLK FUNC bit 0=SCICLK is a general-purpose input pin. 1=SCICLK is a general-purpose output pin"]
    #[inline(always)]
    pub fn clk_dir(&self) -> ClkDirR {
        ClkDirR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Determines the data direction on the SCIRX pin if it is configured with general-purpose I/O functionality (RX FUNC = 0). See Table 12 for bit values. 0=SCIRX is a general-purpose input pin. 1=SCIRX is a general-purpose output pin"]
    #[inline(always)]
    pub fn rx_dir(&self) -> RxDirR {
        RxDirR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Determines the data direction on the SCITX pin if it is configured with general-purpose I/O functionality (TX FUNC = 0). See Table 11 for bit values. 0=SCITX is a general-purpose input pin. 1=SCITX is a general-purpose output pin"]
    #[inline(always)]
    pub fn tx_dir(&self) -> TxDirR {
        TxDirR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Clock data direction. Determines the data direction on the SCICLK pin. The direction is defined differently depending upon the value of the CLK FUNC bit 0=SCICLK is a general-purpose input pin. 1=SCICLK is a general-purpose output pin"]
    #[inline(always)]
    #[must_use]
    pub fn clk_dir(&mut self) -> ClkDirW<Scipio1Spec> {
        ClkDirW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Determines the data direction on the SCIRX pin if it is configured with general-purpose I/O functionality (RX FUNC = 0). See Table 12 for bit values. 0=SCIRX is a general-purpose input pin. 1=SCIRX is a general-purpose output pin"]
    #[inline(always)]
    #[must_use]
    pub fn rx_dir(&mut self) -> RxDirW<Scipio1Spec> {
        RxDirW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Determines the data direction on the SCITX pin if it is configured with general-purpose I/O functionality (TX FUNC = 0). See Table 11 for bit values. 0=SCITX is a general-purpose input pin. 1=SCITX is a general-purpose output pin"]
    #[inline(always)]
    #[must_use]
    pub fn tx_dir(&mut self) -> TxDirW<Scipio1Spec> {
        TxDirW::new(self, 2)
    }
}
#[doc = "SCI Pin I/O Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`scipio1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scipio1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scipio1Spec;
impl crate::RegisterSpec for Scipio1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scipio1::R`](R) reader structure"]
impl crate::Readable for Scipio1Spec {}
#[doc = "`write(|w| ..)` method takes [`scipio1::W`](W) writer structure"]
impl crate::Writable for Scipio1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCIPIO1 to value 0"]
impl crate::Resettable for Scipio1Spec {
    const RESET_VALUE: u32 = 0;
}
