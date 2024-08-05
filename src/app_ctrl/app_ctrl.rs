#[doc = "Register `APP_CTRL` reader"]
pub type R = crate::R<AppCtrlSpec>;
#[doc = "Register `APP_CTRL` writer"]
pub type W = crate::W<AppCtrlSpec>;
#[doc = "Field `ecc_disable_2k_ram` reader - 0:0\\]
Reserved"]
pub type EccDisable2kRamR = crate::BitReader;
#[doc = "Field `ecc_disable_2k_ram` writer - 0:0\\]
Reserved"]
pub type EccDisable2kRamW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Reserved"]
    #[inline(always)]
    pub fn ecc_disable_2k_ram(&self) -> EccDisable2kRamR {
        EccDisable2kRamR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn ecc_disable_2k_ram(&mut self) -> EccDisable2kRamW<AppCtrlSpec> {
        EccDisable2kRamW::new(self, 0)
    }
}
#[doc = "APP_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`app_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AppCtrlSpec;
impl crate::RegisterSpec for AppCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`app_ctrl::R`](R) reader structure"]
impl crate::Readable for AppCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`app_ctrl::W`](W) writer structure"]
impl crate::Writable for AppCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APP_CTRL to value 0"]
impl crate::Resettable for AppCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
