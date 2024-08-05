#[doc = "Register `HWA_SAFETY_IPONG_ERR_LOC` reader"]
pub type R = crate::R<HwaSafetyIpongErrLocSpec>;
#[doc = "Register `HWA_SAFETY_IPONG_ERR_LOC` writer"]
pub type W = crate::W<HwaSafetyIpongErrLocSpec>;
#[doc = "Field `HWA_SAFETY_IPONG_ERR_ADDR` reader - 15:0\\]
\\[Debug \\]Address of parity error location within ACCEL_MEM1 (rows 0-1023)"]
pub type HwaSafetyIpongErrAddrR = crate::FieldReader<u16>;
#[doc = "Field `HWA_SAFETY_IPONG_ERR_ADDR` writer - 15:0\\]
\\[Debug \\]Address of parity error location within ACCEL_MEM1 (rows 0-1023)"]
pub type HwaSafetyIpongErrAddrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `NU` reader - "]
pub type NuR = crate::FieldReader<u16>;
#[doc = "Field `NU` writer - "]
pub type NuW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
\\[Debug \\]Address of parity error location within ACCEL_MEM1 (rows 0-1023)"]
    #[inline(always)]
    pub fn hwa_safety_ipong_err_addr(&self) -> HwaSafetyIpongErrAddrR {
        HwaSafetyIpongErrAddrR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn nu(&self) -> NuR {
        NuR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
\\[Debug \\]Address of parity error location within ACCEL_MEM1 (rows 0-1023)"]
    #[inline(always)]
    #[must_use]
    pub fn hwa_safety_ipong_err_addr(
        &mut self,
    ) -> HwaSafetyIpongErrAddrW<HwaSafetyIpongErrLocSpec> {
        HwaSafetyIpongErrAddrW::new(self, 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    #[must_use]
    pub fn nu(&mut self) -> NuW<HwaSafetyIpongErrLocSpec> {
        NuW::new(self, 16)
    }
}
#[doc = "HWA_SAFETY_IPONG_ERR_LOC\n\nYou can [`read`](crate::Reg::read) this register and get [`hwa_safety_ipong_err_loc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwa_safety_ipong_err_loc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HwaSafetyIpongErrLocSpec;
impl crate::RegisterSpec for HwaSafetyIpongErrLocSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hwa_safety_ipong_err_loc::R`](R) reader structure"]
impl crate::Readable for HwaSafetyIpongErrLocSpec {}
#[doc = "`write(|w| ..)` method takes [`hwa_safety_ipong_err_loc::W`](W) writer structure"]
impl crate::Writable for HwaSafetyIpongErrLocSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HWA_SAFETY_IPONG_ERR_LOC to value 0"]
impl crate::Resettable for HwaSafetyIpongErrLocSpec {
    const RESET_VALUE: u32 = 0;
}
