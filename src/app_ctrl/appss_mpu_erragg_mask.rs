#[doc = "Register `APPSS_MPU_ERRAGG_MASK` reader"]
pub type R = crate::R<AppssMpuErraggMaskSpec>;
#[doc = "Register `APPSS_MPU_ERRAGG_MASK` writer"]
pub type W = crate::W<AppssMpuErraggMaskSpec>;
#[doc = "Field `appss_mpu` reader - 0:0\\]
Mask Interrupt from APSS MPU to aggregated Interrupt MPU_PROT_AGG_ERR 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
pub type AppssMpuR = crate::BitReader;
#[doc = "Field `appss_mpu` writer - 0:0\\]
Mask Interrupt from APSS MPU to aggregated Interrupt MPU_PROT_AGG_ERR 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
pub type AppssMpuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fecss_mpu` reader - 16:16\\]
Mask Interrupt from FECSS MPU to aggregated Interrupt MPU_PROT_AGG_ERR 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
pub type FecssMpuR = crate::BitReader;
#[doc = "Field `fecss_mpu` writer - 16:16\\]
Mask Interrupt from FECSS MPU to aggregated Interrupt MPU_PROT_AGG_ERR 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
pub type FecssMpuW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Mask Interrupt from APSS MPU to aggregated Interrupt MPU_PROT_AGG_ERR 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
    #[inline(always)]
    pub fn appss_mpu(&self) -> AppssMpuR {
        AppssMpuR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Mask Interrupt from FECSS MPU to aggregated Interrupt MPU_PROT_AGG_ERR 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
    #[inline(always)]
    pub fn fecss_mpu(&self) -> FecssMpuR {
        FecssMpuR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Mask Interrupt from APSS MPU to aggregated Interrupt MPU_PROT_AGG_ERR 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
    #[inline(always)]
    #[must_use]
    pub fn appss_mpu(&mut self) -> AppssMpuW<AppssMpuErraggMaskSpec> {
        AppssMpuW::new(self, 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Mask Interrupt from FECSS MPU to aggregated Interrupt MPU_PROT_AGG_ERR 1 : Interrupt is Masked 0 : Interrupt is Unmasked"]
    #[inline(always)]
    #[must_use]
    pub fn fecss_mpu(&mut self) -> FecssMpuW<AppssMpuErraggMaskSpec> {
        FecssMpuW::new(self, 16)
    }
}
#[doc = "APPSS_MPU_ERRAGG_MASK\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_mpu_erragg_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_mpu_erragg_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AppssMpuErraggMaskSpec;
impl crate::RegisterSpec for AppssMpuErraggMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`appss_mpu_erragg_mask::R`](R) reader structure"]
impl crate::Readable for AppssMpuErraggMaskSpec {}
#[doc = "`write(|w| ..)` method takes [`appss_mpu_erragg_mask::W`](W) writer structure"]
impl crate::Writable for AppssMpuErraggMaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APPSS_MPU_ERRAGG_MASK to value 0"]
impl crate::Resettable for AppssMpuErraggMaskSpec {
    const RESET_VALUE: u32 = 0;
}
