#[doc = "Register `REG14` reader"]
pub type R = crate::R<Reg14Spec>;
#[doc = "Register `REG14` writer"]
pub type W = crate::W<Reg14Spec>;
#[doc = "Field `SUM_IFM` reader - 19:0\\]
Sum of GP ADC readings"]
pub type SumIfmR = crate::FieldReader<u32>;
#[doc = "Field `SUM_IFM` writer - 19:0\\]
Sum of GP ADC readings"]
pub type SumIfmW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `NU` reader - 31:20\\]
TI reserved"]
pub type NuR = crate::FieldReader<u16>;
#[doc = "Field `NU` writer - 31:20\\]
TI reserved"]
pub type NuW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:19 - 19:0\\]
Sum of GP ADC readings"]
    #[inline(always)]
    pub fn sum_ifm(&self) -> SumIfmR {
        SumIfmR::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bits 20:31 - 31:20\\]
TI reserved"]
    #[inline(always)]
    pub fn nu(&self) -> NuR {
        NuR::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:19 - 19:0\\]
Sum of GP ADC readings"]
    #[inline(always)]
    #[must_use]
    pub fn sum_ifm(&mut self) -> SumIfmW<Reg14Spec> {
        SumIfmW::new(self, 0)
    }
    #[doc = "Bits 20:31 - 31:20\\]
TI reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu(&mut self) -> NuW<Reg14Spec> {
        NuW::new(self, 20)
    }
}
#[doc = "Sum of GP ADC readings\n\nYou can [`read`](crate::Reg::read) this register and get [`reg14::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg14::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reg14Spec;
impl crate::RegisterSpec for Reg14Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg14::R`](R) reader structure"]
impl crate::Readable for Reg14Spec {}
#[doc = "`write(|w| ..)` method takes [`reg14::W`](W) writer structure"]
impl crate::Writable for Reg14Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REG14 to value 0"]
impl crate::Resettable for Reg14Spec {
    const RESET_VALUE: u32 = 0;
}
