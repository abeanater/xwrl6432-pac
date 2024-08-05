#[doc = "Register `APPSS_MPU_ERRAGG_STATUS_RAW` reader"]
pub type R = crate::R<AppssMpuErraggStatusRawSpec>;
#[doc = "Register `APPSS_MPU_ERRAGG_STATUS_RAW` writer"]
pub type W = crate::W<AppssMpuErraggStatusRawSpec>;
#[doc = "Field `appss_mpu` reader - 0:0\\]
Raw Status of Interrupt from APSS MPU PROT ERR Set irrespective if the Interupt is masked or unmasked in APPSS_MPU_ERRAGG_MASK"]
pub type AppssMpuR = crate::BitReader;
#[doc = "Field `appss_mpu` writer - 0:0\\]
Raw Status of Interrupt from APSS MPU PROT ERR Set irrespective if the Interupt is masked or unmasked in APPSS_MPU_ERRAGG_MASK"]
pub type AppssMpuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fecss_mpu` reader - 16:16\\]
Raw Status of FECSS MPU PROT ERR. Set irrespective if the Interupt is masked or unmasked in APPSS_MPU_ERRAGG_MASK"]
pub type FecssMpuR = crate::BitReader;
#[doc = "Field `fecss_mpu` writer - 16:16\\]
Raw Status of FECSS MPU PROT ERR. Set irrespective if the Interupt is masked or unmasked in APPSS_MPU_ERRAGG_MASK"]
pub type FecssMpuW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Raw Status of Interrupt from APSS MPU PROT ERR Set irrespective if the Interupt is masked or unmasked in APPSS_MPU_ERRAGG_MASK"]
    #[inline(always)]
    pub fn appss_mpu(&self) -> AppssMpuR {
        AppssMpuR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Raw Status of FECSS MPU PROT ERR. Set irrespective if the Interupt is masked or unmasked in APPSS_MPU_ERRAGG_MASK"]
    #[inline(always)]
    pub fn fecss_mpu(&self) -> FecssMpuR {
        FecssMpuR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Raw Status of Interrupt from APSS MPU PROT ERR Set irrespective if the Interupt is masked or unmasked in APPSS_MPU_ERRAGG_MASK"]
    #[inline(always)]
    #[must_use]
    pub fn appss_mpu(&mut self) -> AppssMpuW<AppssMpuErraggStatusRawSpec> {
        AppssMpuW::new(self, 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Raw Status of FECSS MPU PROT ERR. Set irrespective if the Interupt is masked or unmasked in APPSS_MPU_ERRAGG_MASK"]
    #[inline(always)]
    #[must_use]
    pub fn fecss_mpu(&mut self) -> FecssMpuW<AppssMpuErraggStatusRawSpec> {
        FecssMpuW::new(self, 16)
    }
}
#[doc = "APPSS_MPU_ERRAGG_STATUS_RAW\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_mpu_erragg_status_raw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_mpu_erragg_status_raw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AppssMpuErraggStatusRawSpec;
impl crate::RegisterSpec for AppssMpuErraggStatusRawSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`appss_mpu_erragg_status_raw::R`](R) reader structure"]
impl crate::Readable for AppssMpuErraggStatusRawSpec {}
#[doc = "`write(|w| ..)` method takes [`appss_mpu_erragg_status_raw::W`](W) writer structure"]
impl crate::Writable for AppssMpuErraggStatusRawSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APPSS_MPU_ERRAGG_STATUS_RAW to value 0"]
impl crate::Resettable for AppssMpuErraggStatusRawSpec {
    const RESET_VALUE: u32 = 0;
}
