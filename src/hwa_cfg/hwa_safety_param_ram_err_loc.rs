#[doc = "Register `HWA_SAFETY_PARAM_RAM_ERR_LOC` reader"]
pub type R = crate::R<HwaSafetyParamRamErrLocSpec>;
#[doc = "Register `HWA_SAFETY_PARAM_RAM_ERR_LOC` writer"]
pub type W = crate::W<HwaSafetyParamRamErrLocSpec>;
#[doc = "Field `SPARE` reader - 31:0\\]
Reserved.TI internal"]
pub type SpareR = crate::FieldReader<u32>;
#[doc = "Field `SPARE` writer - 31:0\\]
Reserved.TI internal"]
pub type SpareW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved.TI internal"]
    #[inline(always)]
    pub fn spare(&self) -> SpareR {
        SpareR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved.TI internal"]
    #[inline(always)]
    #[must_use]
    pub fn spare(&mut self) -> SpareW<HwaSafetyParamRamErrLocSpec> {
        SpareW::new(self, 0)
    }
}
#[doc = "HWA_SAFETY_PARAM_RAM_ERR_LOC\n\nYou can [`read`](crate::Reg::read) this register and get [`hwa_safety_param_ram_err_loc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwa_safety_param_ram_err_loc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HwaSafetyParamRamErrLocSpec;
impl crate::RegisterSpec for HwaSafetyParamRamErrLocSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hwa_safety_param_ram_err_loc::R`](R) reader structure"]
impl crate::Readable for HwaSafetyParamRamErrLocSpec {}
#[doc = "`write(|w| ..)` method takes [`hwa_safety_param_ram_err_loc::W`](W) writer structure"]
impl crate::Writable for HwaSafetyParamRamErrLocSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HWA_SAFETY_PARAM_RAM_ERR_LOC to value 0"]
impl crate::Resettable for HwaSafetyParamRamErrLocSpec {
    const RESET_VALUE: u32 = 0;
}
