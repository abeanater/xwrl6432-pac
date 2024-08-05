#[doc = "Register `HWACCREG13` reader"]
pub type R = crate::R<Hwaccreg13Spec>;
#[doc = "Register `HWACCREG13` writer"]
pub type W = crate::W<Hwaccreg13Spec>;
#[doc = "Field `CFAR_THRESH` reader - 17:0\\]
CFAR Threshold scale factor: This value is used to either multiply or add to the surrounding noise average to determine the threshold used for detection of the present cell under test. If logarithmic CFAR mode is disabled (i.e., in magnitude or magnitude-squared mode), then the register value is multiplied with the surrounding noise average to determine the threshold, else it is added to the surrounding noise average. In the former case, this 18-bit register is interpreted as a 14.4 value. In the latter case (i.e., logarithmic mode), the 18-bit register is interpreted as a 7.11 value."]
pub type CfarThreshR = crate::FieldReader<u32>;
#[doc = "Field `CFAR_THRESH` writer - 17:0\\]
CFAR Threshold scale factor: This value is used to either multiply or add to the surrounding noise average to determine the threshold used for detection of the present cell under test. If logarithmic CFAR mode is disabled (i.e., in magnitude or magnitude-squared mode), then the register value is multiplied with the surrounding noise average to determine the threshold, else it is added to the surrounding noise average. In the former case, this 18-bit register is interpreted as a 14.4 value. In the latter case (i.e., logarithmic mode), the 18-bit register is interpreted as a 7.11 value."]
pub type CfarThreshW<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
#[doc = "Field `NU` reader - "]
pub type NuR = crate::FieldReader<u16>;
#[doc = "Field `NU` writer - "]
pub type NuW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:17 - 17:0\\]
CFAR Threshold scale factor: This value is used to either multiply or add to the surrounding noise average to determine the threshold used for detection of the present cell under test. If logarithmic CFAR mode is disabled (i.e., in magnitude or magnitude-squared mode), then the register value is multiplied with the surrounding noise average to determine the threshold, else it is added to the surrounding noise average. In the former case, this 18-bit register is interpreted as a 14.4 value. In the latter case (i.e., logarithmic mode), the 18-bit register is interpreted as a 7.11 value."]
    #[inline(always)]
    pub fn cfar_thresh(&self) -> CfarThreshR {
        CfarThreshR::new(self.bits & 0x0003_ffff)
    }
    #[doc = "Bits 18:31"]
    #[inline(always)]
    pub fn nu(&self) -> NuR {
        NuR::new(((self.bits >> 18) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:17 - 17:0\\]
CFAR Threshold scale factor: This value is used to either multiply or add to the surrounding noise average to determine the threshold used for detection of the present cell under test. If logarithmic CFAR mode is disabled (i.e., in magnitude or magnitude-squared mode), then the register value is multiplied with the surrounding noise average to determine the threshold, else it is added to the surrounding noise average. In the former case, this 18-bit register is interpreted as a 14.4 value. In the latter case (i.e., logarithmic mode), the 18-bit register is interpreted as a 7.11 value."]
    #[inline(always)]
    #[must_use]
    pub fn cfar_thresh(&mut self) -> CfarThreshW<Hwaccreg13Spec> {
        CfarThreshW::new(self, 0)
    }
    #[doc = "Bits 18:31"]
    #[inline(always)]
    #[must_use]
    pub fn nu(&mut self) -> NuW<Hwaccreg13Spec> {
        NuW::new(self, 18)
    }
}
#[doc = "HWACCREG13\n\nYou can [`read`](crate::Reg::read) this register and get [`hwaccreg13::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwaccreg13::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hwaccreg13Spec;
impl crate::RegisterSpec for Hwaccreg13Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hwaccreg13::R`](R) reader structure"]
impl crate::Readable for Hwaccreg13Spec {}
#[doc = "`write(|w| ..)` method takes [`hwaccreg13::W`](W) writer structure"]
impl crate::Writable for Hwaccreg13Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HWACCREG13 to value 0"]
impl crate::Resettable for Hwaccreg13Spec {
    const RESET_VALUE: u32 = 0;
}
