#[doc = "Register `APPSS_QSPI_CHAR_EXT_CLK_EN` reader"]
pub type R = crate::R<AppssQspiCharExtClkEnSpec>;
#[doc = "Register `APPSS_QSPI_CHAR_EXT_CLK_EN` writer"]
pub type W = crate::W<AppssQspiCharExtClkEnSpec>;
#[doc = "Field `enable` reader - 0:0\\]
Selects the QSPI system clock. Only for DFT purposes. This should not be changed for functional operation. 0 => QSPI_CLK from APPSS RCM 1 => SPI1_CLK from APPSS RCM"]
pub type EnableR = crate::BitReader;
#[doc = "Field `enable` writer - 0:0\\]
Selects the QSPI system clock. Only for DFT purposes. This should not be changed for functional operation. 0 => QSPI_CLK from APPSS RCM 1 => SPI1_CLK from APPSS RCM"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Selects the QSPI system clock. Only for DFT purposes. This should not be changed for functional operation. 0 => QSPI_CLK from APPSS RCM 1 => SPI1_CLK from APPSS RCM"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Selects the QSPI system clock. Only for DFT purposes. This should not be changed for functional operation. 0 => QSPI_CLK from APPSS RCM 1 => SPI1_CLK from APPSS RCM"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<AppssQspiCharExtClkEnSpec> {
        EnableW::new(self, 0)
    }
}
#[doc = "APPSS_QSPI_CHAR_EXT_CLK_EN\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_qspi_char_ext_clk_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_qspi_char_ext_clk_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AppssQspiCharExtClkEnSpec;
impl crate::RegisterSpec for AppssQspiCharExtClkEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`appss_qspi_char_ext_clk_en::R`](R) reader structure"]
impl crate::Readable for AppssQspiCharExtClkEnSpec {}
#[doc = "`write(|w| ..)` method takes [`appss_qspi_char_ext_clk_en::W`](W) writer structure"]
impl crate::Writable for AppssQspiCharExtClkEnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APPSS_QSPI_CHAR_EXT_CLK_EN to value 0"]
impl crate::Resettable for AppssQspiCharExtClkEnSpec {
    const RESET_VALUE: u32 = 0;
}
