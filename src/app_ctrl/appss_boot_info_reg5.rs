#[doc = "Register `APPSS_BOOT_INFO_REG5` reader"]
pub type R = crate::R<AppssBootInfoReg5Spec>;
#[doc = "Register `APPSS_BOOT_INFO_REG5` writer"]
pub type W = crate::W<AppssBootInfoReg5Spec>;
#[doc = "Field `config` reader - 31:0\\]
Reserved Register for Software use"]
pub type ConfigR = crate::FieldReader<u32>;
#[doc = "Field `config` writer - 31:0\\]
Reserved Register for Software use"]
pub type ConfigW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved Register for Software use"]
    #[inline(always)]
    pub fn config(&self) -> ConfigR {
        ConfigR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved Register for Software use"]
    #[inline(always)]
    #[must_use]
    pub fn config(&mut self) -> ConfigW<AppssBootInfoReg5Spec> {
        ConfigW::new(self, 0)
    }
}
#[doc = "APPSS_BOOT_INFO_REG5\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_boot_info_reg5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_boot_info_reg5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AppssBootInfoReg5Spec;
impl crate::RegisterSpec for AppssBootInfoReg5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`appss_boot_info_reg5::R`](R) reader structure"]
impl crate::Readable for AppssBootInfoReg5Spec {}
#[doc = "`write(|w| ..)` method takes [`appss_boot_info_reg5::W`](W) writer structure"]
impl crate::Writable for AppssBootInfoReg5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APPSS_BOOT_INFO_REG5 to value 0"]
impl crate::Resettable for AppssBootInfoReg5Spec {
    const RESET_VALUE: u32 = 0;
}
