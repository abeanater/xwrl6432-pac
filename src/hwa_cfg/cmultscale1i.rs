#[doc = "Register `CMULTSCALE1I` reader"]
pub type R = crate::R<Cmultscale1iSpec>;
#[doc = "Register `CMULTSCALE1I` writer"]
pub type W = crate::W<Cmultscale1iSpec>;
#[doc = "Field `CMULTSCALE1I` reader - 20:0\\]
In CMULT_MODE : 101 , the input samples are multiplied by a different complex scalar CMULTSCALE1I, CMULTSCALE1Q to CMULTSCALE6I, CMULTSCALE6Q per-iteration based on REG_BCNT. Else, a constant complex scalar CMULTSCALE1I and CMULTSCALEQI is applied to all sample across all iterations."]
pub type Cmultscale1iR = crate::FieldReader<u32>;
#[doc = "Field `CMULTSCALE1I` writer - 20:0\\]
In CMULT_MODE : 101 , the input samples are multiplied by a different complex scalar CMULTSCALE1I, CMULTSCALE1Q to CMULTSCALE6I, CMULTSCALE6Q per-iteration based on REG_BCNT. Else, a constant complex scalar CMULTSCALE1I and CMULTSCALEQI is applied to all sample across all iterations."]
pub type Cmultscale1iW<'a, REG> = crate::FieldWriter<'a, REG, 21, u32>;
#[doc = "Field `NU` reader - "]
pub type NuR = crate::FieldReader<u16>;
#[doc = "Field `NU` writer - "]
pub type NuW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:20 - 20:0\\]
In CMULT_MODE : 101 , the input samples are multiplied by a different complex scalar CMULTSCALE1I, CMULTSCALE1Q to CMULTSCALE6I, CMULTSCALE6Q per-iteration based on REG_BCNT. Else, a constant complex scalar CMULTSCALE1I and CMULTSCALEQI is applied to all sample across all iterations."]
    #[inline(always)]
    pub fn cmultscale1i(&self) -> Cmultscale1iR {
        Cmultscale1iR::new(self.bits & 0x001f_ffff)
    }
    #[doc = "Bits 21:31"]
    #[inline(always)]
    pub fn nu(&self) -> NuR {
        NuR::new(((self.bits >> 21) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:20 - 20:0\\]
In CMULT_MODE : 101 , the input samples are multiplied by a different complex scalar CMULTSCALE1I, CMULTSCALE1Q to CMULTSCALE6I, CMULTSCALE6Q per-iteration based on REG_BCNT. Else, a constant complex scalar CMULTSCALE1I and CMULTSCALEQI is applied to all sample across all iterations."]
    #[inline(always)]
    #[must_use]
    pub fn cmultscale1i(&mut self) -> Cmultscale1iW<Cmultscale1iSpec> {
        Cmultscale1iW::new(self, 0)
    }
    #[doc = "Bits 21:31"]
    #[inline(always)]
    #[must_use]
    pub fn nu(&mut self) -> NuW<Cmultscale1iSpec> {
        NuW::new(self, 21)
    }
}
#[doc = "CMULTSCALE1I\n\nYou can [`read`](crate::Reg::read) this register and get [`cmultscale1i::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmultscale1i::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cmultscale1iSpec;
impl crate::RegisterSpec for Cmultscale1iSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmultscale1i::R`](R) reader structure"]
impl crate::Readable for Cmultscale1iSpec {}
#[doc = "`write(|w| ..)` method takes [`cmultscale1i::W`](W) writer structure"]
impl crate::Writable for Cmultscale1iSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMULTSCALE1I to value 0"]
impl crate::Resettable for Cmultscale1iSpec {
    const RESET_VALUE: u32 = 0;
}
