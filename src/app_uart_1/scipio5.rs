#[doc = "Register `SCIPIO5` reader"]
pub type R = crate::R<Scipio5Spec>;
#[doc = "Register `SCIPIO5` writer"]
pub type W = crate::W<Scipio5Spec>;
#[doc = "Field `CLK_DATA_CLR` reader - 0:0\\]
Clears the data to be output on pin SCITX if the following conditions are met: TX FUNC = 0 (SCITX pin is a general-purpose I/O.) TX DIR = 1 (SCITX pin is a general-purpose output.)"]
pub type ClkDataClrR = crate::BitReader;
#[doc = "Field `CLK_DATA_CLR` writer - 0:0\\]
Clears the data to be output on pin SCITX if the following conditions are met: TX FUNC = 0 (SCITX pin is a general-purpose I/O.) TX DIR = 1 (SCITX pin is a general-purpose output.)"]
pub type ClkDataClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_DATA_CLR` reader - 1:1\\]
Clears the data to be output on pin SCIRX if the following conditions are met: RX FUNC = 0 (SCITX pin is a general-purpose I/O.) RX DIR = 1 (SCITX pin is a general-purpose output.)"]
pub type RxDataClrR = crate::BitReader;
#[doc = "Field `RX_DATA_CLR` writer - 1:1\\]
Clears the data to be output on pin SCIRX if the following conditions are met: RX FUNC = 0 (SCITX pin is a general-purpose I/O.) RX DIR = 1 (SCITX pin is a general-purpose output.)"]
pub type RxDataClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_DATA_CLR` reader - 2:2\\]
Clears the data to be output on pin SCITX if the following conditions are met: TX FUNC = 0 (SCITX pin is a general-purpose I/O.) TX DIR = 1 (SCITX pin is a general-purpose output.)"]
pub type TxDataClrR = crate::BitReader;
#[doc = "Field `TX_DATA_CLR` writer - 2:2\\]
Clears the data to be output on pin SCITX if the following conditions are met: TX FUNC = 0 (SCITX pin is a general-purpose I/O.) TX DIR = 1 (SCITX pin is a general-purpose output.)"]
pub type TxDataClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Clears the data to be output on pin SCITX if the following conditions are met: TX FUNC = 0 (SCITX pin is a general-purpose I/O.) TX DIR = 1 (SCITX pin is a general-purpose output.)"]
    #[inline(always)]
    pub fn clk_data_clr(&self) -> ClkDataClrR {
        ClkDataClrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Clears the data to be output on pin SCIRX if the following conditions are met: RX FUNC = 0 (SCITX pin is a general-purpose I/O.) RX DIR = 1 (SCITX pin is a general-purpose output.)"]
    #[inline(always)]
    pub fn rx_data_clr(&self) -> RxDataClrR {
        RxDataClrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Clears the data to be output on pin SCITX if the following conditions are met: TX FUNC = 0 (SCITX pin is a general-purpose I/O.) TX DIR = 1 (SCITX pin is a general-purpose output.)"]
    #[inline(always)]
    pub fn tx_data_clr(&self) -> TxDataClrR {
        TxDataClrR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Clears the data to be output on pin SCITX if the following conditions are met: TX FUNC = 0 (SCITX pin is a general-purpose I/O.) TX DIR = 1 (SCITX pin is a general-purpose output.)"]
    #[inline(always)]
    #[must_use]
    pub fn clk_data_clr(&mut self) -> ClkDataClrW<Scipio5Spec> {
        ClkDataClrW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Clears the data to be output on pin SCIRX if the following conditions are met: RX FUNC = 0 (SCITX pin is a general-purpose I/O.) RX DIR = 1 (SCITX pin is a general-purpose output.)"]
    #[inline(always)]
    #[must_use]
    pub fn rx_data_clr(&mut self) -> RxDataClrW<Scipio5Spec> {
        RxDataClrW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Clears the data to be output on pin SCITX if the following conditions are met: TX FUNC = 0 (SCITX pin is a general-purpose I/O.) TX DIR = 1 (SCITX pin is a general-purpose output.)"]
    #[inline(always)]
    #[must_use]
    pub fn tx_data_clr(&mut self) -> TxDataClrW<Scipio5Spec> {
        TxDataClrW::new(self, 2)
    }
}
#[doc = "SCI Pin I/O Control Register 5\n\nYou can [`read`](crate::Reg::read) this register and get [`scipio5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scipio5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scipio5Spec;
impl crate::RegisterSpec for Scipio5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scipio5::R`](R) reader structure"]
impl crate::Readable for Scipio5Spec {}
#[doc = "`write(|w| ..)` method takes [`scipio5::W`](W) writer structure"]
impl crate::Writable for Scipio5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCIPIO5 to value 0"]
impl crate::Resettable for Scipio5Spec {
    const RESET_VALUE: u32 = 0;
}
