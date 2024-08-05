#[doc = "Register `APPSS_RAM3_OWRITE_ERR` reader"]
pub type R = crate::R<AppssRam3OwriteErrSpec>;
#[doc = "Register `APPSS_RAM3_OWRITE_ERR` writer"]
pub type W = crate::W<AppssRam3OwriteErrSpec>;
#[doc = "Field `err` reader - 0:0\\]
RAM3 ahb2sram write error - Sticky Bit"]
pub type ErrR = crate::BitReader;
#[doc = "Field `err` writer - 0:0\\]
RAM3 ahb2sram write error - Sticky Bit"]
pub type ErrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
RAM3 ahb2sram write error - Sticky Bit"]
    #[inline(always)]
    pub fn err(&self) -> ErrR {
        ErrR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
RAM3 ahb2sram write error - Sticky Bit"]
    #[inline(always)]
    #[must_use]
    pub fn err(&mut self) -> ErrW<AppssRam3OwriteErrSpec> {
        ErrW::new(self, 0)
    }
}
#[doc = "APPSS_RAM3_OWRITE_ERR\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_ram3_owrite_err::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_ram3_owrite_err::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AppssRam3OwriteErrSpec;
impl crate::RegisterSpec for AppssRam3OwriteErrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`appss_ram3_owrite_err::R`](R) reader structure"]
impl crate::Readable for AppssRam3OwriteErrSpec {}
#[doc = "`write(|w| ..)` method takes [`appss_ram3_owrite_err::W`](W) writer structure"]
impl crate::Writable for AppssRam3OwriteErrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APPSS_RAM3_OWRITE_ERR to value 0"]
impl crate::Resettable for AppssRam3OwriteErrSpec {
    const RESET_VALUE: u32 = 0;
}
