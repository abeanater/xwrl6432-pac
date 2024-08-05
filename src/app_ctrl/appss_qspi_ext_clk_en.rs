#[doc = "Register `APPSS_QSPI_EXT_CLK_EN` reader"]
pub type R = crate::R<AppssQspiExtClkEnSpec>;
#[doc = "Register `APPSS_QSPI_EXT_CLK_EN` writer"]
pub type W = crate::W<AppssQspiExtClkEnSpec>;
#[doc = "Field `enable` reader - 0:0\\]
Selects the QSPI interface clock. This register bit is used only for AC CHAR operation and not for functional usage. 0 => default QSPI IP clock return from PAD 1 => SPI1 IF CLK. (McSPI IF clock)."]
pub type EnableR = crate::BitReader;
#[doc = "Field `enable` writer - 0:0\\]
Selects the QSPI interface clock. This register bit is used only for AC CHAR operation and not for functional usage. 0 => default QSPI IP clock return from PAD 1 => SPI1 IF CLK. (McSPI IF clock)."]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Selects the QSPI interface clock. This register bit is used only for AC CHAR operation and not for functional usage. 0 => default QSPI IP clock return from PAD 1 => SPI1 IF CLK. (McSPI IF clock)."]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Selects the QSPI interface clock. This register bit is used only for AC CHAR operation and not for functional usage. 0 => default QSPI IP clock return from PAD 1 => SPI1 IF CLK. (McSPI IF clock)."]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<AppssQspiExtClkEnSpec> {
        EnableW::new(self, 0)
    }
}
#[doc = "APPSS_QSPI_EXT_CLK_EN\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_qspi_ext_clk_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_qspi_ext_clk_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AppssQspiExtClkEnSpec;
impl crate::RegisterSpec for AppssQspiExtClkEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`appss_qspi_ext_clk_en::R`](R) reader structure"]
impl crate::Readable for AppssQspiExtClkEnSpec {}
#[doc = "`write(|w| ..)` method takes [`appss_qspi_ext_clk_en::W`](W) writer structure"]
impl crate::Writable for AppssQspiExtClkEnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APPSS_QSPI_EXT_CLK_EN to value 0"]
impl crate::Resettable for AppssQspiExtClkEnSpec {
    const RESET_VALUE: u32 = 0;
}
