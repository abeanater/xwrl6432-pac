#[doc = "Register `SCIPIO3` reader"]
pub type R = crate::R<Scipio3Spec>;
#[doc = "Register `SCIPIO3` writer"]
pub type W = crate::W<Scipio3Spec>;
#[doc = "Field `CLK_DATA_OUT` reader - 0:0\\]
Contains the data to be output on pin SCICLK if the following conditions are met: CLK FUNC = 0 (SCICLK pin is a general-purpose I/O.) CLK DATA DIR = 1 (SCICLK pin is a general-purpose output.) 0=Output value on SCICLK is a 0 (logic low). 1=Output value on SCICLK is a 1 (logic high)."]
pub type ClkDataOutR = crate::BitReader;
#[doc = "Field `CLK_DATA_OUT` writer - 0:0\\]
Contains the data to be output on pin SCICLK if the following conditions are met: CLK FUNC = 0 (SCICLK pin is a general-purpose I/O.) CLK DATA DIR = 1 (SCICLK pin is a general-purpose output.) 0=Output value on SCICLK is a 0 (logic low). 1=Output value on SCICLK is a 1 (logic high)."]
pub type ClkDataOutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_DATA_OUT` reader - 1:1\\]
Contains the data to be output on pin SCIRX if the following conditions are met: RX FUNC = 0 (SCIRX pin is a general-purpose I/O.) RX DATA DIR = 1 (SCIRX pin is a general-purpose output.) 0=Output value on SCIRX is 0 (logic low). 1=Output value on SCIRX is 1 (logic high)."]
pub type RxDataOutR = crate::BitReader;
#[doc = "Field `RX_DATA_OUT` writer - 1:1\\]
Contains the data to be output on pin SCIRX if the following conditions are met: RX FUNC = 0 (SCIRX pin is a general-purpose I/O.) RX DATA DIR = 1 (SCIRX pin is a general-purpose output.) 0=Output value on SCIRX is 0 (logic low). 1=Output value on SCIRX is 1 (logic high)."]
pub type RxDataOutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_DATA_OUT` reader - 2:2\\]
Contains the data to be output on pin SCITX if the following conditions are met: TX FUNC = 0 (SCITX pin is a general-purpose I/O.) TX DATA DIR = 1 (SCITX pin is a general-purpose output.) 0=Output value on SCITX is a 0 (logic low). 1=Output value on SCITX is a 1 (logic high)."]
pub type TxDataOutR = crate::BitReader;
#[doc = "Field `TX_DATA_OUT` writer - 2:2\\]
Contains the data to be output on pin SCITX if the following conditions are met: TX FUNC = 0 (SCITX pin is a general-purpose I/O.) TX DATA DIR = 1 (SCITX pin is a general-purpose output.) 0=Output value on SCITX is a 0 (logic low). 1=Output value on SCITX is a 1 (logic high)."]
pub type TxDataOutW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Contains the data to be output on pin SCICLK if the following conditions are met: CLK FUNC = 0 (SCICLK pin is a general-purpose I/O.) CLK DATA DIR = 1 (SCICLK pin is a general-purpose output.) 0=Output value on SCICLK is a 0 (logic low). 1=Output value on SCICLK is a 1 (logic high)."]
    #[inline(always)]
    pub fn clk_data_out(&self) -> ClkDataOutR {
        ClkDataOutR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Contains the data to be output on pin SCIRX if the following conditions are met: RX FUNC = 0 (SCIRX pin is a general-purpose I/O.) RX DATA DIR = 1 (SCIRX pin is a general-purpose output.) 0=Output value on SCIRX is 0 (logic low). 1=Output value on SCIRX is 1 (logic high)."]
    #[inline(always)]
    pub fn rx_data_out(&self) -> RxDataOutR {
        RxDataOutR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Contains the data to be output on pin SCITX if the following conditions are met: TX FUNC = 0 (SCITX pin is a general-purpose I/O.) TX DATA DIR = 1 (SCITX pin is a general-purpose output.) 0=Output value on SCITX is a 0 (logic low). 1=Output value on SCITX is a 1 (logic high)."]
    #[inline(always)]
    pub fn tx_data_out(&self) -> TxDataOutR {
        TxDataOutR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Contains the data to be output on pin SCICLK if the following conditions are met: CLK FUNC = 0 (SCICLK pin is a general-purpose I/O.) CLK DATA DIR = 1 (SCICLK pin is a general-purpose output.) 0=Output value on SCICLK is a 0 (logic low). 1=Output value on SCICLK is a 1 (logic high)."]
    #[inline(always)]
    #[must_use]
    pub fn clk_data_out(&mut self) -> ClkDataOutW<Scipio3Spec> {
        ClkDataOutW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Contains the data to be output on pin SCIRX if the following conditions are met: RX FUNC = 0 (SCIRX pin is a general-purpose I/O.) RX DATA DIR = 1 (SCIRX pin is a general-purpose output.) 0=Output value on SCIRX is 0 (logic low). 1=Output value on SCIRX is 1 (logic high)."]
    #[inline(always)]
    #[must_use]
    pub fn rx_data_out(&mut self) -> RxDataOutW<Scipio3Spec> {
        RxDataOutW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Contains the data to be output on pin SCITX if the following conditions are met: TX FUNC = 0 (SCITX pin is a general-purpose I/O.) TX DATA DIR = 1 (SCITX pin is a general-purpose output.) 0=Output value on SCITX is a 0 (logic low). 1=Output value on SCITX is a 1 (logic high)."]
    #[inline(always)]
    #[must_use]
    pub fn tx_data_out(&mut self) -> TxDataOutW<Scipio3Spec> {
        TxDataOutW::new(self, 2)
    }
}
#[doc = "SCI Pin I/O Control Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`scipio3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scipio3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scipio3Spec;
impl crate::RegisterSpec for Scipio3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scipio3::R`](R) reader structure"]
impl crate::Readable for Scipio3Spec {}
#[doc = "`write(|w| ..)` method takes [`scipio3::W`](W) writer structure"]
impl crate::Writable for Scipio3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCIPIO3 to value 0"]
impl crate::Resettable for Scipio3Spec {
    const RESET_VALUE: u32 = 0;
}
