#[doc = "Register `CFAR_DET_THR` reader"]
pub type R = crate::R<CfarDetThrSpec>;
#[doc = "Register `CFAR_DET_THR` writer"]
pub type W = crate::W<CfarDetThrSpec>;
#[doc = "Field `CFAR_DET_THR` reader - 23:0\\]
This register is used to specify the threshold used for the detection of the present cell under test during CFAR-CA mode when number of samples for left side and right side noise averaging is 0."]
pub type CfarDetThrR = crate::FieldReader<u32>;
#[doc = "Field `CFAR_DET_THR` writer - 23:0\\]
This register is used to specify the threshold used for the detection of the present cell under test during CFAR-CA mode when number of samples for left side and right side noise averaging is 0."]
pub type CfarDetThrW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `NU` reader - "]
pub type NuR = crate::FieldReader;
#[doc = "Field `NU` writer - "]
pub type NuW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
This register is used to specify the threshold used for the detection of the present cell under test during CFAR-CA mode when number of samples for left side and right side noise averaging is 0."]
    #[inline(always)]
    pub fn cfar_det_thr(&self) -> CfarDetThrR {
        CfarDetThrR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn nu(&self) -> NuR {
        NuR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - 23:0\\]
This register is used to specify the threshold used for the detection of the present cell under test during CFAR-CA mode when number of samples for left side and right side noise averaging is 0."]
    #[inline(always)]
    #[must_use]
    pub fn cfar_det_thr(&mut self) -> CfarDetThrW<CfarDetThrSpec> {
        CfarDetThrW::new(self, 0)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    #[must_use]
    pub fn nu(&mut self) -> NuW<CfarDetThrSpec> {
        NuW::new(self, 24)
    }
}
#[doc = "CFAR_DET_THR\n\nYou can [`read`](crate::Reg::read) this register and get [`cfar_det_thr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfar_det_thr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfarDetThrSpec;
impl crate::RegisterSpec for CfarDetThrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfar_det_thr::R`](R) reader structure"]
impl crate::Readable for CfarDetThrSpec {}
#[doc = "`write(|w| ..)` method takes [`cfar_det_thr::W`](W) writer structure"]
impl crate::Writable for CfarDetThrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFAR_DET_THR to value 0"]
impl crate::Resettable for CfarDetThrSpec {
    const RESET_VALUE: u32 = 0;
}
