#[doc = "Register `APPSS_QSPI_CONFIG` reader"]
pub type R = crate::R<AppssQspiConfigSpec>;
#[doc = "Register `APPSS_QSPI_CONFIG` writer"]
pub type W = crate::W<AppssQspiConfigSpec>;
#[doc = "Field `ext_clk` reader - 0:0\\]
Reserved"]
pub type ExtClkR = crate::BitReader;
#[doc = "Field `ext_clk` writer - 0:0\\]
Reserved"]
pub type ExtClkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `clk_loopback` reader - 8:8\\]
Reserved"]
pub type ClkLoopbackR = crate::BitReader;
#[doc = "Field `clk_loopback` writer - 8:8\\]
Reserved"]
pub type ClkLoopbackW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Reserved"]
    #[inline(always)]
    pub fn ext_clk(&self) -> ExtClkR {
        ExtClkR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Reserved"]
    #[inline(always)]
    pub fn clk_loopback(&self) -> ClkLoopbackR {
        ClkLoopbackR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn ext_clk(&mut self) -> ExtClkW<AppssQspiConfigSpec> {
        ExtClkW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn clk_loopback(&mut self) -> ClkLoopbackW<AppssQspiConfigSpec> {
        ClkLoopbackW::new(self, 8)
    }
}
#[doc = "APPSS_QSPI_CONFIG\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_qspi_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_qspi_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AppssQspiConfigSpec;
impl crate::RegisterSpec for AppssQspiConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`appss_qspi_config::R`](R) reader structure"]
impl crate::Readable for AppssQspiConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`appss_qspi_config::W`](W) writer structure"]
impl crate::Writable for AppssQspiConfigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APPSS_QSPI_CONFIG to value 0"]
impl crate::Resettable for AppssQspiConfigSpec {
    const RESET_VALUE: u32 = 0;
}
