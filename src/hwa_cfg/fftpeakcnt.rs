#[doc = "Register `FFTPEAKCNT` reader"]
pub type R = crate::R<FftpeakcntSpec>;
#[doc = "Register `FFTPEAKCNT` writer"]
pub type W = crate::W<FftpeakcntSpec>;
#[doc = "Field `FFTPEAKCNT` reader - 11:0\\]
CFAR Detected Peak Count: This is a read-only register that contains the number of detected peaks that are logged in the destination memory, when CFAR Engine is configured in Detected Peaks List mode."]
pub type FftpeakcntR = crate::FieldReader<u16>;
#[doc = "Field `FFTPEAKCNT` writer - 11:0\\]
CFAR Detected Peak Count: This is a read-only register that contains the number of detected peaks that are logged in the destination memory, when CFAR Engine is configured in Detected Peaks List mode."]
pub type FftpeakcntW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `NU` reader - "]
pub type NuR = crate::FieldReader<u32>;
#[doc = "Field `NU` writer - "]
pub type NuW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
CFAR Detected Peak Count: This is a read-only register that contains the number of detected peaks that are logged in the destination memory, when CFAR Engine is configured in Detected Peaks List mode."]
    #[inline(always)]
    pub fn fftpeakcnt(&self) -> FftpeakcntR {
        FftpeakcntR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:31"]
    #[inline(always)]
    pub fn nu(&self) -> NuR {
        NuR::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:11 - 11:0\\]
CFAR Detected Peak Count: This is a read-only register that contains the number of detected peaks that are logged in the destination memory, when CFAR Engine is configured in Detected Peaks List mode."]
    #[inline(always)]
    #[must_use]
    pub fn fftpeakcnt(&mut self) -> FftpeakcntW<FftpeakcntSpec> {
        FftpeakcntW::new(self, 0)
    }
    #[doc = "Bits 12:31"]
    #[inline(always)]
    #[must_use]
    pub fn nu(&mut self) -> NuW<FftpeakcntSpec> {
        NuW::new(self, 12)
    }
}
#[doc = "FFTPEAKCNT\n\nYou can [`read`](crate::Reg::read) this register and get [`fftpeakcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fftpeakcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FftpeakcntSpec;
impl crate::RegisterSpec for FftpeakcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fftpeakcnt::R`](R) reader structure"]
impl crate::Readable for FftpeakcntSpec {}
#[doc = "`write(|w| ..)` method takes [`fftpeakcnt::W`](W) writer structure"]
impl crate::Writable for FftpeakcntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FFTPEAKCNT to value 0"]
impl crate::Resettable for FftpeakcntSpec {
    const RESET_VALUE: u32 = 0;
}
