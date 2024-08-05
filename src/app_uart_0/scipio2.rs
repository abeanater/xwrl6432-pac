#[doc = "Register `SCIPIO2` reader"]
pub type R = crate::R<Scipio2Spec>;
#[doc = "Register `SCIPIO2` writer"]
pub type W = crate::W<Scipio2Spec>;
#[doc = "Field `CLK_DATA_IN` reader - 0:0\\]
Contains the current value on pin SCICLK. 0=Pin SCICLK value is logic low. 1=Pin SCICLK value is logic high."]
pub type ClkDataInR = crate::BitReader;
#[doc = "Field `CLK_DATA_IN` writer - 0:0\\]
Contains the current value on pin SCICLK. 0=Pin SCICLK value is logic low. 1=Pin SCICLK value is logic high."]
pub type ClkDataInW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_DATA_IN` reader - 1:1\\]
Contains current value on the SCIRX pin. 0=SCIRX value is logic low. 1=SCIRX value is logic high."]
pub type RxDataInR = crate::BitReader;
#[doc = "Field `RX_DATA_IN` writer - 1:1\\]
Contains current value on the SCIRX pin. 0=SCIRX value is logic low. 1=SCIRX value is logic high."]
pub type RxDataInW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_DATA_IN` reader - 2:2\\]
Contains current value on the SCITX pin. 0=SCITX value is logic low. 1=SCITX value is logic high."]
pub type TxDataInR = crate::BitReader;
#[doc = "Field `TX_DATA_IN` writer - 2:2\\]
Contains current value on the SCITX pin. 0=SCITX value is logic low. 1=SCITX value is logic high."]
pub type TxDataInW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Contains the current value on pin SCICLK. 0=Pin SCICLK value is logic low. 1=Pin SCICLK value is logic high."]
    #[inline(always)]
    pub fn clk_data_in(&self) -> ClkDataInR {
        ClkDataInR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Contains current value on the SCIRX pin. 0=SCIRX value is logic low. 1=SCIRX value is logic high."]
    #[inline(always)]
    pub fn rx_data_in(&self) -> RxDataInR {
        RxDataInR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Contains current value on the SCITX pin. 0=SCITX value is logic low. 1=SCITX value is logic high."]
    #[inline(always)]
    pub fn tx_data_in(&self) -> TxDataInR {
        TxDataInR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Contains the current value on pin SCICLK. 0=Pin SCICLK value is logic low. 1=Pin SCICLK value is logic high."]
    #[inline(always)]
    #[must_use]
    pub fn clk_data_in(&mut self) -> ClkDataInW<Scipio2Spec> {
        ClkDataInW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Contains current value on the SCIRX pin. 0=SCIRX value is logic low. 1=SCIRX value is logic high."]
    #[inline(always)]
    #[must_use]
    pub fn rx_data_in(&mut self) -> RxDataInW<Scipio2Spec> {
        RxDataInW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Contains current value on the SCITX pin. 0=SCITX value is logic low. 1=SCITX value is logic high."]
    #[inline(always)]
    #[must_use]
    pub fn tx_data_in(&mut self) -> TxDataInW<Scipio2Spec> {
        TxDataInW::new(self, 2)
    }
}
#[doc = "SCI Pin I/O Control Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`scipio2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scipio2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scipio2Spec;
impl crate::RegisterSpec for Scipio2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scipio2::R`](R) reader structure"]
impl crate::Readable for Scipio2Spec {}
#[doc = "`write(|w| ..)` method takes [`scipio2::W`](W) writer structure"]
impl crate::Writable for Scipio2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCIPIO2 to value 0"]
impl crate::Resettable for Scipio2Spec {
    const RESET_VALUE: u32 = 0;
}
