#[doc = "Register `APPSS_MPU_ERRAGG_STATUS` reader"]
pub type R = crate::R<AppssMpuErraggStatusSpec>;
#[doc = "Register `APPSS_MPU_ERRAGG_STATUS` writer"]
pub type W = crate::W<AppssMpuErraggStatusSpec>;
#[doc = "Field `appss_mpu` reader - 0:0\\]
Status of Interrupt from APSS MPU. Set only if Interupt is unmasked in APPSS_MPU_ERRAGG_MASK Wrie 0x1 to clear this interrupt."]
pub type AppssMpuR = crate::BitReader;
#[doc = "Field `appss_mpu` writer - 0:0\\]
Status of Interrupt from APSS MPU. Set only if Interupt is unmasked in APPSS_MPU_ERRAGG_MASK Wrie 0x1 to clear this interrupt."]
pub type AppssMpuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fecss_mpu` reader - 16:16\\]
Status of Interrupt from FECSS MPU. Set only if Interupt is unmasked in APPSS_MPU_ERRAGG_MASK Wrie 0x1 to clear this interrupt."]
pub type FecssMpuR = crate::BitReader;
#[doc = "Field `fecss_mpu` writer - 16:16\\]
Status of Interrupt from FECSS MPU. Set only if Interupt is unmasked in APPSS_MPU_ERRAGG_MASK Wrie 0x1 to clear this interrupt."]
pub type FecssMpuW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Status of Interrupt from APSS MPU. Set only if Interupt is unmasked in APPSS_MPU_ERRAGG_MASK Wrie 0x1 to clear this interrupt."]
    #[inline(always)]
    pub fn appss_mpu(&self) -> AppssMpuR {
        AppssMpuR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Status of Interrupt from FECSS MPU. Set only if Interupt is unmasked in APPSS_MPU_ERRAGG_MASK Wrie 0x1 to clear this interrupt."]
    #[inline(always)]
    pub fn fecss_mpu(&self) -> FecssMpuR {
        FecssMpuR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Status of Interrupt from APSS MPU. Set only if Interupt is unmasked in APPSS_MPU_ERRAGG_MASK Wrie 0x1 to clear this interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn appss_mpu(&mut self) -> AppssMpuW<AppssMpuErraggStatusSpec> {
        AppssMpuW::new(self, 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Status of Interrupt from FECSS MPU. Set only if Interupt is unmasked in APPSS_MPU_ERRAGG_MASK Wrie 0x1 to clear this interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn fecss_mpu(&mut self) -> FecssMpuW<AppssMpuErraggStatusSpec> {
        FecssMpuW::new(self, 16)
    }
}
#[doc = "APPSS_MPU_ERRAGG_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_mpu_erragg_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_mpu_erragg_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AppssMpuErraggStatusSpec;
impl crate::RegisterSpec for AppssMpuErraggStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`appss_mpu_erragg_status::R`](R) reader structure"]
impl crate::Readable for AppssMpuErraggStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`appss_mpu_erragg_status::W`](W) writer structure"]
impl crate::Writable for AppssMpuErraggStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APPSS_MPU_ERRAGG_STATUS to value 0"]
impl crate::Resettable for AppssMpuErraggStatusSpec {
    const RESET_VALUE: u32 = 0;
}
