#[doc = "Register `SCIPIO0` reader"]
pub type R = crate::R<Scipio0Spec>;
#[doc = "Register `SCIPIO0` writer"]
pub type W = crate::W<Scipio0Spec>;
#[doc = "Field `CLK_FUNC` reader - 0:0\\]
Clock function. Defines the function of pin SCICLK. 0=SCICLK is a general-purpose digital I/O pin. 1=SCICLK is the SCI serial clock pin."]
pub type ClkFuncR = crate::BitReader;
#[doc = "Field `CLK_FUNC` writer - 0:0\\]
Clock function. Defines the function of pin SCICLK. 0=SCICLK is a general-purpose digital I/O pin. 1=SCICLK is the SCI serial clock pin."]
pub type ClkFuncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_FUNC` reader - 1:1\\]
Determines the data direction on the SCIRX pin if it is configured with general-purpose I/O functionality (RX FUNC = 0). See Table 12 for bit values. 0=SCIRX is a general-purpose input pin. 1=SCIRX is a general-purpose output pin"]
pub type RxFuncR = crate::BitReader;
#[doc = "Field `RX_FUNC` writer - 1:1\\]
Determines the data direction on the SCIRX pin if it is configured with general-purpose I/O functionality (RX FUNC = 0). See Table 12 for bit values. 0=SCIRX is a general-purpose input pin. 1=SCIRX is a general-purpose output pin"]
pub type RxFuncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_FUNC` reader - 2:2\\]
Defines the function of pin SCITX. 0=SCITX is a general-purpose digital I/O pin. 1=SCITX is the SCI transmit pin."]
pub type TxFuncR = crate::BitReader;
#[doc = "Field `TX_FUNC` writer - 2:2\\]
Defines the function of pin SCITX. 0=SCITX is a general-purpose digital I/O pin. 1=SCITX is the SCI transmit pin."]
pub type TxFuncW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Clock function. Defines the function of pin SCICLK. 0=SCICLK is a general-purpose digital I/O pin. 1=SCICLK is the SCI serial clock pin."]
    #[inline(always)]
    pub fn clk_func(&self) -> ClkFuncR {
        ClkFuncR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Determines the data direction on the SCIRX pin if it is configured with general-purpose I/O functionality (RX FUNC = 0). See Table 12 for bit values. 0=SCIRX is a general-purpose input pin. 1=SCIRX is a general-purpose output pin"]
    #[inline(always)]
    pub fn rx_func(&self) -> RxFuncR {
        RxFuncR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Defines the function of pin SCITX. 0=SCITX is a general-purpose digital I/O pin. 1=SCITX is the SCI transmit pin."]
    #[inline(always)]
    pub fn tx_func(&self) -> TxFuncR {
        TxFuncR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Clock function. Defines the function of pin SCICLK. 0=SCICLK is a general-purpose digital I/O pin. 1=SCICLK is the SCI serial clock pin."]
    #[inline(always)]
    #[must_use]
    pub fn clk_func(&mut self) -> ClkFuncW<Scipio0Spec> {
        ClkFuncW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Determines the data direction on the SCIRX pin if it is configured with general-purpose I/O functionality (RX FUNC = 0). See Table 12 for bit values. 0=SCIRX is a general-purpose input pin. 1=SCIRX is a general-purpose output pin"]
    #[inline(always)]
    #[must_use]
    pub fn rx_func(&mut self) -> RxFuncW<Scipio0Spec> {
        RxFuncW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Defines the function of pin SCITX. 0=SCITX is a general-purpose digital I/O pin. 1=SCITX is the SCI transmit pin."]
    #[inline(always)]
    #[must_use]
    pub fn tx_func(&mut self) -> TxFuncW<Scipio0Spec> {
        TxFuncW::new(self, 2)
    }
}
#[doc = "SCI Pin I/O Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`scipio0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scipio0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scipio0Spec;
impl crate::RegisterSpec for Scipio0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scipio0::R`](R) reader structure"]
impl crate::Readable for Scipio0Spec {}
#[doc = "`write(|w| ..)` method takes [`scipio0::W`](W) writer structure"]
impl crate::Writable for Scipio0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCIPIO0 to value 0"]
impl crate::Resettable for Scipio0Spec {
    const RESET_VALUE: u32 = 0;
}
