#[doc = "Register `DFACNT0` reader"]
pub type R = crate::R<Dfacnt0Spec>;
#[doc = "Register `DFACNT0` writer"]
pub type W = crate::W<Dfacnt0Spec>;
#[doc = "Field `ADIMENSION_COUNT__NUMBER_1` reader - 22:0\\]
A-Dimension count. Number of bytes to be transferred infirst dimension."]
pub type AdimensionCount_Number1R = crate::FieldReader<u32>;
#[doc = "Field `ADIMENSION_COUNT__NUMBER_1` writer - 22:0\\]
A-Dimension count. Number of bytes to be transferred infirst dimension."]
pub type AdimensionCount_Number1W<'a, REG> = crate::FieldWriter<'a, REG, 23, u32>;
impl R {
    #[doc = "Bits 0:22 - 22:0\\]
A-Dimension count. Number of bytes to be transferred infirst dimension."]
    #[inline(always)]
    pub fn adimension_count__number_1(&self) -> AdimensionCount_Number1R {
        AdimensionCount_Number1R::new(self.bits & 0x007f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:22 - 22:0\\]
A-Dimension count. Number of bytes to be transferred infirst dimension."]
    #[inline(always)]
    #[must_use]
    pub fn adimension_count__number_1(&mut self) -> AdimensionCount_Number1W<Dfacnt0Spec> {
        AdimensionCount_Number1W::new(self, 0)
    }
}
#[doc = "Dst FIFO Set A-Count\n\nYou can [`read`](crate::Reg::read) this register and get [`dfacnt0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfacnt0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dfacnt0Spec;
impl crate::RegisterSpec for Dfacnt0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dfacnt0::R`](R) reader structure"]
impl crate::Readable for Dfacnt0Spec {}
#[doc = "`write(|w| ..)` method takes [`dfacnt0::W`](W) writer structure"]
impl crate::Writable for Dfacnt0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DFACNT0 to value 0"]
impl crate::Resettable for Dfacnt0Spec {
    const RESET_VALUE: u32 = 0;
}
