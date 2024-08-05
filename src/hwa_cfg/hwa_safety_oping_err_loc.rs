#[doc = "Register `HWA_SAFETY_OPING_ERR_LOC` reader"]
pub type R = crate::R<HwaSafetyOpingErrLocSpec>;
#[doc = "Register `HWA_SAFETY_OPING_ERR_LOC` writer"]
pub type W = crate::W<HwaSafetyOpingErrLocSpec>;
#[doc = "Field `HWA_SAFETY_OPING_ERR_ADDR` reader - 15:0\\]
\\[Debug \\]Address of parity error location within ACCEL_MEM2 (rows 0-1023)"]
pub type HwaSafetyOpingErrAddrR = crate::FieldReader<u16>;
#[doc = "Field `HWA_SAFETY_OPING_ERR_ADDR` writer - 15:0\\]
\\[Debug \\]Address of parity error location within ACCEL_MEM2 (rows 0-1023)"]
pub type HwaSafetyOpingErrAddrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `NU` reader - "]
pub type NuR = crate::FieldReader<u16>;
#[doc = "Field `NU` writer - "]
pub type NuW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
\\[Debug \\]Address of parity error location within ACCEL_MEM2 (rows 0-1023)"]
    #[inline(always)]
    pub fn hwa_safety_oping_err_addr(&self) -> HwaSafetyOpingErrAddrR {
        HwaSafetyOpingErrAddrR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn nu(&self) -> NuR {
        NuR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
\\[Debug \\]Address of parity error location within ACCEL_MEM2 (rows 0-1023)"]
    #[inline(always)]
    #[must_use]
    pub fn hwa_safety_oping_err_addr(
        &mut self,
    ) -> HwaSafetyOpingErrAddrW<HwaSafetyOpingErrLocSpec> {
        HwaSafetyOpingErrAddrW::new(self, 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    #[must_use]
    pub fn nu(&mut self) -> NuW<HwaSafetyOpingErrLocSpec> {
        NuW::new(self, 16)
    }
}
#[doc = "HWA_SAFETY_OPING_ERR_LOC\n\nYou can [`read`](crate::Reg::read) this register and get [`hwa_safety_oping_err_loc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwa_safety_oping_err_loc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HwaSafetyOpingErrLocSpec;
impl crate::RegisterSpec for HwaSafetyOpingErrLocSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hwa_safety_oping_err_loc::R`](R) reader structure"]
impl crate::Readable for HwaSafetyOpingErrLocSpec {}
#[doc = "`write(|w| ..)` method takes [`hwa_safety_oping_err_loc::W`](W) writer structure"]
impl crate::Writable for HwaSafetyOpingErrLocSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HWA_SAFETY_OPING_ERR_LOC to value 0"]
impl crate::Resettable for HwaSafetyOpingErrLocSpec {
    const RESET_VALUE: u32 = 0;
}
