#[doc = "Register `APPSS_SHRD_RAM_OWRITE_ERR` reader"]
pub type R = crate::R<AppssShrdRamOwriteErrSpec>;
#[doc = "Register `APPSS_SHRD_RAM_OWRITE_ERR` writer"]
pub type W = crate::W<AppssShrdRamOwriteErrSpec>;
#[doc = "Field `err` reader - 0:0\\]
SHARED RAM ahb2sram write error - Sticky Bit"]
pub type ErrR = crate::BitReader;
#[doc = "Field `err` writer - 0:0\\]
SHARED RAM ahb2sram write error - Sticky Bit"]
pub type ErrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
SHARED RAM ahb2sram write error - Sticky Bit"]
    #[inline(always)]
    pub fn err(&self) -> ErrR {
        ErrR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
SHARED RAM ahb2sram write error - Sticky Bit"]
    #[inline(always)]
    #[must_use]
    pub fn err(&mut self) -> ErrW<AppssShrdRamOwriteErrSpec> {
        ErrW::new(self, 0)
    }
}
#[doc = "APPSS_SHRD_RAM_OWRITE_ERR\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_shrd_ram_owrite_err::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_shrd_ram_owrite_err::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AppssShrdRamOwriteErrSpec;
impl crate::RegisterSpec for AppssShrdRamOwriteErrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`appss_shrd_ram_owrite_err::R`](R) reader structure"]
impl crate::Readable for AppssShrdRamOwriteErrSpec {}
#[doc = "`write(|w| ..)` method takes [`appss_shrd_ram_owrite_err::W`](W) writer structure"]
impl crate::Writable for AppssShrdRamOwriteErrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APPSS_SHRD_RAM_OWRITE_ERR to value 0"]
impl crate::Resettable for AppssShrdRamOwriteErrSpec {
    const RESET_VALUE: u32 = 0;
}
