#[doc = "Register `SCIPIO4` reader"]
pub type R = crate::R<Scipio4Spec>;
#[doc = "Register `SCIPIO4` writer"]
pub type W = crate::W<Scipio4Spec>;
#[doc = "Field `CLK_DATA_SET` reader - 0:0\\]
Sets the data to be output on pin SCICLK if the following conditions are met: CLK FUNC = 0 (SCICLK pin is a general-purpose I/O.) CLK DIR = 1 (SCICLK pin is a general-purpose output.)"]
pub type ClkDataSetR = crate::BitReader;
#[doc = "Field `CLK_DATA_SET` writer - 0:0\\]
Sets the data to be output on pin SCICLK if the following conditions are met: CLK FUNC = 0 (SCICLK pin is a general-purpose I/O.) CLK DIR = 1 (SCICLK pin is a general-purpose output.)"]
pub type ClkDataSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_DATA_SET` reader - 1:1\\]
Sets the data to be output on pin SCIRX if the following conditions are met: RX FUNC = 0 (SCIRX pin is a general-purpose I/O.) RX DIR = 1 (SCIRX pin is a general-purpose output.)"]
pub type RxDataSetR = crate::BitReader;
#[doc = "Field `RX_DATA_SET` writer - 1:1\\]
Sets the data to be output on pin SCIRX if the following conditions are met: RX FUNC = 0 (SCIRX pin is a general-purpose I/O.) RX DIR = 1 (SCIRX pin is a general-purpose output.)"]
pub type RxDataSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_DATA_SET` reader - 2:2\\]
Sets the data to be output on pin SCITX if the following conditions are met: TX FUNC = 0 (SCITX pin is a general-purpose I/O.) TX DIR = 1 (SCITX pin is a general-purpose output.)"]
pub type TxDataSetR = crate::BitReader;
#[doc = "Field `TX_DATA_SET` writer - 2:2\\]
Sets the data to be output on pin SCITX if the following conditions are met: TX FUNC = 0 (SCITX pin is a general-purpose I/O.) TX DIR = 1 (SCITX pin is a general-purpose output.)"]
pub type TxDataSetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Sets the data to be output on pin SCICLK if the following conditions are met: CLK FUNC = 0 (SCICLK pin is a general-purpose I/O.) CLK DIR = 1 (SCICLK pin is a general-purpose output.)"]
    #[inline(always)]
    pub fn clk_data_set(&self) -> ClkDataSetR {
        ClkDataSetR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Sets the data to be output on pin SCIRX if the following conditions are met: RX FUNC = 0 (SCIRX pin is a general-purpose I/O.) RX DIR = 1 (SCIRX pin is a general-purpose output.)"]
    #[inline(always)]
    pub fn rx_data_set(&self) -> RxDataSetR {
        RxDataSetR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Sets the data to be output on pin SCITX if the following conditions are met: TX FUNC = 0 (SCITX pin is a general-purpose I/O.) TX DIR = 1 (SCITX pin is a general-purpose output.)"]
    #[inline(always)]
    pub fn tx_data_set(&self) -> TxDataSetR {
        TxDataSetR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Sets the data to be output on pin SCICLK if the following conditions are met: CLK FUNC = 0 (SCICLK pin is a general-purpose I/O.) CLK DIR = 1 (SCICLK pin is a general-purpose output.)"]
    #[inline(always)]
    #[must_use]
    pub fn clk_data_set(&mut self) -> ClkDataSetW<Scipio4Spec> {
        ClkDataSetW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Sets the data to be output on pin SCIRX if the following conditions are met: RX FUNC = 0 (SCIRX pin is a general-purpose I/O.) RX DIR = 1 (SCIRX pin is a general-purpose output.)"]
    #[inline(always)]
    #[must_use]
    pub fn rx_data_set(&mut self) -> RxDataSetW<Scipio4Spec> {
        RxDataSetW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Sets the data to be output on pin SCITX if the following conditions are met: TX FUNC = 0 (SCITX pin is a general-purpose I/O.) TX DIR = 1 (SCITX pin is a general-purpose output.)"]
    #[inline(always)]
    #[must_use]
    pub fn tx_data_set(&mut self) -> TxDataSetW<Scipio4Spec> {
        TxDataSetW::new(self, 2)
    }
}
#[doc = "SCI Pin I/O Control Register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`scipio4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scipio4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scipio4Spec;
impl crate::RegisterSpec for Scipio4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scipio4::R`](R) reader structure"]
impl crate::Readable for Scipio4Spec {}
#[doc = "`write(|w| ..)` method takes [`scipio4::W`](W) writer structure"]
impl crate::Writable for Scipio4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCIPIO4 to value 0"]
impl crate::Resettable for Scipio4Spec {
    const RESET_VALUE: u32 = 0;
}
